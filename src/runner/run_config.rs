use serde::{Deserialize, Serialize};

use crate::suite::setup::SuiteSetup;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RunConfig {
    pub setup: Option<SuiteSetup>,
}
