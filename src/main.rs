extern crate rftp;

use std::env;
use std::process;

use rftp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(err) = rftp::run(config) {
        println!("Application error: {}", err);
        process::exit(1)
    }
}
