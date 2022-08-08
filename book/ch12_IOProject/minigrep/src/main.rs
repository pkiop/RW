use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // output = ["target/debug/minigrep", "searchstring", "example-filename.txt"]
    // output[0] = name of binary

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {}", e);
        process::exit(1)
    }
}

//separate logic
// main.rs : main interface logic
// lib.rs : program logic
