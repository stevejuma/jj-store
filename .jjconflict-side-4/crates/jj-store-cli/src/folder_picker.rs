//! Fixed native dialog adapter. No request data is ever used as a command.
use jj_store_service::local_management::{
    FolderPicker, FolderSelection, MAX_PATH_BYTES, PickerError,
};
use std::{path::Path, process::Stdio, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::Command,
    sync::{Mutex, oneshot},
};

const SCRIPT: &str = r#"try
    return POSIX path of (choose folder with prompt "Select a local workspace folder")
on error number -128
    return ""
end try"#;

pub(crate) fn native_picker() -> Option<Arc<dyn FolderPicker>> {
    if cfg!(target_os = "macos") {
        Some(Arc::new(NativePicker::default()))
    } else {
        None
    }
}

#[derive(Default)]
struct NativePicker {
    active: Arc<Mutex<()>>,
}

#[async_trait::async_trait]
impl FolderPicker for NativePicker {
    async fn pick(&self) -> Result<FolderSelection, PickerError> {
        let mut command = Command::new("/usr/bin/osascript");
        command.args(["-e", SCRIPT]);
        self.run(command, Duration::from_secs(25)).await
    }
}

impl NativePicker {
    async fn run(
        &self,
        mut command: Command,
        deadline: Duration,
    ) -> Result<FolderSelection, PickerError> {
        let permit = self
            .active
            .clone()
            .try_lock_owned()
            .map_err(|_| PickerError::Busy)?;
        let mut child = command
            .current_dir("/")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|_| PickerError::Unavailable)?;
        let stdout = child.stdout.take().ok_or(PickerError::Unavailable)?;
        let stderr = child.stderr.take().ok_or(PickerError::Unavailable)?;
        let (mut send, receive) = oneshot::channel();
        // This owner survives request-future cancellation long enough to kill and
        // reap the helper. Keep the single-flight permit until cleanup finishes.
        tokio::spawn(async move {
            let result = tokio::select! {
                _ = send.closed() => Err(PickerError::Unavailable),
                _ = tokio::time::sleep(deadline) => Err(PickerError::Timeout),
                result = async {
                    let (status, output, _) = tokio::try_join!(
                        async { child.wait().await.map_err(|_| PickerError::Unavailable) },
                        bounded(stdout),
                        bounded(stderr),
                    )?;
                    if !status.success() {
                        return Err(PickerError::Unavailable);
                    }
                    selection(&output)
                } => result,
            };
            if result.is_err() {
                // kill() also waits, including when a deadline interrupted wait().
                let _ = child.kill().await;
            }
            drop(permit);
            let _ = send.send(result);
        });
        receive.await.map_err(|_| PickerError::Unavailable)?
    }
}

async fn bounded(reader: impl AsyncRead + Unpin) -> Result<Vec<u8>, PickerError> {
    let mut bytes = Vec::new();
    reader
        .take((MAX_PATH_BYTES + 2) as u64)
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| PickerError::Unavailable)?;
    if bytes.len() > MAX_PATH_BYTES + 1 {
        return Err(PickerError::Invalid);
    }
    Ok(bytes)
}

fn selection(output: &[u8]) -> Result<FolderSelection, PickerError> {
    let output = std::str::from_utf8(output).map_err(|_| PickerError::Invalid)?;
    // Remove only the helper's line terminator; spaces are valid path bytes.
    let path = output.strip_suffix('\n').unwrap_or(output);
    if !path.is_empty()
        && (path.len() > MAX_PATH_BYTES || path.contains('\0') || !Path::new(path).is_absolute())
    {
        return Err(PickerError::Invalid);
    }
    Ok(FolderSelection {
        version: 1,
        path: (!path.is_empty()).then(|| path.to_owned()),
    })
}

#[cfg(all(test, unix))]
#[path = "folder_picker_tests.rs"]
mod tests;
