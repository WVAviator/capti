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
    progress_println,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::m_value::MValue;

#[derive(Debug, Default, Clone)]
pub struct Mapping {
    map: IndexMap<MValue, MValue>,
}

impl Mapping {
    pub fn new() -> Self {
        Mapping {
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

impl SuiteVariables for Mapping {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for value in self.map.values_mut() {
            value.populate_variables(variables)?;
        }
        Ok(())
    }
}

impl Serialize for Mapping {
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

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        for (k, v) in &self.map {
            let other_v = other.get(k).unwrap_or(&MValue::Null);
            if v != other_v {
                progress_println!(
                    "Mismatch at key {}:\n  expected: {}\n  found: {}",
                    &k,
                    &v,
                    &other_v
                );
                return false;
            }
        }

        true
    }
}

impl PartialEq<Mapping> for Option<Mapping> {
    fn eq(&self, other: &Mapping) -> bool {
        match self {
            Some(mapping) => mapping.eq(other),
            None => true,
        }
    }
}

impl FromIterator<(MValue, MValue)> for Mapping {
    fn from_iter<T: IntoIterator<Item = (MValue, MValue)>>(iter: T) -> Self {
        let map = iter.into_iter().collect::<IndexMap<MValue, MValue>>();
        Mapping { map }
    }
}

impl Hash for Mapping {
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

impl From<Vec<(MValue, MValue)>> for Mapping {
    fn from(vec: Vec<(MValue, MValue)>) -> Self {
        let mut map = IndexMap::new();
        for (k, v) in vec {
            map.insert(k, v);
        }
        Mapping { map }
    }
}

impl Deref for Mapping {
    type Target = IndexMap<MValue, MValue>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Mapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<'de> Deserialize<'de> for Mapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Mapping;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a YAML mapping")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Mapping::new())
            }

            #[inline]
            fn visit_map<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut mapping = Mapping::new();

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
