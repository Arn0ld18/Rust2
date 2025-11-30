use assert_cmd::Command;
use mockito::Server;
use predicates::str::contains;
use url::Url;
use std::env;

#[test]
fn download_snippet_via_url() {
    unsafe {
        env::set_var("SNIPPETS_APP_STORAGE", "JSON:tests_snippets.json");
    }

    let mut server = Server::new();

    let mock = server.mock("GET", "/snippet")
        .with_status(200)
        .with_body("from_mock")
        .create();

    let url = format!("{}/snippet", server.url());
    let url = Url::parse(&url).unwrap();

    Command::cargo_bin("snippet").unwrap()
        .args(["add", "--name", "mocked", "--download"])
        .arg(url.to_string())
        .assert()
        .success()
        .stdout(contains("saved"));

    mock.assert();
}
