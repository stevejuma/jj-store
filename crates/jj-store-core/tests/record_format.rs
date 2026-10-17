use jj_store_core::{Error, record_format::*};
use jj_store_model::RepositoryId;

#[test]
fn oversized_untrusted_control_bytes_are_rejected_before_json_decoding() {
    let bytes = vec![b' '; MAX_RECORD_BYTES as usize + 1];
    assert!(matches!(
        decode::<Envelope<u32, String>>(&bytes),
        Err(Error::Limit("control record bytes"))
    ));
}

#[test]
fn existing_repository_envelope_bytes_are_preserved() {
    let envelope = Envelope {
        format: 1_u32,
        repository: RepositoryId::new([1; 16]),
        kind: "agent_root",
        payload: 7_u32,
    };
    let bytes = encode(&envelope, 1024).unwrap();
    assert_eq!(
        String::from_utf8(bytes.clone()).unwrap(),
        "{\"format\":1,\"repository\":\"01010101010101010101010101010101\",\"kind\":\"agent_root\",\"payload\":7}"
    );
    let reopened: Envelope<u32, String> = decode(&bytes).unwrap();
    assert_eq!(reopened.payload, 7);
    let mut changed = bytes.clone();
    changed.push(b'\n');
    assert!(matches!(
        decode::<Envelope<u32, String>>(&changed),
        Err(Error::Corrupt(_))
    ));
    assert!(encode(&envelope, 8).is_err());
    assert_eq!(
        digest(&bytes).to_string(),
        "b4ea9668573c56030edb6b28107f3b5250387873e9bb590b49f776cc28f7cb1a"
    );
}
