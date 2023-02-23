//! An application for automating the Arch Linux RISC-V package build process.
//! Basically it will help user do the below tasks:
//!
//! # 1. Find an appropriate server
//!
//! In this step, rvbuild will try to filter all registered remote server by their corresponding
//! types and their current CPU usage.
//!
//! - Type: QEMU-user? RV Board? QEMU-system?
//! - Choose lowest load

use std::str::FromStr;

use clap::Parser;
use colored::Colorize;
use rvbuild::{config::Config, error, msg, msg2, server::find_best_server};

#[derive(Parser)]
#[command(name = "rvbuild")]
#[command(author = "Avimitin <avimitin@gmail.com>")]
#[command(version = "0.1")]
struct CliArgs {
  #[arg(short = 't', long)]
  machine_type: Option<String>,
  #[arg(short = 'f', long)]
  config_file: Option<String>,
}

#[tokio::main]
async fn main() {
  let result = run().await;
  if let Err(err) = result {
    error!("{err}")
  }
}

async fn run() -> anyhow::Result<()> {
  let arg = CliArgs::parse();

  let config = if let Some(file) = &arg.config_file {
    Config::from_str(file)?
  } else {
    Config::new()
  };

  msg!("Testing servers");

  let server = find_best_server(&config, arg.machine_type).await?;
  msg2!("Selected: {}", server);

  Ok(())
}
