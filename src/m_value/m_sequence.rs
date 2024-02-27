use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::Deserialize;

use crate::errors::CaptiError;

use super::{m_match::MMatch, m_value::MValue, match_context::MatchContext};

/// A sequence of `MValue` items. Equivalent to a typical YAML sequence, with the additional
/// matcher handled.
#[derive(Debug, Default, Clone, Hash, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct MSequence(Vec<MValue>);

impl Deref for MSequence {
    type Target = Vec<MValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MSequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<serde_json::Value> for MSequence {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Array(self.0.into_iter().map(Into::into).collect())
    }
}

impl MMatch for MSequence {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if !a.matches(b)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn get_context(&self, other: &Self) -> MatchContext {
        let mut context = MatchContext::new();
        self.0
            .iter()
            .zip(other.0.iter())
            .enumerate()
            .for_each(|(i, (a, b))| match a.matches(b) {
                Ok(true) => {}
                Ok(false) => {
                    context += a.get_context(&b);
                    context.push(format!("Mismatch at sequence index {}:", i));
                    context.push(format!("  expected: {}", a));
                    context.push(format!("  found: {}", b));
                }
                Err(e) => {
                    context.push(format!("Matching error at sequence index {}:", i));
                    context.push(format!("  expected: {}", a));
                    context.push(format!("  found: {}", b));
                    context.push(format!("  error: {}", e));
                }
            });

        context
    }
}

impl fmt::Display for MSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for value in &self.0 {
            write!(f, "{}, ", value)?;
        }
        write!(f, "]")
    }
}

impl From<Vec<MValue>> for MSequence {
    fn from(vec: Vec<MValue>) -> Self {
        MSequence(vec)
    }
}
