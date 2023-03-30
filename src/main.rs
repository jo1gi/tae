mod args;
mod config;
mod find_command;
mod format;

use structopt::StructOpt;
use std::process::{Command, Stdio};
use base64::Engine;

use thiserror::Error;
use displaydoc::Display;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("{}", e)
    }
}

fn run() -> Result<(), Error> {
    let args = args::Arguments::from_args();
    let config = config::load_config(&args.config)?;
    for url_str in args.urls {
        let url = parse_url(&url_str)
            .ok_or_else(|| Error::ParseUrl(url_str.to_string()))?;
        let (new_url, raw_command) = find_command::find_command(url, &config)?;
        let command = format::format_command(&new_url, &raw_command);
        println!("Command: {command:?}");
        if !args.dry {
            run_command(&command, args.hide_output)?;
        }
    }
    Ok(())
}

/// Parse url
fn parse_url(url_str: &str) -> Option<url::Url> {
    match url::Url::parse(url_str) {
        Ok(url) => Some(url),
        // Try decode with base64
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let bytes: Vec<u8> = base64::engine::general_purpose::STANDARD.decode(url_str).ok()?;
            let url_str = std::str::from_utf8(&bytes).ok()?;
            url::Url::parse(url_str).ok()
        },
        _ => None
    }
}

/// Run system command
fn run_command(command: &Vec<String>, hide_output: bool) -> Result<(), Error> {
    Command::new(&command[0])
        .args(&command[1..])
        .stdout( if hide_output { Stdio::null() } else { Stdio::inherit() })
        .spawn()
        .map_err(|_| Error::ExecuteCommand)?
        .wait()
        .map_err(|_| Error::CommandFinish)?;
    Ok(())
}

#[derive(Error, Display, Debug)]
pub enum Error {
    /// Failed to parse url: {0}
    ParseUrl(String),
    /// Invalid url on redirect
    InvalidRedirect,
    /// No rule found
    NoRuleFound,
    /// Could not parse config
    ParseConfig,
    /// Could not find config file
    ConfigNotFound,
    /// Could not execute command
    ExecuteCommand,
    /// Failed to wait for command to finish
    CommandFinish,
}
