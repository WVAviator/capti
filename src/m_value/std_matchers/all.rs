use colored::Colorize;

use crate::{
    errors::CaptiError,
    m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor},
};

/// The $includes matcher checks an array to see if the provided value is included.
/// Returns true if a matching (following standard matching rules) value is found in the array.
/// Returns false if no match is found.
pub struct All;

impl All {
    pub fn new() -> Box<Self> {
        Box::new(All)
    }
}

impl MatchProcessor for All {
    fn key(&self) -> String {
        String::from("$all")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        match value {
            MValue::Sequence(arr) => {
                arr.iter()
                    .try_fold(true, |acc, item| match args.matches(item) {
                        Ok(true) => Ok(true && acc),
                        Ok(false) => Ok(false),
                        Err(e) => Err(CaptiError::matcher_error(format!(
                            "Cannot process $all matcher due to argument match error:\n{}",
                            e.to_string()
                        ))),
                    })
            }
            _ => Err(CaptiError::matcher_error(format!(
                "Invalid comparison for $includes: {}\nValue must be an array.",
                value.to_string().red()
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_all_values() {
        let value_json = r#"[1, 1, 1]"#;
        let value = serde_json::from_str::<MValue>(value_json).unwrap();
        let args_json = r#"1"#;
        let args = serde_json::from_str::<MValue>(args_json).unwrap();
        let matcher = All::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn fails_with_one_mismatch() {
        let value_json = r#"[1, 2, 1]"#;
        let value = serde_json::from_str::<MValue>(value_json).unwrap();
        let args_json = r#"1"#;
        let args = serde_json::from_str::<MValue>(args_json).unwrap();
        let matcher = All::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, false);
    }
}
