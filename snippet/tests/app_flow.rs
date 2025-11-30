use assert_cmd::Command;
use predicates::str::contains;
use std::env;

#[test]
fn full_flow() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    // ADD
    Command::cargo_bin("snippet").unwrap()
        .args(["add", "--name", "flow"])
        .write_stdin("abc")
        .assert()
        .success();

    // READ
    Command::cargo_bin("snippet").unwrap()
        .args(["read", "--name", "flow"])
        .assert()
        .success()
        .stdout(contains("abc"));

    // DELETE
    Command::cargo_bin("snippet").unwrap()
        .args(["delete", "--name", "flow"])
        .assert()
        .success();

    // READ FAIL
    Command::cargo_bin("snippet").unwrap()
        .args(["read", "--name", "flow"])
        .assert()
        .failure();
}
