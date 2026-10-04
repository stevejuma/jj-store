use serde_json::{Value, json};
use std::{fs, path::Path};
pub async fn qualify(endpoint: &str, id: &str, folder: &Path) -> Vec<Value> {
    let client = reqwest::Client::new();
    let post = |suffix: &str, body: Value| {
        client
            .post(format!("{endpoint}/local/repos/{id}/working-copy/{suffix}"))
            .header("x-jj-store-protocol", "1")
            .json(&body)
    };
    let before = super::jj(
        folder,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "10",
            "-T",
            "id",
        ],
    );
    let bookmarks = super::jj(
        folder,
        &["--ignore-working-copy", "bookmark", "list", "-T", "name"],
    );
    let metadata = fs::read(folder.join(".git/refs/lapis/state/v1")).unwrap();
    let original = fs::read(folder.join("file.txt")).unwrap();
    fs::write(folder.join("file.txt"), "physical unsnapshotted\n").unwrap();
    let file: Value = post("read", json!({"path":"file.txt"}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(file["contents"], "physical unsnapshotted\n");
    let request = json!({"selection":file["selection"],"contents":"physical saved\n"});
    let saved: Value = post("save", request.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        fs::read_to_string(folder.join("file.txt")).unwrap(),
        "physical saved\n"
    );
    assert_eq!(post("save", request).send().await.unwrap().status(), 409);
    assert_eq!(
        post("read", json!({"path":".jj/repo/config.toml"}))
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    assert_eq!(
        super::jj(
            folder,
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "--no-graph",
                "-n",
                "10",
                "-T",
                "id"
            ]
        ),
        before
    );
    assert_eq!(
        super::jj(
            folder,
            &["--ignore-working-copy", "bookmark", "list", "-T", "name"]
        ),
        bookmarks
    );
    assert_eq!(
        fs::read(folder.join(".git/refs/lapis/state/v1")).unwrap(),
        metadata
    );
    fs::write(folder.join("file.txt"), original).unwrap();
    let sample = |suffix: &str, data: Value| json!({"path":format!("/v1/local/repos/{{repo}}/working-copy/{suffix}"),"method":"post","status":200,"media":"application/json","data":data});
    vec![sample("read", file), sample("save", saved)]
}
