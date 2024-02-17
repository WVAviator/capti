use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::Deserialize;

use super::m_value::MValue;

#[derive(Debug, Default, Clone, Hash, Deserialize)]
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

impl PartialEq for MSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
