//! Bounded execution of a fixed JJ binary, never a shell or browser command.
use super::{Provider, Result};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{io::AsyncReadExt, process::Command};
impl Provider {
    pub(super) async fn run(&self, root: &Path, args: &[&str]) -> Result<String> {
        let mut command = Command::new(&self.executable);
        command
            .current_dir(root)
            .args(["--no-pager", "--config", "fsmonitor.backend=none", "-R"])
            .arg(root)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let mut child = command.spawn().map_err(|_| "jj_executable_unavailable")?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or("jj_output_unavailable")?
            .take(32769);
        let mut bytes = Vec::new();
        let result = tokio::time::timeout(Duration::from_secs(60), async {
            stdout
                .read_to_end(&mut bytes)
                .await
                .map_err(|_| "jj_output_unavailable")?;
            if bytes.len() > 32768 {
                return Err("jj_output_limit");
            }
            if !child
                .wait()
                .await
                .map_err(|_| "jj_outcome_unknown")?
                .success()
            {
                return Err("jj_command_failed");
            }
            String::from_utf8(bytes.clone()).map_err(|_| "jj_output_invalid")
        })
        .await;
        match result {
            Ok(Ok(value)) => Ok(value),
            other => {
                #[cfg(unix)]
                if let Some(pid) = child
                    .id()
                    .and_then(|pid| rustix::process::Pid::from_raw(pid as i32))
                {
                    let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
                }
                let _ = child.start_kill();
                let _ = tokio::time::timeout(Duration::from_secs(5), child.wait()).await;
                match other {
                    Ok(Err(error)) => Err(error),
                    _ => Err("jj_outcome_unknown"),
                }
            }
        }
    }
}
