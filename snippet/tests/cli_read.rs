use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::env;

#[test]
fn cli_read_existing() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    cargo_bin_cmd!("snippet")
        .args(["add", "--name", "x"])
        .write_stdin("aaa")
        .assert()
        .success();

    cargo_bin_cmd!("snippet")
        .args(["read", "--name", "x"])
        .assert()
        .success()
        .stdout(contains("aaa"));
}
