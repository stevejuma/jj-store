//! Single bounded body-free mailbox; no body, policy guard, frame or storage owner.
use super::*;
use crate::capture_management::CaptureImportDispatch;
pub(super) struct ImportCommand {
    pub dispatch: CaptureImportDispatch,
    pub settled: tokio::sync::oneshot::Sender<Result<()>>,
}
impl CaptureRuntimeControl {
    /// Library dispatch only. This acknowledges volatile scheduling, not durable intent.
    /// An unknown acknowledgement requires the original command; never mint a new one.
    pub async fn schedule_import(&self, dispatch: CaptureImportDispatch) -> Result<()> {
        self.admit_import()?;
        let (settled, completion) = tokio::sync::oneshot::channel();
        self.imports
            .try_send(ImportCommand { dispatch, settled })
            .map_err(|_| Error::Service("capture import scheduling unavailable"))?;
        completion
            .await
            .map_err(|_| Error::Service("capture import scheduling outcome unknown"))?
    }
}
#[cfg(test)]
pub(super) fn empty() -> tokio::sync::mpsc::Sender<ImportCommand> {
    tokio::sync::mpsc::channel(1).0
}
