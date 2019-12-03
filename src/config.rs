use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;
use std::env;
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME : &str = "multirun.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub directory: Option<String>,
    pub command: String,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub services: BTreeMap<String, Service>
}

impl Config {

    pub fn max_service_name_length (&self) -> usize {
        self.services.keys().map(|name| name.len()).max().unwrap_or(0)
    }

    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {

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
    
        Ok(config)
    }
}