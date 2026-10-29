//! Native preparation/replay proof with memory object storage, not RustFS.
use super::*;
use jj_store_conversations::{capture::*, privacy::Sanitizer, store_capture::*};
use jj_store_model::RequestId;
use serde_json::{Value, json};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

struct JournalFile {
    path: PathBuf,
    lease: Mutex<()>,
}
impl JournalFile {
    fn read(&self, request: RequestId) -> std::result::Result<Option<EntryV1>, CaptureError> {
        match std::fs::read(self.path.join(request.to_string())) {
            Ok(bytes) => EntryV1::decode(&bytes).map(Some),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err(CaptureError::Unknown),
        }
    }
    fn write(&self, entry: &EntryV1) {
        let mut file = tempfile::NamedTempFile::new_in(&self.path).unwrap();
        file.write_all(&entry.encode().unwrap()).unwrap();
        file.as_file().sync_all().unwrap();
        file.persist(self.path.join(entry.candidate.request().to_string()))
            .unwrap();
        File::open(&self.path).unwrap().sync_all().unwrap();
    }
}
#[async_trait::async_trait]
impl Journal for JournalFile {
    async fn load(&self, request: RequestId) -> std::result::Result<Option<EntryV1>, CaptureError> {
        let _lease = self.lease.lock().unwrap();
        self.read(request)
    }
    async fn begin(&self, candidate: &CandidateV1) -> std::result::Result<Claim, CaptureError> {
        let _lease = self.lease.lock().unwrap();
        if let Some(entry) = self.read(candidate.request())? {
            if entry.candidate != *candidate {
                return Err(CaptureError::ReceiptMismatch);
            }
            return Ok(Claim::Existing(Box::new(entry)));
        }
        let bytes = EntryV1::new(candidate.clone())?.encode()?;
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path.join(candidate.request().to_string()))
            .unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all().unwrap();
        File::open(&self.path).unwrap().sync_all().unwrap();
        Ok(Claim::New)
    }
    async fn prepared(
        &self,
        candidate: &CandidateV1,
        prepared: &PreparedV1,
    ) -> std::result::Result<(), CaptureError> {
        let _lease = self.lease.lock().unwrap();
        let mut entry = self
            .read(candidate.request())?
            .ok_or(CaptureError::Unknown)?;
        if entry.candidate != *candidate || entry.prepared.as_ref().is_some_and(|p| p != prepared) {
            return Err(CaptureError::ReceiptMismatch);
        }
        entry.prepared = Some(prepared.clone());
        self.write(&entry);
        Ok(())
    }
    async fn confirm(
        &self,
        candidate: &CandidateV1,
        confirmation: &ConfirmationV1,
    ) -> std::result::Result<(), CaptureError> {
        let _lease = self.lease.lock().unwrap();
        let mut entry = self
            .read(candidate.request())?
            .ok_or(CaptureError::Unknown)?;
        if entry.candidate != *candidate
            || entry
                .confirmation
                .as_ref()
                .is_some_and(|p| p != confirmation)
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        entry.confirmation = Some(confirmation.clone());
        self.write(&entry);
        Ok(())
    }
}
struct Privacy;
impl Sanitizer for Privacy {
    fn sanitize(&self, value: Value) -> jj_store_conversations::Result<Value> {
        Ok(value)
    }
    fn replace_stream_text(
        &self,
        _: &mut String,
        chunk: &str,
        _: bool,
    ) -> jj_store_conversations::Result<String> {
        Ok(chunk.into())
    }
}

#[tokio::test]
async fn native_sink_retains_exact_preparation_and_recovers_unavailable_original_receipts_read_only()
 {
    for mode in [0, 2, 4] {
        let (storage, c, host, stores) = fixture().await;
        let t = committed(stores.thread().unwrap().execute(&thread(81)).await.unwrap());
        let context = ContextV1 {
            repository: t.repository,
            thread: jj_collab::ThreadId::new([81; 16]),
            binding: jj_agent::NativeCaptureBindingV1 {
                scope: "vault".into(),
                worker: "worker".into(),
                request: "open".into(),
                target: "target".into(),
                window: "main".into(),
                admission: "generation".into(),
            },
            policy: "fixture/v1".into(),
        };
        let directory = tempfile::tempdir().unwrap();
        let journal = Arc::new(JournalFile {
            path: directory.path().into(),
            lease: Mutex::new(()),
        });
        let stores = Arc::new(stores);
        let sink = StoreCapture::new(context.clone(), stores.clone(), journal.clone()).unwrap();
        let mut owner = NativeCapture::new(
            Binding {
                scope: "vault".into(),
                worker: "worker".into(),
                request: "open".into(),
                target: "target".into(),
                window: "main".into(),
                admission: "generation".into(),
            },
            context.policy.clone(),
            Arc::new(Privacy),
            Box::new(sink),
        )
        .unwrap();
        storage.mode.store(mode, Ordering::SeqCst);
        let frame = json!({"version":3,"type":"opened","sessionId":"worker","requestId":"open","adapterSessionId":"provider","sequence":2});
        let output = owner
            .capture(Input {
                worker: "worker",
                ordinal: 1,
                publication: Publication::Facts,
                frame: &frame,
            })
            .await;
        if mode == 0 {
            assert!(output.is_ok());
        } else {
            assert_eq!(output.err(), Some(CaptureError::Unknown));
        }
        storage.read_blocked.store(false, Ordering::SeqCst);
        storage.read_missing.store(false, Ordering::SeqCst);
        let entry = journal.load(context.request(1)).await.unwrap().unwrap();
        let prepared = entry.prepared.as_ref().unwrap();
        assert!(matches!(prepared.command, PreparedCommandV1::Native { .. }));
        assert_eq!(
            prepared.publication_id.unwrap().as_bytes(),
            context.request(1).as_bytes()
        );
        assert_eq!(EntryV1::decode(&entry.encode().unwrap()).unwrap(), entry);
        let snapshot = c.read().await.unwrap();
        assert_eq!(snapshot.manifest().agent_state_version, 1);
        assert_eq!(snapshot.manifest().thread_state_version, 2);
        committed(stores.thread().unwrap().execute(&thread(82)).await.unwrap());
        c.checkpoint().await.unwrap();
        c.recover(100).await.unwrap();
        drop(owner);
        drop(stores);
        drop(journal);
        let stores = Arc::new(open(
            coordinator(storage.clone()),
            host.clone(),
            |_, _, _| {},
        ));
        let journal = Arc::new(JournalFile {
            path: directory.path().into(),
            lease: Mutex::new(()),
        });
        let sink = StoreCapture::new(context.clone(), stores, journal.clone()).unwrap();
        let puts = storage.puts.load(Ordering::SeqCst);
        let confirmation = sink.recover(1).await.unwrap();
        assert_eq!(confirmation.version, 1);
        assert_eq!(confirmation.batch, None);
        assert_eq!(confirmation.actor, ACTOR);
        assert_eq!(
            journal
                .load(context.request(1))
                .await
                .unwrap()
                .unwrap()
                .prepared,
            entry.prepared
        );
        assert_eq!(sink.recover(1).await.unwrap(), confirmation);
        assert_eq!(
            storage.puts.load(Ordering::SeqCst),
            puts,
            "receipt recovery stages no native objects"
        );
        host.denied.store(true, Ordering::SeqCst);
        assert_eq!(sink.recover(1).await.err(), Some(CaptureError::Unknown));
    }
}
