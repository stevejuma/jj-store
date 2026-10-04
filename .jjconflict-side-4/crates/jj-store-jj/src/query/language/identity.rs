//! Read-only identity hints. Configuration never enters query evaluation settings.
use etcetera::BaseStrategy as _;
use jj_lib::config::{ConfigLayer, ConfigResolutionContext, ConfigSource, StackedConfig};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn valid(value: String) -> Option<String> {
    let value = value.trim();
    (!value.is_empty() && value.len() <= 1024 && !value.chars().any(char::is_control))
        .then(|| value.to_owned())
}
fn config_dir() -> Option<PathBuf> {
    etcetera::choose_base_strategy()
        .ok()
        .map(|strategy| strategy.config_dir().join("jj"))
}
fn user_paths() -> Vec<PathBuf> {
    if let Some(paths) = std::env::var_os("JJ_CONFIG") {
        return std::env::split_paths(&paths)
            .filter(|p| !p.as_os_str().is_empty())
            .collect();
    }
    let mut paths = Vec::new();
    if cfg!(unix) {
        paths.extend([
            PathBuf::from("/etc/jj/config.toml"),
            PathBuf::from("/etc/jj/conf.d"),
        ]);
    }
    if let Ok(home) = etcetera::home_dir() {
        paths.push(home.join(".jjconfig.toml"));
    }
    if let Some(root) = config_dir() {
        paths.extend([root.join("config.toml"), root.join("conf.d")]);
    }
    paths
}
fn email(paths: &[PathBuf], repository: Option<&Path>) -> Option<String> {
    let mut config = StackedConfig::empty();
    for path in paths {
        let mut files = if path.is_dir() {
            let mut files: Vec<_> = std::fs::read_dir(path)
                .ok()?
                .take(65)
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect();
            if files.len() > 64 {
                return None;
            }
            files.sort();
            files
        } else {
            vec![path.clone()]
        };
        for file in files.drain(..) {
            let Ok(metadata) = std::fs::metadata(&file) else {
                continue;
            };
            if !metadata.is_file() || metadata.len() > 262144 {
                continue;
            }
            if let Ok(layer) = ConfigLayer::load_from_file(ConfigSource::User, file) {
                config.add_layer(layer);
            }
        }
    }
    let home = etcetera::home_dir().ok();
    let hostname = whoami::hostname().unwrap_or_default();
    let environment: HashMap<_, _> = std::env::vars_os()
        .filter_map(|(k, v)| Some((k.into_string().ok()?, v.into_string().ok()?)))
        .collect();
    let context = ConfigResolutionContext {
        home_dir: home.as_deref(),
        repo_path: repository,
        workspace_path: repository.and_then(Path::parent).and_then(Path::parent),
        command: None,
        hostname: &hostname,
        environment: &environment,
    };
    let config = jj_lib::config::resolve(&config, &context).ok()?;
    valid(config.get::<String>("user.email").ok()?)
}
/// Host preference; no config values other than the email are returned.
pub fn host_email() -> Option<String> {
    std::env::var("JJ_EMAIL")
        .ok()
        .and_then(valid)
        .or_else(|| email(&user_paths(), None))
}
fn repository_paths(metadata: &Path) -> Vec<PathBuf> {
    repository_paths_at(metadata, config_dir().as_deref())
}
fn repository_paths_at(metadata: &Path, root: Option<&Path>) -> Vec<PathBuf> {
    // Do not call JJ's secure-config loader: it can migrate configs during reads.
    let id_file = metadata.join("config-id");
    if std::fs::metadata(&id_file).is_ok_and(|m| m.len() > 128) {
        return vec![];
    }
    if let Ok(id) = std::fs::read_to_string(id_file) {
        let id = id.trim();
        if id.len() != 20 || !id.bytes().all(|c| c.is_ascii_hexdigit()) {
            return vec![];
        }
        if let Some(root) = root {
            let folder = root.join("repo").join(id);
            if let Ok(binding) = jj_lib::secure_config::read_metadata(&folder)
                && jj_lib::secure_config::metadata_path(&binding)
                    .ok()
                    .flatten()
                    == Some(metadata)
            {
                return vec![folder.join("config.toml")];
            }
        }
        return vec![];
    }
    vec![metadata.join("config.toml")]
}
/// Local registered folder fallback, without loading custom aliases into the evaluator.
pub fn local_email(metadata: &Path, git: &gix::Repository) -> Option<String> {
    local_email_with(host_email(), metadata, git)
}
fn local_email_with(
    host: Option<String>,
    metadata: &Path,
    git: &gix::Repository,
) -> Option<String> {
    host.or_else(|| email(&repository_paths(metadata), Some(metadata)))
        .or_else(|| {
            git.config_snapshot()
                .string("user.email")
                .and_then(|v| String::from_utf8(v.to_vec()).ok())
                .and_then(valid)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn secure_repository_config_requires_matching_identity_without_migration() {
        use prost::Message as _;
        let temp = tempfile::tempdir().unwrap();
        let metadata = temp.path().join("metadata");
        let root = temp.path().join("user/jj");
        let id = "12".repeat(10);
        let folder = root.join("repo").join(&id);
        std::fs::create_dir_all(&metadata).unwrap();
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(metadata.join("config-id"), &id).unwrap();
        let binding = jj_lib::protos::secure_config::ConfigMetadata {
            path: Some(metadata.as_os_str().as_encoded_bytes().to_vec()),
        }
        .encode_to_vec();
        std::fs::write(folder.join("metadata.binpb"), &binding).unwrap();
        std::fs::write(
            folder.join("config.toml"),
            "[user]\nemail='secure@example.invalid'\n",
        )
        .unwrap();
        assert_eq!(
            repository_paths_at(&metadata, Some(&root)),
            vec![folder.join("config.toml")]
        );
        let foreign = temp.path().join("foreign");
        std::fs::create_dir(&foreign).unwrap();
        std::fs::write(foreign.join("config-id"), &id).unwrap();
        assert!(repository_paths_at(&foreign, Some(&root)).is_empty());
        assert_eq!(
            std::fs::read(folder.join("metadata.binpb")).unwrap(),
            binding
        );
        assert_eq!(
            std::fs::read_to_string(metadata.join("config-id")).unwrap(),
            id
        );
        assert!(!metadata.join("config.toml").exists());
    }
    #[test]
    fn identity_reads_are_bounded_scoped_and_do_not_modify_configs() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("config.toml");
        let text =
            "[user]\nemail = 'host@example.invalid'\n[revset-aliases]\n'mutable()' = 'none()'\n";
        std::fs::write(&path, text).unwrap();
        assert_eq!(
            email(std::slice::from_ref(&path), None).as_deref(),
            Some("host@example.invalid")
        );
        assert_eq!(std::fs::read_to_string(&path).unwrap(), text);
        std::fs::write(&path, "[user]\nemail = ' ' ").unwrap();
        assert_eq!(email(std::slice::from_ref(&path), None), None);
        assert_eq!(valid("bad\nemail".into()), None);
        assert_eq!(valid("x".repeat(1025)), None);
        assert_eq!(repository_paths(temp.path()), vec![path]);
    }
}

#[cfg(test)]
mod fallback_tests {
    use super::*;
    #[test]
    fn host_then_repository_jj_then_git_with_read_only_fallback() {
        let temp = tempfile::tempdir().unwrap();
        let git_path = temp.path().join("git");
        gix::init_bare(&git_path).unwrap();
        std::fs::write(
            git_path.join("config"),
            "[core]\nbare = true\n[user]\nemail = git@example.invalid\n",
        )
        .unwrap();
        let git = gix::open(&git_path).unwrap();
        let metadata = temp.path().join("jj");
        std::fs::create_dir(&metadata).unwrap();
        assert_eq!(
            local_email_with(None, &metadata, &git).as_deref(),
            Some("git@example.invalid")
        );
        std::fs::write(
            metadata.join("config.toml"),
            "[user]\nemail = 'repo@example.invalid'\n",
        )
        .unwrap();
        assert_eq!(
            local_email_with(None, &metadata, &git).as_deref(),
            Some("repo@example.invalid")
        );
        assert_eq!(
            local_email_with(Some("host@example.invalid".into()), &metadata, &git).as_deref(),
            Some("host@example.invalid")
        );
        assert!(!metadata.join("config-id").exists());
        std::fs::write(metadata.join("config.toml"), "invalid TOML").unwrap();
        assert_eq!(
            local_email_with(None, &metadata, &git).as_deref(),
            Some("git@example.invalid")
        );
    }
}
