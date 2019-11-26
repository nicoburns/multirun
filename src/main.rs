use std::process::Stdio;
use std::io::{stdout, Write};

use futures::stream::select_all;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::new("yes");
    cmd1.arg("foo");

    let mut cmd2 = Command::new("yes");
    cmd2.arg("bar");

    let mut cmd3 = Command::new("yes");
    cmd3.arg("baz");

    let cmds = vec![("FOO PROCESS", cmd1), ("BAR PROCESS", cmd2), ("BAZ PROCESS", cmd3)];

    let readers : Vec<_> = cmds.into_iter().map(|(name, mut cmd)| {

        // Specify that we want the command's standard output piped back to us, not inherited from our process
        cmd.stdout(Stdio::piped());

        let mut child = cmd.spawn().expect("failed to spawn command");
        let stdout = child.stdout().take().expect("child did not have a handle to stdout");
        let reader = BufReader::new(stdout).lines();

        // Ensure the child process is spawned in the runtime so it can
        // make progress on its own while we await for any output.
        tokio::spawn(async {
            let status = child.await.expect("child process encountered an error");
            println!("child status was: {}", status);
        });

        reader.map(move |line_result| line_result.map(|line| format!("{}: {}", name, line)))
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