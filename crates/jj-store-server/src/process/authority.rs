use super::*;
use jj_store_service::administration::RepositoryHost;
use std::{collections::BTreeMap, path::Path};

type Authority = (
    Option<AuthoritativeStorage>,
    Option<RepositoryHost>,
    Option<crate::search::PreparedSearch>,
    BTreeMap<RepositoryId, jj_store_http::SearchIndex>,
    Option<crate::replication::PreparedReplication>,
    PreparedRpc,
);
pub(super) async fn prepare(
    config: &Config,
    state: &Path,
    installation: &Path,
    registry: Arc<Registry>,
    search_access: crate::search::SearchAccess,
    state_lock: &mut Option<File>,
) -> Result<Authority> {
    let authority = if matches!(config.storage, crate::config::Storage::Filesystem) {
        (
            None,
            None,
            None,
            std::collections::BTreeMap::new(),
            None,
            PreparedRpc::Disabled,
        )
    } else {
        let storage = AuthoritativeStorage::start(config)
            .await?
            .with_state_lock(state_lock.take())?;
        let host = config.host(storage.store())?;
        let rpc_limits = Limits {
            concurrent: config.limits.concurrent_requests,
            timeout: Duration::from_secs(config.limits.request_seconds),
            object_bytes: config.limits.max_blob_bytes
                + jj_store_model::envelope::HEADER_LEN as u64,
        };
        let (search, indexes) = crate::search::PreparedSearch::new(
            &host,
            state,
            &config.search,
            &config.limits,
            search_access,
            crate::search::environment,
        )?;
        let search_health = indexes
            .iter()
            .map(|(repository, index)| (*repository, index.health.clone()))
            .collect();
        let replication_principal = Identity::load(installation)?.principal(b"replication-worker");
        let replication = crate::replication::PreparedReplication::new(
            &host,
            storage.store(),
            &config.replication,
            replication_principal,
            search_health,
        )?;
        let rpc = match &config.access {
            Access::Authenticated { .. } => PreparedRpc::Shared(
                NativeRpc::hosted(host.clone(), rpc_limits)
                    .map_err(|_| Error::Config("RPC limits"))?
                    .with_runtime_policy(host.runtime_policy().clone())
                    .with_registry(registry.clone())
                    .with_replication_controls(replication.controls()),
            ),
            Access::LocalTrusted {
                installation,
                rpc_socket,
                permission,
                ..
            } => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::{
                        FileTypeExt as _, MetadataExt as _, PermissionsExt as _,
                    };
                    let owner_uid = fs::metadata(state)?.uid();
                    let identity = Identity::load(installation)?;
                    if let Ok(metadata) = fs::symlink_metadata(rpc_socket) {
                        if !metadata.file_type().is_socket() {
                            return Err(Error::Config("local RPC path must be a Unix socket"));
                        }
                        fs::remove_file(rpc_socket)?;
                    }
                    let listener = tokio::net::UnixListener::bind(rpc_socket)?;
                    fs::set_permissions(rpc_socket, fs::Permissions::from_mode(0o600))?;
                    let principal = identity.principal(format!("unix-uid:{owner_uid}").as_bytes());
                    PreparedRpc::Local(LocalRpc {
                        service: NativeRpc::hosted_local_trusted(
                            host.clone(),
                            rpc_limits,
                            principal,
                            *permission,
                            owner_uid,
                        )
                        .map_err(|_| Error::Config("local RPC access"))?
                        .with_runtime_policy(host.runtime_policy().clone())
                        .with_registry(registry.clone())
                        .with_replication_controls(replication.controls()),
                        listener,
                        socket: SocketGuard(rpc_socket.clone()),
                    })
                }
                #[cfg(not(unix))]
                return Err(Error::Config("local trusted RPC requires Unix"));
            }
        };
        (
            Some(storage),
            Some(host),
            Some(search),
            indexes,
            Some(replication),
            rpc,
        )
    };
    Ok(authority)
}
