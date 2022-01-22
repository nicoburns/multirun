use std::collections::{HashMap, BTreeMap};
use std::convert::AsRef;
use std::path::{PathBuf, Path};
use path_clean::{PathClean};
use std::fs;
use std::env;
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME : &str = "multirun.json";

fn compute_relative_path(base_path: impl AsRef<Path>, input: impl AsRef<str>) -> PathBuf {
    let mut path = base_path.as_ref().to_owned();
    path.pop();
    path.push(&input.as_ref());
    path = path.clean();
    path
}

fn compute_relative_path_string(base_path: impl AsRef<Path>, input: impl AsRef<str>) -> String {
    compute_relative_path(base_path, input).to_string_lossy().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub directory: Option<String>,
    pub command: String,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub paths: Option<BTreeMap<String, String>>,
    pub services: BTreeMap<String, Service>,
}

impl Config {
    pub fn max_service_name_length (&self) -> usize {
        self.services.keys().map(|name| name.len()).max().unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub config: Config,
    pub config_file_path: PathBuf,
}

impl ConfigFile {
    pub fn load() -> Result<ConfigFile, Box<dyn std::error::Error>> {

        let config_file_path = env::current_dir()?.ancestors()
            .find_map::<PathBuf, _>(|dir| {
                for entry in dir.read_dir().ok()? {
                    let entry = entry.ok()?;
                    if entry.file_name() == CONFIG_FILE_NAME { return Some(entry.path()); }
                }
                None
            })
            .ok_or("Config file not found")?;
    
        let config : Config = serde_json::from_str(fs::read_to_string(&config_file_path)?.as_str())?;
        let mut config_file = ConfigFile { config, config_file_path: config_file_path.clone() };

        // Resolve relative paths in paths map (relative to config file path)
        config_file.config.paths = Some(
            config_file.config.paths.clone().unwrap_or_else(|| BTreeMap::new()).into_iter().map(|(name, input_path)| {
                let resolved_path = compute_relative_path_string(&config_file_path, &input_path);
                (name, resolved_path)
            }).collect()
        );

        // Compile HashMap of variables to substitute
        let replacement_variables: HashMap<String, String> = config_file.config.paths
            .clone().unwrap()
            .into_iter().map(|(name, value)| (format!("paths.{name}"), value))
            .collect();
        
        // Normalize service definition
        for (_, service) in config_file.config.services.iter_mut() {

            // Apply variable subsitution and relative path resolution to directory
            service.directory = service.directory.take().map(|dir| {
                let replaced_dir = envsubst::substitute(dir, &replacement_variables).unwrap();
                compute_relative_path_string(&config_file_path, replaced_dir)
            });

            // Apply variable subsitution to env vars
            for (_, env_var) in service.environment.iter_mut() {
                *env_var = envsubst::substitute(env_var.clone(), &replacement_variables).unwrap();
            }
        }

        Ok(config_file)
    }
}