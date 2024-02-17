use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::Deserialize;

use super::{m_match::MMatch, m_value::MValue};

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

impl MMatch for MSequence {
    fn matches(&self, other: &Self) -> bool {
        return self.0.iter().zip(other.0.iter()).all(|(a, b)| a.matches(b));
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
