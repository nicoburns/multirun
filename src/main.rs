use std::process::Stdio;
use std::io::{stdout, Write};

use futures::stream::{Stream, select_all};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

mod config;
use config::Config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::load()?;
    let max_service_name_length = config.max_service_name_length();

    let readers : Vec<_> = config.services.iter().flat_map(move |(name, service)| {

        let command_parts = shellwords::split(&service.command).unwrap();

        let mut command = Command::new(command_parts[0].clone());
        command.args(command_parts.iter().skip(1));
        command.envs(&service.environment);
        command.kill_on_drop(true);
        
        if let Some(dir) = &service.directory {
            command.current_dir(dir.clone());
        }

        // Specify that we want the command's standard output piped back to us, not inherited from our process
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().expect("failed to spawn command");
        let stdout = child.stdout().take().expect("child did not have a handle to stdout");
        let stderr = child.stderr().take().expect("child did not have a handle to stderr");
        let stdout_reader = BufReader::new(stdout).lines();
        let stderr_reader = BufReader::new(stderr).lines();

        // Ensure the child process is spawned in the runtime so it can
        // make progress on its own while we await for any output.
        tokio::spawn(async {
            let status = child.await.expect("child process encountered an error");
            println!("child status was: {}", status);
        });

        let stdout_prefix = format!("{:width$} | ", name.to_uppercase(), width=max_service_name_length);
        let stderr_prefix = stdout_prefix.clone();
        return vec![
            Box::new(stdout_reader.map(move |line_result| line_result.map(|line| format!("{}{}", stderr_prefix, line)))) as Box<dyn Stream<Item=Result<String, _>> + Unpin>,
            Box::new(stderr_reader.map(move |line_result| line_result.map(|line| format!("{}{}", stdout_prefix, line)))) as Box<dyn Stream<Item=Result<String, _>> + Unpin>,
        ];
    }).collect();

    let line_stream = select_all(readers);
    pin_mut!(line_stream); // needed for iteration

    let stdout = stdout();
    let mut handle = stdout.lock();

    while let Some(line) = line_stream.next().await {
        writeln!(handle, "{}", line?)?;
    }

    Ok(())
}