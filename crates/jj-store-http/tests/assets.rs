use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_http::assets::BrowserAssets;
use std::fs;
use tower::ServiceExt as _;
fn bundle(root: &std::path::Path) {
    fs::create_dir(root.join("assets")).unwrap();
    fs::create_dir(root.join("agent-icons")).unwrap();
    fs::write(
        root.join("index.html"),
        "<!doctype html><title>Browser</title>",
    )
    .unwrap();
    fs::write(root.join("assets/app-123.js"), "export const version=1;").unwrap();
    fs::write(root.join("assets/font-123.woff2"), b"wOF2fixture").unwrap();
    fs::write(root.join("assets/font-123.woff"), b"wOFFfixture").unwrap();
    fs::write(root.join("assets/font-123.ttf"), b"TTFfixture").unwrap();
    fs::write(
        root.join("agent-icons/codex.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#,
    )
    .unwrap();
    fs::write(
        root.join("asset-manifest.json"),
        r#"{"version":1,"files":["index.html","assets/app-123.js","assets/font-123.woff2","assets/font-123.woff","assets/font-123.ttf","agent-icons/codex.svg"]}"#,
    )
    .unwrap();
}

#[tokio::test]
async fn strict_browser_csp_allows_only_empty_style_attribute_reset() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    let response = BrowserAssets::load(root.path())
        .unwrap()
        .router()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let csp = response.headers()["content-security-policy"]
        .to_str()
        .unwrap();
    assert!(csp.contains(
        "style-src-attr 'unsafe-hashes' 'sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=';"
    ));
    assert!(csp.contains("script-src 'self';"));
    assert!(!csp.contains("'unsafe-inline'"));
    assert!(!csp.contains("'unsafe-eval'"));
}

#[tokio::test]
async fn development_annotations_allow_inline_helpers_without_widening_resource_sources() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    let assets = BrowserAssets::load(root.path()).unwrap();
    for (router, inline) in [
        (assets.router(), false),
        (assets.router_for_development_annotations(), true),
    ] {
        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let csp = response.headers()["content-security-policy"]
            .to_str()
            .unwrap();
        assert_eq!(csp.contains("'unsafe-inline'"), inline);
        for directive in [
            "default-src 'none'",
            "connect-src 'self'",
            "frame-ancestors 'none'",
            "base-uri 'none'",
            "form-action 'self'",
        ] {
            assert!(csp.contains(directive));
        }
        assert!(!csp.contains("'unsafe-eval'"));
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
        assert_eq!(response.headers()["cache-control"], "no-store");
    }
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
    assert!(csp.contains("font-src 'self'"));
    assert!(csp.contains("img-src 'self' data: https:"));
    assert!(csp.contains("style-src 'self' 'sha256-+A14ONesVdzkn6nr37Osn+rUqNz4oFGZFDbLXrlae04='"));
    assert!(!csp.contains("unsafe-inline"));
    assert!(
        std::str::from_utf8(&to_bytes(response.into_body(), 1024).await.unwrap())
            .unwrap()
            .contains("Browser")
    );
    for (uri, content_type) in [
        ("/assets/font-123.woff2", "font/woff2"),
        ("/assets/font-123.woff", "font/woff"),
        ("/assets/font-123.ttf", "font/ttf"),
        ("/agent-icons/codex.svg", "image/svg+xml"),
    ] {
        let response = router
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], content_type);
    }
    for uri in [
        "/asset-manifest.json",
        "/../secret",
        "/assets/unknown.js",
        "/agent-icons/unknown.svg",
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
        "agent-icons/../secret.svg",
        "agent-icons/nested/codex.svg",
        "agent-icons/codex.png",
        "agent-icons/SOURCES.md",
        "agent-icons/missing.svg",
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
    fs::remove_file(root.path().join("assets/font-123.woff2")).unwrap();
    fs::remove_file(root.path().join("assets/font-123.woff")).unwrap();
    fs::remove_file(root.path().join("assets/font-123.ttf")).unwrap();
    fs::remove_dir(root.path().join("assets")).unwrap();
    symlink(root.path(), root.path().join("assets")).unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}

#[cfg(unix)]
#[test]
fn browser_agent_icon_directory_symlink_is_rejected() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    fs::write(
        root.path().join("codex.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#,
    )
    .unwrap();
    fs::remove_file(root.path().join("agent-icons/codex.svg")).unwrap();
    fs::remove_dir(root.path().join("agent-icons")).unwrap();
    symlink(root.path(), root.path().join("agent-icons")).unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}

fn help(root: &std::path::Path, files: &[&str]) {
    fs::create_dir_all(root.join("help/concepts")).unwrap();
    fs::write(root.join("help/index.html"), "<h1>Help</h1>").unwrap();
    fs::write(root.join("help/concepts/index.html"), "<h1>Concepts</h1>").unwrap();
    fs::write(
        root.join("help/asset-manifest.json"),
        serde_json::json!({"version":1,"files":files}).to_string(),
    )
    .unwrap();
}
#[tokio::test]
async fn documentation_routes_are_preloaded_and_have_scoped_framing_policy() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    help(root.path(), &["index.html", "concepts/index.html"]);
    let router = BrowserAssets::load(root.path()).unwrap().router();
    fs::remove_dir_all(root.path().join("help")).unwrap();
    for path in ["/help/", "/help/index.html", "/help/concepts/"] {
        let response = router
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let csp = response.headers()["content-security-policy"]
            .to_str()
            .unwrap();
        assert!(csp.contains("frame-ancestors 'self'"));
        assert!(csp.contains("script-src 'self';"));
        assert!(!csp.contains("unsafe-eval"));
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    }
    for path in [
        "/help/asset-manifest.json",
        "/help/missing.html",
        "/help/../index.html",
    ] {
        assert_eq!(
            router
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap()
                .status(),
            StatusCode::NOT_FOUND
        );
    }
    assert_eq!(
        router
            .oneshot(Request::builder().uri("/help").body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status(),
        StatusCode::PERMANENT_REDIRECT
    );
}
#[test]
fn documentation_manifest_rejects_invalid_closures() {
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    for files in [
        vec!["index.html", "index.html"],
        vec!["index.html", "../index.html"],
        vec!["index.html", "unknown.js"],
        vec!["concepts/index.html"],
        vec!["index.html", "asset-manifest.json"],
        vec!["index.html", "a/b/c/d/e/f/g/h/i.html"],
        vec!["index.html"; 513],
    ] {
        help(root.path(), &files);
        assert!(BrowserAssets::load(root.path()).is_err(), "{files:?}");
    }
    help(root.path(), &["index.html"]);
    fs::File::create(root.path().join("help/index.html"))
        .unwrap()
        .set_len(8 * 1024 * 1024 + 1)
        .unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}
#[cfg(unix)]
#[test]
fn documentation_rejects_leaf_directory_and_root_symlinks() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    bundle(root.path());
    help(root.path(), &["index.html", "concepts/index.html"]);
    fs::remove_file(root.path().join("help/index.html")).unwrap();
    symlink(
        root.path().join("index.html"),
        root.path().join("help/index.html"),
    )
    .unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
    fs::remove_file(root.path().join("help/index.html")).unwrap();
    help(root.path(), &["index.html", "concepts/index.html"]);
    fs::remove_dir_all(root.path().join("help/concepts")).unwrap();
    symlink(root.path(), root.path().join("help/concepts")).unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
    fs::remove_dir_all(root.path().join("help")).unwrap();
    symlink(root.path(), root.path().join("help")).unwrap();
    assert!(BrowserAssets::load(root.path()).is_err());
}
