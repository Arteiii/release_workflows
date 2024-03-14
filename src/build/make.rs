use std::process::{Command, Output};

pub async fn execute_makefile(path: &str) -> Result<String, String> {
    let can_execute_makefile = check_makefile_dependencies();
    if !can_execute_makefile {
        return Err("Make command is not installed or not executable".to_string());
    }

    // Execute Makefile
    let output = execute_command("make", &["-f", "-"], path)
        .map_err(|e| format!("Failed to execute Makefile: {}", e))?;

    // Check if makefile execution was successful
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn check_makefile_dependencies() -> bool {
    let make_command_output = Command::new("make")
        .arg("--version")
        .output()
        .expect("Failed to execute make command");

    // Check if the make command executed successfully
    make_command_output.status.success()
}

fn execute_command(command: &str, args: &[&str], path: &str) -> Result<Output, std::io::Error> {
    Command::new(command)
        .args(args)
        .arg(path)
        .stdin(std::process::Stdio::null()) // No need for stdin input
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
}
