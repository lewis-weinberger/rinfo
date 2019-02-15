use std::env;
use std::process;

use rinfo;
use rinfo::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = rinfo::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
