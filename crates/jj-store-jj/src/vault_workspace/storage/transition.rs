use super::*;
use jj_lib::git;
fn notify(phase: &str, callback: &mut impl FnMut(StorageProgress) -> Result<()>) -> Result<()> {
    callback(StorageProgress {
        phase: phase.into(),
        files: 0,
        bytes: 0,
    })
}
impl Storage {
    async fn native_operation(&self, path: &Path) -> Result<String> {
        let loader = RepoLoader::init_from_file_system(
            &settings()?,
            &path.join("repo"),
            &default_backend_factories(),
        )
        .map_err(jj)?;
        let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
        let [id] = heads.as_slice() else {
            return Err(Error::Binding);
        };
        let operation = loader.load_operation(id).await.map_err(jj)?;
        let repo = loader.load_at(&operation).await.map_err(jj)?;
        let git = git::get_git_backend(repo.store())
            .map_err(jj)?
            .git_repo_path()
            .canonicalize()?;
        if git != path.join("repo/store/git") {
            return Err(Error::Jj(
                "Move requires a managed internal Git object database".into(),
            ));
        }
        Ok(id.hex())
    }
    fn stage_binding(&self, value: &Move) -> Result<()> {
        let path = value.stage.join("vault.json");
        let mut stored: StoredBinding = record::read(&path)?;
        if stored.version != 1
            || stored.binding.vault_id != self.base.vault_id
            || stored.root_identity != value.root_identity
        {
            return Err(Error::Binding);
        }
        if !stored.previous_states.contains(&stored.binding.state) {
            stored.previous_states.push(stored.binding.state.clone());
        }
        if !stored.previous_states.contains(&value.source.state) {
            stored.previous_states.push(value.source.state.clone());
        }
        if !stored.relocations.contains(&value.request_id) {
            stored.relocations.push(value.request_id.clone());
        }
        if stored.previous_states.len() > 64 || stored.relocations.len() > 2048 {
            return Err(Error::Jj(
                "History relocation lineage reached its work limit".into(),
            ));
        }
        stored.binding = value.destination.clone();
        record::save(&path, &stored)?;
        use jj_lib::{
            ref_name::WorkspaceName, simple_workspace_store::SimpleWorkspaceStore,
            workspace_store::WorkspaceStore as _,
        };
        SimpleWorkspaceStore::load(&value.stage.join("repo"))
            .map_err(jj)?
            .add(WorkspaceName::DEFAULT, &self.base.root)
            .map_err(jj)?;
        Ok(())
    }
    pub(super) async fn run(
        &self,
        mut value: Move,
        progress: &mut impl FnMut(StorageProgress) -> Result<()>,
    ) -> Result<()> {
        // The transfer fence is independent of the capture lease. Publication
        // holds its own lock through every receipt update; do not copy mid-push.
        let publication_dir = value.source.state.join("remote-push-receipts");
        let _publication = if publication_dir.exists() {
            let file = OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(publication_dir.join("publication.lock"))?;
            file.try_lock().map_err(|_| Error::Busy)?;
            Some(file)
        } else {
            None
        };
        let fence = serde_json::json!({"request_id":value.request_id,"control":self.control});
        for binding in [&value.source, &value.destination] {
            let path = binding::fence(binding)?;
            if path.exists() && record::read::<serde_json::Value>(&path)? != fence {
                // A completed earlier move may have left this coordinate
                // fenced. Only the same vault's retained layout journal may
                // replace it during a new explicit move back.
                let old: serde_json::Value = record::read(&path)?;
                if old["control"] != self.control.to_string_lossy().as_ref() {
                    return Err(Error::Binding);
                }
                let id = old["request_id"].as_str().ok_or(Error::Binding)?;
                remote::publication::request_id(id)?;
                let earlier: Move = record::read(&self.receipt(id))?;
                self.valid(&earlier)?;
                if earlier.phase != "complete" {
                    return Err(Error::Busy);
                }
            }
            record::save(&path, &fence)?;
        }
        if value.phase == "prepared" {
            if value.source.state.exists() {
                if !record::same(&value.source.state, &value.source_identity)
                    || value.archive.exists()
                {
                    return Err(Error::Binding);
                }
                fs::rename(&value.source.state, &value.archive)?;
                File::open(value.source.state.parent().ok_or(Error::Binding)?)?.sync_all()?;
                File::open(value.archive.parent().ok_or(Error::Binding)?)?.sync_all()?;
            }
            if !record::same(&value.archive, &value.source_identity) {
                return Err(Error::Binding);
            }
            value.phase = "archived".into();
            record::save(&self.receipt(&value.request_id), &value)?;
            notify("archived", progress)?;
        }
        if value.phase == "archived" {
            if !record::same(&value.archive, &value.source_identity)
                || !record::same(&value.stage, &value.stage_identity)
            {
                return Err(Error::Binding);
            }
            let before = self.native_operation(&value.archive).await?;
            copy::all(&value.archive, &value.stage, progress)?;
            self.stage_binding(&value)?;
            if before != self.native_operation(&value.archive).await?
                || before != self.native_operation(&value.stage).await?
            {
                return Err(Error::Busy);
            }
            value.phase = "copied".into();
            record::save(&self.receipt(&value.request_id), &value)?;
            notify("copied", progress)?;
        }
        if value.phase == "copied" {
            if value.destination.state.exists() {
                if !record::same(&value.destination.state, &value.stage_identity) {
                    return Err(Error::Binding);
                }
            } else {
                if !record::same(&value.stage, &value.stage_identity) {
                    return Err(Error::Binding);
                }
                fs::rename(&value.stage, &value.destination.state)?;
                File::open(value.destination.state.parent().ok_or(Error::Binding)?)?.sync_all()?;
            }
            value.phase = "installed".into();
            record::save(&self.receipt(&value.request_id), &value)?;
            notify("installed", progress)?;
        }
        if value.phase == "installed" {
            if !record::same(&value.destination.state, &value.stage_identity) {
                return Err(Error::Binding);
            }
            let stored: StoredBinding = record::read(&value.destination.state.join("vault.json"))?;
            if stored.binding != value.destination
                || !stored.relocations.contains(&value.request_id)
            {
                return Err(Error::Binding);
            }
            let current = record::current(self)?;
            if current.revision == value.revision {
                record::save(
                    &self.control.join("current.json"),
                    &Current {
                        revision: value.revision.checked_add(1).ok_or(Error::Binding)?,
                        location: value.to,
                        ..current
                    },
                )?;
            } else if current.revision != value.revision + 1 || current.location != value.to {
                return Err(Error::Binding);
            }
            let target_fence = binding::fence(&value.destination)?;
            if record::read::<serde_json::Value>(&target_fence)? != fence {
                return Err(Error::Binding);
            }
            fs::remove_file(&target_fence)?;
            File::open(target_fence.parent().ok_or(Error::Binding)?)?.sync_all()?;
            value.phase = "complete".into();
            record::save(&self.receipt(&value.request_id), &value)?;
            record::clear_active(self, &value.request_id)?;
            notify("complete", progress)?;
        }
        Ok(())
    }
}
