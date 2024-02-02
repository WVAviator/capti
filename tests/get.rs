use std::process::Command;

static PATH: &str = std::env!("CARGO_BIN_EXE_surf");

#[test]
fn get_200_status() {
    let output = Command::new(PATH)
        .arg("https://jsonplaceholder.typicode.com/todos/1")
        .output()
        .unwrap();

    let output_text = String::from_utf8(output.stdout).unwrap();
    let status_str = output_text.split("\n").next().unwrap();

    assert_eq!(status_str, "Status: 200");
}
