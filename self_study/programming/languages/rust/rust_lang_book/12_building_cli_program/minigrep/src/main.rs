use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for '{}' in {}", &config.query, &config.file_path);
    if config.ignore_case {
        println!(" with case sensitive on.\n");
    }
    else {
        println!(" with case sensitive off.\n");
    }

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {e}"); // print to stderr
        process::exit(1);
    }
}
