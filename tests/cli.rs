use std::{
    fs,
    io::Write,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const CLI: &str = env!("CARGO_BIN_EXE_potoo-gportugol");

fn run(source: &str, input: &str) -> Output {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("potoo-gportugol-{unique}.gpt"));
    fs::write(&path, source).unwrap();

    let mut child = Command::new(CLI)
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    fs::remove_file(path).unwrap();
    output
}

#[test]
fn runs_a_simple_source_file() {
    let output = run(
        "algoritmo exemplo; início imprima(\"Olá, G-Portugol!\"); fim",
        "",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Olá, G-Portugol!\n"
    );
}

#[test]
fn routes_terminal_input_and_output() {
    let output = run(
        "algoritmo exemplo; variáveis valor: inteiro; fim-variáveis início valor := leia(); imprima(valor); fim",
        "7\n",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("7"));
}

#[test]
fn reports_source_diagnostics_with_a_failure_status() {
    let output = run("@", "");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("1:1"));
}
