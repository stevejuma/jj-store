use jj_store_service::agent_execution::{providers, settings::RuntimeOptions};
#[test]
fn chatgpt_plan_has_distinct_oauth_identity_and_legacy_options_load() {
    let provider = providers::get(Some("chatgpt-plan")).expect("subscription provider");
    assert_eq!(provider.runtime_provider, "chatgpt-plan");
    assert!(provider.credential_env.is_none());
    assert_eq!(
        provider.default_endpoint.as_deref(),
        Some("https://api.openai.com/v1")
    );
    assert!(!provider.managed);
    let old: RuntimeOptions = serde_json::from_str(
        r#"{"provider":"openai","endpoint":null,"environment":{},"inherit_defaults":false}"#,
    )
    .unwrap();
    assert!(old.account.is_none());
    assert_eq!(
        providers::canonical(old.provider.as_deref()),
        "openai-compat"
    );
}
