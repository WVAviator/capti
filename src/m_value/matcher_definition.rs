use super::{m_value::MValue, matcher_map::MatcherMap};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MatcherDefintion {
    match_key: String,
    args: MValue,
}

impl MatcherDefintion {
    fn is_match(&self, value: &MValue) -> bool {
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            return matcher.is_match(&self.args, value);
        }

        false
    }
}

impl TryFrom<&str> for MatcherDefintion {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ");
        if let Some(key_candidate) = parts.next() {
            if let Some(_) = MatcherMap::get_matcher(key_candidate) {
                let args = parts.collect::<String>();
                let args = serde_yaml::from_str::<MValue>(&args).unwrap_or(MValue::String(args));
                return Ok(MatcherDefintion {
                    match_key: key_candidate.to_string(),
                    args,
                });
            }
        }

        return Err(());
    }
}
