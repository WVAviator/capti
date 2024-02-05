pub enum Matcher {
    Exact(String),
    Exists,
}

impl Matcher {
    pub fn matches_value(&self, value: &serde_json::Value) -> bool {
        match self {
            Matcher::Exists => value.ne(&serde_json::Value::Null),
            Matcher::Exact(expected) => match value {
                serde_json::Value::String(s) => expected.eq(s),
                _ => false,
            },
        }
    }
}

impl From<&str> for Matcher {
    fn from(value: &str) -> Self {
        match value {
            s if s.starts_with("\\$") => Matcher::Exact(s[1..].to_string()),
            "$exists" => Matcher::Exists,
            _ => Matcher::Exact(value.to_string()),
        }
    }
}

impl From<&String> for Matcher {
    fn from(value: &String) -> Self {
        Matcher::from(value.as_str())
    }
}

impl From<String> for Matcher {
    fn from(value: String) -> Self {
        Matcher::from(value.as_str())
    }
}

#[cfg(test)]
mod test {
    use serde_json::Number;

    use super::*;

    #[test]
    fn exact_values() {
        let matches =
            Matcher::from("123").matches_value(&serde_json::Value::String(String::from("123")));

        assert!(matches);
    }

    #[test]
    fn exists_matches() {
        let matches =
            Matcher::from("$exists").matches_value(&serde_json::Value::Number(Number::from(3)));
        assert!(matches);
    }

    #[test]
    fn exists_nomatch_null() {
        let matches = Matcher::from("$exists").matches_value(&serde_json::Value::Null);
        assert!(!matches);
    }

    #[test]
    fn ignores_escaped_matcher_symbol() {
        let matches = Matcher::from("\\$exists")
            .matches_value(&serde_json::Value::String(String::from("$exists")));
        assert!(matches);
    }
}
