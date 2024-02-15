use std::collections::HashMap;

use serde_yaml::Number;

use super::matcher_definition::MatcherDefintion;

pub enum MValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<MValue>),
    Object(HashMap<String, MValue>),
    Matcher(Box<MatcherDefintion>),
    Variable(String),
}
