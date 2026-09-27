use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_http::assets::BrowserAssets;
use std::fs;
use tower::ServiceExt as _;
fn bundle(root: &std::path::Path) {
    fs::create_dir(root.join("assets")).unwrap();
    fs::write(
        root.join("index.html"),
        "<!doctype html><title>Browser</title>",
    )
    .unwrap();
    fs::write(root.join("assets/app-123.js"), "export const version=1;").unwrap();
    fs::write(
        root.join("asset-manifest.json"),
        r#"{"version":1,"files":["index.html","assets/app-123.js"]}"#,
    )
    .unwrap();
}
#[tokio::test]
async fn browser_assets_are_allowlisted_preloaded_and_sandboxed() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    let router = BrowserAssets::load(root.path()).unwrap().router();
    fs::remove_file(root.path().join("index.html")).unwrap();
    let response = router
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    let csp = response.headers()["content-security-policy"]
        .to_str()
        .unwrap();
    assert!(csp.contains("script-src 'self'"));
    assert!(!csp.contains("unsafe-inline"));
    assert!(
        std::str::from_utf8(&to_bytes(response.into_body(), 1024).await.unwrap())
            .unwrap()
            .contains("Browser")
    );
    for uri in [
        "/asset-manifest.json",
        "/../secret",
        "/assets/unknown.js",
        "/v1/repos/nope/snapshot",
    ] {
        let response = router
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{uri}");
    }
    let response = router
        .oneshot(
            Request::builder()
                .method("HEAD")
                .uri("/assets/app-123.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        to_bytes(response.into_body(), 1024)
            .await
            .unwrap()
            .is_empty()
    );
}
#[test]
fn browser_manifest_rejects_escape_missing_duplicate_and_oversized_assets() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    for name in [
        "../secret",
        "assets/../secret",
        "assets/nested/file.js",
        "assets/source.html",
        "missing.js",
        "assets/missing.js",
    ] {
        fs::write(
            root.path().join("asset-manifest.json"),
            serde_json::json!({"version":1,"files":["index.html",name]}).to_string(),
        )
        .unwrap();
        assert!(BrowserAssets::load(root.path()).is_err(), "{name}");
    }
    fs::write(
        root.path().join("asset-manifest.json"),
        r#"{"version":1,"files":["index.html","index.html"]}"#,
    )
    .unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
    fs::write(
        root.path().join("asset-manifest.json"),
        r#"{"version":1,"files":["index.html"]}"#,
    )
    .unwrap();
    fs::File::create(root.path().join("index.html"))
        .unwrap()
        .set_len(8 * 1024 * 1024 + 1)
        .unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}
#[cfg(unix)]
#[test]
fn browser_asset_symlinks_are_rejected() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    fs::remove_file(root.path().join("assets/app-123.js")).unwrap();
    symlink(
        root.path().join("index.html"),
        root.path().join("assets/app-123.js"),
    )
    .unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
    fs::remove_file(root.path().join("assets/app-123.js")).unwrap();
    fs::remove_dir(root.path().join("assets")).unwrap();
    symlink(root.path(), root.path().join("assets")).unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}
