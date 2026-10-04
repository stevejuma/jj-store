use super::*;

#[tokio::test]
async fn changed_reader_association_withholds_body_work_and_cannot_retarget_original_intent() {
    let mut f = ImportFixture::new().await;
    f.f.message(
        "history",
        "history withheld after changed reader association",
    );
    let mut pipeline = enroll(&mut f).await;
    let original = pipeline.receipt().clone();
    let path = f.config.state_dir.join("agent-capture/imports/state.json");
    let before = fs::read(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&before).unwrap();
    let mut changed = *pipeline.partition().source.as_bytes();
    changed[0] ^= 1;
    let changed = jj_store_service::collaboration::AgentSourceInstanceIdentity::new(changed);
    value["readers"][0]["source"] = serde_json::to_value(changed).unwrap();
    let bytes = serde_json::to_vec(&value).unwrap();
    fs::write(&path, &bytes).unwrap();
    assert!(pipeline.ingest_step(&mut f.f.context()).await.is_err());
    assert!(f.f.spool.pending().unwrap().is_empty());
    assert_eq!(f.imports.entries().unwrap(), vec![original.clone()]);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    fs::write(&path, before).unwrap();
    enqueue(&mut f, &mut pipeline).await;
    assert_eq!(pipeline.receipt(), &original);
    publish(&mut f, &mut pipeline).await;
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn accepted_import_selects_a_durable_reader_association_before_returning_its_pipeline() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "accepted history awaiting publication");
    let pipeline = enroll(&mut f).await;
    let original = pipeline.receipt().clone();
    let path = f.config.state_dir.join("agent-capture/imports/state.json");
    let catalog: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let readers = catalog["readers"]
        .as_array()
        .expect("accepted import reader association missing");
    assert_eq!(readers.len(), 1);
    assert_eq!(readers[0]["ordinal"], original.ordinal);
    assert_eq!(
        readers[0]["source"],
        serde_json::to_value(pipeline.partition().source).unwrap()
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    drop(pipeline);
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn exact_reader_association_survives_reopen_tail_and_absent_transcript_lookup() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "original accepted prefix");
    let mut pipeline = enroll(&mut f).await;
    let original = pipeline.receipt().clone();
    let binding = f.imports.reader_binding(&f.intent).unwrap().unwrap();
    assert_eq!(binding.source, pipeline.partition().source);
    assert_eq!(
        binding.generation_digest,
        f.f.qualified().generation_digest()
    );
    assert!(
        original
            .source
            .codex()
            .matches_generation_digest(binding.generation_digest)
    );
    enqueue(&mut f, &mut pipeline).await;
    publish(&mut f, &mut pipeline).await;
    drop(pipeline);
    f.f.message(
        "tail",
        "later transcript bytes cannot extend accepted intent",
    );
    let mut replay = enroll(&mut f).await;
    assert_eq!(replay.receipt(), &original);
    assert_eq!(
        f.imports.reader_binding(&f.intent).unwrap(),
        Some(binding.clone())
    );
    enqueue(&mut f, &mut replay).await;
    publish(&mut f, &mut replay).await;
    drop(replay);
    let path = f.config.state_dir.join("agent-capture/imports");
    let reopened = crate::capture_import_state::CaptureImportJournal::open(
        &path,
        [0xab; 16],
        f.intent.generation,
    )
    .unwrap();
    assert_eq!(
        reopened.reader_binding(&f.intent).unwrap(),
        Some(binding.clone())
    );
    let before = fs::read(path.join("state.json")).unwrap();
    fs::rename(&f.f.transcript, f.f.transcript.with_extension("held")).unwrap();
    assert_eq!(reopened.reader_binding(&f.intent).unwrap(), Some(binding));
    assert_eq!(reopened.entries().unwrap(), vec![original]);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    let body = String::from_utf8(before).unwrap();
    for denied in [
        "original accepted prefix",
        "native_session_key",
        "payload",
        f.f.root.path().to_str().unwrap(),
    ] {
        assert!(!body.contains(denied));
    }
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn receipt_only_import_is_unqualified_until_original_source_admission_reconstructs_reader_link()
 {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "history accepted before reader association");
    let original = f
        .imports
        .record(&f.intent, &f.source_binding())
        .unwrap()
        .receipt;
    assert!(f.imports.reader_binding(&f.intent).unwrap().is_none());
    let host = f.f.host.clone();
    f.f.without_native_authority();
    let mut import = prepare(&f);
    assert!(import.step(&mut f.f.context()).await.is_err());
    drop(import);
    assert!(f.imports.reader_binding(&f.intent).unwrap().is_none());
    assert_eq!(f.imports.entries().unwrap(), vec![original.clone()]);
    f.f.host = host;
    let pipeline = enroll(&mut f).await;
    assert_eq!(pipeline.receipt(), &original);
    assert!(f.imports.reader_binding(&f.intent).unwrap().is_some());
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}
