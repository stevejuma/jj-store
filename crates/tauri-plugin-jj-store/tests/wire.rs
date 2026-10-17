use tauri_plugin_jj_store::{Error, frames, wire};

#[test]
fn text_envelopes_preserve_full_width_integers_unicode_and_exact_escaping() {
    let source =
        serde_json::json!({"generation":u64::MAX,"time":i64::MIN,"text":"笔记 \"quote\"\\line"});
    let encoded = wire::encode(&source).unwrap();
    assert!(encoded.contains("18446744073709551615"));
    assert!(encoded.contains("-9223372036854775808"));
    // Tauri carries a JSON string; its outer parse never interprets inner numbers.
    let outer = serde_json::to_string(&encoded).unwrap();
    let retained: String = serde_json::from_str(&outer).unwrap();
    assert_eq!(retained, encoded);
    let decoded: serde_json::Value = wire::decode(&retained).unwrap();
    assert_eq!(decoded, source);
}
#[test]
fn envelope_version_shape_and_byte_limits_fail_closed() {
    for input in [
        r#"{"version":0,"data":{"vault":"token"}}"#,
        r#"{"version":2,"data":{"vault":"token"}}"#,
        r#"{"version":1,"data":{"vault":"token","root":"outside"}}"#,
        r#"{"version":1,"data":{"vault":"token"},"window":"other"}"#,
        r#"{"vault":"token"}"#,
    ] {
        assert!(matches!(
            wire::decode::<frames::Vault>(input),
            Err(Error::Invalid)
        ));
    }
    assert!(matches!(
        wire::decode::<frames::Vault>(&" ".repeat(wire::MAX_REQUEST_BYTES + 1)),
        Err(Error::Limit)
    ));
    assert!(matches!(
        wire::encode("x".repeat(wire::MAX_RESPONSE_BYTES + 1)),
        Err(Error::Limit)
    ));
    let valid = wire::encode(frames::Vault {
        vault: "token".into(),
    })
    .unwrap();
    assert_eq!(
        wire::decode::<frames::Vault>(&valid).unwrap().vault,
        "token"
    );
}
