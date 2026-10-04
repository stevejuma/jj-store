//! Invocation-owned transport supervisor. Parent-pipe loss covers SIGKILL;
//! the supervisor alone owns and reaps the Git process group.
use std::{io::Read as _, process::Stdio, sync::mpsc, time::Duration};
pub const MARKER: &str = "__jj-store-git-worker-v1";
#[cfg(unix)]
fn kill_group(pid: u32) {
    if let Some(pid) = rustix::process::Pid::from_raw(pid as i32) {
        let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
    }
}
/// Internal executable entry point, before config, credentials or CliRunner.
/// Arguments originate from the invoking CLI, never from native records.
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
        let mut command = std::process::Command::new("git");
        command.args(args).stdin(Stdio::null()).process_group(0);
        let Ok(mut child) = command.spawn() else {
            return Some(1);
        };
        let pid = child.id();
        let (sender, receiver) = mpsc::channel();
        let loss = sender.clone();
        std::thread::spawn(move || {
            let mut byte = [0];
            // This pipe carries no credentials or data. EOF or any unexpected
            // input cancels the invocation. SIGKILL closes it in the kernel.
            let _ = std::io::stdin().read(&mut byte);
            let _ = loss.send(false);
        });
        let monitor = std::thread::spawn(move || {
            if receiver.recv_timeout(Duration::from_secs(60)) != Ok(true) {
                kill_group(pid);
            }
        });
        // Keep the exited leader unreaped until both cleanup owners finish.
        // Its PID cannot be recycled into an unrelated process group.
        if let Some(pid) = rustix::process::Pid::from_raw(pid as i32) {
            while let Err(error) = rustix::process::waitid(
                rustix::process::WaitId::Pid(pid),
                rustix::process::WaitIdOptions::EXITED | rustix::process::WaitIdOptions::NOWAIT,
            ) {
                if error != rustix::io::Errno::INTR {
                    break;
                }
            }
        }
        let _ = sender.send(true);
        let _ = monitor.join();
        // Kill any remaining owned children retaining output descriptors.
        kill_group(pid);
        let status = child.wait();
        Some(if status.is_ok_and(|v| v.success()) {
            0
        } else {
            1
        })
    }
}
