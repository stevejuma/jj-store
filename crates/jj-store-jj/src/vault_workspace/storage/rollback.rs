use super::*;
impl Storage {
    /// Keep the previous layout only before an owned staged copy is installed.
    /// Never delete a destination, or reinterpret an already published layout.
    pub async fn rollback(&self, id: &str) -> Result<StorageStatus> {
        self.validate()?;
        remote::publication::request_id(id)?;
        let control_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(self.control.join("control.lock"))?;
        control_lock.try_lock().map_err(|_| Error::Busy)?;
        let mut value: Move = record::read(&self.receipt(id))?;
        self.valid(&value)?;
        if ["complete", "rolled_back"].contains(&value.phase.as_str()) {
            record::clear_active(self, id)?;
            return self.status();
        }
        let current = record::current(self)?;
        if current.revision != value.revision || current.location != value.from {
            return Err(Error::Busy);
        }
        if value.destination.state.exists()
            && record::same(&value.destination.state, &value.stage_identity)
        {
            return Err(Error::Jj(
                "Finish the installed storage move before making another layout choice".into(),
            ));
        }
        let _source = binding::lease(&value.source)?;
        let _destination = binding::lease(&value.destination)?;
        let _transport = binding::transport(&value.source, true)?;
        let _target = binding::transport(&value.destination, true)?;
        if value.source.state.exists() {
            if !record::same(&value.source.state, &value.source_identity) {
                return Err(Error::Binding);
            }
        } else {
            if !record::same(&value.archive, &value.source_identity) {
                return Err(Error::Binding);
            }
            fs::rename(&value.archive, &value.source.state)?;
            File::open(value.source.state.parent().ok_or(Error::Binding)?)?.sync_all()?;
        }
        for binding in [&value.source, &value.destination] {
            let fence = binding::fence(binding)?;
            if fence.exists() {
                let selected: serde_json::Value = record::read(&fence)?;
                if selected["request_id"] != id
                    || selected["control"] != self.control.to_string_lossy().as_ref()
                {
                    return Err(Error::Binding);
                }
                fs::remove_file(&fence)?;
                File::open(fence.parent().ok_or(Error::Binding)?)?.sync_all()?;
            }
        }
        // The archive retains the original binding and workspace registration.
        // Validate under the source lease we already own; relink would try to
        // acquire that same exclusive lease again.
        value.source.verify_saved()?;
        value.phase = "rolled_back".into();
        record::save(&self.receipt(id), &value)?;
        record::clear_active(self, id)?;
        self.status()
    }
}
