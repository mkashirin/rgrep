use std::env;
use std::process;

use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    eprintln!(
        "Searching for {} in file {}",
        config.query, config.file_name
    );

    if let Err(err) = rgrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    };
}
