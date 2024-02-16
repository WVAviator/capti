use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_yaml::Number;

use crate::variables::SuiteVariables;

use super::{m_map::Mapping, matcher_definition::MatcherDefintion};

#[derive(Debug, Hash, Clone)]
pub enum MValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Sequence(Sequence),
    Mapping(Mapping),
    Matcher(Box<MatcherDefintion>),
}

impl Default for MValue {
    fn default() -> Self {
        MValue::Null
    }
}

impl Serialize for MValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            MValue::Null => serializer.serialize_unit(),
            MValue::Bool(b) => serializer.serialize_bool(*b),
            MValue::Number(n) => n.serialize(serializer),
            MValue::String(s) => serializer.serialize_str(s),
            MValue::Sequence(arr) => arr.serialize(serializer),
            MValue::Mapping(m) => m.serialize(serializer),
            MValue::Matcher(m) => m.serialize(serializer),
        }
    }
}

impl Eq for MValue {}

pub type Sequence = Vec<MValue>;

impl<'de> Deserialize<'de> for MValue {
    fn deserialize<D>(deserializer: D) -> Result<MValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MValueVisitor;

        impl<'de> Visitor<'de> for MValueVisitor {
            type Value = MValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid matcher or yaml value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match MatcherDefintion::try_from(value) {
                    Ok(matcher) => Ok(MValue::Matcher(Box::new(matcher))),
                    Err(_) => Ok(MValue::String(String::from(value))),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match MatcherDefintion::try_from(value.as_str()) {
                    Ok(matcher) => Ok(MValue::Matcher(Box::new(matcher))),
                    Err(_) => Ok(MValue::String(value)),
                }
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Number(value.into()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Number(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Number(value.into()))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Null)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MValue::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_seq<A>(self, data: A) -> Result<MValue, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let de = serde::de::value::SeqAccessDeserializer::new(data);
                let sequence = Sequence::deserialize(de)?;
                Ok(MValue::Sequence(sequence))
            }

            fn visit_map<A>(self, data: A) -> Result<MValue, A::Error>
            where
                A: MapAccess<'de>,
            {
                let de = serde::de::value::MapAccessDeserializer::new(data);
                let mapping = Mapping::deserialize(de)?;
                Ok(MValue::Mapping(mapping))
            }
        }

        deserializer.deserialize_any(MValueVisitor)
    }
}

impl PartialEq for MValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Sequence(l0), Self::Sequence(r0)) => l0 == r0,
            (Self::Mapping(l0), Self::Mapping(r0)) => l0 == r0,
            (Self::Matcher(l0), other) => l0.is_match(&other),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialEq<MValue> for Option<MValue> {
    fn eq(&self, other: &MValue) -> bool {
        match self {
            Some(value) => value == other,
            None => true,
        }
    }
}

impl SuiteVariables for MValue {
    fn populate_variables(
        &mut self,
        variables: &mut crate::variables::variable_map::VariableMap,
    ) -> Result<(), crate::errors::CaptiError> {
        match self {
            MValue::String(s) => {
                *s = variables.replace_variables(&s)?;
            }
            MValue::Sequence(seq) => {
                for value in seq {
                    value.populate_variables(variables)?;
                }
            }
            MValue::Mapping(mapping) => {
                for value in mapping.values_mut() {
                    value.populate_variables(variables)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl fmt::Display for MValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MValue::Null => write!(f, "null")?,
            MValue::Bool(b) => write!(f, "{}", b)?,
            MValue::Number(n) => write!(f, "{}", n)?,
            MValue::String(s) => write!(f, "\"{}\"", s)?,
            MValue::Sequence(arr) => {
                write!(f, "[")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")?;
            }
            MValue::Mapping(m) => {
                writeln!(f, "{{")?;
                for (i, (key, value)) in m.iter().enumerate() {
                    if i > 0 {
                        writeln!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                writeln!(f, " ")?;
                write!(f, "}}")?;
            }
            MValue::Matcher(m) => write!(f, "{}", m)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserializes_from_standard_yaml() {
        let yaml = r#"
        hello:
            - null
            - true
            - 1
        world:
            - 1.0
            - "string"
            - false
            - test: "true"
              abc: def
        "#;

        let mut mapping = Mapping::new();
        mapping.insert(
            MValue::String("hello".to_string()),
            MValue::Sequence(vec![
                MValue::Null,
                MValue::Bool(true),
                MValue::Number(1.into()),
            ]),
        );
        let mut nested_mapping = Mapping::new();
        nested_mapping.insert(
            MValue::String("test".to_string()),
            MValue::String("true".to_string()),
        );
        nested_mapping.insert(
            MValue::String("abc".to_string()),
            MValue::String("def".to_string()),
        );
        mapping.insert(
            MValue::String("world".to_string()),
            MValue::Sequence(vec![
                MValue::Number(1.0.into()),
                MValue::String("string".to_string()),
                MValue::Bool(false),
                MValue::Mapping(nested_mapping),
            ]),
        );

        let expected = MValue::Mapping(mapping);

        let value = serde_yaml::from_str::<MValue>(yaml).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn deserializes_yaml_with_matchers() {
        let yaml = r#"
        hello:
            - $exists
            - true
            - 1
        "#;

        let matcher = MatcherDefintion::try_from("$exists").unwrap();

        let mut mapping = Mapping::new();
        mapping.insert(
            MValue::String("hello".to_string()),
            MValue::Sequence(vec![
                MValue::Matcher(Box::new(matcher)),
                MValue::Bool(true),
                MValue::Number(1.into()),
            ]),
        );

        let expected = MValue::Mapping(mapping);

        let value = serde_yaml::from_str::<MValue>(yaml).unwrap();

        assert_eq!(expected, value);
    }

    #[test]
    fn matcher_equality_after_deserialize() {
        let yaml1 = r#"
        hello: $exists
        "#;

        let yaml2 = r#"
        hello: something 
        "#;

        let yaml3 = r#"
        world: not hello
        "#;

        let yaml1 = serde_yaml::from_str::<MValue>(yaml1).unwrap();
        let yaml2 = serde_yaml::from_str::<MValue>(yaml2).unwrap();
        let yaml3 = serde_yaml::from_str::<MValue>(yaml3).unwrap();

        assert_eq!(yaml1, yaml2);
        assert_ne!(yaml1, yaml3);
    }

    #[test]
    fn populates_variables_in_string() {
        let mut variables = crate::variables::variable_map::VariableMap::new();
        variables.insert("HELLO", "hi");
        let mut value = MValue::String("Say ${HELLO}!".to_string());
        value.populate_variables(&mut variables).unwrap();
        assert_eq!(value, MValue::String("Say hi!".to_string()));
    }

    #[test]
    fn populates_variables_nested() {
        let mut variables = crate::variables::variable_map::VariableMap::new();
        variables.insert("HELLO", "hi");
        let mut value = MValue::Mapping(Mapping::from(vec![
            (
                MValue::String("hello".to_string()),
                MValue::String("Say ${HELLO}!".to_string()),
            ),
            (
                MValue::String("world".to_string()),
                MValue::Sequence(vec![
                    MValue::String("Say ${HELLO}!".to_string()),
                    MValue::String("Say ${HELLO}!".to_string()),
                ]),
            ),
        ]));
        value.populate_variables(&mut variables).unwrap();
        assert_eq!(
            value,
            MValue::Mapping(Mapping::from(vec![
                (
                    MValue::String("hello".to_string()),
                    MValue::String("Say hi!".to_string()),
                ),
                (
                    MValue::String("world".to_string()),
                    MValue::Sequence(vec![
                        MValue::String("Say hi!".to_string()),
                        MValue::String("Say hi!".to_string()),
                    ]),
                ),
            ]))
        );
    }
}
