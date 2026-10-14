//! Parent-pipe supervised executor; loss of its host stops and reaps the owned group.
use std::{io::Read as _, process::Stdio, sync::mpsc, time::Duration};
pub const MARKER: &str = "__jj-store-workflow-worker-v1";
pub const RUNNER_MARKER: &str = "__jj-store-forgejo-worker-v1";
pub fn entry() -> Option<u8> {
    let mut args = std::env::args_os().skip(1);
    let marker = args.next()?;
    let runner = marker.as_os_str() == RUNNER_MARKER;
    if marker.as_os_str() != MARKER && !runner {
        return None;
    }
    #[cfg(not(unix))]
    {
        return Some(1);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        let seconds = args.next()?.to_str()?.parse::<u64>().ok()?;
        if !(1..=86400).contains(&seconds) {
            return Some(1);
        }
        let cleanup = if runner {
            Some((args.next()?, args.next()?.to_str()?.to_owned()))
        } else {
            None
        };
        let program = args.next()?;
        if !std::path::Path::new(&program).is_absolute() {
            return Some(1);
        }
        if let Some((docker, attempt)) = &cleanup {
            if attempt.len() != 32
                || !attempt.bytes().all(|b| b.is_ascii_hexdigit())
                || !std::path::Path::new(docker).is_absolute()
            {
                return Some(1);
            }
            let label = format!("jj-store.attempt={attempt}");
            let network = format!("jj-store-{attempt}");
            if docker_call(docker, &["network", "create", "--label", &label, &network]).is_none() {
                return Some(1);
            }
        }
        let mut command = std::process::Command::new(program);
        command.args(args).stdin(Stdio::null()).process_group(0);
        let Ok(mut child) = command.spawn() else {
            if let Some((docker, attempt)) = &cleanup
                && !cleanup_runner(docker, attempt)
            {
                return Some(2);
            }
            return Some(1);
        };
        let id = child.id();
        let kill = move || {
            if let Some(pid) = rustix::process::Pid::from_raw(id as i32) {
                let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
            }
        };
        let (sender, receiver) = mpsc::channel();
        let loss = sender.clone();
        std::thread::spawn(move || {
            let _ = std::io::stdin().read(&mut [0]);
            let _ = loss.send(false);
        });
        let monitor = std::thread::spawn(move || {
            if receiver.recv_timeout(Duration::from_secs(seconds)) != Ok(true) {
                if runner {
                    if let Some(pid) = rustix::process::Pid::from_raw(id as i32) {
                        let _ =
                            rustix::process::kill_process_group(pid, rustix::process::Signal::TERM);
                    }
                    std::thread::sleep(Duration::from_secs(10));
                }
                kill();
            }
        });
        if let Some(pid) = rustix::process::Pid::from_raw(id as i32) {
            loop {
                match rustix::process::waitid(
                    rustix::process::WaitId::Pid(pid),
                    rustix::process::WaitIdOptions::EXITED | rustix::process::WaitIdOptions::NOWAIT,
                ) {
                    Ok(_) => break,
                    Err(rustix::io::Errno::INTR) => {}
                    Err(_) => break,
                }
            }
        }
        let _ = sender.send(true);
        let _ = monitor.join();
        kill();
        let successful = child.wait().is_ok_and(|s| s.success());
        if let Some((docker, attempt)) = cleanup
            && !cleanup_runner(&docker, &attempt)
        {
            return Some(2);
        }
        Some(if successful { 0 } else { 1 })
    }
}

#[cfg(unix)]
fn cleanup_runner(docker: &std::ffi::OsStr, attempt: &str) -> bool {
    if attempt.len() != 32
        || !attempt.bytes().all(|b| b.is_ascii_hexdigit())
        || !std::path::Path::new(docker).is_absolute()
    {
        return false;
    }
    let label = format!("label=jj-store.attempt={attempt}");
    let Some(bytes) = docker_call(docker, &["ps", "-aq", "--filter", &label]) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        return false;
    };
    let ids: Vec<_> = text.split_whitespace().collect();
    if ids.len() > 256
        || ids
            .iter()
            .any(|id| !id.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return false;
    }
    if !ids.is_empty() {
        let mut args = vec!["rm", "-f"];
        args.extend(ids);
        if docker_call(docker, &args).is_none() {
            return false;
        }
    }
    let Some(bytes) = docker_call(docker, &["network", "ls", "-q", "--filter", &label]) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        return false;
    };
    for id in text.split_whitespace() {
        if !id.bytes().all(|b| b.is_ascii_hexdigit())
            || docker_call(docker, &["network", "rm", id]).is_none()
        {
            return false;
        }
    }
    docker_call(docker, &["ps", "-aq", "--filter", &label])
        .is_some_and(|v| v.iter().all(u8::is_ascii_whitespace))
}

#[cfg(unix)]
fn docker_call(program: &std::ffi::OsStr, args: &[&str]) -> Option<Vec<u8>> {
    let file = tempfile::tempfile().ok()?;
    let mut child = std::process::Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(file.try_clone().ok()?)
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().ok()? {
            if !status.success() {
                return None;
            }
            use std::io::{Read as _, Seek as _};
            let mut file = file;
            file.seek(std::io::SeekFrom::Start(0)).ok()?;
            let mut bytes = Vec::new();
            file.take(65537).read_to_end(&mut bytes).ok()?;
            return (bytes.len() <= 65536).then_some(bytes);
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
