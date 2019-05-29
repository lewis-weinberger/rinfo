use std::env;
use std::process;

use rinfo;
use rinfo::Config;
use rinfo::Output;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    match config.output {
        Output::Help => {
            rinfo::print_help();
            process::exit(0);
        },
        _ => {
            if let Err(e) = rinfo::run(config) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
    }
}
