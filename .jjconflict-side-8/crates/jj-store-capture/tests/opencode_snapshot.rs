use jj_store_capture::opencode_snapshot::*;
use serde_json::{Value, json};
fn info() -> Value {
    json!({"id":"ses_one", "version":PRODUCT_VERSION, "directory":"/workspace/one", "time":{"updated":9999999999999u64}})
}
fn message() -> Value {
    json!({"info":{"id":"msg_one", "sessionID":"ses_one", "role":"assistant", "path":{"cwd":"/workspace/two", "root":"/workspace"}}, "parts":[{"id":"prt_one", "sessionID":"ses_one", "messageID":"msg_one", "type":"text", "text":"Recorded content"}]})
}
fn bytes(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap()
}
#[test]
fn metadata_never_proves_activity_and_status_uses_pinned_idle_semantics() {
    let session = OpenCodeSessionSnapshot::from_api_info(&bytes(&info()), "ses_one").unwrap();
    assert_eq!(session.directory(), std::path::Path::new("/workspace/one"));
    assert!(
        !OpenCodeActivity::from_status_map(b"{}", "ses_one")
            .unwrap()
            .active()
    );
    assert!(
        OpenCodeActivity::from_status_map(br#"{"ses_one":{"type":"busy"}}"#, "ses_one")
            .unwrap()
            .active()
    );
    assert!(
        OpenCodeActivity::from_status_map(br#"{"ses_one":{"type":"retry"}}"#, "ses_one")
            .unwrap()
            .active()
    );
    assert_eq!(
        OpenCodeActivity::from_status_map(br#"{"ses_one":{"type":"future"}}"#, "ses_one").unwrap(),
        OpenCodeActivity::Unsupported
    );
}
#[test]
fn api_and_export_share_native_message_shape_and_keep_per_record_context() {
    let session = OpenCodeSessionSnapshot::from_api_info(&bytes(&info()), "ses_one").unwrap();
    let page = session.messages(&bytes(&json!([message()]))).unwrap();
    assert_eq!(
        page.records[0].directory(),
        Some(std::path::Path::new("/workspace/two"))
    );
    let (_, exported) = OpenCodeSessionSnapshot::from_export(&bytes(
        &json!({"info":info(), "messages":[message()]}),
    ))
    .unwrap();
    assert_eq!(page.records[0].envelope(), exported.records[0].envelope());
    let mut missing = message();
    missing["info"]["path"]["cwd"] = json!("../unverified");
    let page = session.messages(&bytes(&json!([missing]))).unwrap();
    assert_eq!(page.records[0].directory(), None);
}
#[test]
fn mismatched_owners_versions_paths_duplicates_and_oversize_pages_are_withheld() {
    let session = OpenCodeSessionSnapshot::from_api_info(&bytes(&info()), "ses_one").unwrap();
    for invalid in [
        json!({"id":"ses_two", "version":PRODUCT_VERSION, "directory":"/workspace/one"}),
        json!({"id":"ses_one", "version":"1.18.31", "directory":"/workspace/one"}),
        json!({"id":"ses_one", "version":PRODUCT_VERSION, "directory":"/workspace/../other"}),
    ] {
        assert!(OpenCodeSessionSnapshot::from_api_info(&bytes(&invalid), "ses_one").is_err());
    }
    let mut foreign = message();
    foreign["parts"][0]["sessionID"] = json!("ses_two");
    assert!(session.messages(&bytes(&json!([foreign]))).is_err());
    assert!(
        session
            .messages(&bytes(&json!([message(), message()])))
            .is_err()
    );
    assert!(
        session
            .messages(&vec![b' '; MAX_RESPONSE_BYTES + 1])
            .is_err()
    );
    assert!(
        session
            .messages(&bytes(&json!(vec![message(); MAX_PAGE_MESSAGES + 1])))
            .is_err()
    );
}
