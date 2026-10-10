use super::*;
fn setup() -> Setup {
    Setup {
        id: "a".into(),
        name: "Agent".into(),
        avatar: String::new(),
        description: String::new(),
        instructions: String::new(),
        runner: "codex".into(),
        preferences: Preferences {
            model: None,
            effort: None,
            mode: None,
        },
        options: RuntimeOptions::default(),
    }
}
#[test]
fn old_agents_keep_harness_defaults_and_new_agents_inherit_compatible_preferences() {
    let mut defaults = Settings::default();
    defaults.preferences.model = Some("model".into());
    let mut agent = setup();
    assert_eq!(resolve(&agent, &defaults).preferences.model, None);
    agent.options.inherit_defaults = true;
    assert_eq!(
        resolve(&agent, &defaults).preferences.model.as_deref(),
        Some("model")
    );
    agent.runner = "cursor".into();
    assert_eq!(resolve(&agent, &defaults).preferences.model, None);
}
#[test]
fn agent_environment_overrides_and_explicit_unsets_global_values() {
    let mut defaults = Settings::default();
    defaults
        .options
        .environment
        .insert("MODE".into(), EnvironmentValue::Literal("global".into()));
    let mut agent = setup();
    agent
        .options
        .environment
        .insert("MODE".into(), EnvironmentValue::Unset);
    assert_eq!(
        resolve(&agent, &defaults).options.environment["MODE"],
        EnvironmentValue::Unset
    );
}
#[test]
fn launch_identity_credentials_in_urls_and_invalid_references_are_refused() {
    let mut options = RuntimeOptions::default();
    options.environment.insert(
        "JJ_ACP_TASK_ID".into(),
        EnvironmentValue::Literal("spoof".into()),
    );
    assert!(validate(&options).is_err());
    options.environment.clear();
    options.endpoint = Some("https://user:secret@example.com/v1".into());
    assert!(validate(&options).is_err());
    options.endpoint = Some("http://127.0.0.1:9337/v1".into());
    assert!(validate(&options).is_ok());
    options.environment.insert(
        "API_KEY".into(),
        EnvironmentValue::Environment("invalid-name".into()),
    );
    assert!(validate(&options).is_err());
}

#[test]
fn local_worker_digest_tracks_compute_generation_and_host_selection_only_for_managed_inference() {
    let mut agent = setup();
    agent.runner = "local".into();
    let mut state = State::default();
    let before = launch_digest(&agent, &state).unwrap();
    state.compute_generation = 1;
    assert_ne!(before, launch_digest(&agent, &state).unwrap());
    agent.options.provider = Some("paired".into());
    let paired = launch_digest(&agent, &state).unwrap();
    state.compute.target_peer = Some("peer".into());
    assert_ne!(paired, launch_digest(&agent, &state).unwrap());
    agent.options.provider = Some("openai".into());
    let external = launch_digest(&agent, &state).unwrap();
    state.compute_generation = 2;
    assert_eq!(external, launch_digest(&agent, &state).unwrap());
}

#[test]
fn ordinary_environment_references_are_not_credential_redactions() {
    assert!(!credential(
        "MODE",
        Some(&EnvironmentValue::Environment("AGENT_MODE".into()))
    ));
    assert!(credential(
        "MODE",
        Some(&EnvironmentValue::Environment("PROVIDER_API_KEY".into()))
    ));
    assert!(credential(
        "API_KEY",
        Some(&EnvironmentValue::Environment("PROVIDER_VALUE".into()))
    ));
    assert!(credential(
        "MODE",
        Some(&EnvironmentValue::Keychain("provider".into()))
    ));
}

#[test]
fn bundled_cli_path_cannot_be_supplied_by_a_setup() {
    let mut options = RuntimeOptions::default();
    options.environment.insert(
        "JJ_STORE_AGENT_JJ_EXECUTABLE".into(),
        EnvironmentValue::Literal("/incorrect/jj".into()),
    );
    assert!(matches!(validate(&options), Err(Error::Invalid)));
}

#[test]
fn provider_catalog_accepts_buzz_providers_and_preserves_legacy_compatible_endpoint() {
    for provider in [
        "local",
        "paired",
        "anthropic",
        "openai-api",
        "openai",
        "openai-compat",
        "openrouter",
        "databricks",
        "databricks_v2",
    ] {
        let mut options = RuntimeOptions::default();
        options.provider = Some(provider.into());
        assert!(validate(&options).is_ok(), "{provider}");
    }
}

#[test]
fn provider_changes_do_not_inherit_foreign_models_endpoints_or_effort() {
    let mut defaults = Settings::default();
    defaults.runner = "local".into();
    defaults.options.provider = Some("openai".into());
    defaults.options.endpoint = Some("https://old.example/v1".into());
    defaults.preferences.model = Some("old-model".into());
    defaults.preferences.effort = Some("high".into());
    let mut agent = setup();
    agent.runner = "local".into();
    agent.options.inherit_defaults = true;
    agent.options.provider = Some("anthropic".into());
    let resolved = resolve(&agent, &defaults);
    assert_eq!(resolved.preferences.model, None);
    assert_eq!(resolved.preferences.effort, None);
    assert_eq!(resolved.options.endpoint, None);
}
#[test]
fn subscription_account_changes_do_not_inherit_a_foreign_model_or_endpoint() {
    let mut defaults = Settings::default();
    defaults.runner = "local".into();
    defaults.options.provider = Some("chatgpt-plan".into());
    defaults.options.account = Some("account-one".into());
    defaults.preferences.model = Some("account-one-model".into());
    let mut agent = setup();
    agent.runner = "local".into();
    agent.options.inherit_defaults = true;
    assert_eq!(
        resolve(&agent, &defaults).options.account.as_deref(),
        Some("account-one")
    );
    agent.options.account = Some("account-two".into());
    assert!(resolve(&agent, &defaults).preferences.model.is_none());
    agent.options.provider = Some("chatgpt-plan".into());
    agent.options.endpoint = Some("https://foreign.example/v1".into());
    assert!(validate(&agent.options).is_err());
    agent.options.endpoint = None;
    agent.options.environment.insert(
        "OPENAI_COMPAT_BASE_URL".into(),
        EnvironmentValue::Literal("https://foreign.example/v1".into()),
    );
    assert!(validate(&agent.options).is_err());
}
