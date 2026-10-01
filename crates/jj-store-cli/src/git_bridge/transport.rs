//! Bounded ordinary Git subprocesses. No shell, hooks or repository config.
use super::*;
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{io::AsyncReadExt as _, process::Command};
const OUTPUT_BYTES: u64 = 1024 * 1024;
const CACHE_BYTES: u64 = 512 * 1024 * 1024;
struct ProcessGroup(Option<u32>);
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(pid) = self.0.take() {
            let _ = std::process::Command::new("/bin/kill")
                .args(["-KILL", "--", &format!("-{pid}")])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}
fn disk(path: &Path) -> Result<u64> {
    let mut pending = vec![path.to_owned()];
    let mut bytes = 0u64;
    let mut files = 0;
    while let Some(path) = pending.pop() {
        files += 1;
        if files > 100_000 {
            return Err("Git cache entry limit".into());
        }
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err("Git cache symlink".into());
        }
        if metadata.is_dir() {
            for entry in std::fs::read_dir(path)? {
                pending.push(entry?.path());
            }
        } else {
            bytes = bytes
                .checked_add(metadata.len())
                .ok_or("Git cache byte overflow")?;
        }
        if bytes > CACHE_BYTES {
            return Err("Git cache byte limit".into());
        }
    }
    Ok(bytes)
}
async fn read(reader: impl tokio::io::AsyncRead + Unpin) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take(OUTPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .await?;
    if bytes.len() as u64 > OUTPUT_BYTES {
        return Err("Git subprocess output limit".into());
    }
    Ok(bytes)
}
pub struct Output {
    pub success: bool,
    pub stdout: Vec<u8>,
}
pub async fn run(cache: &Path, args: &[String]) -> Result<Output> {
    let mut command = Command::new("git");
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    // SSH agent and ordinary user SSH configuration stay device-local. Only
    // caller-selected askpass programs are restored; replicated data cannot set them.
    for key in ["GIT_ASKPASS", "SSH_ASKPASS"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "protocol.allow=never",
            "-c",
            "protocol.file.allow=always",
            "-c",
            "protocol.https.allow=always",
            "-c",
            "protocol.ssh.allow=always",
            "-c",
            "http.followRedirects=false",
            "-c",
            "fetch.fsckObjects=true",
            "-c",
            "receive.fsckObjects=true",
            "-c",
            "gc.auto=0",
            "-c",
            "maintenance.auto=false",
            "-c",
            "pack.windowMemory=32m",
            "-c",
            "pack.threads=1",
        ])
        .arg("--git-dir")
        .arg(cache)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(target_os = "macos")]
    {
        command.args(["-c", "credential.helper=osxkeychain"]);
    }
    command.args(args);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        command.as_std_mut().process_group(0);
    }
    let mut child = command.spawn()?;
    let mut group = ProcessGroup(child.id());
    let stdout = child.stdout.take().ok_or("Git stdout missing")?;
    let stderr = child.stderr.take().ok_or("Git stderr missing")?;
    let work = async {
        let streams = async {
            let (stdout, _) = tokio::try_join!(read(stdout), read(stderr))?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(stdout)
        };
        let watch = async {
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
                disk(cache)?;
            }
            #[allow(unreachable_code)]
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        };
        tokio::pin!(streams);
        tokio::pin!(watch);
        let stdout = tokio::select! {output=&mut streams => output?, checked=&mut watch => {checked?; return Err("Git monitor stopped".into());}};
        let status = child.wait().await?;
        group.0 = None;
        disk(cache)?;
        Ok(Output {
            success: status.success(),
            stdout,
        })
    };
    tokio::time::timeout(Duration::from_secs(60), work)
        .await
        .map_err(|_| "Git transport timeout; external outcome may be unknown")?
}
pub async fn fetch(
    cache: &Path,
    endpoint: &str,
    patterns: &[String],
) -> Result<BTreeMap<String, String>> {
    qualify(cache).await?;
    let mut args = vec![
        "fetch".into(),
        "--no-write-fetch-head".into(),
        "--no-tags".into(),
        "--prune".into(),
        "--".into(),
        endpoint.into(),
    ];
    // Inspect one completed fetch, including literal names that may be absent.
    // The bounded cache can contain other heads; only the requested scope is
    // converted or recorded in the native repository.
    args.push("+refs/heads/*:refs/bridge/heads/*".into());
    args.push("+refs/tags/*:refs/bridge/tags/*".into());
    if !run(cache, &args).await?.success {
        return Err("Git fetch failed; no import accepted".into());
    }
    let output = run(
        cache,
        &[
            "for-each-ref".into(),
            "--format=%(refname) %(objectname)".into(),
            "refs/bridge/".into(),
        ],
    )
    .await?;
    if !output.success {
        return Err("Git fetched ref inspection failed".into());
    }
    let mut refs = BTreeMap::new();
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let (name, oid) = line.split_once(' ').ok_or("Git ref output shape")?;
        let name = if let Some(name) = name.strip_prefix("refs/bridge/heads/") {
            format!("refs/heads/{name}")
        } else if let Some(name) = name.strip_prefix("refs/bridge/tags/") {
            format!("refs/tags/{name}")
        } else {
            return Err("Git fetched namespace mismatch".into());
        };
        if !valid_git_ref(&name)
            || !valid_git_oid(oid)
            || refs.insert(name, oid.into()).is_some()
            || refs.len() > 4096
        {
            return Err("Git fetched ref validation/limit".into());
        }
    }
    refs.retain(|name, _| {
        name.strip_prefix("refs/heads/").is_none_or(|name| {
            patterns
                .iter()
                .any(|pattern| match pattern.split_once('*') {
                    Some((prefix, suffix)) => {
                        name.starts_with(prefix)
                            && name.ends_with(suffix)
                            && name.len() >= prefix.len() + suffix.len()
                    }
                    None => pattern == name,
                })
        })
    });
    Ok(refs)
}
pub async fn qualify(cache: &Path) -> Result<()> {
    let output = run(cache, &["version".into()]).await?;
    if !output.success || std::str::from_utf8(&output.stdout)?.trim() != "git version 2.55.0" {
        return Err("NATIVE_GIT_CAPABILITY: this transport profile requires Git 2.55.0".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_monitor_rejects_excess_bytes_and_symlinks() {
        let cache = tempfile::tempdir().unwrap();
        let path = cache.path().join("sparse");
        std::fs::File::create(&path)
            .unwrap()
            .set_len(CACHE_BYTES + 1)
            .unwrap();
        assert!(disk(cache.path()).is_err());
        std::fs::remove_file(path).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/tmp", cache.path().join("link")).unwrap();
            assert!(disk(cache.path()).is_err());
        }
    }
}
