use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn git_command(args: &[&str], cwd: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .with_context(|| format!("Failed to execute git {}", args.join(" ")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    let stdout =
        String::from_utf8(output.stdout).with_context(|| "git output is not valid UTF-8")?;

    Ok(stdout)
}
