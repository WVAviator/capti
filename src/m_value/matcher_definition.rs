use std::fmt;

use serde::Serialize;

use super::{m_value::MValue, matcher_map::MatcherMap};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MatcherDefintion {
    match_key: String,
    args: MValue,
}

impl Serialize for MatcherDefintion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{} {}", self.match_key, self.args))
    }
}

impl MatcherDefintion {
    pub fn is_match(&self, value: &MValue) -> bool {
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            return matcher.is_match(&self.args, value);
        }

        false
    }
}

impl fmt::Display for MatcherDefintion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.match_key, self.args)?;
        Ok(())
    }
}

impl TryFrom<&str> for MatcherDefintion {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ");
        if let Some(key_candidate) = parts.next() {
            if let Some(_) = MatcherMap::get_matcher(key_candidate) {
                let args = parts.map(|s| s.into()).collect::<Vec<String>>().join(" ");
                let args = serde_json::from_str::<MValue>(&args).unwrap_or(MValue::Null);
                return Ok(MatcherDefintion {
                    match_key: key_candidate.to_string(),
                    args,
                });
            }
        }

        return Err(());
    }
}
