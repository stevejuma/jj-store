use jj_agent::{
    AgentCatalogKey, AgentIndexPageV1, AgentObjectId, AgentObjectKind, AgentObjectRef,
    AgentObjectV1, AgentRootV1, AgentSessionId, ToolCallId, decode_object, decode_root,
    encode_object, encode_root,
};
use std::collections::BTreeMap;

fn object(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

#[test]
fn stable_ids_reject_noncanonical_or_cross_type_values() {
    let session: AgentSessionId = "11".repeat(16).parse().unwrap();
    let tool: ToolCallId = session.to_string().parse().unwrap();
    assert_eq!(session.to_string(), tool.to_string());
    assert!("11".repeat(15).parse::<AgentSessionId>().is_err());
    assert!("AA".repeat(16).parse::<AgentSessionId>().is_err());
}

#[test]
fn agent_root_and_index_pages_validate_strong_dependencies() {
    let session = object(AgentObjectKind::SessionSnapshot, 7);
    let key = AgentCatalogKey::derive("session", &[9; 16]).unwrap();
    let page = AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
        prefix: String::new(),
        entries: BTreeMap::from([(key, session)]),
    });
    assert_eq!(page.dependencies().unwrap(), vec![session]);

    let page_ref = object(AgentObjectKind::IndexPage, 8);
    let root = AgentRootV1 {
        sessions: Some(page_ref),
        ..AgentRootV1::empty()
    };
    assert_eq!(root.dependencies().unwrap(), vec![page_ref]);

    let wrong = AgentRootV1 {
        sessions: Some(session),
        ..AgentRootV1::empty()
    };
    assert!(wrong.validate().is_err());
}

#[test]
fn canonical_codec_rejects_unknown_fields_noncanonical_bytes_and_wrong_ids() {
    let root = AgentRootV1::empty();
    let bytes = encode_root(&root).unwrap();
    assert_eq!(bytes, br#"{"schema_version":1}"#);
    assert_eq!(decode_root(&bytes).unwrap(), root);
    assert!(decode_root(br#"{ "schema_version": 1 }"#).is_err());
    assert!(decode_root(br#"{"schema_version":1,"future":true}"#).is_err());

    let value = AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
        prefix: String::new(),
        entries: BTreeMap::new(),
    });
    let encoded = encode_object(&value).unwrap();
    assert_eq!(decode_object(&encoded).unwrap(), value);

    let wrong_prefix = AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
        prefix: "f".into(),
        entries: BTreeMap::new(),
    });
    assert!(encode_object(&wrong_prefix).is_err());
}
