extern crate rpassword;
extern crate regex;

use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::net::TcpStream;
use std::error::Error;
use std::io::{BufReader, BufWriter};
use regex::Regex;

pub mod core;

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
    
    let regex_cwd = Regex::new(r"^cd (\S+)$")?;

    let stream = TcpStream::connect(&config.host)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    println!("Connected to {}", &config.host);

    login_to_server(&mut reader, &mut writer)?;    

    loop {
        print!("rftp>");
        io::stdout().flush()?;

        let mut user_command = String::new();
        io::stdin().read_line(&mut user_command)?;
        let user_command = user_command.trim();

        if user_command == "q" || user_command == "quit" {
            send_command_to_server(&mut writer, "QUIT\r\n")?;
            read_server_response(&mut reader)?;
            break;
        }

        if user_command == "pwd" {
            send_command_to_server(&mut writer, "PWD\r\n")?;
            read_server_response(&mut reader)?;
            continue;
        }

        if regex_cwd.is_match(user_command) {
            let caps = regex_cwd.captures(user_command).unwrap();
            let mut cwd_command = "CWD ".to_owned();
            cwd_command.push_str(&caps[1]);
            cwd_command.push_str("\r\n");
            send_command_to_server(&mut writer, &mut cwd_command)?;
            read_server_response(&mut reader)?;
            continue;
        }

        if user_command == "list" || user_command == "ls" {
            list(&mut reader, &mut writer)?;
            continue;
        }

        println!("Invalid command");
    }

    Ok(())
}

fn login_to_server(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>) -> Result<(), std::io::Error> {

    // Show server's welcome message.
    read_server_response(reader)?;

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

fn list(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>) -> Result<(), Box<dyn Error>> {
    
    let (ip_addr, port) = enter_passive_mode(reader, writer)?;

    // Build host connection string.
    let mut host = ip_addr.clone();
    host.push(':');
    host.push_str(&port.to_string());

    // Open data stream.
    let mut data_stream = TcpStream::connect(host)?;

    // Send list command
    send_command_to_server(writer, "LIST\r\n")?;
    read_server_response(reader)?;

    // Read result in data connection
    let mut buffer = String::new();
    data_stream.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    read_server_response(reader)?;

    Ok(())
}

fn enter_passive_mode(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>) -> Result<(String, u32), Box<dyn Error>> {

    let regex_pasv = Regex::new(r"^227 Entering Passive Mode \(([0-9]+),([0-9]+),([0-9]+),([0-9]+),([0-9]+),([0-9]+)\)$")?;

    send_command_to_server(writer, "PASV\r\n")?;
    let response = read_server_response(reader)?;
    let response = response.trim();

    if regex_pasv.is_match(&response) {

        let caps = regex_pasv.captures(&response).unwrap();

        let ip_addr = format!("{}.{}.{}.{}", &caps[1], &caps[2], &caps[3], &caps[4]);

        let p1 = &caps[5].parse::<u32>()?;
        let p2 = &caps[6].parse::<u32>()?;
        let port = (p1 * 256) + p2;
        
        return Ok((ip_addr, port));
    }
    
    Err(Box::new(core::RftpError("Server response couldn't be parsed".into())))
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