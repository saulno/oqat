use std::{env, process};

use oqat::{models::config::Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("There's been a problem reading the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Execution error: {}", e);
        process::exit(1);
    }
}
