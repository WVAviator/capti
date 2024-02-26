use std::{path::PathBuf, sync::Mutex};

use colored::Colorize;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{progress_println, suite::setup::SuiteSetup};

use super::environment::Environment;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RunConfig {
    pub setup: Option<SuiteSetup>,
    #[serde(default, rename = "env_file")]
    pub env: Environment,
}

impl RunConfig {
    pub fn load(config_path: Option<PathBuf>) {
        match &config_path {
            Some(path) => {
                progress_println!("Loading configuration from {:?}", path)
            }
            None => progress_println!("No configuration provided."),
        };

        let mut path = CONFIG_PATH
            .lock()
            .expect("Failed to load configuration file:\n  Static lock unavailable.");
        *path = config_path;

        drop(path);

        Lazy::force(&CONFIG);
    }

    pub fn global() -> &'static RunConfig {
        &CONFIG
    }
}

impl Default for RunConfig {
    fn default() -> RunConfig {
        RunConfig {
            setup: None,
            env: Environment::default(),
        }
    }
}

static CONFIG_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
static CONFIG: Lazy<RunConfig> = Lazy::new(|| {
    let config_path = CONFIG_PATH.lock().unwrap();
    load_config(config_path.as_ref())
});

fn load_config(config_path: Option<&PathBuf>) -> RunConfig {
    let config = match config_path {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(config) => serde_yaml::from_str::<RunConfig>(&config).ok(),
            Err(e) => {
                progress_println!("{}: Failed to read config file:\n  {}", "ERROR".red(), e);
                None
            }
        },
        None => None,
    };

    let mut config = config.unwrap_or_default();
    if let Err(e) = config.env.load() {
        progress_println!(
            "{}: Failed to load env file specified in configuration:\n  {}",
            "ERROR".red(),
            e
        );
    }

    config
}
