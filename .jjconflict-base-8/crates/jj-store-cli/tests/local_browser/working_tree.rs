//! Cold registered-folder transport evidence; no command or combination acceptance.
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let metadata = fs::symlink_metadata(&path).unwrap();
            if metadata.is_dir() {
                visit(root, &path, out);
            } else {
                let bytes = if metadata.is_symlink() {
                    fs::read_link(&path)
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes()
                        .to_vec()
                } else {
                    fs::read(&path).unwrap()
                };
                out.insert(path.strip_prefix(root).unwrap().into(), bytes);
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}
pub async fn qualify(endpoint: &str, id: &str, folder: &Path) -> Vec<Value> {
    let client = reqwest::Client::new();
    let read = |suffix: &str| {
        client
            .get(format!("{endpoint}/local/repos/{id}/{suffix}"))
            .header("x-jj-store-protocol", "1")
    };
    let post = |body: Value| {
        client
            .post(format!("{endpoint}/local/repos/{id}/working-tree/observe"))
            .header("x-jj-store-protocol", "1")
            .json(&body)
    };
    let snapshot: Value = read("snapshot")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let operation = snapshot["heads"][0].clone();
    let generation = snapshot["generation"].clone();
    assert!(
        operation.is_string() && generation.is_number(),
        "{snapshot}"
    );
    let original = fs::read(folder.join("file.txt")).unwrap();
    let git_config = folder.join(".git/config");
    let original_config = fs::read(&git_config).unwrap();
    let ignore = folder
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join("observation-ignore");
    fs::write(&ignore, "observation-obstacle\n").unwrap();
    fs::write(
        &git_config,
        format!(
            "{}\n[core]\nexcludesFile={}\n",
            String::from_utf8(original_config.clone()).unwrap(),
            ignore.display()
        ),
    )
    .unwrap();
    fs::write(
        folder.join("file.txt"),
        "unsnapshotted transport observation\n",
    )
    .unwrap();
    fs::write(
        folder.join("observation-obstacle"),
        "ignored checkout obstacle",
    )
    .unwrap();
    let before = files(folder);
    let body = json!({"version":1,"binding_generation":generation,"operation":operation});
    let value: Value = post(body.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(value["snapshot"]["operation"], operation);
    assert_eq!(value["snapshot"]["binding_generation"], generation);
    let paths = value["changes"].as_array().unwrap();
    assert!(
        paths
            .iter()
            .any(|c| c["path"] == "file.txt" && c["status"] == "modified")
    );
    assert!(!paths.iter().any(|c| c["path"] == "observation-obstacle"));
    assert!(value["obstacle_count"].as_u64().unwrap() >= 1);
    assert!(
        !value
            .to_string()
            .contains("unsnapshotted transport observation")
            && !value.to_string().contains("excludesFile")
    );
    assert_eq!(
        files(folder),
        before,
        "observation must leave physical and metadata bytes unchanged"
    );
    let after: Value = read("snapshot")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(after["heads"], snapshot["heads"]);
    assert_eq!(after["metadata_commit"], snapshot["metadata_commit"]);
    let mut stale = body;
    stale["binding_generation"] = json!(generation.as_u64().unwrap() + 1);
    let rejected = post(stale).send().await.unwrap();
    assert_eq!(rejected.status(), 409);
    let rejected: Value = rejected.json().await.unwrap();
    assert_eq!(rejected["code"], "stale_working_tree");
    fs::write(folder.join("file.txt"), original).unwrap();
    fs::write(&git_config, original_config).unwrap();
    fs::remove_file(folder.join("observation-obstacle")).unwrap();
    fs::remove_file(ignore).unwrap();
    vec![
        json!({"path":"/v1/local/repos/{repo}/working-tree/observe","method":"post","status":200,"media":"application/json","data":value}),
        json!({"path":"/v1/local/repos/{repo}/working-tree/observe","method":"post","status":409,"media":"application/json","data":rejected}),
    ]
}
