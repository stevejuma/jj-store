//! One resolver for defaults, catalogs and launches. Values never enter diagnostics.
use super::*;
use settings::{EnvironmentValue, RuntimeOptions, Settings, SettingsCommand};

pub(super) fn validate(options: &RuntimeOptions) -> Result<()> {
    if options
        .provider
        .as_ref()
        .is_some_and(|p| !matches!(p.as_str(), "local" | "paired" | "openai"))
    {
        return Err(Error::Invalid);
    }
    if options.environment.len() > 64 {
        return Err(Error::Limit);
    }
    for (name, value) in &options.environment {
        if !env_name(name) || name.starts_with("JJ_ACP_") || name.starts_with("JJ_STORE_") {
            return Err(Error::Invalid);
        }
        let text = match value {
            EnvironmentValue::Literal(v)
            | EnvironmentValue::Environment(v)
            | EnvironmentValue::Keychain(v) => Some(v),
            EnvironmentValue::Unset => None,
        };
        if text.is_some_and(|v| v.len() > 4096 || v.contains('\0')) {
            return Err(Error::Invalid);
        }
        if let EnvironmentValue::Literal(literal) = value {
            if !literal.is_empty() && sensitive(name) {
                return Err(Error::Invalid);
            }
            if let Ok(url) = url::Url::parse(literal) {
                if !url.username().is_empty()
                    || url.password().is_some()
                    || url.query().is_some()
                    || url.fragment().is_some()
                {
                    return Err(Error::Invalid);
                }
            }
        }
        if let EnvironmentValue::Environment(reference) = value {
            if !env_name(reference) {
                return Err(Error::Invalid);
            }
        }
        if let EnvironmentValue::Keychain(reference) = value {
            if !label(reference, 64)
                || !reference
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return Err(Error::Invalid);
            }
        }
    }
    for value in [&options.provider, &options.endpoint].into_iter().flatten() {
        if !label(value, 2048) {
            return Err(Error::Invalid);
        }
    }
    if let Some(endpoint) = &options.endpoint {
        let url = url::Url::parse(endpoint).map_err(|_| Error::Invalid)?;
        let loopback = matches!(
            url.host_str(),
            Some("localhost" | "127.0.0.1" | "::1" | "[::1]")
        );
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}
pub(super) fn sensitive(name: &str) -> bool {
    name.to_ascii_uppercase().split('_').any(|part| {
        matches!(
            part,
            "KEY" | "TOKEN" | "PASSWORD" | "SECRET" | "CREDENTIAL" | "AUTHORIZATION"
        )
    })
}
fn credential(name: &str, source: Option<&EnvironmentValue>) -> bool {
    sensitive(name)
        || match source {
            Some(EnvironmentValue::Keychain(_)) => true,
            Some(EnvironmentValue::Environment(reference)) => sensitive(reference),
            _ => false,
        }
}
fn env_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    name.len() <= 128
        && bytes
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

pub(super) fn resolve(setup: &Setup, defaults: &Settings) -> Setup {
    let mut effective = setup.clone();
    if setup.options.inherit_defaults && setup.runner == defaults.runner {
        effective.preferences.model = setup
            .preferences
            .model
            .clone()
            .or_else(|| defaults.preferences.model.clone());
        effective.preferences.effort = setup
            .preferences
            .effort
            .clone()
            .or_else(|| defaults.preferences.effort.clone());
        effective.preferences.mode = setup
            .preferences
            .mode
            .clone()
            .or_else(|| defaults.preferences.mode.clone());
        effective.options.provider = setup
            .options
            .provider
            .clone()
            .or_else(|| defaults.options.provider.clone());
        effective.options.endpoint = setup
            .options
            .endpoint
            .clone()
            .or_else(|| defaults.options.endpoint.clone());
    }
    if effective.runner == "local" && effective.preferences.mode.is_none() {
        effective.preferences.mode = Some("ask".into());
    }
    effective.options.environment = defaults.options.environment.clone();
    effective
        .options
        .environment
        .extend(setup.options.environment.clone());
    effective
}
pub(super) fn inference_host(setup: &Setup, state: &State) -> Option<String> {
    if setup.runner != "local" {
        return None;
    }
    match setup.options.provider.as_deref() {
        Some("openai") => setup.options.endpoint.clone(),
        Some("paired") => Some(
            state
                .compute
                .target_peer
                .as_ref()
                .and_then(|id| {
                    state
                        .compute
                        .peers
                        .get(id)
                        .map(|p| format!("{} · {id}", p.name))
                })
                .unwrap_or_else(|| "Paired compute unavailable".into()),
        ),
        _ => Some(
            state
                .compute_identity
                .as_ref()
                .map(|id| format!("This machine · {id}"))
                .unwrap_or_else(|| "This machine".into()),
        ),
    }
}
pub(super) fn launch_digest(setup: &Setup, state: &State) -> Result<String> {
    if setup.runner != "local" || setup.options.provider.as_deref() == Some("openai") {
        return journal::digest(setup);
    }
    journal::digest(
        &json!({"setup":setup,"computeGeneration":state.compute_generation,"model":state.compute.model,"target":if setup.options.provider.as_deref()==Some("paired"){state.compute.target_peer.clone()}else{None}}),
    )
}
impl Supervisor {
    pub(super) fn effective_setup(&self, id: &str) -> Result<Setup> {
        self.read(|s| Ok(resolve(s.setups.get(id).ok_or(Error::Stale)?, &s.settings)))
    }
    pub(super) async fn save_configuration(
        &self,
        actor: PrincipalId,
        command: SettingsCommand,
    ) -> Result<Settings> {
        self.actor(actor)?;
        if command.version != 1
            || !matches!(
                command.settings.runner.as_str(),
                "codex" | "cursor" | "local"
            )
        {
            return Err(Error::Invalid);
        }
        validate(&command.settings.options)?;
        preferences(&command.settings.preferences)?;
        let digest = journal::digest(&command)?;
        let _guard = self.0.mutation.lock().await;
        if self.duplicate(command.request_id, &digest)?.is_some() {
            return self.read(|s| Ok(s.settings.clone()));
        }
        let result = self.update(|s| {
            if command.expected_revision != s.settings.revision {
                return Err(Error::Stale);
            }
            if s.intents.len() >= 4096 {
                return Err(Error::Limit);
            }
            let mut settings = command.settings;
            settings.keychain_supported = cfg!(target_os = "macos");
            settings.revision = command
                .expected_revision
                .checked_add(1)
                .ok_or(Error::Limit)?;
            settings.options.inherit_defaults = false;
            s.settings = settings;
            s.intents.insert(
                command.request_id,
                Intent {
                    digest,
                    prompt: None,
                    outcome: Outcome {
                        version: 1,
                        request_id: command.request_id,
                        session_id: None,
                        state: "completed".into(),
                        error: None,
                    },
                },
            );
            Ok(s.settings.clone())
        })?;
        drop(_guard);
        self.0.catalogs.lock().await.clear();
        Ok(result)
    }
    pub(super) fn launch_environment(
        &self,
        command: &mut tokio::process::Command,
        setup: &Setup,
    ) -> Result<()> {
        command.env_clear();
        for name in [
            "HOME",
            "USER",
            "LOGNAME",
            "TMPDIR",
            "TMP",
            "TEMP",
            "LANG",
            "LC_ALL",
            "SystemRoot",
        ] {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
        for (name, value) in &setup.options.environment {
            let resolved = match value {
                EnvironmentValue::Literal(v) => Some(v.clone()),
                EnvironmentValue::Environment(reference) => {
                    Some(std::env::var(reference).map_err(|_| Error::Unavailable)?)
                }
                EnvironmentValue::Keychain(reference) => {
                    Some(secrets::get(&self.0.resources.root, reference)?)
                }
                EnvironmentValue::Unset => None,
            };
            if let Some(value) = resolved {
                if value.len() > 4096
                    || value.contains('\0')
                    || (matches!(value.len(), 1..=7)
                        && credential(name, setup.options.environment.get(name)))
                {
                    return Err(Error::Invalid);
                }

                command.env(name, value);
            }
        }
        Ok(())
    }
    pub(super) fn register_launch_secrets(
        &self,
        command: &tokio::process::Command,
        setup: &Setup,
    ) -> Result<()> {
        let mut values = Vec::new();
        for (name, value) in command.as_std().get_envs() {
            let Some(name) = name.to_str() else {
                return Err(Error::Invalid);
            };
            if credential(name, setup.options.environment.get(name)) {
                let Some(value) = value.and_then(|v| v.to_str()).filter(|v| !v.is_empty()) else {
                    continue;
                };
                if value.len() < 8 || value.len() > 4096 {
                    return Err(Error::Invalid);
                }
                values.push(value.to_owned());
            }
        }
        let mut slot = self.0.sanitizer.lock().map_err(|_| Error::Unavailable)?;
        let sanitizer = slot
            .as_ref()
            .ok_or(Error::Busy)?
            .for_secrets(&values)
            .map_err(|_| Error::Invalid)?;
        *slot = Some(sanitizer);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
