use std::fmt;

use serde::de::{self, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub enum WaitInstruction {
    Finished,
    Seconds(u32),
    Port(u32),
}

impl<'de> Deserialize<'de> for WaitInstruction {
  fn deserialize<D>(deserializer: D) -> Result<WaitInstruction, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(WaitInstructionVisitor);
  }
}

struct WaitInstructionVisitor;

impl<'de> Visitor<'de> for WaitInstructionVisitor {
  type Value = WaitInstruction;

  fn expecting(&self, formatter: &mut fmt:: Formatter) -> fmt::Result {
    formatter.write_str("a value representing a wait_for instruction")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
  where 
    E: de::Error
  {
    match value {
      s if s.starts_with("port") => {
        let port = s.split(" ")
          .skip(1)
          .next()
          .ok_or(|e| E::custom(String::from("Missing port number.")))?;
        let port = port
          .parse::<u32>()
          .map_err(|e| E::custom(format!("Invalid port number: {}", port)))?;
        
        Ok(WaitInstruction::Port(port))
      }
      s if s.ends_with("seconds") => {
        let seconds = s.split(" ").next().ok_or(E::custom(String::from("Missing seconds value.")))?;
        let seconds = seconds.parse::<u32>().map_err(|_| E::custom(format!("Invalid seconds value: {}", seconds)))?;
        Ok(WaitInstruction::Seconds(seconds))
      }
      "finished" | "finish" => Ok(WaitInstruction::Finished),
      _ => E::custom(format!("Invalid wait_for instuction: {}", value)),
    }
  }
}