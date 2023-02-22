use std::fmt::Display;

use anyhow::{Context, Result};
use regex::Regex;
use tokio::process::Command;

use crate::config::Config;

pub struct Server {
  alias: String,
}

impl Display for Server {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.alias)
  }
}

impl Server {
  fn new<T: ToString>(host: T) -> Self {
    Self {
      alias: host.to_string(),
    }
  }

  async fn get_load(&self) -> Result<f32> {
    let handle = Command::new("ssh")
      .arg(&self.alias)
      .arg("uptime")
      .output()
      .await
      .with_context(|| format!("fail to collect uptime for server {}", self.alias))?;

    let Some(exit_code) = handle.status.code() else {
      anyhow::bail!("load fetcher was killed unexpectedly")
    };

    if exit_code != 0 {
      anyhow::bail!(
        "fail to execute `uptime` on `{}`. STDERR: {}",
        self.alias,
        String::from_utf8(handle.stderr).unwrap_or_else(|_| "Empty stderr output".to_string())
      )
    }

    let stdout =
      String::from_utf8(handle.stdout).with_context(|| "got unexpected non utf-8 output")?;

    let load = Regex::new(r#"load average: [\d\.]+, ([\d\.]+)"#)
      .unwrap()
      .captures(&stdout)
      .ok_or_else(|| anyhow::anyhow!("uptime output is unexpected"))?
      .get(1)
      .ok_or_else(|| anyhow::anyhow!("uptime load output is unexpected"))?
      .as_str();

    Ok(load.parse()?)
  }
}

pub async fn find_best_server(cfg: &Config) -> Result<Server> {
  let mut servers: Vec<Server> = cfg.servers.iter().map(Server::new).collect();

  let mut lowest = 101.0;
  let mut i = 0;

  for (idx, s) in servers.iter().enumerate() {
    let current_load = s
      .get_load()
      .await
      .with_context(|| format!("fail to get load for server: {}", s.alias))?;

    if current_load < lowest {
      lowest = current_load;
      i = idx;
    }
  }

  Ok(servers.swap_remove(i))
}
