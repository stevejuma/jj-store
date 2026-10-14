//! Resolve job identities through existing verified installation workspace links.
use super::*;
use jj_store_service::{
    local_management::{WorkspaceAvailability, WorkspaceManager, WorkspaceProfile},
    registry::RepositoryRegistry,
    workflow_host::{Environment, Source},
};
use std::{collections::BTreeMap, sync::Arc};
#[derive(Clone)]
pub struct Discovery {
    registry: Arc<dyn RepositoryRegistry>,
    workspaces: Arc<dyn WorkspaceManager>,
    pub(super) principal: m::PrincipalId,
    browser_principal: m::PrincipalId,
    socket: PathBuf,
    state: PathBuf,
    jj: PathBuf,
    endpoint: jj_store_service::workflow_host::AgentEndpoint,
}
impl Discovery {
    pub fn local(
        config: &jj_store_server::config::Config,
        directory: Option<(Arc<dyn RepositoryRegistry>, Arc<dyn WorkspaceManager>)>,
    ) -> Result<Option<Self>> {
        let Some((registry, workspaces)) = directory else {
            return Ok(None);
        };
        let jj_store_server::config::Access::LocalTrusted {
            rpc_socket,
            installation,
            ..
        } = &config.access
        else {
            return Ok(None);
        };
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            let identity = jj_store_server::installation::Identity::load(installation)?;
            let uid = std::fs::metadata(&config.state_dir)?.uid();
            Ok(Some(Self {
                registry,
                workspaces,
                principal: identity.principal(format!("unix-uid:{uid}").as_bytes()),
                browser_principal: identity.principal(b"loopback-http"),
                socket: rpc_socket.clone(),
                state: config.state_dir.join("workflow-hosts"),
                jj: std::env::current_exe()?.with_file_name("jj-store-jj"),
                endpoint: jj_store_service::workflow_host::AgentEndpoint {
                    url: config.public_url.clone(),
                    local: true,
                    token_environment: None,
                },
            }))
        }
        #[cfg(not(unix))]
        {
            let _ = (registry, workspaces, rpc_socket, installation);
            Ok(None)
        }
    }
    async fn sources(&self) -> Result<Vec<Source>> {
        let mut sources = BTreeMap::new();
        for repository in self.registry.snapshot(None).await?.repositories {
            let links = self.workspaces.list(repository.id).await?;
            let mut ready = links
                .links
                .into_iter()
                .filter(|link| {
                    link.profile == WorkspaceProfile::Native
                        && link.availability == WorkspaceAvailability::Ready
                })
                .collect::<Vec<_>>();
            ready.sort_by(|a, b| a.path.cmp(&b.path));
            for link in ready {
                if let Some(source) = registered_source(repository.id, &link, &self.socket) {
                    sources.insert(repository.id, source);
                    break;
                }
            }
        }
        if sources.len() > 32 {
            return Err("Runner attachment limit exceeded".into());
        }
        Ok(sources.into_values().collect())
    }
    pub(super) async fn configurations(&self, environment: &Environment) -> Result<Vec<Config>> {
        environment.validate()?;
        let sources = self.sources().await?;
        let state = environment
            .state_dir
            .clone()
            .unwrap_or_else(|| self.state.join(&environment.host));
        let jj = environment
            .jj_executable
            .clone()
            .unwrap_or_else(|| self.jj.clone());
        jj_store_server::policy::outside_checkout(&state)?;
        if !jj.is_file() || environment.commands.values().any(|c| !c.program.is_file()) {
            return Err("Runner executable is unavailable".into());
        }
        let legacy = if state.join("journal.json").exists() {
            let journal: Journal = serde_json::from_slice(&jj_store_server::policy::read_regular(
                &state.join("journal.json"),
                8 * 1024 * 1024,
            )?)?;
            if journal.host != environment.host || journal.principal != self.principal {
                return Err("Runner journal identity differs".into());
            }
            Some(journal.home)
        } else {
            None
        };
        let mut result = Vec::new();
        let mut live = std::collections::BTreeSet::new();
        for home in &sources {
            let client = config::connection(&home.workspace)?.connect().await?;
            let page = client.workflow_page(&w::Query::default()).await?;
            if page.root.is_none() {
                continue;
            }
            let directory = if legacy == Some(home.repository) {
                state.clone()
            } else {
                state.join(home.repository.to_string())
            };
            let mut configuration = environment.bind(
                self.principal,
                home.workspace.clone(),
                directory,
                jj.clone(),
                sources.clone(),
            );
            if environment.agent_endpoint.is_none() && !environment.agents.is_empty() {
                configuration.agent_endpoint = Some(self.endpoint.clone());
            }
            if environment.allowed_owners.is_empty() {
                configuration.allowed_owners = [self.principal, self.browser_principal].into();
            }
            live.insert(home.repository);
            let config = Config(configuration);
            config.validate()?;
            result.push(config);
        }
        retained_journals(&state, &live)?;
        Ok(result)
    }
}

// A removed attachment must not hide an unresolved reporting/attempt journal.
fn retained_journals(
    state: &Path,
    live: &std::collections::BTreeSet<m::RepositoryId>,
) -> Result<()> {
    if !state.exists() {
        return Ok(());
    }
    let mut paths = vec![state.join("journal.json")];
    for entry in std::fs::read_dir(state)? {
        let entry = entry?;
        if entry
            .file_name()
            .to_str()
            .is_some_and(|v| v.parse::<m::RepositoryId>().is_ok())
        {
            paths.push(entry.path().join("journal.json"));
        }
        if paths.len() > 65 {
            return Err("Runner recovery directory limit exceeded".into());
        }
    }
    for path in paths.into_iter().filter(|p| p.exists()) {
        let journal: Journal = serde_json::from_slice(&jj_store_server::policy::read_regular(
            &path,
            8 * 1024 * 1024,
        )?)?;
        if journal.version != 1
            || ((journal.active.is_some() || journal.pending.is_some())
                && !live.contains(&journal.home))
        {
            return Err("Retained workflow home is unavailable; original journal preserved".into());
        }
    }
    Ok(())
}

fn registered_source(
    repository: m::RepositoryId,
    link: &jj_store_service::local_management::WorkspaceLink,
    socket: &Path,
) -> Option<Source> {
    if link.profile != WorkspaceProfile::Native || link.availability != WorkspaceAvailability::Ready
    {
        return None;
    }
    let connection = config::connection(Path::new(&link.path)).ok()?;
    (connection.repository == repository && connection.unix_socket.as_deref() == Some(socket)).then(
        || Source {
            repository,
            workspace: link.path.clone().into(),
            target_id: None,
        },
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_service::local_management::WorkspaceLink;
    fn link(root: &Path, repository: m::RepositoryId, socket: &Path) -> WorkspaceLink {
        let cache = root.join(".jj/repo");
        let marker = format!(
            "jj-store-client-cache-v1\n{}\n{repository}\n",
            jj_store_jj::factories::PROFILE
        );
        for role in ["store", "op_store", "op_heads"] {
            std::fs::create_dir_all(cache.join(role)).unwrap();
            std::fs::write(cache.join(role).join("native-profile"), &marker).unwrap();
        }
        crate::binding::Connection::new_local(socket.into(), repository)
            .unwrap()
            .write(&cache)
            .unwrap();
        WorkspaceLink {
            id: "registered".into(),
            path: root.to_str().unwrap().into(),
            profile: WorkspaceProfile::Native,
            availability: WorkspaceAvailability::Ready,
        }
    }
    #[test]
    fn registered_source_requires_exact_identity_socket_and_ready_native_link() {
        let root = tempfile::tempdir().unwrap();
        let repository = m::RepositoryId::new([3; 16]);
        let socket = root.path().join("native.sock");
        let mut link = link(root.path(), repository, &socket);
        assert_eq!(
            registered_source(repository, &link, &socket)
                .unwrap()
                .repository,
            repository
        );
        assert!(registered_source(m::RepositoryId::new([4; 16]), &link, &socket).is_none());
        assert!(registered_source(repository, &link, &root.path().join("other.sock")).is_none());
        link.availability = WorkspaceAvailability::Unavailable;
        assert!(registered_source(repository, &link, &socket).is_none());
        link.availability = WorkspaceAvailability::Ready;
        link.profile = WorkspaceProfile::LocalJjGit;
        assert!(registered_source(repository, &link, &socket).is_none());
    }
    #[test]
    fn changed_or_missing_native_binding_is_not_a_source() {
        let root = tempfile::tempdir().unwrap();
        let repository = m::RepositoryId::new([3; 16]);
        let socket = root.path().join("native.sock");
        let link = link(root.path(), repository, &socket);
        std::fs::write(root.path().join(".jj/repo/store/native-profile"), "changed").unwrap();
        assert!(registered_source(repository, &link, &socket).is_none());
        std::fs::remove_file(root.path().join(".jj/repo/store-connection.json")).unwrap();
        assert!(registered_source(repository, &link, &socket).is_none());
    }
    #[test]
    fn removed_home_cannot_hide_an_uncertain_attempt_journal() {
        let root = tempfile::tempdir().unwrap();
        let home = m::RepositoryId::new([3; 16]);
        let journal = Journal {
            version: 1,
            host: "test".into(),
            principal: m::PrincipalId::new([1; 16]),
            home,
            generation: m::RequestId::new([2; 16]),
            counter: 1,
            cursors: Default::default(),
            pending: None,
            active: Some(Active {
                run: w::RunId::new([4; 16]),
                attempt: w::ExecutionAttemptId::new([5; 16]),
                sequence: 0,
            }),
        };
        let state = root.path().join(home.to_string());
        std::fs::create_dir(&state).unwrap();
        let path = state.join("journal.json");
        let bytes = serde_json::to_vec(&journal).unwrap();
        std::fs::write(&path, &bytes).unwrap();
        assert!(retained_journals(root.path(), &Default::default()).is_err());
        retained_journals(root.path(), &[home].into()).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}
