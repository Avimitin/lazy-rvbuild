use tokio::process::Command;

/// Information about executing commands on remote server
pub struct RemoteExecution {
  remote: String,
  commands: Vec<String>,
}

/// Execute the given command on the given remote server, and wait for it returning stdout/stderr
pub async fn ssh_exec() {}
