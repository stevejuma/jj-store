//! Test host's single-owner, fsynced private journal, with explicit fault points.
use jj_store_conversations::{capture::CaptureError, store_capture::*};
use jj_store_model::RequestId;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
    },
};

pub struct FileJournal {
    directory: PathBuf,
    lease: Mutex<()>,
    pub failure: AtomicU8,
    pub events: Arc<Mutex<Vec<&'static str>>>,
}
impl FileJournal {
    pub fn open(directory: PathBuf, events: Arc<Mutex<Vec<&'static str>>>) -> Self {
        std::fs::create_dir_all(&directory).unwrap();
        Self {
            directory,
            lease: Mutex::new(()),
            failure: AtomicU8::new(0),
            events,
        }
    }
    fn path(&self, request: RequestId) -> PathBuf {
        self.directory.join(format!("{request}.json"))
    }
    fn read(&self, request: RequestId) -> Result<Option<EntryV1>, CaptureError> {
        match std::fs::read(self.path(request)) {
            Ok(bytes) => EntryV1::decode(&bytes).map(Some),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err(CaptureError::Unknown),
        }
    }
    pub fn replace(&self, entry: &EntryV1) {
        let mut file = tempfile::NamedTempFile::new_in(&self.directory).unwrap();
        file.write_all(&entry.encode().unwrap()).unwrap();
        file.as_file().sync_all().unwrap();
        file.persist(self.path(entry.candidate.request())).unwrap();
        File::open(&self.directory).unwrap().sync_all().unwrap();
    }
}
#[async_trait::async_trait]
impl Journal for FileJournal {
    async fn load(&self, request: RequestId) -> Result<Option<EntryV1>, CaptureError> {
        let _lease = self.lease.lock().unwrap();
        self.events.lock().unwrap().push("load");
        self.read(request)
    }
    async fn begin(&self, candidate: &CandidateV1) -> Result<Claim, CaptureError> {
        let result = {
            let _lease = self.lease.lock().unwrap();
            self.events.lock().unwrap().push("begin");
            if self.failure.load(Ordering::SeqCst) == 1 {
                return Err(CaptureError::Rejected);
            }
            candidate.validate()?;
            if let Some(entry) = self.read(candidate.request())? {
                if entry.candidate != *candidate {
                    return Err(CaptureError::ReceiptMismatch);
                }
                Claim::Existing(Box::new(entry))
            } else {
                let bytes = EntryV1::new(candidate.clone())?.encode()?;
                let mut file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(self.path(candidate.request()))
                    .unwrap();
                file.write_all(&bytes).unwrap();
                file.sync_all().unwrap();
                File::open(&self.directory).unwrap().sync_all().unwrap();
                Claim::New
            }
        };
        if self.failure.load(Ordering::SeqCst) == 5 {
            std::future::pending::<()>().await;
        }
        Ok(result)
    }
    async fn prepared(
        &self,
        candidate: &CandidateV1,
        prepared: &PreparedV1,
    ) -> Result<(), CaptureError> {
        let _lease = self.lease.lock().unwrap();
        self.events.lock().unwrap().push("prepared");
        if self.failure.load(Ordering::SeqCst) == 2 {
            return Err(CaptureError::Rejected);
        }
        let mut entry = self
            .read(candidate.request())?
            .ok_or(CaptureError::Unknown)?;
        if entry.candidate != *candidate
            || entry.confirmation.is_some()
            || entry
                .prepared
                .as_ref()
                .is_some_and(|prior| prior != prepared)
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        entry.prepared = Some(prepared.clone());
        self.replace(&entry);
        Ok(())
    }
    async fn confirm(
        &self,
        candidate: &CandidateV1,
        confirmation: &ConfirmationV1,
    ) -> Result<(), CaptureError> {
        if self.failure.load(Ordering::SeqCst) == 4 {
            std::future::pending::<()>().await;
        }
        let _lease = self.lease.lock().unwrap();
        self.events.lock().unwrap().push("confirm");
        if self.failure.load(Ordering::SeqCst) == 3 {
            return Err(CaptureError::Unknown);
        }
        let mut entry = self
            .read(candidate.request())?
            .ok_or(CaptureError::Unknown)?;
        if entry.candidate != *candidate
            || entry
                .confirmation
                .as_ref()
                .is_some_and(|prior| prior != confirmation)
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        entry.confirmation = Some(confirmation.clone());
        self.replace(&entry);
        Ok(())
    }
}
