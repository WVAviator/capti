use std::{
    net::{SocketAddr, TcpStream},
    process::Command,
    time::Duration,
};

use serde::Deserialize;

use super::wait_instruction::WaitInstruction;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SuiteSetup {
    before_all: Option<Vec<SetupInstruction>>,
    before_each: Option<Vec<SetupInstruction>>,
    after_all: Option<Vec<SetupInstruction>>,
    after_each: Option<Vec<SetupInstruction>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SetupInstruction {
    description: Option<String>,
    script: String,
    wait_until: Option<WaitInstruction>,
}

impl SuiteSetup {
    pub fn execute_before_all(&self) {
        if let Some(before_all) = &self.before_all {
            SuiteSetup::execute_instructions(before_all)
                .expect("Failed to execute 'before_all' setup instructions.");
        }
    }

    pub fn execute_before_each(&self) {
        if let Some(before_each) = &self.before_each {
            SuiteSetup::execute_instructions(before_each)
                .expect("Failed to execute 'before_each' setup instructions.");
        }
    }

    pub fn execute_after_all(&self) {
        if let Some(after_all) = &self.after_all {
            SuiteSetup::execute_instructions(after_all)
                .expect("Failed to execute 'after_all' setup instructions.");
        }
    }

    pub fn execute_after_each(&self) {
        if let Some(after_each) = &self.after_each {
            SuiteSetup::execute_instructions(after_each)
                .expect("Failed to execute 'after_each' setup instructions.");
        }
    }

    fn execute_instructions(instructions: &Vec<SetupInstruction>) -> Result<(), ()> {
        for setup_instruction in instructions {
            if let Some(description) = setup_instruction.description.as_ref() {
                println!("{}", description);
            }

            SuiteSetup::execute_single_instruction(setup_instruction)?;
        }

        Ok(())
    }

    fn execute_single_instruction(instruction: &SetupInstruction) -> Result<(), ()> {
        let mut cmd = Command::new(match cfg!(target_os = "windows") {
            true => "cmd",
            false => "sh",
        });

        match cfg!(target_os = "windows") {
            true => cmd.args(["/C", &instruction.script]),
            false => cmd.arg("-c").arg(&instruction.script),
        };

        match instruction.wait_until {
            Some(WaitInstruction::Finished) => {
                cmd.output().map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;
            }
            Some(WaitInstruction::Seconds(seconds)) => {
                cmd.spawn().map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;
                std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
            }
            Some(WaitInstruction::Port(port)) => {
                let addr: SocketAddr = format!("127.0.0.1:{}", &port).parse().map_err(|e| {
                    eprintln!(
                        "Failed to parse socket from port address: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;

                if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
                    // In case app is already running on this port
                    return Ok(());
                }

                cmd.spawn().map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;

                println!("Waiting for port {} to open.", &port);

                loop {
                    if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
                        break;
                    }
                }
            }
            None => {
                cmd.spawn().map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_wait_instruction() {
        let setup = json!({
            "before_all": [
                {
                    "script": "echo 'no_wait'",
                },
                {
                    "script": "echo 'wait seconds'",
                    "wait_until": 1,
                },
                {
                    "script": "echo 'wait finished'",
                    "wait_until": "finished",
                },
            ]
        });

        let setup = serde_json::from_value::<SuiteSetup>(setup).unwrap();
        assert!(setup.before_all.as_ref().unwrap()[0].wait_until.is_none());
        assert_eq!(
            setup.before_all.as_ref().unwrap()[1].wait_until,
            Some(WaitInstruction::Seconds(1.0))
        );
        assert_eq!(
            setup.before_all.as_ref().unwrap()[2].wait_until,
            Some(WaitInstruction::Finished)
        );
    }

    #[test]
    fn before_all_script_executes_simple_command() {
        let setup = json!({
            "before_all": [
                {
                    "script": "echo 'Hello, World!'",
                }

            ]
        });
        let setup = serde_json::from_value::<SuiteSetup>(setup).unwrap();

        setup.execute_before_all();
    }

    #[test]
    fn executes_multiple_long_running_commands_concurrently() {
        let setup = json!({
            "before_all": [
                {
                    "script": "sleep 1",
                },
                {
                    "script": "sleep 1",
                },
                {
                    "script": "sleep 1",
                },
            ]
        });
        let setup = serde_json::from_value::<SuiteSetup>(setup).unwrap();

        let now = std::time::Instant::now();
        setup.execute_before_all();
        assert!(now.elapsed().as_millis() < 1100);
    }

    #[test]
    fn executes_sequentially_when_wait_specified() {
        let setup = json!({
            "before_all": [
                {
                    "script": "sleep 0.5",
                    "wait_until": "finished"
                },
                {
                    "script": "sleep 1",
                    "wait_until": 0.5
                },
            ]
        });
        let setup = serde_json::from_value::<SuiteSetup>(setup).unwrap();

        let now = std::time::Instant::now();
        setup.execute_before_all();
        let elapsed = now.elapsed().as_millis();
        assert!(elapsed >= 1000);
        assert!(elapsed < 1100);
    }
}
