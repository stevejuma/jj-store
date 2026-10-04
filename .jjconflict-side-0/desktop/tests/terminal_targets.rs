use jj_store_desktop::terminal::TargetResolver;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};
fn service(
    path: &'static str,
    responses: Vec<&'static str>,
) -> (TargetResolver, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let task = thread::spawn(move || {
        for body in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                .unwrap();
            let mut request = vec![0; 8192];
            let count = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..count]).to_lowercase();
            assert!(request.starts_with(&format!("get {path} ")));
            assert!(request.contains("x-jj-store-protocol: 1"));
            assert!(request.contains(&format!("origin: http://{address}")));
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        }
    });
    (
        TargetResolver::new(format!("http://{address}/").parse().unwrap()).unwrap(),
        task,
    )
}
const REPO: &str = "0123456789abcdef0123456789abcdef";
const READY: &str = r#"{"version":1,"revision":1,"bindings":[{"repository":"0123456789abcdef0123456789abcdef","name":"Working folder","path":"/tmp","generation":1,"availability":"ready"},{"repository":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","name":"Other","path":"/tmp","generation":2,"availability":"ready"}]}"#;
const STALE: &str = r#"{"version":1,"revision":2,"bindings":[{"repository":"0123456789abcdef0123456789abcdef","name":"Working folder","path":"/tmp","generation":1,"availability":"identity_changed"}]}"#;
#[test]
fn target_resolution_uses_protected_reads_and_rechecks_identity() {
    let (resolver, task) = service("/v1/local/repositories", vec![READY, READY, STALE]);
    let targets = resolver.list(REPO, "git_sha1_jj045").unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(
        resolver
            .resolve(REPO, "git_sha1_jj045", &targets[0].id)
            .unwrap()
            .path,
        "/tmp"
    );
    assert!(
        resolver
            .resolve(REPO, "git_sha1_jj045", &targets[0].id)
            .is_err()
    );
    task.join().unwrap();
}
#[test]
fn terminal_rejects_foreign_origins_paths_and_unknown_profiles() {
    for origin in [
        "https://example.com/",
        "http://localhost:3000/",
        "http://127.0.0.1:3000/redirect",
        "http://user@127.0.0.1:3000/",
    ] {
        assert!(TargetResolver::new(origin.parse().unwrap()).is_err());
    }
    let resolver = TargetResolver::new("http://127.0.0.1:3000/".parse().unwrap()).unwrap();
    assert!(resolver.list("../other", "native").is_err());
    assert!(resolver.list(REPO, "arbitrary").is_err());
}

#[test]
fn native_targets_require_ready_workspace_links_and_current_revision() {
    const LINKS: &str = r#"{"version":1,"revision":7,"links":[{"id":"one","path":"/tmp","availability":"ready"},{"id":"two","path":"/private/tmp","availability":"ready"},{"id":"missing","path":"/missing","availability":"unavailable"}]}"#;
    const CHANGED: &str =
        r#"{"version":1,"revision":8,"links":[{"id":"one","path":"/tmp","availability":"ready"}]}"#;
    let (resolver, task) = service(
        "/v1/repos/0123456789abcdef0123456789abcdef/local/workspaces",
        vec![LINKS, CHANGED],
    );
    let targets = resolver.list(REPO, "native").unwrap();
    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0].id, "workspace:7:one");
    assert!(resolver.resolve(REPO, "native", &targets[0].id).is_err());
    task.join().unwrap();
}
