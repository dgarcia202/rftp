extern crate rpassword;

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
    
    let stream = TcpStream::connect(&config.host)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    println!("Connected to {}", &config.host);

    // Show server's welcome message.
    read_server_response(&mut reader)?;

    login_to_server(&mut reader, &mut writer)?;    

    loop {
        print!("rftp>");
        io::stdout().flush()?;

        let mut user_command = String::new();
        io::stdin().read_line(&mut user_command)?;

        if user_command.trim() == "q" || user_command.trim() == "quit" {
            send_command_to_server(&mut writer, "QUIT\r\n")?;
            read_server_response(&mut reader)?;
            break;
        }

        if user_command.trim() == "pwd" {
            send_command_to_server(&mut writer, "PWD\r\n")?;
            read_server_response(&mut reader)?;
            continue;
        }

        println!("Invalid command");
    }

    Ok(())
}

fn login_to_server(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>) -> Result<(), std::io::Error> {

    // Obtain username from console prompt.
    print!("User:");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    // Build ftp USER command.
    let mut user_command: String = "USER ".to_owned();
    user_command.push_str(&mut username);

    // Send command to server, show response.
    send_command_to_server(writer, &mut user_command)?;  // enter login
    read_server_response(reader)?;

    // Obtain password from prompt.
    print!("Password:");
    io::stdout().flush()?;
    let mut password = rpassword::read_password()?;
    password.push_str("\r\n");    // Ftp protocol need line feed and carriage return that rpassword is not providing.

    // Build ftp PASS command.
    let mut pass_command: String = "PASS ".to_owned();
    pass_command.push_str(&mut password);

    // Send command to server and show response.
    send_command_to_server(writer, &mut pass_command)?;  // enter password
    read_server_response(reader)?;

    Ok(())
}

fn read_server_response(reader: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{}", buffer.trim());
    Ok(buffer)
}

fn send_command_to_server(writer: &mut BufWriter<&TcpStream>, command: &str) -> Result<(), std::io::Error> {
    writer.write(command.as_bytes())?;
    writer.flush()?;
    Ok(())
}