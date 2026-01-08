use anyhow::Result;
use clap::Parser;
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
struct Args {
    /// Not required if you export LBIN_AUTH=<token>.
    #[arg(short, long, env("LBIN_AUTH"), hide_env(true))]
    lbin_auth: String,
    /// INPUT
    #[arg(value_name("INPUT"), required(true))]
    input: Vec<String>,
    /// Upload a file
    #[arg(short, long)]
    file: bool,
    /// Command-line input to upload.
    #[arg(short('i'), long, conflicts_with("file"))]
    std_input: bool,
    /// How many minutes until file expires.
    #[arg(short, long)]
    time: Option<u64>,
    /// Upload a file that can only be seen once.
    #[arg(short, long, conflicts_with("time"))]
    oneshot: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = args.input.join(" ");
    if args.std_input {
        write_to_file(&input)?
    };

    let result = result_formatter(&args, input);

    Command::new("curl")
        .args(result)
        .status()
        .expect("Failed to run command.");

    if args.std_input {
        delete_file()?
    }
    Ok(())
}

fn result_formatter(args: &Args, input: String) -> Vec<String> {
    let header_auth = format!("Authorization: Bearer {}", args.lbin_auth);
    let server_address = "https://bin.liminal.cafe";
    let mut result_vector = vec!["-F".to_owned()];
    if args.file {
        result_vector.push(format!("file=@{input}"))
    } else if args.std_input {
        result_vector.push("file=@temp_input.txt".to_owned())
    } else {
        eprintln!("{}", "Invalid command or input.");
        std::process::exit(1)
    }
    if let Some(n) = &args.time {
        result_vector.push("-F".to_owned());
        result_vector.push(format!("time={n}"))
    }
    result_vector.push("-H".to_owned());
    result_vector.push(header_auth);
    if args.oneshot {
        result_vector.push(format!("{}/o", server_address.to_owned()))
    } else {
        result_vector.push(server_address.to_owned());
    }
    result_vector
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
