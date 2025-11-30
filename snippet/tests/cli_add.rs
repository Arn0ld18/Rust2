use assert_cmd::Command;
use predicates::str::contains;
use std::env;

#[test]
fn cli_add_from_stdin() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    Command::cargo_bin("snippet")
        .unwrap()
        .args(["add", "--name", "hello"])
        .write_stdin("println!(\"hi\");")
        .assert()
        .success()
        .stdout(contains("saved"));
}
