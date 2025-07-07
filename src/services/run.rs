use std::process::{Command, Stdio};
use std::io::Write;

use crate::globals::{DELIMITER, PODMAN_ARGUMENTS};
use crate::models::{ExecutionResult, ExecutionError};

pub async fn run(bash_script: String) -> Result<ExecutionResult, ExecutionError> {
    let mut child = Command::new("podman")
        .args(PODMAN_ARGUMENTS)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ExecutionError {
            message: format!("Failed to start Podman - Ensure Podman is installed and running. Error: {}", e),
        })?;

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(bash_script.as_bytes()).unwrap();

    let output = child.wait_with_output().expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        return Err(ExecutionError {
            message: format!("Unknown server error: {}", stderr),
        });
    }

    let results: Vec<&str> = stdout.split(&format!("{}\n", DELIMITER)).collect();

    let compile_output = results.get(0).unwrap_or(&"").to_string();
    let result_stdout = results.get(1).unwrap_or(&"").to_string();
    let result_stderr = results.get(2).unwrap_or(&"").to_string();
    let runtime_status = results.get(3).unwrap_or(&"0").trim().parse::<i32>().unwrap_or(0);
    let elapsed_us = results.get(4).unwrap_or(&"0").trim().parse::<u64>().unwrap_or(0);

    let result = ExecutionResult {
        compile_output,
        stdout: result_stdout,
        stderr: result_stderr,
        time: elapsed_us,
        runtime_status,
    };

    Ok(result)
}
