use ::minigrep::Config;
use ::minigrep::run;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}


