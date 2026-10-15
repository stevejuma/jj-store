use super::*;
use f::accounts::{Detection, Manager};

pub(super) async fn detect(
    local: &LocalForge,
    device: &mut Device,
    auto: bool,
) -> f::Result<Detection> {
    detect_with(local, device, auto, f::accounts::native()?).await
}
pub(super) async fn detect_with(
    local: &LocalForge,
    device: &mut Device,
    auto: bool,
    manager: &Manager,
) -> f::Result<Detection> {
    if device
        .connection()
        .is_some_and(|c| c.provider != f::Provider::Github)
    {
        return Ok(Detection {
            provider: device.connection().map(|c| c.provider),
            readiness: "legacy".into(),
            ..Default::default()
        });
    }
    let accounts = manager.inventory()?.accounts;
    let names = local.git(vec!["remote".into()]).await?;
    let mut remotes = vec![];
    for name in names.lines().take(32) {
        if !f::safe_name(name) {
            continue;
        }
        let url = local
            .git(vec![
                "remote".into(),
                "get-url".into(),
                "--push".into(),
                name.into(),
            ])
            .await?;
        if let Some(remote) = f::accounts::detect_remote(name, url.trim(), &accounts) {
            remotes.push(remote);
        }
    }
    let mut result = Detection {
        provider: if remotes.is_empty() {
            None
        } else {
            Some(f::Provider::Github)
        },
        readiness: "unconfigured".into(),
        remotes,
        accounts,
        account_id: device.account_id(),
    };
    if let Some(id) = device.account_id() {
        if let (Ok(account), Some(connection)) = (manager.account(&id), device.connection()) {
            let matches = account.api_url == connection.api_url
                && result.remotes.iter().any(|r| {
                    r.name == connection.remote
                        && r.repository == connection.source_repository
                        && r.api_url == connection.api_url
                });
            if matches
                && manager
                    .repository(&account, &connection.repository, f::accounts::now()?)
                    .await
                    .is_ok()
            {
                result.readiness = "connected".into();
                return Ok(result);
            }
        }
        result.readiness = "reconnect_required".into();
        return Ok(result);
    }
    let candidates: Vec<_> = result
        .accounts
        .iter()
        .filter(|a| a.available && result.remotes.iter().any(|r| r.api_url == a.api_url))
        .cloned()
        .collect();
    if !candidates.is_empty() {
        result.readiness = "selection_required".into();
    }
    if auto && candidates.len() == 1 && result.remotes.len() == 1 {
        let account = &candidates[0];
        let remote = &result.remotes[0];
        let connection = match device.connection() {
            Some(connection)
                if connection.api_url == account.api_url
                    && connection.source_repository == remote.repository
                    && connection.remote == remote.name =>
            {
                connection
            }
            Some(_) => return Ok(result),
            None => {
                let repo = match manager
                    .repository(account, &remote.repository, f::accounts::now()?)
                    .await
                {
                    Ok(v) => v,
                    Err(_) => return Ok(result),
                };
                let target = repo
                    .get("default_branch")
                    .and_then(serde_json::Value::as_str)
                    .filter(|v| f::safe_name(v))
                    .ok_or(f::Error::Invalid)?;
                // Forks require an explicit upstream choice.
                if repo.get("fork").and_then(serde_json::Value::as_bool) == Some(true) {
                    return Ok(result);
                }
                f::Connection {
                    provider: f::Provider::Github,
                    api_url: account.api_url.clone(),
                    repository: remote.repository.clone(),
                    source_repository: remote.repository.clone(),
                    remote: remote.name.clone(),
                    target_remote: remote.name.clone(),
                    target: target.into(),
                    native_stacks: false,
                }
            }
        };
        bind(local, manager, device, account.id.clone(), connection).await?;
        result.account_id = Some(account.id.clone());
        result.readiness = "connected".into();
    }
    Ok(result)
}
pub(super) async fn bind(
    local: &LocalForge,
    manager: &Manager,
    device: &mut Device,
    id: String,
    mut connection: f::Connection,
) -> f::Result<()> {
    if connection.provider != f::Provider::Github {
        return Err(f::Error::Invalid);
    }
    let account = manager.account(&id)?;
    if account.api_url != connection.api_url {
        return Err(f::Error::Denied);
    }
    if connection.target.is_empty() {
        let repo = manager
            .repository(&account, &connection.repository, f::accounts::now()?)
            .await?;
        connection.target = repo
            .get("default_branch")
            .and_then(serde_json::Value::as_str)
            .ok_or(f::Error::Invalid)?
            .into();
    }
    connection.validate()?;
    let inspector = LocalForge {
        access: local.access.clone(),
        root: local.root.clone(),
        connection: connection.clone(),
        credential: None,
        account_id: None,
    };
    inspector.verify_remote().await?;
    let mut target_connection = connection.clone();
    target_connection.remote = connection.target_remote.clone();
    target_connection.source_repository = connection.repository.clone();
    let target = LocalForge {
        access: local.access.clone(),
        root: local.root.clone(),
        connection: target_connection,
        credential: None,
        account_id: None,
    };
    target.verify_remote().await?;
    manager
        .repository(&account, &connection.repository, f::accounts::now()?)
        .await?;
    manager
        .repository(&account, &connection.source_repository, f::accounts::now()?)
        .await?;
    if device
        .connection()
        .as_ref()
        .is_some_and(|old| old != &connection)
    {
        return Err(f::Error::Stale);
    }
    // Check pending intent before adding/changing any account reference.
    device.bind_account(id)?;
    device.configure(connection)
}
pub(super) fn provisional() -> f::Connection {
    f::Connection {
        provider: f::Provider::Github,
        api_url: "https://api.github.com".into(),
        repository: "unknown/repository".into(),
        source_repository: "unknown/repository".into(),
        remote: "origin".into(),
        target_remote: "origin".into(),
        target: "main".into(),
        native_stacks: false,
    }
}
