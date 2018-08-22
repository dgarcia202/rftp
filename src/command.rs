extern crate rpassword;
extern crate regex;

use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::error::Error;
use regex::Regex;

pub use ::core;

pub fn login_to_server(
    reader: &mut BufReader<&TcpStream>, 
    writer: &mut BufWriter<&TcpStream>) -> Result<(), io::Error> {

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

pub fn list(
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

pub fn enter_passive_mode(
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

pub fn read_server_response(reader: &mut BufReader<&TcpStream>) -> Result<String, io::Error> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{}", buffer.trim());
    Ok(buffer)
}

pub fn send_command_to_server(writer: &mut BufWriter<&TcpStream>, command: &str) -> Result<(), io::Error> {
    writer.write(command.as_bytes())?;
    writer.flush()?;
    Ok(())
}