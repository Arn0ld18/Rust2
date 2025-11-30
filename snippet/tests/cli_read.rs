use assert_cmd::Command;
use predicates::str::contains;
use std::env;

#[test]
fn cli_read_existing() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    Command::cargo_bin("snippet").unwrap()
        .args(["add", "--name", "x"])
        .write_stdin("aaa")
        .assert()
        .success();

    Command::cargo_bin("snippet").unwrap()
        .args(["read", "--name", "x"])
        .assert()
        .success()
        .stdout(contains("aaa"));
}
