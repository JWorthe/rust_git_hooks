extern crate git2;

use std::env;
use std::io::{stdin, BufRead};

use git2::Repository;


pub fn log() {
    let name_arg = env::args().nth(0).unwrap_or(String::from("unknown"));
    let args: Vec<_> = env::args().skip(1).collect();
    println!("{} called with {:?}", name_arg, args);

    println!("BEGIN STDIN");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", line);
    }
    println!("END STDIN");
}

pub fn get_current_branch() -> Result<String, git2::Error> {
    let git_repo = Repository::discover("./")?;
    let head = git_repo.head()?;
    let head_name =  head.shorthand();
    match head_name {
        Some(name) => Ok(name.to_string()),
        None => Err(git2::Error::from_str("No branch name found"))
    }
}

