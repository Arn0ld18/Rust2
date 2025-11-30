use assert_cmd::cargo::cargo_bin_cmd;
use mockito::Server;
use predicates::str::contains;
use std::env;
use url::Url;

#[test]
fn download_snippet_via_url() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    let mut server = Server::new();

    let mock = server
        .mock("GET", "/snippet")
        .with_status(200)
        .with_body("from_mock")
        .create();

    let url = format!("{}/snippet", server.url());
    let url = Url::parse(&url).unwrap();

    cargo_bin_cmd!("snippet")
        .args(["add", "--name", "mocked", "--download"])
        .arg(url.to_string())
        .assert()
        .success()
        .stdout(contains("saved"));

    mock.assert();
}
