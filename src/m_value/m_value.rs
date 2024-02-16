use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_yaml::Number;

use super::{m_map::Mapping, matcher_definition::MatcherDefintion};

#[derive(Debug, Hash, PartialEq, Clone)]
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
}
