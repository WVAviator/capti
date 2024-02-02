use std::{collections::HashMap, hash::Hash};

use serde_json::Value;

/// The MatchCmp trait implements match_cmp, which allows an object to be compared to another,
/// where the other object can have additional fields that are ignored. This is commonly referenced
/// as an "includes" comparison.
///
/// All fields in A are required in B and must match, but fields absent from A that are present in B are ignored.
///
/// Fields in A may also match fields in B if certain matcher conditions are met.
pub trait MatchCmp {
    fn match_cmp(&self, other: &Self) -> bool;
}

impl<T> MatchCmp for Option<T>
where
    T: MatchCmp,
{
    fn match_cmp(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => return a.match_cmp(b),
            (None, _) => return true,
            (Some(_), None) => return false,
        }
    }
}

impl MatchCmp for String {
    fn match_cmp(&self, other: &Self) -> bool {
        return self.eq(other);
    }
}

impl MatchCmp for u16 {
    fn match_cmp(&self, other: &Self) -> bool {
        return self.eq(other);
    }
}

impl<K, V> MatchCmp for HashMap<K, V>
where
    K: Eq + PartialEq + Hash,
    V: MatchCmp,
{
    fn match_cmp(&self, other: &Self) -> bool {
        for (key, value) in self {
            match other.get(&key) {
                Some(other_value) if value.match_cmp(other_value) => {}
                _ => return false,
            }
        }

        return true;
    }
}

impl MatchCmp for serde_json::Map<String, serde_json::Value> {
    fn match_cmp(&self, other: &Self) -> bool {
        for (key, value) in self.iter() {
            match other.get(key.as_str()) {
                Some(other_value) if value.match_cmp(other_value) => {}
                _ => return false,
            }
        }

        return true;
    }
}

impl<T: MatchCmp> MatchCmp for [T]
where
    T: MatchCmp,
{
    fn match_cmp(&self, other: &Self) -> bool {
        let mut self_iter = self.iter().peekable();
        let mut other_iter = other.iter();
        while let Some(other_val) = other_iter.next() {
            match self_iter.peek() {
                Some(value) => {
                    if value.match_cmp(other_val) {
                        self_iter.next();
                    }
                }
                None => return true,
            }
        }
        return false;
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

impl MatchCmp for serde_json::Value {
    fn match_cmp(&self, other: &Self) -> bool {
        match (self, &other) {
            (Value::Object(map), Value::Object(other_map)) => map.match_cmp(other_map),
            (Value::Array(arr), Value::Array(other_arr)) => arr.match_cmp(other_arr),
            (Value::Null, _) => true,
            (Value::Bool(b), Value::Bool(other_b)) => b == other_b,
            (Value::Number(n), Value::Number(other_n)) => n == other_n,
            (Value::String(s), Value::String(other_s)) => s == other_s,
            _ => false,
        }
    }
}

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

        assert!(json1.match_cmp(&json2));
    }

    #[test]
    fn json_value_includes_array() {
        let json1 = json!([1, 2, 3]);
        let json2 = json!([1, 2, 3, 4]);
        assert!(json1.match_cmp(&json2));
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

        assert!(map1.match_cmp(&map2));
    }
}
