//! Lazy personal metadata home with stable provisioning identities.
use super::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Home {
    pub repository: RepositoryId,
    pub epoch: jj_store_model::EpochId,
    pub profile: String,
    pub ready: bool,
    #[serde(default)]
    pub provisioning_error: Option<String>,
    #[serde(default)]
    pub registration: Option<jj_store_service::local_browser::RegisterFolder>,
}
impl Supervisor {
    pub(super) async fn conversation_home(
        &self,
        selected: Option<RepositoryId>,
    ) -> Result<(RepositoryId, String, Workspace)> {
        if let Some(repository) = selected {
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(repository).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            self.authorized(repository, profile, true).await?;
            if profile == "native" {
                let pin = self
                    .native_request(repository)?
                    .into_session()
                    .snapshot()
                    .await
                    .map_err(|_| Error::Unavailable)?;
                if pin.manifest().work_root.is_none() {
                    return Err(Error::Invalid);
                }
            }
            return Ok((
                repository,
                profile.into(),
                self.home_workspace(repository, profile).await?,
            ));
        }
        if self.remote_mode() {
            return Err(Error::Denied);
        }
        let home = if let Some(home) = self.read(|s| Ok(s.personal_home.clone()))? {
            home
        } else {
            let mut id = [0; 16];
            let mut epoch = [0; 16];
            getrandom::fill(&mut id).map_err(|_| Error::Unavailable)?;
            getrandom::fill(&mut epoch).map_err(|_| Error::Unavailable)?;
            let home = Home {
                repository: RepositoryId::new(id),
                epoch: jj_store_model::EpochId::new(epoch),
                profile: if self.0.resources.host.is_some() {
                    "native"
                } else {
                    "local_jj_git"
                }
                .into(),
                ready: false,
                provisioning_error: None,
                registration: None,
            };
            self.update(|s| {
                s.personal_home = Some(home.clone());
                Ok(())
            })?;
            home
        };
        let request = RequestId::new(publication::identity(&format!(
            "personal-home/{}",
            home.epoch
        )));
        let work = jj_store_service::work::Request {
            version: 1,
            request_id: RequestId::new(publication::identity(&format!(
                "personal-home-work/{}",
                home.epoch
            ))),
            command: jj_store_service::work::WorkCommandV1::CreateProject {
                id: jj_store_service::work::ProjectId::new(*request.as_bytes()),
                title: "Personal agent chats".into(),
                body: "Installation-owned conversation metadata".into(),
            },
        };
        let mut repository = home.repository;
        if !home.ready {
            if home.profile == "native" {
                let host = self.0.resources.host.as_ref().ok_or(Error::Unavailable)?;
                let name = format!("personal-agent-chats-{}", home.repository);
                self.0
                    .resources
                    .registry
                    .reserve(home.repository, name.clone())
                    .await
                    .map_err(|_| Error::Unavailable)?;
                match host
                    .create_local_trusted(
                        home.repository,
                        home.epoch,
                        jj_store_service::auth::Permission::Admin,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(error) => {
                        if !matches!(
                            jj_store_service::transport::Error::from(error),
                            jj_store_service::transport::Error::Conflict
                        ) {
                            return Err(Error::Unavailable);
                        }
                    }
                }
                let session = host
                    .endpoint(home.repository)
                    .map_err(|_| Error::Unavailable)?
                    .bind_local_trusted(
                        self.0.resources.actor,
                        jj_store_service::auth::Permission::Admin,
                    )
                    .map_err(|_| Error::Denied)?
                    .into_session();
                if session
                    .snapshot()
                    .await
                    .map_err(|_| Error::Unavailable)?
                    .manifest()
                    .epoch
                    != home.epoch
                {
                    return Err(Error::Stale);
                }
                match jj_store_service::work::WorkCommandService
                    .apply(session, &work)
                    .await
                    .map_err(|_| Error::Unavailable)?
                {
                    jj_store_service::work::Outcome::Committed { .. } => {}
                    _ => return Err(Error::Unavailable),
                }
                self.0
                    .resources
                    .registry
                    .register(home.repository, name, None)
                    .await
                    .map_err(|_| Error::Unavailable)?;
            } else {
                let path = self.0.resources.root.join("personal-home");
                self.0
                    .resources
                    .driver
                    .initialize_personal(path.clone())
                    .await?;
                let folders = self
                    .0
                    .resources
                    .folders
                    .as_ref()
                    .ok_or(Error::Unavailable)?;
                let registration = if let Some(registration) = &home.registration {
                    registration.clone()
                } else {
                    let registration = jj_store_service::local_browser::RegisterFolder {
                        version: 1,
                        request_id: request,
                        expected_revision: folders.list().map_err(|_| Error::Unavailable)?.revision,
                        path: path.to_string_lossy().into_owned(),
                        name: format!("personal-agent-chats-{}", home.epoch),
                    };
                    self.update(|s| {
                        s.personal_home
                            .as_mut()
                            .ok_or(Error::Unavailable)?
                            .registration = Some(registration.clone());
                        Ok(())
                    })?;
                    registration
                };
                // Recover the exact committed receipt before retrying the unchanged intent.
                let outcome = match folders
                    .status(request)
                    .await
                    .map_err(|_| Error::Unavailable)?
                {
                    jj_store_service::local_browser::RegistrationOutcome::Committed {
                        binding,
                        ..
                    } => {
                        repository = binding.repository;
                        None
                    }
                    _ => Some(folders.register(registration).await.map_err(|error| {
                        let _ = self.update(|s| {
                            s.personal_home
                                .as_mut()
                                .ok_or(Error::Unavailable)?
                                .provisioning_error = Some(format!("registration: {error}"));
                            Ok(())
                        });
                        Error::Unavailable
                    })?),
                };
                if let Some(outcome) = outcome {
                    match outcome {
                        jj_store_service::local_browser::RegistrationOutcome::Committed {
                            binding,
                            ..
                        } => repository = binding.repository,
                        _ => return Err(Error::Unavailable),
                    }
                }
                let local = folders.open(repository).map_err(|_| Error::Unavailable)?;
                if !matches!(
                    local.apply_work_command(&work).map_err(|error| {
                        let _ = self.update(|s| {
                            s.personal_home
                                .as_mut()
                                .ok_or(Error::Unavailable)?
                                .provisioning_error = Some(format!("work_home: {error}"));
                            Ok(())
                        });
                        Error::Unavailable
                    })?,
                    jj_store_service::local_repository::LocalWorkOutcome::Committed { .. }
                ) {
                    return Err(Error::Unavailable);
                }
            }
            self.update(|s| {
                let home = s.personal_home.as_mut().ok_or(Error::Unavailable)?;
                home.repository = repository;
                home.ready = true;
                home.provisioning_error = None;
                Ok(())
            })?;
        }
        Ok((
            repository,
            home.profile.clone(),
            self.home_workspace(repository, &home.profile).await?,
        ))
    }
    async fn home_workspace(&self, repository: RepositoryId, profile: &str) -> Result<Workspace> {
        if profile == "local_jj_git" {
            let (folder, _) = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .folder(repository)
                .map_err(|_| Error::Unavailable)?;
            return self
                .0
                .resources
                .driver
                .inspect(PathBuf::from(folder.path))
                .await;
        }
        let snapshot = self
            .native_request(repository)?
            .into_session()
            .snapshot()
            .await
            .map_err(|_| Error::Unavailable)?;
        let operation = *snapshot
            .manifest()
            .heads
            .first()
            .ok_or(Error::Unavailable)?;
        // This home-only compatibility projection carries the real pinned operation
        // and native root commit. It never claims a physical code workspace.
        let path = self.0.resources.root.join("personal-home");
        Ok(Workspace {
            source: path.clone(),
            path: path.clone(),
            metadata: path,
            operation,
            commit: "00".repeat(64),
            name: String::new(),
        })
    }
}
