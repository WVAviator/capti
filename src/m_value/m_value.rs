use std::fmt;

use colored::Colorize;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_yaml::Number;

use crate::{errors::CaptiError, formatting::indent::Indent, variables::SuiteVariables};

use super::{
    m_map::MMap, m_match::MMatch, m_sequence::MSequence, match_context::MatchContext,
    matcher_definition::MatcherDefinition,
};

/// The MValue is very similar to a typical YAML value, however instead of handling tags, special
/// matchers are handled instead during deserialization. These would normally appear as strings in
/// YAML values, but are separated out based on the matcher's presence in the MatcherMap.
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum MValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Sequence(MSequence),
    Mapping(MMap),
    Matcher(Box<MatcherDefinition>),
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
                match MatcherDefinition::try_from(value) {
                    Ok(matcher) => Ok(MValue::Matcher(Box::new(matcher))),
                    Err(_) => Ok(MValue::String(String::from(value))),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match MatcherDefinition::try_from(value.as_str()) {
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
                let sequence = MSequence::deserialize(de)?;
                Ok(MValue::Sequence(sequence))
            }

            fn visit_map<A>(self, data: A) -> Result<MValue, A::Error>
            where
                A: MapAccess<'de>,
            {
                let de = serde::de::value::MapAccessDeserializer::new(data);
                let mapping = MMap::deserialize(de)?;
                Ok(MValue::Mapping(mapping))
            }
        }

        deserializer.deserialize_any(MValueVisitor)
    }
}

impl MMatch for MValue {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        match (self, other) {
            (MValue::Bool(left), MValue::Bool(right)) => Ok(left.eq(right)),
            (MValue::String(left), MValue::String(right)) => Ok(left.eq(right)),
            (MValue::Number(left), MValue::Number(right)) => Ok(left.eq(right)),
            (MValue::Sequence(left), MValue::Sequence(right)) => left.matches(right),
            (MValue::Mapping(left), MValue::Mapping(right)) => left.matches(right),
            (MValue::Matcher(left), right) => left.matches(&right),
            (MValue::Null, _) => Ok(true),
            _ => Ok(false),
        }
    }

    fn get_context(&self, other: &Self) -> super::match_context::MatchContext {
        match (self, other) {
            (MValue::Bool(left), MValue::Bool(right)) => {
                if !left.eq(right) {
                    let mut context = MatchContext::new();
                    context.push(format!(
                        "Assertion failed at {} == {}",
                        &self.to_string().yellow(),
                        &other.to_string().red()
                    ));
                    return context;
                }
            }
            (MValue::String(left), MValue::String(right)) => {
                if !left.eq(right) {
                    let mut context = MatchContext::new();
                    context.push(format!(
                        "Assertion failed at {} == {}",
                        &self.to_string().yellow(),
                        &other.to_string().red()
                    ));
                    return context;
                }
            }
            (MValue::Number(left), MValue::Number(right)) => {
                if !left.eq(right) {
                    let mut context = MatchContext::new();
                    context.push(format!(
                        "Assertion failed at {} == {}",
                        &self.to_string().yellow(),
                        &other.to_string().red()
                    ));
                    return context;
                }
            }
            (MValue::Sequence(left), MValue::Sequence(right)) => {
                return left.get_context(right);
            }
            (MValue::Mapping(left), MValue::Mapping(right)) => {
                return left.get_context(right);
            }
            (MValue::Matcher(left), right) => return left.get_context(right),
            (left, right) => {
                let mut context = MatchContext::new();
                context.push(String::from("Mismatched types"));
                context.push(format!("expected: {}", &left.to_string()).indent());
                context.push(format!("found: {}", &right.to_string()).indent());
                return context;
            }
        }

        MatchContext::new()
    }
}

impl SuiteVariables for MValue {
    fn populate_variables(
        &mut self,
        variables: &mut crate::variables::variable_map::VariableMap,
    ) -> Result<(), crate::errors::CaptiError> {
        match self {
            MValue::String(_) => {
                let new_s = variables.replace_variables(self.clone())?;
                *self = new_s;
            }
            MValue::Sequence(seq) => {
                for value in seq.iter_mut() {
                    value.populate_variables(variables)?;
                }
            }
            MValue::Mapping(mapping) => mapping.populate_variables(variables)?,
            MValue::Matcher(m) => m.populate_variables(variables)?,
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
                write!(f, "{}", m)?;
            }
            MValue::Matcher(m) => write!(f, "{}", m)?,
        }

        Ok(())
    }
}

impl Into<String> for MValue {
    fn into(self) -> String {
        match self {
            MValue::Null => "null".to_string(),
            MValue::Bool(b) => b.to_string(),
            MValue::Number(n) => n.to_string(),
            MValue::String(s) => s,
            MValue::Sequence(arr) => arr.to_string(),
            MValue::Mapping(m) => m.to_string(),
            MValue::Matcher(m) => m.to_string(),
        }
    }
}

impl Into<MValue> for &str {
    fn into(self) -> MValue {
        self.to_string().into()
    }
}

impl Into<MValue> for &String {
    fn into(self) -> MValue {
        self.to_string().into()
    }
}

impl Into<MValue> for String {
    fn into(self) -> MValue {
        match MatcherDefinition::try_from(self.as_str()) {
            Ok(matcher) => MValue::Matcher(Box::new(matcher)),
            Err(_) => MValue::String(self),
        }
    }
}

impl Into<serde_json::Value> for MValue {
    fn into(self) -> serde_json::Value {
        match self {
            MValue::Null => serde_json::Value::Null,
            MValue::Bool(b) => serde_json::Value::Bool(b),
            MValue::Number(n) => {
                let n_json = serde_json::to_string(&n).unwrap_or("0".into());
                let n =
                    serde_json::to_value(&n_json).unwrap_or(serde_json::Value::Number(0.into()));
                n
            }
            MValue::String(s) => serde_json::Value::String(s),
            MValue::Sequence(arr) => arr.into(),
            MValue::Mapping(m) => m.into(),
            MValue::Matcher(m) => m.to_string().into(),
        }
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

        let mut mapping = MMap::new();
        mapping.insert(
            MValue::String("hello".to_string()),
            MValue::Sequence(MSequence::from(vec![
                MValue::Null,
                MValue::Bool(true),
                MValue::Number(1.into()),
            ])),
        );
        let mut nested_mapping = MMap::new();
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
            MValue::Sequence(MSequence::from(vec![
                MValue::Number(1.0.into()),
                MValue::String("string".to_string()),
                MValue::Bool(false),
                MValue::Mapping(nested_mapping),
            ])),
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

        let matcher = MatcherDefinition::try_from("$exists").unwrap();

        let mut mapping = MMap::new();
        mapping.insert(
            MValue::String("hello".to_string()),
            MValue::Sequence(MSequence::from(vec![
                MValue::Matcher(Box::new(matcher)),
                MValue::Bool(true),
                MValue::Number(1.into()),
            ])),
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

        assert!(yaml1.matches(&yaml2).unwrap());
        assert!(!yaml1.matches(&yaml3).unwrap());
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
        let mut value = MValue::Mapping(MMap::from(vec![
            (
                MValue::String("hello".to_string()),
                MValue::String("Say ${HELLO}!".to_string()),
            ),
            (
                MValue::String("world".to_string()),
                MValue::Sequence(MSequence::from(vec![
                    MValue::String("Say ${HELLO}!".to_string()),
                    MValue::String("Say ${HELLO}!".to_string()),
                ])),
            ),
        ]));
        value.populate_variables(&mut variables).unwrap();
        assert_eq!(
            value,
            MValue::Mapping(MMap::from(vec![
                (
                    MValue::String("hello".to_string()),
                    MValue::String("Say hi!".to_string()),
                ),
                (
                    MValue::String("world".to_string()),
                    MValue::Sequence(MSequence::from(vec![
                        MValue::String("Say hi!".to_string()),
                        MValue::String("Say hi!".to_string()),
                    ])),
                ),
            ]))
        );
    }
}
