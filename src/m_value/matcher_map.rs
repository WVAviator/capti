use std::collections::HashMap;

use super::match_processor::MatchProcessor;

pub struct MatcherMap(HashMap<String, Box<dyn MatchProcessor>>);
