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

use colored::Colorize;
use rvbuild::{config::Config, msg, msg2, server::find_best_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  run().await
}

async fn run() -> anyhow::Result<()> {
  let config = Config::new();

  msg!("Searching best server");

  let server = find_best_server(&config).await?;
  msg2!("Selected: {}", server);

  Ok(())
}
