use std::{collections::HashMap, fmt};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
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

impl<'de> Deserialize<'de> for MValue {
    fn deserialize<D>(deserializer: D) -> Result<MValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MValueVisitor)
    }
}

struct MValueVisitor;

impl<'de> Visitor<'de> for MValueVisitor {
    type Value = MValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid matcher or yaml value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        todo!("Check MatcherMap for matching string");
    }
}
