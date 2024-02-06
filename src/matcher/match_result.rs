use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MatchResult {
    Matches,
    ValueMismatch {
        expected: String,
        actual: String,
        context: Option<String>,
    },
    Missing {
        key: String,
        context: Option<String>,
    },
    CollectionMismatch {
        expected: String,
        actual: String,
        remaining: usize,
        context: Option<String>,
    },
}

impl MatchResult {
    pub fn with_context(self, ctx: impl Into<String>) -> Self {
        match self {
            MatchResult::Matches => self,
            MatchResult::ValueMismatch {
                expected,
                actual,
                context,
            } => {
                let context = match context {
                    Some(context) => format!("{}\n    {}", context, ctx.into()),
                    None => ctx.into(),
                };
                MatchResult::ValueMismatch {
                    expected,
                    actual,
                    context: Some(context),
                }
            }
            MatchResult::Missing { key, context } => {
                let context = match context {
                    Some(context) => format!("{}\n    {}", context, ctx.into()),
                    None => ctx.into(),
                };
                MatchResult::Missing {
                    key,
                    context: Some(context),
                }
            }
            MatchResult::CollectionMismatch {
                expected,
                actual,
                remaining,
                context,
            } => {
                let context = match context {
                    Some(context) => format!("{}\n    {}", context, ctx.into()),
                    None => ctx.into(),
                };
                MatchResult::CollectionMismatch {
                    expected,
                    actual,
                    remaining,
                    context: Some(context),
                }
            }
        }
    }
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(match self {
            MatchResult::Matches => {
                write!(f, "")?;
            }
            MatchResult::ValueMismatch {
                expected,
                actual,
                context,
            } => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Match failed at assertion:")?;
                writeln!(f, "  [ {} == {} ]", expected, actual)?;
                if let Some(context) = context {
                    writeln!(f, "    {}", context)?;
                }
            }
            MatchResult::Missing { key, context } => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Match failed due to missing item.")?;
                writeln!(f, "  Expected {}, Found None", key)?;
                if let Some(context) = context {
                    writeln!(f, "    {}", context)?;
                }
            }
            MatchResult::CollectionMismatch {
                expected,
                actual,
                remaining,
                context,
            } => {
                writeln!(f, "Response match failed.")?;
                writeln!(f, "  Array values mismatch:")?;
                writeln!(f, "  Expected items: [ {} ]", expected)?;
                writeln!(f, "  Found items: [ {} ]", actual)?;
                writeln!(
                    f,
                    "  Matching unavailable for remaining {} elements.",
                    remaining
                )?;
                if let Some(context) = context {
                    writeln!(f, "    {}", context)?;
                }
            }
        })
    }
}
