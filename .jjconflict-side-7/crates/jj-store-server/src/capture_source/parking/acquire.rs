//! Preserve parked channel ownership across failed or cancelled admission awaits.
use super::*;
use jj_store_capture::source_registry::{
    LeasedCodexTranscriptReader, RegisteredCodexSource, ReservedCodexPhysicalLease,
};
pub(super) enum Acquisition {
    Fresh(RegisteredCodexSource),
    Reserved(ReservedCodexPhysicalLease),
}
pub(super) async fn acquire(
    reader: &ParkedCodexTranscriptReader,
    sources: Arc<SourceRegistry>,
    context: &CaptureStepContext<'_>,
) -> Result<Option<Acquisition>> {
    if reader.has_channel_reservation() {
        // Automatic source steps already run on the owned blocking executor.
        // Never move the parked channel into a cancellable I/O closure.
        return reader
            .acquire_reserved(&sources)
            .map(|r| r.map(Acquisition::Reserved))
            .map_err(|_| Error::Service("capture parked reserved source unavailable"));
    }
    let qualified = reader.qualified().clone();
    let expected = reader.source_instance_id();
    let registered = context
        .admission
        .source_io(move || {
            if sources.lookup_admitted(&qualified)? != Some(expected) {
                return Err("capture parked channel changed".into());
            }
            sources.acquire_admitted(qualified)
        })
        .await
        .map_err(|_| Error::Service("capture parked acquisition unavailable"))?
        .map_err(|_| Error::Service("capture parked source unavailable"))?;
    if registered
        .as_ref()
        .is_some_and(|r| r.source_instance_id() != expected)
    {
        return Err(Error::Service("capture parked channel changed"));
    }
    Ok(registered.map(Acquisition::Fresh))
}
impl Acquisition {
    pub(super) fn open(
        self,
        parked: &mut ParkedCodexTranscriptReader,
        bounds: &jj_store_capture::reader_progress::ReaderSourceProgress,
        advanced: bool,
    ) -> Result<LeasedCodexTranscriptReader> {
        match self {
            Self::Reserved(acquired) => parked
                .resume_reserved_at(acquired, bounds, advanced)
                .map_err(|_| Error::Service("capture parked reserved reader unavailable"))?
                .ok_or(Error::Service("capture parked source changed")),
            Self::Fresh(registered) if advanced => registered
                .open_reader_at(bounds)
                .map_err(|_| Error::Service("capture parked reopen unavailable"))?
                .ok_or(Error::Service("capture parked source changed")),
            Self::Fresh(registered) => parked
                .resume_at(registered, bounds)
                .map_err(|_| Error::Service("capture parked reader unavailable")),
        }
    }
}
