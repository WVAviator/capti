use std::{
    collections::hash_map::DefaultHasher,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    errors::CaptiError,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{m_match::MMatch, m_value::MValue, match_context::MatchContext};

/// A map of `MValue` keys to `MValue` values. Equivalent to a typical YAML mapping, with the
/// additional matcher type handled.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct MMap {
    map: IndexMap<MValue, MValue>,
}

impl MMap {
    pub fn new() -> Self {
        MMap {
            map: IndexMap::new(),
        }
    }

    fn entry(&mut self, key: MValue) -> Entry {
        match self.map.entry(key) {
            indexmap::map::Entry::Occupied(occupied) => Entry::Occupied(OccupiedEntry { occupied }),
            indexmap::map::Entry::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant }),
        }
    }
}

impl SuiteVariables for MMap {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for value in self.map.values_mut() {
            value.populate_variables(variables)?;
        }
        Ok(())
    }
}

impl fmt::Display for MMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " ")?;
        let pretty_json = serde_json::to_string_pretty(&self).unwrap_or("".to_string());
        for line in pretty_json.lines() {
            writeln!(f, "    {}", line)?;
        }

        Ok(())
    }
}

impl Into<serde_json::Value> for MMap {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Object(
            self.map
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        )
    }
}

impl Serialize for MMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl MMatch for MMap {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        for (k, v) in &self.map {
            let other_v = other.get(k).unwrap_or(&MValue::Null);
            match v.matches(other_v) {
                Ok(true) => {}
                Ok(false) => return Ok(false),
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }

    fn get_context(&self, other: &Self) -> super::match_context::MatchContext {
        let mut context = MatchContext::new();

        for (k, v) in &self.map {
            let other_v = other.get(k).unwrap_or(&MValue::Null);
            match v.matches(other_v) {
                Ok(true) => {}
                Ok(false) => {
                    context += v.get_context(&other_v);
                    context.push(format!("Mismatch at key {}:", &k));
                    context.push(format!("  expected: {}", &v));
                    context.push(format!("  found: {}", &other_v));
                }
                Err(e) => {
                    context += v.get_context(&other_v);
                    context.push(format!("Matching error at key {}:", &k));
                    context.push(format!("  expected: {}", &v));
                    context.push(format!("  found: {}", &other_v));
                    context.push(format!("  error: {}", e));
                }
            }
        }

        context
    }
}

impl FromIterator<(MValue, MValue)> for MMap {
    fn from_iter<T: IntoIterator<Item = (MValue, MValue)>>(iter: T) -> Self {
        let map = iter.into_iter().collect::<IndexMap<MValue, MValue>>();
        MMap { map }
    }
}

impl Hash for MMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut xor = 0;
        for (k, v) in &self.map {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            v.hash(&mut hasher);
            xor ^= hasher.finish();
        }
        xor.hash(state);
    }
}

impl From<Vec<(MValue, MValue)>> for MMap {
    fn from(vec: Vec<(MValue, MValue)>) -> Self {
        let mut map = IndexMap::new();
        for (k, v) in vec {
            map.insert(k, v);
        }
        MMap { map }
    }
}

impl Deref for MMap {
    type Target = IndexMap<MValue, MValue>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for MMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<'de> Deserialize<'de> for MMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MMap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a YAML mapping")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MMap::new())
            }

            #[inline]
            fn visit_map<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut mapping = MMap::new();

                while let Some(key) = data.next_key()? {
                    match mapping.entry(key) {
                        Entry::Occupied(entry) => {
                            return Err(serde::de::Error::custom(DuplicateKeyError { entry }));
                        }
                        Entry::Vacant(entry) => {
                            let value = data.next_value()?;
                            entry.insert(value);
                        }
                    }
                }

                Ok(mapping)
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

struct DuplicateKeyError<'a> {
    entry: OccupiedEntry<'a>,
}

impl<'a> Display for DuplicateKeyError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("duplicate entry ")?;
        match self.entry.key() {
            MValue::Null => formatter.write_str("with null key"),
            MValue::Bool(boolean) => write!(formatter, "with key `{}`", boolean),
            MValue::Number(number) => write!(formatter, "with key {}", number),
            MValue::String(string) => {
                write!(formatter, "with key {:?}", string)
            }
            MValue::Matcher(matcher) => {
                write!(formatter, "with matched key {:?}", matcher)
            }
            MValue::Sequence(_) | MValue::Mapping(_) => formatter.write_str("in YAML map"),
        }
    }
}

enum Entry<'a> {
    Occupied(OccupiedEntry<'a>),
    Vacant(VacantEntry<'a>),
}

struct OccupiedEntry<'a> {
    occupied: indexmap::map::OccupiedEntry<'a, MValue, MValue>,
}

impl<'a> OccupiedEntry<'a> {
    fn key(&self) -> &MValue {
        self.occupied.key()
    }
}

struct VacantEntry<'a> {
    vacant: indexmap::map::VacantEntry<'a, MValue, MValue>,
}

impl<'a> VacantEntry<'a> {
    fn insert(self, value: MValue) {
        self.vacant.insert(value);
    }
}
