use super::{CaptureResult, MAX_FILES, PrivateDirectory, lock_name, private, sync_dir};
use std::fs;
impl PrivateDirectory {
    /// Only called under the registry gate. Bounded crash-temp recovery, no lease eviction.
    pub(in crate::source_registry) fn recover(&self) -> CaptureResult<()> {
        self.inspect_entries(true)
    }
    pub(in crate::source_registry) fn validate_contents(&self) -> CaptureResult<()> {
        self.inspect_entries(false)
    }
    fn inspect_entries(&self, recover: bool) -> CaptureResult<()> {
        self.revalidate()?;
        let mut count = 0;
        let mut removed = 0;
        let mut temporary = 0;
        for entry in fs::read_dir(&self.physical)? {
            let entry = entry?;
            count += 1;
            if count > MAX_FILES + 1 {
                return Err("source registry file capacity exhausted".into());
            }
            private(&fs::symlink_metadata(entry.path())?, self.owner, false)?;
            let name = entry.file_name();
            let name = name.to_str().ok_or("unexpected source registry entry")?;
            if name.starts_with(".tmp") {
                temporary += 1;
                if temporary > 1 {
                    return Err("source registry temporary capacity exhausted".into());
                }
                if recover {
                    fs::remove_file(entry.path())?;
                    removed += 1;
                }
            } else if name != "sources.json"
                && name != "registry.lock"
                && !lock_name(name, "physical-", 64)
                && !lock_name(name, "source-", 32)
            {
                return Err("unexpected source registry entry".into());
            }
        }
        if removed > 0 {
            sync_dir(&self.physical)?;
        }
        if count - temporary > MAX_FILES {
            return Err("source registry file capacity exhausted".into());
        }
        self.revalidate()
    }
}
