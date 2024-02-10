use serde::Deserialize;

use crate::suite::setup::SuiteSetup;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RunConfig {
    pub setup: Option<SuiteSetup>,
}
