use super::*;
fn path(value: &str) -> Result<()> {
    crate::local_browser::working_tree::safe_path(value)?;
    if value.len() > 4096 || value.split('/').count() > 64 {
        return Err(Error::Limit);
    }
    Ok(())
}
fn parent(value: &str) -> &str {
    value.rsplit_once('/').map_or("", |(p, _)| p)
}
impl PhysicalInventoryV1 {
    pub(super) fn structure(&self) -> Result<()> {
        if self.version != 1
            || self.repository.as_bytes() == &[0; 16]
            || self.generation == 0
            || self.workspace == m::RecordDigest::new([0; 32])
            || self.directories.is_empty()
        {
            return Err(Error::Invalid);
        }
        if self.directories.len() + self.files.len() > MAX_RECORDS {
            return Err(Error::Limit);
        }
        let mut directories = BTreeSet::new();
        let mut previous: Option<&str> = None;
        for directory in &self.directories {
            if !directory.path.is_empty() {
                path(&directory.path)?;
            }
            if previous.is_some_and(|p| p >= directory.path.as_str())
                || directory.stamp[1] == 0
                || directory.stamp[3] & 0o170000 != 0o040000
                || (!directory.path.is_empty() && !directories.contains(parent(&directory.path)))
            {
                return Err(Error::Invalid);
            }
            directories.insert(directory.path.as_str());
            previous = Some(&directory.path);
        }
        if !self.directories[0].path.is_empty() {
            return Err(Error::Invalid);
        }
        previous = None;
        for file in &self.files {
            path(&file.path)?;
            if previous.is_some_and(|p| p >= file.path.as_str())
                || directories.contains(file.path.as_str())
                || !directories.contains(parent(&file.path))
                || file.fingerprint == m::RecordDigest::new([0; 32])
            {
                return Err(Error::Invalid);
            }
            previous = Some(&file.path);
        }
        Ok(())
    }
}
