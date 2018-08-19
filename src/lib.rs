use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::net::TcpStream;
use std::error::Error;

use std::io::{BufReader, BufWriter};

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

    
    let stream = TcpStream::connect(config.host)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    read_server_response(&mut reader)?;

    login_to_server(&mut reader, &mut writer, "anonymous", "wft2000@gmail.com")?;

    loop {
        print!("rftp>");
        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        if command.trim() == "q" || command.trim() == "quit" {
            println!("bye!");
            break;
        }
    }

    Ok(())
}

fn login_to_server(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>,
    username: &str,
    password: &str) -> Result<(), std::io::Error> {

    let mut user_command: String = "USER ".to_owned();
    user_command.push_str(username);
    user_command.push('\n');
    send_command_to_server(writer, user_command.as_bytes())?;  // enter login
    read_server_response(reader)?;

    let mut pass_command: String = "PASS ".to_owned();
    pass_command.push_str(password);
    pass_command.push('\n');
    send_command_to_server(writer, pass_command.as_bytes())?;  // enter password
    read_server_response(reader)?;

    Ok(())
}

fn read_server_response(reader: &mut BufReader<&TcpStream>) -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{}", buffer.trim());
    Ok(())
}

fn send_command_to_server(writer: &mut BufWriter<&TcpStream>, command: &[u8]) -> Result<(), std::io::Error> {
    writer.write(command)?;
    writer.flush()?;
    Ok(())
}