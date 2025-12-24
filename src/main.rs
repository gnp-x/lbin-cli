use anyhow::Result;
use clap::{ArgGroup, CommandFactory, Parser};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(about = "lbin-cli")]
#[command(override_usage("lbin-cli -l <LBIN_AUTH> [OPTIONS] <INPUT>..."))]
#[command(
    author,
    version,
    help_template("{about} - {version}\n{usage-heading} {usage}\n\n{all-args}")
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
    let header_auth = format!("Authorization: Bearer {}", args.lbin_auth);
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
    if args.file {
        format!("file=@{}", input)
    } else if args.std_input {
        format!("file=@temp_input.txt")
    } else {
        eprintln!("{}", "Invalid command or input.");
        std::process::exit(1)
    }
}

fn write_to_file(input: &str) -> std::io::Result<()> {
    let mut file = File::create("temp_input.txt")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}

fn delete_file() -> std::io::Result<()> {
    fs::remove_file("temp_input.txt")?;
    Ok(())
}
