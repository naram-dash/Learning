use std::env;
use std::process;

// mod lib;
// use lib::Config;
use minigrep::Config;

fn main() {
    // let args = env::args().collect::<Vec<String>>();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

