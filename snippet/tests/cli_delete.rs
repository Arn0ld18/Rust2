use assert_cmd::Command;
use predicates::str::contains;
use std::env;

#[test]
fn cli_delete() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    Command::cargo_bin("snippet").unwrap()
        .args(["add", "--name", "to_del"])
        .write_stdin("zzz")
        .assert()
        .success();

    Command::cargo_bin("snippet").unwrap()
        .args(["delete", "--name", "to_del"])
        .assert()
        .success()
        .stdout(contains("deleted"));
}
