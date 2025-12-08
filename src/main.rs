use anyhow::{Result, bail};
use clap::Parser;
use regex::Regex;
use std::process::Command;

/// lbin: CLI tool for bin.liminal.cafe
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Not required if you run export AUTH_TOKEN=<token_here> in the terminal.
    #[arg(short, long, env("AUTH_TOKEN"), hide_env(true))]
    auth_token: String,
    /// INPUT
    #[arg(value_name("INPUT"), required(true))]
    input: String,
    /// Use file option
    #[arg(short, long)]
    file: bool,
    /// Use oneshot_file option
    #[arg(short('o'), long)]
    oneshot_file: bool,
    /// Use url option
    #[arg(short, long)]
    url: bool,
    /// Use oneshot_url option
    #[arg(short('O'), long)]
    oneshot_url: bool,
    /// Use remote_url option
    #[arg(short, long)]
    remote_url: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let url_prefix = Regex::new(r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)$").unwrap();

    let result = match (
        url_prefix.is_match(args.input.as_str()),
        args.oneshot_url,
        args.remote_url,
        args.url,
        args.file,
        args.oneshot_file,
    ) {
        (true, true, false, false, false, false) => format!("oneshot_url={}", args.input),
        (true, false, true, false, false, false) => format!("remote={}", args.input),
        (true, false, false, true, false, false) => format!("url={}", args.input),
        (false, false, false, false, true, false) => format!("file=@{}", args.input),
        (false, false, false, false, false, true) => format!("oneshot=@{}", args.input),
        _ => bail!("Not valid input."),
    };

    let header_auth = format!("Authorization: {}", args.auth_token);
    let server_address = "https://bin.liminal.cafe";

    Command::new("curl")
        .args([
            "-F",
            result.as_str(),
            "-H",
            header_auth.as_str(),
            server_address,
        ])
        .status()
        .expect("Failed to run command.");

    Ok(())
}
