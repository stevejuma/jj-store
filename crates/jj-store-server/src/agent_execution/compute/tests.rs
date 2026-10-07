use super::*;
#[test]
fn pairing_refuses_unusable_identity_addresses_and_credential_urls() {
    let valid = json!({"id":"11".repeat(32),"addrs":[{"Ip":"127.0.0.1:9999"}]});
    assert!(process::valid_invitation(&valid));
    for value in [
        json!({"id":"name","addrs":[]}),
        json!({"id":"11".repeat(32),"addrs":[{"Ip":"0.0.0.0:0"}]}),
        json!({"id":"11".repeat(32),"addrs":[{"Relay":"https://user:secret@example.com"}]}),
    ] {
        assert!(!process::valid_invitation(&value));
    }
}
#[test]
fn model_references_and_selected_peers_are_validated_before_side_effects() {
    let mut settings = dto::Settings::default();
    settings.model = Some("../outside.gguf".into());
    assert!(process::validate(&settings).is_err());
    settings.model = Some("unsloth/Qwen3-0.6B-GGUF:Q4_K_M".into());
    assert!(process::validate(&settings).is_ok());
    settings.target_peer = Some("unpaired".into());
    assert!(process::validate(&settings).is_err());
}
