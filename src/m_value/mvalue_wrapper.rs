use serde::{Deserialize, Serialize};

use super::m_value::MValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MValueWrapper {
    key: MValue,
}

impl MValueWrapper {
    /// attempts to first parse string as a yaml value (not a key) and then try a JSON value if YAML
    /// does not succeed. If both fail, the value is treated as a string.
    pub fn from_yaml_value(value: &str) -> MValue {
        let yaml_value = format!("key: >\n  {}", value);
        let json_value = format!(r#"{{ "key": "{}" }}"#, value);
        let wrapper = serde_yaml::from_str::<MValueWrapper>(&yaml_value).unwrap_or_else(|_| {
            serde_json::from_str::<MValueWrapper>(&json_value).unwrap_or(MValueWrapper {
                key: MValue::String(value.to_string()),
            })
        });
        return wrapper.key;
    }

    /// attempts to first parse string as a JSON value and then try a YAML value if JSON does not
    /// succeed. If both fail, the value is treated as a string.
    pub fn from_json_value(value: &str) -> MValue {
        serde_json::from_str::<MValue>(value).unwrap_or_else(|_| {
            let yaml_value = format!("key: >\n  {}", value);
            let wrapper =
                serde_yaml::from_str::<MValueWrapper>(&yaml_value).unwrap_or(MValueWrapper {
                    key: MValue::String(value.to_string()),
                });
            wrapper.key
        })
    }
}
