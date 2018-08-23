extern crate regex;

use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::io::{BufReader, BufWriter};
use std::error::Error;
use regex::Regex;

pub mod core;
pub mod command;

use command::*;

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
    let regex_get = Regex::new(r"^get (\S+)$")?;

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

        if user_command == "" {
            continue;
        }

        if user_command == "h" || user_command == "help" {
            show_help();
            continue;
        }

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

        if regex_get.is_match(user_command) {
            let caps = regex_get.captures(user_command).unwrap();
            get_file(&mut reader, &mut writer, &caps[1])?;
            continue;
        }

        println!("Invalid command");
    }

    Ok(())
}

fn show_help() {
    println!("\th | help: Shows this help.");
    println!("\tpwd: shows current remote directory.");
    println!("\tcd <directory>: Moves current directory to the specified one.");
    println!("\tls: List contents of the current directory.");
    println!("\tq | quit: Ends remote session.");
}