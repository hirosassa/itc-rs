#![allow(clippy::use_self)]
mod fetch;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{debug, error};
use std::io::{self, Read};
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,

    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Fetch,
    Inspect,
}

#[async_std::main]
async fn main() {
    if let Err(err) = run().await {
        error!("Error: {:#?}", err);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    debug!("verbose mode");

    match cli.subcommand {
        SubCommands::Fetch => run_fetch().await,
        SubCommands::Inspect => run_inspect(),
    }
}

async fn run_fetch() -> Result<()> {
    fetch::fetch_db_info().await?;
    debug!("fetched DB info");
    Ok(())
}

fn run_inspect() -> Result<()> {
    let mut body = String::new();
    io::stdin().read_to_string(&mut body)?;
    println!("inspect called: {body}");
    Ok(())
}
