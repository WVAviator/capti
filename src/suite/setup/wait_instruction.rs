use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone, PartialEq)]
pub enum WaitInstruction {
    Finished,
    Seconds(f64),
    Port(u64),
}

impl<'de> Deserialize<'de> for WaitInstruction {
    fn deserialize<D>(deserializer: D) -> Result<WaitInstruction, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WaitInstructionVisitor)
    }
}

struct WaitInstructionVisitor;

impl<'de> Visitor<'de> for WaitInstructionVisitor {
    type Value = WaitInstruction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a value representing a wait_for instruction")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            s if s.starts_with("port") => {
                let port = s
                    .split(" ")
                    .skip(1)
                    .next()
                    .ok_or(E::custom(String::from("Missing port number.")))?;
                let port = port
                    .parse::<u64>()
                    .map_err(|_| E::custom(format!("Invalid port number: {}", port)))?;

                Ok(WaitInstruction::Port(port))
            }
            s if s.ends_with("seconds") => {
                let seconds = s
                    .split(" ")
                    .next()
                    .ok_or(E::custom(String::from("Missing seconds value.")))?;
                let seconds = seconds
                    .parse::<f64>()
                    .map_err(|_| E::custom(format!("Invalid seconds value: {}", seconds)))?;
                Ok(WaitInstruction::Seconds(seconds))
            }
            "finished" | "finish" => Ok(WaitInstruction::Finished),
            _ => Err(E::custom(format!("Invalid wait_for instuction: {}", value))),
        }
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(WaitInstruction::Seconds(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(WaitInstruction::Seconds(value as f64))
    }
}

