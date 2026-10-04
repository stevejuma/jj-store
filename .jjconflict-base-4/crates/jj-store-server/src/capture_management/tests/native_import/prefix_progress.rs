use super::*;

#[tokio::test]
async fn fresh_import_cannot_reuse_published_reader_history_from_a_rewritten_prefix() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "BEFORE transcript history");
    let mut pipeline = enroll(&mut f).await;
    enqueue(&mut f, &mut pipeline).await;
    publish(&mut f, &mut pipeline).await;
    let original = f.imports.entries().unwrap().remove(0);
    drop(pipeline);
    let text = fs::read_to_string(&f.f.transcript).unwrap();
    fs::write(&f.f.transcript, text.replace("BEFORE", "AFTER!")).unwrap();
    f.intent.request = RequestId::new([6; 16]);
    let mut next = prepare(&f);
    let mut refused = false;
    for _ in 0..10 {
        match next.step(&mut f.f.context()).await {
            Err(_) => {
                refused = true;
                break;
            }
            Ok(CaptureImportStep::Ready(_)) => {
                panic!("rewritten bytes inherited an association to old publication receipts")
            }
            Ok(_) => {}
        }
    }
    assert!(refused, "retained prefix verification never settled");
    drop(next);
    assert_eq!(f.imports.entries().unwrap()[0], original);
    assert!(f.imports.reader_binding(&f.intent).unwrap().is_none());
    f.runtime.shutdown().await.unwrap();
}
