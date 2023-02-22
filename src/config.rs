#[derive(serde::Deserialize)]
pub struct Config {
  pub servers: Vec<String>,
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
  std::fs::write(path, r#"server = []"#).expect("fail to create config");
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

impl Config {
  pub fn new() -> Config {
    let config_dir;
    if let Ok(xdg_config_dir) = std::env::var("XDG_CONFIG_DIR") {
      config_dir = std::path::Path::new(&xdg_config_dir).join("rvbuild");
    } else {
      let home = std::env::var("HOME").expect("Variable `HOME` not found, unsupported environment");
      config_dir = std::path::Path::new(&home).join(".config").join("rvbuild");
    };

    ensure_config_dir(&config_dir);

    let config_file = config_dir.join("rvbuild.toml");

    ensure_config_file(&config_file);

    let raw_config: String =
      std::fs::read_to_string(&config_file).expect("fail to read from config file");

    toml::from_str(&raw_config).expect("invalid config content")
  }
}
