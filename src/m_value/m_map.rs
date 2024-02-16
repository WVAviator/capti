use std::{
    collections::hash_map::DefaultHasher,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer};

use super::m_value::MValue;

#[derive(Debug, Clone)]
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

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        for (k, v) in &self.map {
            if v != other.get(k).unwrap_or(&MValue::Null) {
                return false;
            }
        }

        true
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
