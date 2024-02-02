use std::process::Command;

static PATH: &str = std::env!("CARGO_BIN_EXE_surf");

#[test]
fn url_only() {
    let output = Command::new(PATH).arg("www.test.com").output().unwrap();

    assert_eq!(output.status.code().unwrap(), 0);
}

#[test]
fn url_omitted_throws_error() {
    let output = Command::new(PATH).output().unwrap();

    assert_ne!(output.status.code().unwrap(), 0)
}
