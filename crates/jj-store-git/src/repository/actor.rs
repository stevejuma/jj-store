use super::LocalRepository;
use crate::work_api::{Error, Result};
use jj_store_model::PrincipalId;
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt as _, OpenOptionsExt as _};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read as _, Write as _},
};

impl LocalRepository {
    pub fn actor(&self) -> Result<Option<PrincipalId>> {
        if let Some(host) = &self.host {
            return host.actor(false).map_err(Into::into);
        }
        let path = self.repo_path.join("lapis/actor-v1");
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Unavailable),
        };
        if fs::symlink_metadata(path)
            .map_err(|_| Error::Unavailable)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::Corrupt);
        }
        let mut bytes = Vec::new();
        file.take(34)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::Unavailable)?;
        if bytes.len() != 33 || bytes[32] != b'\n' {
            return Err(Error::Corrupt);
        }
        Ok(Some(
            std::str::from_utf8(&bytes[..32])
                .map_err(|_| Error::Corrupt)?
                .parse()
                .map_err(|_| Error::Corrupt)?,
        ))
    }
    /// Establish device provenance for an explicitly requested local write.
    /// Read-only facades use `actor` and must never call this method.
    pub fn ensure_actor(&self) -> Result<PrincipalId> {
        if let Some(host) = &self.host {
            return host.actor(true)?.ok_or(Error::Denied);
        }
        if let Some(actor) = self.actor()? {
            return Ok(actor);
        }
        let directory = self.repo_path.join("lapis");
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        builder.mode(0o700);
        builder.create(&directory).map_err(|_| Error::Unavailable)?;
        if fs::symlink_metadata(&directory)
            .map_err(|_| Error::Unavailable)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::Corrupt);
        }
        let mut bytes = [0; 16];
        getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
        let actor = PrincipalId::new(bytes);
        let staging = directory.join(format!(".actor-{actor}"));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        options.mode(0o600);
        let mut file = options.open(&staging).map_err(|_| Error::Unavailable)?;
        file.write_all(format!("{actor}\n").as_bytes())
            .map_err(|_| Error::Unavailable)?;
        file.sync_all().map_err(|_| Error::Unavailable)?;
        let result = fs::hard_link(&staging, directory.join("actor-v1"));
        fs::remove_file(staging).map_err(|_| Error::Unavailable)?;
        match result {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err(Error::Unavailable),
        }
        File::open(&directory)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)?;
        File::open(&self.repo_path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)?;
        self.actor()?.ok_or(Error::Corrupt)
    }
}
