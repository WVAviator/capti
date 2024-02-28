use crate::{
    errors::CaptiError,
    formatting::indent::Indent,
    m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor},
};

pub struct Or;

impl Or {
    pub fn new() -> Box<Self> {
        Box::new(Or)
    }
}

impl MatchProcessor for Or {
    fn key(&self) -> String {
        String::from("$or")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        match args {
            MValue::Sequence(arr) => {
                arr.iter()
                    .try_fold(false, |acc, arg| match arg.matches(value) {
                        Ok(true) => Ok(true),
                        Ok(false) => Ok(false || acc),
                        Err(e) => Err(CaptiError::matcher_error(format!(
                            "Cannot process $or matcher due to argument error:\n{}",
                            e.to_string().indent()
                        ))),
                    })
            }
            _ => Err(CaptiError::matcher_error(format!(
                "Invalid format for $or matcher. Should be an array/sequence of matchers."
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_two_matchers() {
        let json = r#"["$regex /d/", "$regex /b/"]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::String(String::from("abc"));
        let matcher = Or::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn matches_multiple_matchers() {
        let json = r#"["$regex /d/", "$regex /b/", "$absent", "$length 1"]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::String(String::from("abc"));
        let matcher = Or::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn fails_with_no_matches() {
        let json = r#"["$regex /x/", "$regex /d/", "$absent", "$length 1"]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::String(String::from("abc"));
        let matcher = Or::new();
        let result = matcher.is_match(&args, &value).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn errors_with_invalid_args() {
        let json = r#""$regex /a/""#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::String(String::from("abc"));
        let matcher = Or::new();
        let result = matcher.is_match(&args, &value);
        assert!(result.is_err())
    }

    #[test]
    fn errors_with_invalid_nested_arg() {
        let json = r#"["$regex /a/", "$regex /b", "$exists", "$length 3"]"#;
        let args = serde_json::from_str::<MValue>(json).unwrap();
        let value = MValue::String(String::from("abc"));
        let matcher = Or::new();
        let result = matcher.is_match(&args, &value);
        assert!(result.is_err());
    }
}
