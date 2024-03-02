use crate::{
    errors::CaptiError,
    formatting::indent::Indent,
    m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor},
};

pub struct If;

impl If {
    pub fn new() -> Box<Self> {
        Box::new(If)
    }
}

impl MatchProcessor for If {
    fn key(&self) -> String {
        String::from("$if")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        match args {
            MValue::Sequence(arr) => {
                if arr.len() != 2 && arr.len() != 3 {
                    return Err(CaptiError::matcher_error(format!(
                        "Invalid format for $if matcher. Argument should be an array/sequence of either 2 (if/then) or 3 (if/then/else) matchers. {} arguments provided.", arr.len()
                    )));
                }

                match (arr[0].matches(value), arr.len()) {
                    (Ok(true), _) => arr[1].matches(value),
                    (Ok(false), 2) => Ok(true),
                    (Ok(false), 3) => arr[2].matches(value),
                    (Err(e), _) => Err(CaptiError::matcher_error(format!(
                        "Cannot process $if matcher due to argument error:\n{}",
                        e.to_string().indent()
                    ))),
                    _ => Err(CaptiError::matcher_error(format!(
                        "Invalid format for $if matcher. Argument should be an array/sequence of either 2 (if/then) or 3 (if/then/else) matchers. {} arguments provided.", arr.len()
                    ))),
                }
            }
            _ => Err(CaptiError::matcher_error(format!(
                "Invalid format for $if matcher. Argument should be an array/sequence of either 2 (if/then) or 3 (if/then/else) matchers."
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_if_2args_true() {
        let json = r#"[1, 1]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn fails_if_2args_false() {
        let json = r#"[1, 2]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn matches_if_2args_if_false() {
        let json = r#"[2, 2]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn matches_if_3args_true() {
        let json = r#"[1, 1, 2]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn matches_if_3args_else() {
        let json = r#"[2, 2, 1]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn matches_if_3args_else_false() {
        let json = r#"["$eq 2", "$eq 2", "$eq 2"]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::Number(1.into());
        let matcher = If::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, false);
    }
}
