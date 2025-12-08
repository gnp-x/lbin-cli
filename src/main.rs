use anyhow::Result;
use clap::{ArgGroup, CommandFactory, Parser};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(about = r#"
 ████  █████      ███
▒▒███ ▒▒███      ▒▒▒
 ▒███  ▒███████  ████  ████████
 ▒███  ▒███▒▒███▒▒███ ▒▒███▒▒███
 ▒███  ▒███ ▒███ ▒███  ▒███ ▒███
 ▒███  ▒███ ▒███ ▒███  ▒███ ▒███
 █████ ████████  █████ ████ █████
 ▒▒▒▒▒ ▒▒▒▒▒▒▒▒  ▒▒▒▒▒ ▒▒▒▒ ▒▒▒▒▒
  "#)]
#[command(override_usage("lbin -l <LBIN_AUTH> [OPTIONS] <INPUT>..."))]
#[command(
    author,
    version,
    help_template("{about}\n({version}) - {author}\n\n{usage-heading} {usage}\n\n{all-args}")
)]
#[command(group = ArgGroup::new("mode").required(true).multiple(false))]
struct Args {
    /// Not required if you export LBIN_AUTH=<token>.
    #[arg(short, long, env("LBIN_AUTH"), hide_env(true))]
    lbin_auth: String,
    /// INPUT
    #[arg(value_name("INPUT"), required(true))]
    input: Vec<String>,
    /// Upload a file
    #[arg(short, long, group("mode"))]
    file: bool,
    /// One-time use URL for an uploaded file
    #[arg(short('F'), long, group("mode"))]
    oneshot_file: bool,
    /// URL shortener
    #[arg(short, long, group("mode"))]
    url: bool,
    /// Make a one-time use URL
    #[arg(short('U'), long, group("mode"))]
    oneshot_url: bool,
    /// Link a file from a remote URL
    #[arg(short, long, group("mode"))]
    remote_url: bool,
    /// Command-line input to file upload.
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

    let result = result_formatter(&args, input);
    let header_auth = format!("Authorization: {}", args.lbin_auth);
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

fn result_formatter(args: &Args, input: String) -> String {
    if args.url {
        format!("url={}", input)
    } else if args.oneshot_url {
        format!("oneshot_url={}", input)
    } else if args.remote_url {
        format!("remote={}", input)
    } else if args.file {
        format!("file=@{}", input)
    } else if args.oneshot_file {
        format!("oneshot=@{}", input)
    } else if args.std_input {
        format!("file=@temp_input")
    } else {
        eprintln!("{}", "Invalid command or input.");
        std::process::exit(1)
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
