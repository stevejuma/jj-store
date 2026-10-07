//! One resolver for discovery and launch; never searches disposable npm caches.
use super::*;
pub(super) fn prefix(root: &Path, request: RequestId) -> PathBuf {
    root.join("tooling").join(request.to_string())
}
pub(super) fn executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        false
    }
}
pub(super) fn directories(root: &Path, selection: Option<&Selection>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(selected) = selection {
        paths.push(prefix(root, selected.request_id).join("bin"));
    }
    if let Some(ambient) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&ambient).filter(|p| p.is_absolute()));
    }
    if let Some(home) = std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
    {
        paths.extend([home.join(".local/bin"), home.join(".cargo/bin")]);
    }
    paths.extend(["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin", "/bin"].map(PathBuf::from));
    paths
}
pub(super) fn resolve(paths: &[PathBuf], name: &str) -> Option<PathBuf> {
    paths.iter().map(|p| p.join(name)).find(|p| executable(p))
}
pub(super) fn package_version(executable: &Path, name: &str) -> Option<String> {
    let resolved = executable.canonicalize().ok()?;
    for parent in resolved.ancestors().skip(1).take(4) {
        let manifest = parent.join("package.json");
        let metadata = fs::metadata(&manifest).ok();
        if !metadata.is_some_and(|m| m.is_file() && m.len() <= 65536) {
            continue;
        }
        let value: Value = serde_json::from_slice(&fs::read(manifest).ok()?).ok()?;
        if value["name"].as_str() == Some(name) {
            return value["version"]
                .as_str()
                .filter(|v| version(v).is_some())
                .map(str::to_owned);
        }
    }
    None
}
pub(super) fn version(text: &str) -> Option<(u32, u32, u32)> {
    text.split_whitespace().find_map(|word| {
        let mut parts = word.trim_start_matches('v').split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        let patch = parts.next()?.parse().ok()?;
        parts.next().is_none().then_some((major, minor, patch))
    })
}
pub(super) async fn probe(bin: &Path, paths: &[PathBuf]) -> Option<String> {
    let mut command = tokio::process::Command::new(bin);
    command
        .arg("--version")
        .env("PATH", std::env::join_paths(paths).ok()?)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true);
    #[cfg(unix)]
    command.process_group(0);
    let mut child = super::install::Process(command.spawn().ok()?);
    let mut stdout = child.0.stdout.take()?;
    use tokio::io::AsyncReadExt as _;
    let mut bytes = Vec::new();
    tokio::time::timeout(Duration::from_secs(3), async {
        (&mut stdout)
            .take(4097)
            .read_to_end(&mut bytes)
            .await
            .ok()?;
        if bytes.len() > 4096 || !child.0.wait().await.ok()?.success() {
            return None;
        }
        let (a, b, c) = version(std::str::from_utf8(&bytes).ok()?)?;
        Some(format!("{a}.{b}.{c}"))
    })
    .await
    .ok()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn managed_discovery_survives_missing_ambient_path_and_checks_executable_bits() {
        let root = tempfile::tempdir().unwrap();
        let id = RequestId::new([9; 16]);
        let bin = prefix(root.path(), id).join("bin");
        fs::create_dir_all(&bin).unwrap();
        let adapter = bin.join("codex-acp");
        fs::write(&adapter, "fixture").unwrap();
        assert!(resolve(std::slice::from_ref(&bin), "codex-acp").is_none());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&adapter, fs::Permissions::from_mode(0o700)).unwrap();
            let selected = Selection { request_id: id };
            let paths = directories(root.path(), Some(&selected));
            assert_eq!(paths[0], bin);
            assert_eq!(resolve(&paths, "codex-acp"), Some(adapter));
        }
    }
    #[test]
    fn versions_are_bounded_numeric_tuples() {
        assert_eq!(version("codex-cli 0.156.1\n"), Some((0, 156, 1)));
        assert_eq!(version("v24.18.0"), Some((24, 18, 0)));
        assert_eq!(version("1.13.1-beta"), None);
    }
}
