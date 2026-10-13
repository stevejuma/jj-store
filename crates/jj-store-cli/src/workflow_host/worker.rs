//! Parent-pipe supervised executor; loss of its host stops and reaps the owned group.
use std::{io::Read as _, process::Stdio, sync::mpsc, time::Duration};
pub const MARKER: &str = "__jj-store-workflow-worker-v1";
pub fn entry() -> Option<u8> {
    let mut args = std::env::args_os().skip(1);
    if args.next()?.as_os_str() != MARKER {
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
        let program = args.next()?;
        if !std::path::Path::new(&program).is_absolute() {
            return Some(1);
        }
        let mut command = std::process::Command::new(program);
        command.args(args).stdin(Stdio::null()).process_group(0);
        let Ok(mut child) = command.spawn() else {
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
        Some(if child.wait().is_ok_and(|s| s.success()) {
            0
        } else {
            1
        })
    }
}
