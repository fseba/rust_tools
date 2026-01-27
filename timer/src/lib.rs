use anyhow::Result;
use std::{
    process::Command,
    time::{Duration, Instant},
};

pub struct Report {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}

/// Runs `program` with `args`, returning its output and the time taken.
///
/// # Errors
///
/// Returns any error running the command.
pub fn time(program: &str, args: &[String]) -> Result<Report> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    let start = Instant::now();
    let output = cmd.output()?;
    let elapsed = start.elapsed();
    Ok(Report {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        elapsed,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn times_times_echo_cmd() {
        let rep = time("echo", &["hey".into()]).unwrap();
        assert_eq!(rep.stdout.trim_end(), "hey", "wrong stdout");
        assert_eq!(rep.stderr.trim_end(), "", "wrong stderr");
        assert!(!rep.elapsed.is_zero(), "zero elapsed time");
    }
}
