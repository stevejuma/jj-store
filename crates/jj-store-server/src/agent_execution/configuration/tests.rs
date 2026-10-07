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
