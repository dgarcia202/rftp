extern crate rftp;

use std::env;
use std::process;

use rftp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "h" || args[1] == "help" {
        show_usage();
        process::exit(0)        
    }

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        show_usage();
        process::exit(0)
    });

    if let Err(err) = rftp::run(config) {
        println!("Application error: {}", err);
        process::exit(1)
    }
}

fn show_usage() {
    println!("Usage:");
    println!("\trftp <host name or IP>:<port>");
    println!("\tNote: port is mandatory even if it is the default 21.");
    println!("\trftp h or rftp help");
    println!("\tShows this help.");
}