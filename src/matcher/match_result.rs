use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MatchResult {
    Matches,
    ValueMismatch(String, String),
    Missing(String),
    CollectionMismatch(String, String, usize),
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(match self {
            MatchResult::Matches => {
                write!(f, "")?;
            }
            MatchResult::ValueMismatch(expected, actual) => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Match failed at assertion:")?;
                writeln!(f, "  [ {} == {} ]", expected, actual)?;
            }
            MatchResult::Missing(key) => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Match failed due to missing item.")?;
                writeln!(f, "  Expected {}, Found None", key)?;
            }
            MatchResult::CollectionMismatch(expected, actual, remaining) => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Array values mismatch:")?;
                writeln!(f, "  Expected items: [ {} ]", expected)?;
                writeln!(f, "  Found items: [ {} ]", actual)?;
                writeln!(
                    f,
                    "  Matching unavailable for remaining {} elements.",
                    remaining
                )?;
            }
        })
    }
}
