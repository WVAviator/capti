use walkdir::WalkDir;

use crate::Suite;

pub struct Runner {
    suites: Vec<Suite>,
    total_tests: usize,
}

impl Runner {
    pub fn from_path(path: String) -> Self {
        let suites = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().unwrap_or_default() == "yaml"
                    || e.path().extension().unwrap_or_default() == "yml"
            })
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .filter_map(|data| serde_yaml::from_str::<Suite>(&data).ok())
            .collect::<Vec<Suite>>();

        let total_tests = suites.iter().map(|suite| suite.get_test_count()).sum();

        Runner {
            suites,
            total_tests,
        }
    }
}
