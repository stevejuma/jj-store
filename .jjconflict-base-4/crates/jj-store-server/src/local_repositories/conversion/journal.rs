//! Small name reservations are separate from bounded immutable archives and reports.
use super::*;
impl DeviceRepositories {
    pub(super) fn conversion_file(&self, request: RequestId, extension: &str) -> PathBuf {
        self.path
            .parent()
            .unwrap()
            .join(format!("conversion-{request}.{extension}"))
    }
    pub(super) fn read_conversion_file(&self, path: &Path, max: u64) -> Result<Vec<u8>> {
        let info = fs::symlink_metadata(path).map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            if info.permissions().mode() & 0o077 != 0 {
                return Err(Error::Unavailable);
            }
        }
        if !info.is_file() || info.file_type().is_symlink() {
            return Err(Error::Unavailable);
        }
        policy::read_regular(path, max).map_err(|_| Error::Unavailable)
    }
    pub(super) fn write_conversion_file(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        let mut file = tempfile::NamedTempFile::new_in(path.parent().unwrap())
            .map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            file.as_file()
                .set_permissions(fs::Permissions::from_mode(0o600))
                .map_err(|_| Error::Unavailable)?;
        }
        file.write_all(bytes).map_err(|_| Error::Unavailable)?;
        file.as_file().sync_all().map_err(|_| Error::Unavailable)?;
        file.persist(path).map_err(|_| Error::Unavailable)?;
        fs::File::open(path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)
    }
    pub(super) fn save_conversion(&self, value: &Conversion) -> Result<()> {
        self.write_conversion_file(
            &self.conversion_file(value.request.request_id, "json"),
            &serde_json::to_vec(value).map_err(|_| Error::Unavailable)?,
        )
    }
    pub(super) fn read_conversion(&self, request: RequestId) -> Result<Option<Conversion>> {
        let path = self.conversion_file(request, "json");
        match fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Unavailable),
            Ok(_) => {}
        }
        let bytes = self.read_conversion_file(&path, 16 * 1024)?;
        let value: Conversion = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if value.version != 1
            || value.installation != self.identity.installation
            || value.request.request_id != request
            || value.request.intent.version != 1
            || value.request.intent.source_profile == value.request.intent.destination_profile
            || !crate::registry::valid_name(&value.request.intent.name)
            || value.request.prepared_source.len() != 64
            || hex::decode(&value.request.prepared_source).is_err()
            || value.repository == value.request.intent.source
            || value
                .attempt
                .as_ref()
                .is_some_and(|s| s.len() != 32 || hex::decode(s).is_err())
            || value.prepared != value.prepared_digest.is_some()
            || value
                .prepared_digest
                .as_ref()
                .is_some_and(|s| s.len() != 64 || hex::decode(s).is_err())
            || value.prepared && value.attempt.is_none()
            || value.committed && !value.prepared
            || value.publication_attempted && !value.prepared
            || serde_json::to_vec(&value).map_err(|_| Error::Unavailable)? != bytes
        {
            return Err(Error::Unavailable);
        }
        match (&value.destination, value.request.intent.destination_profile) {
            (Some(path), ConversionProfile::LocalJjGit) if path.is_absolute() => {}
            (None, ConversionProfile::Native) => {}
            _ => return Err(Error::Unavailable),
        }
        Ok(Some(value))
    }
    pub(super) fn read_prepared(&self, value: &Conversion) -> Result<Prepared> {
        let bytes = self.read_conversion_file(
            &self.conversion_file(value.request.request_id, "prepared"),
            32 * 1024 * 1024,
        )?;
        let prepared: Prepared = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if value.prepared_digest.as_deref() != Some(digest(&bytes).as_str())
            || serde_json::to_vec(&prepared).map_err(|_| Error::Unavailable)? != bytes
            || prepared.report.version != 1
            || prepared.report.request_id != value.request.request_id
            || prepared.report.source != value.request.intent.source
            || prepared.report.destination != value.repository
            || prepared.report.destination_profile != value.request.intent.destination_profile
            || prepared.report.name != value.request.intent.name
            || prepared.report.archive_digest != value.request.prepared_source
        {
            return Err(Error::IdentityChanged);
        }
        match value.request.intent.destination_profile {
            ConversionProfile::Native if prepared.native && prepared.metadata.is_none() => {}
            ConversionProfile::LocalJjGit if !prepared.native && prepared.metadata.is_some() => {}
            _ => return Err(Error::Unavailable),
        }
        Ok(prepared)
    }
}
