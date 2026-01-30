use std::path::{Path, PathBuf};

use std::io::Result;
use std::process::{Command, Output};

use walkdir::WalkDir;

#[derive(Default)]
pub struct Slimmer {
    pub dry_run: bool,
}

impl Slimmer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slim(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut output = String::new();
        for target in manifests(path)? {
            let mut cmd = self.cargo_clean_cmd(&target);
            let cmd_output = cmd.output()?;
            output.push_str(&summary(target, &cmd_output));
        }
        Ok(output)
    }

    fn cargo_clean_cmd(&self, path: impl AsRef<Path>) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.args(["clean", "--manifest-path", &path.as_ref().to_string_lossy()]);
        if self.dry_run {
            cmd.arg("--dry-run");
        }
        cmd
    }
}

fn manifests(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !e.path().ends_with("target/package"))
    {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            targets.push(entry.path().to_path_buf());
        }
    }
    Ok(targets)
}

fn summary(target: impl AsRef<Path>, output: &Output) -> String {
    format!(
        "{}: {}",
        target.as_ref().parent().unwrap().display(),
        String::from_utf8_lossy(&output.stderr).trim_start()
    )
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, process::ExitStatus};

    use super::*;

    #[test]
    fn manifests_returns_cargo_toml_paths() {
        let path = "tests/data";
        let mut manifests = manifests(path).unwrap();
        manifests.sort();
        let paths = vec![
            PathBuf::from("tests/data/proj_1/Cargo.toml"),
            PathBuf::from("tests/data/proj_2/Cargo.toml"),
            PathBuf::from("tests/data/proj_3/Cargo.toml"),
        ];
        assert_eq!(manifests, paths, "wrong paths");
    }

    #[test]
    fn cargo_clean_cmd_returns_cmd_for_path() {
        let slimmer = Slimmer::new();
        let cmd = slimmer.cargo_clean_cmd(PathBuf::from("code/proj_1/Cargo.toml"));
        assert_eq!(cmd.get_program(), "cargo");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            ["clean", "--manifest-path", "code/proj_1/Cargo.toml"],
            "wrong args"
        );
    }

    #[test]
    fn summary_returns_summary_string() {
        let path = PathBuf::from("./target/Cargo.toml");
        let output = Output {
            status: ExitStatus::default(),
            stdout: Vec::new(),
            stderr: String::from("Removed 2 files, 1.6MiB total\n").into_bytes(),
        };
        let sum = summary(path, &output);
        assert_eq!(sum, format!("./target: Removed 2 files, 1.6MiB total\n"));
    }

    #[test]
    fn cargo_clean_cmd_returns_cmd_for_path_with_dry_run_flag() {
        let mut slimmer = Slimmer::new();
        slimmer.dry_run = true;
        let cmd = slimmer.cargo_clean_cmd(PathBuf::from("code/proj_1/Cargo.toml"));
        assert_eq!(cmd.get_program(), "cargo");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            [
                "clean",
                "--manifest-path",
                "code/proj_1/Cargo.toml",
                "--dry-run",
            ],
            "wrong args"
        );
    }
}
