//! Apply provider-owned launch names after resolving user environment references.
use super::*;
use jj_store_service::agent_execution::providers;

pub(super) fn configure(
    command: &mut tokio::process::Command,
    setup: &Setup,
    local_model: Option<String>,
) -> Result<()> {
    let provider = providers::get(setup.options.provider.as_deref()).ok_or(Error::Invalid)?;
    command.env("BUZZ_AGENT_PROVIDER", &provider.runtime_provider);
    if provider.id == "chatgpt-plan" {
        if setup.options.endpoint.is_some() {
            return Err(Error::Invalid);
        }
        for name in [
            "OPENAI_COMPAT_API_KEY",
            "OPENAI_COMPAT_BASE_URL",
            "OPENAI_COMPAT_API",
            "OPENAI_API_KEY",
        ] {
            command.env_remove(name);
        }
    }
    if let Some(model) = setup.preferences.model.as_ref().or(local_model.as_ref()) {
        command.env("BUZZ_AGENT_MODEL", model);
    } else if provider.managed {
        command.env("BUZZ_AGENT_MODEL", "auto");
    }
    if let Some(effort) = &setup.preferences.effort {
        command.env("BUZZ_AGENT_THINKING_EFFORT", effort);
    }
    if provider.managed {
        command.env("OPENAI_COMPAT_API", "chat");
    } else {
        if let Some(endpoint) = setup.options.endpoint.as_ref() {
            command.env(&provider.endpoint_env, endpoint);
        } else if !setup
            .options
            .environment
            .contains_key(&provider.endpoint_env)
            && let Some(endpoint) = &provider.default_endpoint
        {
            command.env(&provider.endpoint_env, endpoint);
        }
        if provider.endpoint_required
            && !command.as_std().get_envs().any(|(name, value)| {
                name == provider.endpoint_env.as_str() && value.is_some_and(|v| !v.is_empty())
            })
        {
            return Err(Error::Invalid);
        }
        // Preserve the old explicit-compatible profile's Chat Completions default.
        if setup.options.provider.as_deref() == Some("openai")
            && !setup.options.environment.contains_key("OPENAI_COMPAT_API")
        {
            command.env("OPENAI_COMPAT_API", "chat");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compatible_endpoint_is_required_and_legacy_chat_default_is_retained() {
        let mut setup: Setup = serde_json::from_value(json!({"id":"a","name":"A","avatar":"","instructions":"","runner":"local","preferences":{"model":null,"effort":null,"mode":null},"options":{"provider":"openai"}})).unwrap();
        assert!(configure(&mut tokio::process::Command::new("fixture"), &setup, None).is_err());
        setup.options.endpoint = Some("https://compatible.example/v1".into());
        let mut command = tokio::process::Command::new("fixture");
        configure(&mut command, &setup, None).unwrap();
        assert!(
            command
                .as_std()
                .get_envs()
                .any(|(name, value)| name == "OPENAI_COMPAT_API" && value == Some("chat".as_ref()))
        );
    }
    #[test]
    fn external_providers_use_native_keys_and_do_not_force_chat() {
        for provider in providers::catalog()
            .into_iter()
            .filter(|p| !p.managed && p.authentication.as_deref() != Some("oauth"))
        {
            let setup: Setup = serde_json::from_value(json!({"id":"a","name":"A","avatar":"","instructions":"","runner":"local","preferences":{"model":"chosen","effort":"high","mode":"ask"},"options":{"provider":provider.id,"endpoint":"https://provider.example/v1"}})).unwrap();
            let mut command = tokio::process::Command::new("fixture");
            configure(&mut command, &setup, None).unwrap();
            let env: BTreeMap<_, _> = command
                .as_std()
                .get_envs()
                .map(|(k, v)| (k.to_str().unwrap(), v.and_then(|v| v.to_str())))
                .collect();
            assert_eq!(
                env["BUZZ_AGENT_PROVIDER"],
                Some(provider.runtime_provider.as_str())
            );
            assert_eq!(env["BUZZ_AGENT_MODEL"], Some("chosen"));
            assert_eq!(
                env[provider.endpoint_env.as_str()],
                Some("https://provider.example/v1")
            );
            assert!(!env.contains_key("OPENAI_COMPAT_API"));
        }
    }
}
