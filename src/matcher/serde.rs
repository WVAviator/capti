use serde_json::Value;

use super::{matcher::Matcher, MatchCmp, MatchResult};

impl MatchCmp for serde_json::Value {
    fn match_cmp(&self, other: &Self) -> MatchResult {
        match (self, &other) {
            (Value::Object(map), Value::Object(other_map)) => map
                .match_cmp(other_map)
                .with_context(format!("at compare ( {:#?} : {:#?} )", self, other)),
            (Value::Array(arr), Value::Array(other_arr)) => arr
                .match_cmp(other_arr)
                .with_context(format!("at compare ( {:#?} : {:#?} )", self, other)),
            (Value::Null, _) => MatchResult::Matches,
            (Value::Bool(b), Value::Bool(other_b)) if b == other_b => MatchResult::Matches,
            (Value::Number(n), Value::Number(other_n)) if n == other_n => MatchResult::Matches,
            (Value::String(s), other) if Matcher::from(s).matches_value(other) => {
                MatchResult::Matches
            }
            _ => MatchResult::ValueMismatch {
                expected: format!("{:#?}", self),
                actual: format!("{:#?}", other),
                context: None,
            },
        }
    }
}

impl MatchCmp for serde_json::Map<String, serde_json::Value> {
    fn match_cmp(&self, other: &Self) -> MatchResult {
        for (key, value) in self.iter() {
            match other.get(key.as_str()) {
                Some(other_value) => match value.match_cmp(other_value) {
                    MatchResult::Matches => continue,
                    o => {
                        return o
                            .with_context(format!("at compare ( {:#?}: {:#?} )", &key, &value))
                            .with_context(format!("at compare ( {:#?} : {:#?} )", &self, &other))
                    }
                },
                _ => match value {
                    serde_json::Value::String(s)
                        if Matcher::from(s).matches_value(&serde_json::Value::Null) =>
                    {
                        continue;
                    }
                    _ => {
                        return MatchResult::Missing {
                            key: format!("{:#?}", &key),
                            context: Some(format!("at compare ( {:#?}: {:#?} )", &key, &value)),
                        }
                    }
                },
            }
        }

        return MatchResult::Matches;
    }
}
