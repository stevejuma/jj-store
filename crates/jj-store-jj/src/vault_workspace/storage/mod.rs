//! Device-private, recoverable layout selection. Native JJ files remain native.
mod copy;
mod record;
mod rollback;
mod transition;
use super::*;
use record::{Current, Move};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StorageLocation {
    External,
    Colocated,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingStorage {
    pub request_id: String,
    pub location: StorageLocation,
    pub phase: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageStatus {
    pub revision: u64,
    pub location: StorageLocation,
    pub pending: Option<PendingStorage>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageProgress {
    pub phase: String,
    pub files: u64,
    pub bytes: u64,
}
/// Both paths are admitted by the native host. Control stays outside the vault
/// and both repository locations. No renderer path selects storage authority.
pub struct Storage {
    base: Binding,
    control: PathBuf,
}
impl Storage {
    pub fn new(base: Binding, control: PathBuf) -> Self {
        Self { base, control }
    }
    fn at(&self, location: StorageLocation) -> Binding {
        Binding {
            state: match location {
                StorageLocation::External => self.base.state.clone(),
                StorageLocation::Colocated => self.base.root.join(".jj"),
            },
            ..self.base.clone()
        }
    }
    fn validate(&self) -> Result<()> {
        self.base.validate()?;
        if self.control.exists() && fs::canonicalize(&self.control)? != self.control {
            return Err(Error::Binding);
        }
        if self.control.components().any(|c| {
            matches!(
                c,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        }) || !self.control.is_absolute()
            || self.control.starts_with(&self.base.root)
            || self.control.starts_with(&self.base.state)
            || self.base.state.starts_with(&self.base.root)
        {
            return Err(Error::Binding);
        }
        Ok(())
    }
    fn receipt(&self, id: &str) -> PathBuf {
        self.control.join("receipts").join(format!("{id}.json"))
    }
    pub fn binding(&self) -> Result<Binding> {
        self.validate()?;
        Ok(self.at(record::current(self)?.location))
    }
    pub fn status(&self) -> Result<StorageStatus> {
        self.validate()?;
        let current = record::current(self)?;
        let active = match record::read::<String>(&self.control.join("active.json")) {
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => None,
            other => Some(other?),
        };
        let pending = active
            .map(|id| -> Result<_> {
                remote::publication::request_id(&id)?;
                let receipt: Move = record::read(&self.receipt(&id))?;
                self.valid(&receipt)?;
                Ok(PendingStorage {
                    request_id: id,
                    location: receipt.to,
                    phase: receipt.phase,
                })
            })
            .transpose()?;
        Ok(StorageStatus {
            revision: current.revision,
            location: current.location,
            pending,
        })
    }
    fn valid(&self, value: &Move) -> Result<()> {
        if ![
            "prepared",
            "archived",
            "copied",
            "installed",
            "complete",
            "rolled_back",
        ]
        .contains(&value.phase.as_str())
            || value.from == value.to
            || value.version != 1
            || value.vault_id != self.base.vault_id
            || value.root_identity != root_identity(&self.base.root)?
            || value.source != self.at(value.from)
            || value.destination != self.at(value.to)
        {
            return Err(Error::Binding);
        }
        remote::publication::request_id(&value.request_id)?;
        if !value.stage.is_absolute() || !value.archive.is_absolute() {
            return Err(Error::Binding);
        }
        Ok(())
    }
    pub async fn recover(&self) -> Result<StorageStatus> {
        let Some(pending) = self.status()?.pending else {
            return self.status();
        };
        let value: Move = record::read(&self.receipt(&pending.request_id))?;
        self.change(value.revision, value.to, &value.request_id)
            .await
    }
    pub async fn change(
        &self,
        revision: u64,
        location: StorageLocation,
        id: &str,
    ) -> Result<StorageStatus> {
        self.change_with(revision, location, id, |_| Ok(())).await
    }
    pub async fn change_with(
        &self,
        revision: u64,
        location: StorageLocation,
        id: &str,
        mut progress: impl FnMut(StorageProgress) -> Result<()>,
    ) -> Result<StorageStatus> {
        self.validate()?;
        remote::publication::request_id(id)?;
        record::safe_dir(&self.control)?;
        let control_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(self.control.join("control.lock"))?;
        control_lock.try_lock().map_err(|_| Error::Busy)?;
        let current = record::current(self)?;
        let receipt_path = self.receipt(id);
        if receipt_path.exists() {
            let receipt: Move = record::read(&receipt_path)?;
            self.valid(&receipt)?;
            if receipt.to != location || receipt.revision != revision {
                return Err(Error::Binding);
            }
            if ["complete", "rolled_back"].contains(&receipt.phase.as_str()) {
                record::clear_active(self, id)?;
                return self.status();
            }
        } else {
            if self.status()?.pending.is_some() {
                return Err(Error::Busy);
            }
            if current.revision != revision {
                return Err(Error::Jj(
                    "Storage settings changed; refresh before moving history".into(),
                ));
            }
            if current.location == location {
                return self.status();
            }
        }
        let from = if receipt_path.exists() {
            record::read::<Move>(&receipt_path)?.from
        } else {
            current.location
        };
        let source = self.at(from);
        let destination = self.at(location);
        if !receipt_path.exists() && destination.state.exists() {
            return Err(Error::Jj(
                "The selected location already contains repository data".into(),
            ));
        }
        let _source = binding::lease(&source)?;
        let _destination = binding::lease(&destination)?;
        let _transport = binding::transport(&source, true)?;
        let _target_transport = binding::transport(&destination, true)?;
        if !receipt_path.exists() && !source.state.exists() {
            if binding::fence(&source)?.exists() {
                return Err(Error::Busy);
            }
            initialize::ensure(&source, &settings()?).await?;
        }
        let receipt = if receipt_path.exists() {
            record::read(&receipt_path)?
        } else {
            self.prepare(&source, &destination, &current, id)?
        };
        self.valid(&receipt)?;
        self.run(receipt, &mut progress).await?;
        self.status()
    }
    fn prepare(
        &self,
        source: &Binding,
        destination: &Binding,
        current: &Current,
        id: &str,
    ) -> Result<Move> {
        source.verify_saved()?;
        if destination.state.exists() {
            return Err(Error::Jj(
                "The selected location already contains repository data".into(),
            ));
        }
        let stage_parent = if destination.state == destination.root.join(".jj") {
            binding::private(&destination.root, "vault-history-staging")?
        } else {
            destination
                .state
                .parent()
                .ok_or(Error::Binding)?
                .join(".vault-history-staging")
        };
        let archive_parent = if source.state == source.root.join(".jj") {
            binding::private(&source.root, "vault-history-retired")?
        } else {
            source
                .state
                .parent()
                .ok_or(Error::Binding)?
                .join(".vault-history-retired")
        };
        record::safe_dir(&stage_parent)?;
        record::safe_dir(&archive_parent)?;
        let stage = stage_parent.join(format!("{}-{id}", source.vault_id));
        let archive = archive_parent.join(format!("{}-{id}", source.vault_id));
        if stage.exists() || archive.exists() {
            return Err(Error::Binding);
        }
        fs::create_dir(&stage)?;
        let receipt = Move {
            version: 1,
            vault_id: source.vault_id.clone(),
            root_identity: root_identity(&source.root)?,
            request_id: id.into(),
            from: current.location,
            to: if destination.state == destination.root.join(".jj") {
                StorageLocation::Colocated
            } else {
                StorageLocation::External
            },
            revision: current.revision,
            source: source.clone(),
            destination: destination.clone(),
            source_identity: root_identity(&source.state)?,
            stage_identity: root_identity(&stage)?,
            stage,
            archive,
            phase: "prepared".into(),
        };
        record::save(&self.receipt(id), &receipt)?;
        record::save(&self.control.join("active.json"), &id)?;
        Ok(receipt)
    }
}
