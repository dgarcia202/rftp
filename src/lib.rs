use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::net::TcpStream;
use std::error::Error;

pub struct Config {
    pub host: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        }

        let host = args[1].clone();
        Ok(Config { host })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut stream = TcpStream::connect(config.host)?;

    let mut buffer = [0; 128];
    stream.read(&mut buffer)?;
    println!("{}", String::from_utf8(buffer.to_vec()).unwrap());

    loop {
        print!("rftp>");
        io::stdout().flush().expect("Problem writing output!");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        if command.trim() == "q" || command.trim() == "quit" {
            println!("bye!");
            break;
        }
    }

    Ok(())
}