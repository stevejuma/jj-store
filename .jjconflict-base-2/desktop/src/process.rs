//! Bounded child output and owned stdin lifetime, with no PID-based adoption.
use crate::Result;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    time::{Duration, Instant},
};
const OUTPUT_LIMIT: usize = 64 * 1024;
pub struct OwnedProcess {
    child: Child,
    owner: Option<ChildStdin>,
    lines: mpsc::Receiver<Result<String>>,
    errors: Arc<Mutex<VecDeque<u8>>>,
}
impl OwnedProcess {
    pub fn spawn(command: &mut Command) -> Result<Self> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Cannot start bundled service: {e}"))?;
        let owner = child.stdin.take();
        let output = child.stdout.take().ok_or("Missing readiness pipe")?;
        let stderr = child.stderr.take().ok_or("Missing error pipe")?;
        let errors = Arc::new(Mutex::new(VecDeque::new()));
        let captured = errors.clone();
        std::thread::spawn(move || {
            let mut reader = stderr;
            let mut chunk = [0_u8; 4096];
            while let Ok(count) = reader.read(&mut chunk) {
                if count == 0 {
                    break;
                }
                if let Ok(mut bytes) = captured.lock() {
                    bytes.extend(&chunk[..count]);
                    while bytes.len() > OUTPUT_LIMIT {
                        bytes.pop_front();
                    }
                }
            }
        });
        let (send, lines) = mpsc::sync_channel(1);
        std::thread::spawn(move || {
            let mut output = BufReader::new(output);
            let mut line = Vec::new();
            let read = output
                .by_ref()
                .take((OUTPUT_LIMIT + 1) as u64)
                .read_until(b'\n', &mut line);
            let result = match read {
                Ok(_) if line.len() > OUTPUT_LIMIT => {
                    Err("Readiness output exceeded 64 KiB".into())
                }
                Ok(_) => String::from_utf8(line).map_err(|_| "Invalid readiness encoding".into()),
                Err(error) => Err(format!("Cannot read service readiness: {error}")),
            };
            let _ = send.send(result);
            let _ = std::io::copy(&mut output, &mut std::io::sink());
        });
        Ok(Self {
            child,
            owner,
            lines,
            errors,
        })
    }
    pub fn first_line(&self, timeout: Duration) -> Result<String> {
        self.lines
            .recv_timeout(timeout)
            .map_err(|e| format!("Service readiness unavailable: {e}; {}", self.diagnostics()))?
    }
    pub fn diagnostics(&self) -> String {
        self.errors
            .lock()
            .map(|b| String::from_utf8_lossy(&b.iter().copied().collect::<Vec<_>>()).into_owned())
            .unwrap_or_else(|_| "Service diagnostics unavailable".into())
    }
    pub fn running(&mut self) -> Result<bool> {
        self.child
            .try_wait()
            .map(|s| s.is_none())
            .map_err(|e| e.to_string())
    }
    pub fn finish(&mut self, timeout: Duration) -> Result<()> {
        self.owner.take();
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(status) = self.child.try_wait().map_err(|e| e.to_string())? {
                return if status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "Service exited with {status}: {}",
                        self.diagnostics()
                    ))
                };
            }
            if Instant::now() >= deadline {
                self.child.kill().map_err(|e| e.to_string())?;
                self.child.wait().map_err(|e| e.to_string())?;
                return Err("Service shutdown timed out; in-flight outcomes may be unknown".into());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}
impl Drop for OwnedProcess {
    fn drop(&mut self) {
        let _ = self.finish(Duration::from_secs(20));
    }
}
