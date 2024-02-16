use crate::{
    m_value::{m_value::MValue, match_processor::MatchProcessor},
    progress_println,
};

#[derive(Default)]
pub struct Regex;

impl Regex {
    pub fn new() -> Box<Self> {
        Box::new(Regex)
    }
}

impl MatchProcessor for Regex {
    fn key(&self) -> String {
        String::from("$regex")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> bool {
        match (args, value) {
            (MValue::String(args), MValue::String(value)) => {
                match regex_match(args.clone(), value.clone()) {
                    Ok(result) => result,
                    Err(_) => {
                        progress_println!("Invalid regex: {}\nBe sure to include '/' characters around your regex matcher.", args);
                        false
                    }
                }
            }
            (MValue::String(_args), _) => false,
            _ => {
                progress_println!("Invalid regex type: {}\nBe sure to include '/' characters around your regex matcher.", args);
                false
            }
        }
    }
}

fn regex_match(args: String, value: String) -> Result<bool, ()> {
    let first_char = args.chars().nth(0).ok_or(())?;
    let last_char = args.chars().last().ok_or(())?;

    let regex_str = match (first_char, last_char) {
        ('/', '/') => &args[1..args.len() - 1],
        _ => return Err(()),
    };

    let regex = regex::Regex::new(regex_str).map_err(|_| ())?;
    Ok(regex.is_match(&value))
}

#[cfg(test)]
mod test {
    use serde_yaml::Number;

    use super::*;

    #[test]
    fn matches_with_valid_regex() {
        let regex = Regex::new();
        let args = MValue::String(String::from("/^abc$/"));
        let value = MValue::String(String::from("abc"));
        assert!(regex.is_match(&args, &value));
    }

    #[test]
    fn fails_with_invalid_regex() {
        let regex = Regex::new();
        let args = MValue::String(String::from("^abc$"));
        let value = MValue::String(String::from("abc"));
        assert!(!regex.is_match(&args, &value));
    }

    #[test]
    fn fails_with_invalid_regex_non_str() {
        let regex = Regex::new();
        let args = MValue::Number(Number::from(1));
        let value = MValue::String(String::from("1"));
        assert!(!regex.is_match(&args, &value));
    }
}
