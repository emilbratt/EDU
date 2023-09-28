use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // print to stderr
        process::exit(1);
    });

    println!("Searching for {}", config.query); // print to stdout
    println!("In file {}", config.file_path); // print to stdout

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}"); // print to stderr
        process::exit(1);
    }
}
