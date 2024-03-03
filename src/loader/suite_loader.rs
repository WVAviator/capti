use std::path::PathBuf;

use colored::Colorize;
use walkdir::WalkDir;

use crate::{formatting::indent::Indent, progress_println, runner::run_config::RunConfig, Suite};

pub struct SuiteLoader<'a> {
    path: &'a PathBuf,
}

impl<'a> SuiteLoader<'a> {
    pub fn new(path: &'a PathBuf) -> Self {
        SuiteLoader { path }
    }

    pub fn load_suites(&self) -> Vec<Suite> {
        let suites = WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().unwrap_or_default() == "yaml"
                    || e.path().extension().unwrap_or_default() == "yml"
            })
            .filter(|e| e.file_name() != "capti-config.yaml" && e.file_name() != "capti-config.yml")
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| {
                std::fs::read_to_string(&path)
                    .map_err(|e| {
                        progress_println!("{}: The file {:?} could not be read as a Capti test. Please confirm the file contains valid UTF-8 encoding.\n{}", "WARN".yellow(), &path, e.to_string().indent());
                        e
                    }).ok().map(|data| (data, path))
            })
            .filter_map(|(data, path)| {
                serde_yaml::from_str::<Suite>(&data)
                    .map_err(|e| {
                        eprintln!("Failed to parse suite: {}", e);
                        progress_println!("{}: The file {:?} exists in the specified path for Capti tests, but could not be parsed as a Capti test.\n Please confirm the file contains valid YAML structure and Capti fields.\n{}", "WARN".yellow(), &path, e.to_string().indent());
                        e
                    })
                    .ok()
            })
            .collect::<Vec<Suite>>();

        progress_println!("Found and loaded {} test suites.", suites.len());

        suites
    }

    pub fn load_config(&self, config_path: &Option<PathBuf>) {
        let config_path = match config_path {
            Some(path) => Some(path.clone()),
            None => WalkDir::new(&self.path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| match e.path().file_name() {
                    Some(name) => name == "capti-config.yaml" || name == "capti-config.yml",
                    None => false,
                })
                .map(|e| e.path().to_path_buf())
                .next(),
        };
        RunConfig::load(config_path);
    }
}
