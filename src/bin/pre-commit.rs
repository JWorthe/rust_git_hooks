extern crate rust_git_hooks;
use rust_git_hooks::*;

use std::env;
use std::io::{stdin, BufRead};
use std::process;
use std::process::{Command, Stdio};

fn main() {
    log();

    let command = Command::new("cargo")
        .arg("test")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    process::exit(command.status.code().unwrap_or(0));
}
