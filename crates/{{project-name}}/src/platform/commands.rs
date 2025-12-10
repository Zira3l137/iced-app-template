use {{crate_name}}_core::error::{Result, other_error};
use std::process;

pub fn execute_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let output = process::Command::new(cmd).args(args).output()?;
    if !output.stderr.is_empty() {
        Err(other_error(
            format!("Failed to execute command: {}", String::from_utf8_lossy(&output.stderr)).as_str(),
            "execute_cmd",
        ))
    } else if !output.stdout.is_empty() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::new())
    }
}
