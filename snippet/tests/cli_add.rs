use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::env;

#[test]
fn cli_add_from_stdin() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    cargo_bin_cmd!("snippet")
        .args(["add", "--name", "hello"])
        .write_stdin("println!(\"hi\");")
        .assert()
        .success()
        .stdout(contains("saved"));
}
