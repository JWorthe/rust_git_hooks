use std::env;
use std::io::{stdin, BufRead};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    println!("pre-commit called with {:?}", args);

    println!("BEGIN STDIN for pre-commit");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", line);
    }
    println!("END STDIN");
}
