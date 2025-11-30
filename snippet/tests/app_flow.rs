use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::env;

#[test]
fn full_flow() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    // ADD
    cargo_bin_cmd!("snippet")
        .args(["add", "--name", "flow"])
        .write_stdin("abc")
        .assert()
        .success();

    // READ
    cargo_bin_cmd!("snippet")
        .args(["read", "--name", "flow"])
        .assert()
        .success()
        .stdout(contains("abc"));

    // DELETE
    cargo_bin_cmd!("snippet")
        .args(["delete", "--name", "flow"])
        .assert()
        .success();

    // READ FAIL
    cargo_bin_cmd!("snippet")
        .args(["read", "--name", "flow"])
        .assert()
        .failure();
}
