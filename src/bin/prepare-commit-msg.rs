extern crate rust_git_hooks;
use rust_git_hooks::*;

use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::process;
use std::env;

fn main() {
    log();
    
    let commit_filename = env::args().nth(1);
    let commit_source = env::args().nth(2);
    
    let current_branch = get_current_branch();
    
    match (current_branch, commit_filename, commit_source) {
        (Ok(branch), Some(filename), None) => {
            let write_result = prepend_branch_name(branch, filename);
            match write_result {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to prepend message. {}", e);
                    process::exit(2);
                }
            };
        },
        (_, _, Some(_)) => {
            // do nothing silently. This comes up on merge commits,
            // ammendment commits, if a message was specified on the
            // cli.
        }
        (Err(e), _, _) => {
            eprintln!("Failed to find current branch. {}", e);
            process::exit(1);
        },
        (_, None, _) => {
            eprintln!("Commit file was not provided");
            process::exit(2);
        }
    }
}

fn prepend_branch_name(branch_name: String, commit_filename: String) -> Result<(), std::io::Error> {
    // It turns out that prepending a string to a file is not an
    // obvious action. You can only write to the end of a file :(
    //
    // The solution is to read the existing contents, then write a new
    // file starting with the branch name, and then writing the rest
    // of the file.
    
    let mut read_commit_file = File::open(commit_filename.clone())?;
    let mut current_message = String::new();
    read_commit_file.read_to_string(&mut current_message)?;
    
    let mut commit_file = File::create(commit_filename)?;

    writeln!(commit_file, "{}:", branch_name)?;
    write!(commit_file, "{}", current_message)
}
