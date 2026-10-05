//! Real process admission and parent-pipe lifetime for desktop launches.
use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

fn init(root: &Path, runtime: &Path) -> std::path::PathBuf {
    let state = root.join("installation");
    let result = Command::new(env!("CARGO_BIN_EXE_jj-store"))
        .args(["--format=json", "init", "--local", "--state-dir"])
        .arg(&state)
        .arg("--rustfs-executable")
        .arg(runtime)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    state.join("server.toml")
}

#[test]
fn desktop_flags_require_supervision_and_a_pipe_before_startup() {
    let root = tempfile::tempdir().unwrap();
    let config = init(root.path(), Path::new("/usr/bin/true"));
    let before = std::fs::read(&config).unwrap();
    let base = || {
        let mut c = Command::new(env!("CARGO_BIN_EXE_jj-store"));
        c.arg("run").arg("--config").arg(&config);
        c
    };
    let result = base()
        .args(["--desktop-bind", "127.0.0.1:17440"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("--desktop-supervised"));
    let result = base()
        .arg("--desktop-supervised")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("owner stdin pipe"));
    let result = base()
        .args(["--desktop-supervised", "--desktop-bind", "0.0.0.0:17440"])
        .stdin(Stdio::piped())
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("concrete literal loopback"));
    assert_eq!(std::fs::read(config).unwrap(), before);
}

#[test]
#[ignore = "requires explicit paired embedded RustFS executable"]
fn desktop_owner_eof_stops_real_service_and_retains_installation() {
    let runtime = std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("explicit runtime");
    let root = tempfile::tempdir().unwrap();
    let config = init(root.path(), Path::new(&runtime));
    let before = std::fs::read(&config).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    for _ in 0..2 {
        let mut child = Command::new(env!("CARGO_BIN_EXE_jj-store"))
            .args(["--format=json", "run", "--config"])
            .arg(&config)
            .args([
                "--desktop-supervised",
                "--desktop-bind",
                &address.to_string(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let output = child.stdout.take().unwrap();
        let (send, receive) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut line = String::new();
            let _ = BufReader::new(output).read_line(&mut line);
            let _ = send.send(line);
        });
        let line = receive
            .recv_timeout(Duration::from_secs(30))
            .unwrap_or_else(|e| {
                let _ = child.kill();
                let _ = child.wait();
                panic!("readiness: {e}")
            });
        let event: serde_json::Value = serde_json::from_str(&line).unwrap();
        assert_eq!(event["event"], "listening");
        assert_eq!(event["address"], address.to_string());
        drop(child.stdin.take());
        let deadline = Instant::now() + Duration::from_secs(20);
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("owner EOF did not stop service");
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        assert!(status.success(), "{status}");
        assert!(std::net::TcpStream::connect(address).is_err());
        assert!(!root.path().join("installation/native.sock").exists());
        assert_eq!(std::fs::read(&config).unwrap(), before);
    }
}
