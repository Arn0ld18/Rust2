use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::env;

#[test]
fn cli_delete() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    cargo_bin_cmd!("snippet")
        .args(["add", "--name", "to_del"])
        .write_stdin("zzz")
        .assert()
        .success();

    cargo_bin_cmd!("snippet")
        .args(["delete", "--name", "to_del"])
        .assert()
        .success()
        .stdout(contains("deleted"));
}
