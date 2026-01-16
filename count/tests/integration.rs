use assert_cmd::{Command, cargo::cargo_bin};
use predicates::prelude::predicate;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::new(cargo_bin!("count"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    Command::new(cargo_bin!("count"))
        .arg("./tests/data/test_file_1")
        .arg("./tests/data/test_file_2")
        .assert()
        .success()
        .stdout("./tests/data/test_file_1: 1 lines\n./tests/data/test_file_2: 2 lines\n");
}

#[test]
fn binary_counts_words_in_named_files() {
    Command::new(cargo_bin!("count"))
        .arg("-w")
        .arg("./tests/data/test_file_1")
        .assert()
        .success()
        .stdout("./tests/data/test_file_1: 2 words\n");
}
