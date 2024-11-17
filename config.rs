use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    paths: HashMap<String, String>,
    thresholds: HashMap<String, i32>,
    other_configs: HashMap<String, String>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig {
            paths: HashMap::new(),
            thresholds: HashMap::new(),
            other_configs: HashMap::new(),
        }
    }

    fn from_file(file_path: &str) -> Result<AppConfig> {
        let mut file = File::open(file_path).expect("File should open read only");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Something went wrong reading the file");
        serde_json::from_str(&contents)
    }

    fn get_env(&mut self) {
        if let Ok(env_file) = env::var("ENV_FILE") {
            let mut env_contents = String::new();
            File::open(env_file).and_then(|mut f| f.read_to_string(&mut env_contents)).expect("Failed to read env file");
            env_contents
                .lines()
                .for_each(|line| {
                     if line.trim().is_empty() || line.starts_with('#'){
                        return;
                     }
                     let parts: Vec<&str> = line.split('=').collect();
                     if parts.len() == 2 {
                        self.other_configs.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                     }
                });
        }
    }
}

fn main() -> io::Result<()> {
    let mut config = AppConfig::new();

    config.get_env();

    let loaded_config = AppConfig::from_file("config.json").unwrap_or_else(|_| AppConfig::new());
    config.paths.extend(loaded_config.paths);
    config.thresholds.extend(loaded_config.thresholds);
    config.other_configs.extend(loaded_config.other_configs);

    println!("{:?}", config.paths.get("example_path"));
    Ok(())
}