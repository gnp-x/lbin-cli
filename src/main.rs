use anyhow::Result;
use clap::{ArgGroup, CommandFactory, Parser};
use colored::Colorize;
use regex::Regex;
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, group = ArgGroup::new("mode").required(true).multiple(false))]
#[command(about = r#"
 ████  █████      ███
▒▒███ ▒▒███      ▒▒▒
 ▒███  ▒███████  ████  ████████
 ▒███  ▒███▒▒███▒▒███ ▒▒███▒▒███
 ▒███  ▒███ ▒███ ▒███  ▒███ ▒███
 ▒███  ▒███ ▒███ ▒███  ▒███ ▒███
 █████ ████████  █████ ████ █████
▒▒▒▒▒ ▒▒▒▒▒▒▒▒  ▒▒▒▒▒ ▒▒▒▒ ▒▒▒▒▒
- sakura <mail.liminal.pm.me>
  "#)]
struct Args {
    /// Not required if you run export AUTH_TOKEN=<token_here> in the terminal.
    #[arg(short, long, env("AUTH_TOKEN"), hide_env(true))]
    auth_token: String,
    /// INPUT
    #[arg(value_name("INPUT"), required(true))]
    input: Vec<String>,
    /// Upload a file
    #[arg(short, long, group("mode"))]
    file: bool,
    /// One-time use URL for an uploaded file
    #[arg(short('o'), long, group("mode"))]
    oneshot_file: bool,
    /// URL shortener
    #[arg(short, long, group("mode"))]
    url: bool,
    /// Make a one-time use URL
    #[arg(short('O'), long, group("mode"))]
    oneshot_url: bool,
    /// Link a file from a remote URL
    #[arg(short, long, group("mode"))]
    remote_url: bool,
    /// Upload input. Note: You may need to use quotes around your input.
    #[arg(short('i'), long, group("mode"))]
    std_input: bool,
}

fn main() -> Result<()> {
    if std::env::args().len() == 1 {
        Args::command().print_help()?;
        std::process::exit(0);
    }

    let args = Args::parse();
    let input = args.input.join(" ");
    if args.std_input {
        write_to_file(&input)?;
    };

    let url_prefix = Regex::new(r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)$").unwrap();

    let is_url = url_prefix.is_match(&input);
    let result = result_formatter(&args, is_url, input);
    let header_auth = format!("Authorization: {}", args.auth_token);
    let server_address = "https://bin.liminal.cafe";

    Command::new("curl")
        .args(["-F", &result, "-H", &header_auth, server_address])
        .status()
        .expect("Failed to run command.");

    if args.std_input {
        delete_file()?
    }
    Ok(())
}

fn result_formatter(args: &Args, is_url: bool, input: String) -> String {
    if is_url {
        if args.url {
            format!("url={}", input)
        } else if args.oneshot_url {
            format!("oneshot_url={}", input)
        } else if args.remote_url {
            format!("remote={}", input)
        } else {
            eprintln!("{}", "Invalid command or input.".bright_red().bold());
            std::process::exit(1)
        }
    } else {
        if args.file {
            format!("file=@{}", input)
        } else if args.oneshot_file {
            format!("oneshot=@{}", input)
        } else if args.std_input {
            format!("file=@temp_input")
        } else {
            eprintln!("{}", "Invalid command or input.".bright_red().bold());
            std::process::exit(1)
        }
    }
}

fn write_to_file(input: &str) -> std::io::Result<()> {
    let mut file = File::create("temp_input")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}

fn delete_file() -> std::io::Result<()> {
    fs::remove_file("temp_input")?;
    Ok(())
}
