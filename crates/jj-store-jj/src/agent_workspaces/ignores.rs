//! Explicit ignore-policy inputs shared with a read-only workspace listing.
use super::{Provider, Result, Selection};
use std::{fs::File, io::Read as _, path::PathBuf};
impl Provider {
    pub fn ignore_inputs(&self, selection: &Selection) -> Result<Vec<Vec<u8>>> {
        let store = selection.metadata.join("store");
        let kind =
            std::fs::read_to_string(store.join("type")).map_err(|_| "workspace_unavailable")?;
        let mut configured = None;
        let mut exclude = None;
        if kind.trim() == "git" {
            let settings = jj_lib::settings::UserSettings::from_config(
                jj_lib::config::StackedConfig::with_defaults(),
            )
            .map_err(|_| "ignore_policy_unavailable")?;
            let metadata = crate::local_repository::metadata_store(&settings, &selection.metadata)
                .map_err(|_| "ignore_policy_unavailable")?;
            let git = gix::open(metadata.git_dir()).map_err(|_| "ignore_policy_unavailable")?;
            configured = git
                .config_snapshot()
                .string("core.excludesFile")
                .map(|s| {
                    std::str::from_utf8(&s)
                        .map(jj_lib::file_util::expand_home_path)
                        .map_err(|_| "invalid_ignore_policy")
                })
                .transpose()?;
            exclude = Some(metadata.git_dir().join("info/exclude"));
        }
        let global = match configured {
            Some(path) => selection.root.join(path),
            None => match std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
                Some(path) => PathBuf::from(path).join("git/ignore"),
                None => etcetera::home_dir()
                    .map_err(|_| "ignore_policy_unavailable")?
                    .join(".config/git/ignore"),
            },
        };
        let mut inputs = Vec::new();
        for path in std::iter::once(global).chain(exclude) {
            if !path.is_absolute() {
                return Err("invalid_ignore_policy");
            }
            match rustix::fs::open(
                &path,
                rustix::fs::OFlags::RDONLY
                    | rustix::fs::OFlags::NOFOLLOW
                    | rustix::fs::OFlags::CLOEXEC
                    | rustix::fs::OFlags::NONBLOCK,
                rustix::fs::Mode::empty(),
            ) {
                Ok(fd) => {
                    let file = File::from(fd);
                    if !file
                        .metadata()
                        .map_err(|_| "ignore_policy_unavailable")?
                        .is_file()
                    {
                        return Err("invalid_ignore_policy");
                    }
                    let mut bytes = vec![];
                    file.take(65537)
                        .read_to_end(&mut bytes)
                        .map_err(|_| "ignore_policy_unavailable")?;
                    if bytes.len() > 65536 {
                        return Err("ignore_policy_limit");
                    }
                    inputs.push(bytes);
                }
                Err(rustix::io::Errno::NOENT) => inputs.push(vec![]),
                Err(_) => return Err("ignore_policy_unavailable"),
            }
        }
        Ok(inputs)
    }
}
