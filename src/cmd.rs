use std::io;
use std::io::BufRead;
use std::process::Command;

pub(crate) fn exec(args: Vec<String>) -> Result<Vec<String>, io::Error> {
    // The idea is to collect both stdout and stderr in one writer
    let (reader, writer) = io::pipe().expect("failed to build pipe");

    let mut cargo = Command::new("cargo")
        .arg("test")
        .args(args)
        .stdout(writer.try_clone().expect("failed to clone pipe"))
        .stderr(writer)
        .spawn()
        .expect("failed to execute child");

    cargo.wait().expect("failed to execute command");

    let mut lines = vec![];
    for line in std::io::BufReader::new(reader).lines() {
        lines.push(line?);
    }

    Ok(lines)
}
