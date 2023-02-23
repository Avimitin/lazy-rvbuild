use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Config {
  pub servers: HashMap<String, Vec<String>>,
  pub core: Core,
}

#[derive(serde::Deserialize)]
pub struct Core {
  workspace: String,
}

fn create_config_dir(path: &std::path::Path) {
  std::fs::create_dir_all(path)
    .unwrap_or_else(|_| panic!("fail to create config directory `{path:?}`"));
}

fn ensure_config_dir(path: &std::path::Path) {
  let stat = std::fs::metadata(path);
  match stat {
    Ok(stat) => {
      if !stat.is_dir() {
        panic!("{path:?} is not a directory, please clean up your environment!",);
      }
    }
    Err(err) => match err.kind() {
      std::io::ErrorKind::NotFound => create_config_dir(path),
      _ => panic!("met unexpected error: {err} when reading config directory",),
    },
  }
}

fn create_config_file(path: &std::path::Path) {
  let template = format!(
    r#"[core]
workspace="{}/.local/opt/rvbuild/"

[servers]
qemu-user = []"#,
    std::env::var("HOME").unwrap()
  );
  std::fs::write(path, template).expect("fail to create config");
}

fn ensure_config_file(path: &std::path::Path) {
  let stat = std::fs::metadata(path);
  match stat {
    Ok(stat) => {
      if stat.is_dir() {
        panic!("{path:?} is directory! Please clean up your config!");
      }
    }
    Err(err) => match err.kind() {
      std::io::ErrorKind::NotFound => create_config_file(path),
      _ => panic!("met unexpectd error {err} when reading config {path:?}"),
    },
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}

impl Config {
  /// Read or create configuration from $HOME/.config/rvbuild/config.toml or $XDG_CONFIG_DIR/rvbuild/config.toml
  pub fn new() -> Config {
    let config_dir;
    if let Ok(xdg_config_dir) = std::env::var("XDG_CONFIG_DIR") {
      config_dir = std::path::Path::new(&xdg_config_dir).join("rvbuild");
    } else {
      let home = std::env::var("HOME").expect("Variable `HOME` not found, unsupported environment");
      config_dir = std::path::Path::new(&home).join(".config").join("rvbuild");
    };

    ensure_config_dir(&config_dir);

    let config_file = config_dir.join("config.toml");

    ensure_config_file(&config_file);

    let raw_config: String =
      std::fs::read_to_string(&config_file).expect("fail to read from config file");

    toml::from_str(&raw_config).expect("invalid config content")
  }
}

impl std::str::FromStr for Config {
  type Err = anyhow::Error;
  fn from_str(path: &str) -> anyhow::Result<Self> {
    let raw: String = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&raw)?)
  }
}

#[test]
fn test_config_initialize() {
  use std::{env, fs};
  let tmpdir = tempfile::tempdir().expect("Fail to create new tmp dir");
  env::set_var("XDG_CONFIG_DIR", tmpdir.path().to_str().unwrap());

  let config = Config::new();

  let cfg_path = tmpdir.path().join("rvbuild").join("config.toml");

  assert!(fs::metadata(&cfg_path).is_ok());
  assert_eq!(
    config.core.workspace,
    format!("{}/.local/opt/rvbuild/", env::var("HOME").unwrap())
  );

  tmpdir.close().unwrap();
}
