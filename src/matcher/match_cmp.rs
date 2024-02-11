use std::{collections::HashMap, fmt::Debug, hash::Hash};

use serde::Serialize;

use super::{match_result::MatchResult, matcher::Matcher};

/// The MatchCmp trait implements match_cmp, which allows an object to be compared to another,
/// where the other object can have additional fields that are ignored. This is commonly referenced
/// as an "includes" comparison.
///
/// All fields in A are required in B and must match, but fields absent from A that are present in B are ignored.
///
/// Fields in A may also match fields in B if certain matcher conditions are met.
pub trait MatchCmp {
    fn match_cmp(&self, other: &Self) -> MatchResult;
}

impl<T> MatchCmp for Option<T>
where
    T: MatchCmp + Debug,
{
    fn match_cmp(&self, other: &Self) -> MatchResult {
        match (self, other) {
            (Some(a), Some(b)) => return a.match_cmp(b),
            (None, _) => return MatchResult::Matches,
            (Some(a), None) => {
                return MatchResult::Missing {
                    key: format!("{:#?}", a),
                    context: None,
                }
            }
        }
    }
}

impl MatchCmp for String {
    fn match_cmp(&self, other: &Self) -> MatchResult {
        let string_value = serde_json::to_value(self.clone());

        match string_value {
            Ok(serde_json::Value::String(s))
                if Matcher::from(s.clone())
                    .matches_value(&serde_json::Value::String(other.to_string())) =>
            {
                MatchResult::Matches
            }
            _ => MatchResult::ValueMismatch {
                expected: self.into(),
                actual: other.into(),
                context: None,
            },
        }
    }
}

impl<K, V> MatchCmp for HashMap<K, V>
where
    K: Eq + PartialEq + Hash + Debug,
    V: MatchCmp + Serialize + Clone + Debug,
{
    fn match_cmp(&self, other: &Self) -> MatchResult {
        for (key, value) in self {
            match other.get(&key) {
                Some(other_value) => match value.match_cmp(other_value) {
                    MatchResult::Matches => continue,
                    o => {
                        return o.with_context(format!("at compare ( {:#?}: {:#?} )", &key, &value))
                    }
                },
                _ => {
                    if let Ok(serde_json::Value::String(s)) =
                        serde_json::to_value::<V>(value.clone())
                    {
                        match s {
                            s if Matcher::from(s.clone())
                                .matches_value(&serde_json::Value::Null) =>
                            {
                                continue
                            }
                            _ => {
                                return MatchResult::Missing {
                                    key: format!("{:#?}", &key),
                                    context: Some(format!(
                                        "at compare ( {:#?}: {:#?} )",
                                        &key, &value
                                    )),
                                }
                            }
                        }
                    }
                    return MatchResult::Missing {
                        key: format!("{:#?}", &key),
                        context: Some(format!("at compare ( {:#?}: {:#?} )", &key, &value)),
                    };
                }
            }
        }

        return MatchResult::Matches;
    }
}

impl<T> MatchCmp for [T]
where
    T: MatchCmp + Debug,
{
    fn match_cmp(&self, other: &Self) -> MatchResult {
        let mut self_iter = self.iter().peekable();
        let mut other_iter = other.iter();
        while let Some(other_val) = other_iter.next() {
            match self_iter.peek() {
                Some(value) => {
                    if value.match_cmp(other_val) == MatchResult::Matches {
                        self_iter.next();
                    }
                }
                None => return MatchResult::Matches,
            }
        }
        return MatchResult::CollectionMismatch {
            expected: format!("{:#?}", self),
            actual: format!("{:#?}", other),
            remaining: self_iter.count(),
            context: None,
        };
    }
}

// The below code will work if specialization is stabilized.

// impl<T> MatchCmp for [T]
// where
//     T: MatchCmp + Ord,
// {
//     fn match_cmp(&self, other: &Self) -> bool {
//         let self_sorted = self.clone();
//         self_sorted.sort();
//         let other_sorted = other.clone();
//         other_sorted.sort();

//         let mut self_iter = self_sorted.iter().peekable();
//         let mut other_iter = other_sorted.iter();
//         while let Some(other_val) = other_iter.next() {
//             match self_iter.peek() {
//                 Some(value) => {
//                     if value.match_cmp(other_val) {
//                         self_iter.next();
//                     }
//                 }
//                 None => return true,
//             }
//         }
//         return false;
//     }
// }

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn json_value_includes() {
        let json1 = json!({
            "string": "abc",
            "number": 123,
            "object": {
                "key": "value"
            }
        });
        let json2 = json!({
            "string": "abc",
            "number": 123,
            "extra": "extra",
            "object": {
                "key": "value",
                "extra": [
                    "extra"
                ]
            }
        });

        assert!(json1.match_cmp(&json2) == MatchResult::Matches);
    }

    #[test]
    fn json_value_includes_array() {
        let json1 = json!([1, 2, 3]);
        let json2 = json!([1, 2, 3, 4]);
        assert!(json1.match_cmp(&json2) == MatchResult::Matches);
    }

    #[test]
    fn hashmap_includes() {
        let mut map1 = HashMap::new();
        map1.insert(String::from("key1"), String::from("value1"));
        map1.insert(String::from("key2"), String::from("value2"));

        let mut map2 = HashMap::new();
        map2.insert(String::from("key1"), String::from("value1"));
        map2.insert(String::from("key2"), String::from("value2"));
        map2.insert(String::from("key3"), String::from("value3"));

        assert!(map1.match_cmp(&map2) == MatchResult::Matches);
    }
}
