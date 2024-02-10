use std::{
    io::{BufRead, BufReader},
    net::{SocketAddr, TcpStream},
    process::{Command, Stdio},
    time::Duration,
};

use serde::Deserialize;

use crate::{progress::Spinner, progress_println};

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
    pub async fn execute_before_all(&self) {
        if let Some(before_all) = &self.before_all {
            SuiteSetup::execute_instructions(before_all)
                .await
                .expect("Failed to execute 'before_all' setup instructions.");
        }
    }

    pub async fn execute_before_each(&self) {
        if let Some(before_each) = &self.before_each {
            SuiteSetup::execute_instructions(before_each)
                .await
                .expect("Failed to execute 'before_each' setup instructions.");
        }
    }

    pub async fn execute_after_all(&self) {
        if let Some(after_all) = &self.after_all {
            SuiteSetup::execute_instructions(after_all)
                .await
                .expect("Failed to execute 'after_all' setup instructions.");
        }
    }

    pub async fn execute_after_each(&self) {
        if let Some(after_each) = &self.after_each {
            SuiteSetup::execute_instructions(after_each)
                .await
                .expect("Failed to execute 'after_each' setup instructions.");
        }
    }

    async fn execute_instructions(instructions: &Vec<SetupInstruction>) -> Result<(), ()> {
        for setup_instruction in instructions {
            let progress_str = match setup_instruction.description.as_ref() {
                Some(description) => description,
                None => "Setup script",
            };

            let spinner = Spinner::start(progress_str).await;

            SuiteSetup::execute_single_instruction(setup_instruction).await?;

            spinner.finish("Done.");
            progress_println!(" ");
        }

        Ok(())
    }

    async fn execute_single_instruction(instruction: &SetupInstruction) -> Result<(), ()> {
        let mut cmd = Command::new(match cfg!(target_os = "windows") {
            true => "cmd",
            false => "sh",
        });

        match cfg!(target_os = "windows") {
            true => cmd.args(["/C", &instruction.script]),
            false => cmd.arg("-c").arg(&instruction.script),
        };

        let script = instruction.script.clone();

        match &instruction.wait_until {
            Some(WaitInstruction::Finished) => {
                tokio::task::spawn_blocking(move || {
                    cmd.output().map_err(|e| {
                        eprintln!("Failed to spawn script instruction: {}\n{:#?}", script, e);
                    })?;

                    Ok(()) as Result<(), ()>
                })
                .await
                .map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })??;
            }
            Some(WaitInstruction::Stdout(output)) => {
                let mut process = cmd
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .map_err(|e| {
                        eprintln!(
                            "Failed to spawn script instruction: {}\n{:#?}",
                            &instruction.script, e
                        );
                    })?;

                let spinner =
                    Spinner::start(format!("Detecting output '{}'", output.as_str())).await;
                let output = output.clone();

                tokio::task::spawn_blocking(move || {
                    let stdout = process
                        .stdout
                        .take()
                        .ok_or(String::from("Stdout unavailable"))
                        .map_err(|e| {
                            eprintln!(
                                "Failed to read stdout for script instruction: {}\n{:#?}",
                                script, e
                            );
                        })?;

                    // process.stderr.take();

                    let mut reader = BufReader::new(stdout);
                    let mut line = String::new();

                    loop {
                        let bytes_read = reader.read_line(&mut line).map_err(|e| {
                            eprintln!(
                                "Failed to read stdout for script instruction: {}\n{:#?}",
                                script, e
                            );
                        })?;

                        if bytes_read == 0 {
                            break;
                        }

                        if line.contains(output.as_str()) {
                            break;
                        }

                        line.clear();
                    }

                    if let Err(e) = process.kill() {
                        eprintln!("Failed to kill process: {}\n{:#?}", script, e);
                    }

                    Ok(()) as Result<(), ()>
                })
                .await
                .map_err(|e| {
                    eprintln!(
                        "Failed to read stdout for script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })??;

                spinner.finish("Detected.");
            }
            Some(WaitInstruction::Seconds(seconds)) => {
                cmd.spawn().map_err(|e| {
                    eprintln!(
                        "Failed to spawn script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })?;

                tokio::time::sleep(Duration::from_secs_f64(*seconds)).await;
            }
            Some(WaitInstruction::Port(port)) => {
                let port = port.clone();

                let spinner = Spinner::start(format!("Waiting for open port: {}", &port)).await;

                tokio::task::spawn_blocking(move || {
                    let addr: SocketAddr = format!("127.0.0.1:{}", &port).parse().map_err(|e| {
                        eprintln!(
                            "Failed to parse socket from port address: {}\n{:#?}",
                            script, e
                        );
                    })?;

                    if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
                        // In case app is already running on this port
                        return Ok(());
                    }

                    cmd.spawn().map_err(|e| {
                        eprintln!("Failed to spawn script instruction: {}\n{:#?}", script, e);
                    })?;

                    loop {
                        if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
                            break;
                        }
                    }

                    Ok(()) as Result<(), ()>
                })
                .await
                .map_err(|e| {
                    eprintln!(
                        "Failed to read stdout for script instruction: {}\n{:#?}",
                        &instruction.script, e
                    );
                })??;

                spinner.finish("Opened.")
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

    #[tokio::test]
    async fn before_all_script_executes_simple_command() {
        let setup = json!({
            "before_all": [
                {
                    "script": "echo 'Hello, World!'",
                }

            ]
        });
        let setup = serde_json::from_value::<SuiteSetup>(setup).unwrap();

        setup.execute_before_all().await;
    }

    #[tokio::test]
    async fn executes_multiple_long_running_commands_concurrently() {
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
        setup.execute_before_all().await;
        assert!(now.elapsed().as_millis() < 1100);
    }

    #[tokio::test]
    async fn executes_sequentially_when_wait_specified() {
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
        setup.execute_before_all().await;
        let elapsed = now.elapsed().as_millis();
        assert!(elapsed >= 1000);
        assert!(elapsed < 1100);
    }
}
