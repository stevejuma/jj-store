//! Snapshot policy uses JJ settings and Git's global/repository excludes.
use super::*;
use jj_lib::{
    config::{ConfigLayer, ConfigSource, StackedConfig},
    gitignore::GitIgnoreFile,
    repo_path::RepoPath,
    settings::{HumanByteSize, UserSettings},
    ui_path::RepoPathUiConverter,
};
use std::path::Path;
pub(super) fn load(root: &Path) -> Result<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    config.add_layer(
        ConfigLayer::parse(
            ConfigSource::Default,
            "[snapshot]\nauto-track = 'all()'\nmax-new-file-size = '1MiB'\n",
        )
        .map_err(jj)?,
    );
    let home = etcetera::home_dir().map_err(jj)?;
    let paths = if let Some(paths) = std::env::var_os("JJ_CONFIG") {
        std::env::split_paths(&paths).collect::<Vec<_>>()
    } else {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        vec![home.join(".jjconfig.toml"), base.join("jj/config.toml")]
    };
    for path in paths {
        if path.is_dir() {
            config.load_dir(ConfigSource::User, path).map_err(jj)?;
        } else if path.exists() {
            config.load_file(ConfigSource::User, path).map_err(jj)?;
        }
    }
    // The shared repo config belongs to the resolved JJ repository, including linked workspaces.
    let defaults = UserSettings::from_config(config.clone()).map_err(jj)?;
    let workspace = Workspace::load(
        &defaults,
        root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .map_err(jj)?;
    let path = workspace.repo_path().join("config.toml");
    if path.exists() {
        config.load_file(ConfigSource::Repo, path).map_err(jj)?;
    }
    let mut overrides = String::new();
    for (env, key) in [("JJ_USER", "user.name"), ("JJ_EMAIL", "user.email")] {
        if let Ok(value) = std::env::var(env) {
            overrides.push_str(&format!(
                "{key} = {}\n",
                serde_json::to_string(&value).map_err(jj)?
            ));
        }
    }
    if !overrides.is_empty() {
        config.add_layer(ConfigLayer::parse(ConfigSource::EnvOverrides, &overrides).map_err(jj)?);
    }
    UserSettings::from_config(config).map_err(jj)
}
pub(super) fn tracking(settings: &UserSettings) -> Result<Box<dyn jj_lib::matchers::Matcher>> {
    let expression = settings.get_string("snapshot.auto-track").map_err(jj)?;
    let mut aliases = jj_lib::fileset::FilesetAliasesMap::default();
    for key in settings.table_keys("fileset-aliases") {
        aliases
            .insert(
                key,
                settings.get_string(["fileset-aliases", key]).map_err(jj)?,
                None,
            )
            .map_err(jj)?;
    }
    let converter = RepoPathUiConverter::Fs {
        cwd: PathBuf::new(),
        base: PathBuf::new(),
    };
    let parsed = jj_lib::fileset::parse(
        &mut Default::default(),
        &expression,
        &jj_lib::fileset::FilesetParseContext {
            aliases_map: &aliases,
            path_converter: &converter,
        },
    )
    .map_err(jj)?;
    Ok(parsed.to_matcher())
}
pub(super) fn max_file_size(settings: &UserSettings) -> Result<u64> {
    let HumanByteSize(size) = settings
        .get_value_with("snapshot.max-new-file-size", TryInto::try_into)
        .map_err(jj)?;
    Ok(if size == 0 { u64::MAX } else { size })
}
pub(super) fn ignores(repo: &ReadonlyRepo, root: &Path) -> Result<Arc<GitIgnoreFile>> {
    let backend = jj_lib::git::get_git_backend(repo.store()).map_err(jj)?;
    let git_repo = backend.git_repo();
    let config = git_repo.config_snapshot();
    let global = config
        .string("core.excludesFile")
        .map(|value| {
            std::str::from_utf8(&value)
                .map(jj_lib::file_util::expand_home_path)
                .map_err(jj)
        })
        .transpose()?;
    let global = global.or_else(|| {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| etcetera::home_dir().ok().map(|h| h.join(".config")))
            .map(|p| p.join("git/ignore"))
    });
    let mut ignores = GitIgnoreFile::empty();
    if let Some(path) = global {
        ignores = ignores
            .chain_with_file(RepoPath::root(), root.join(path))
            .map_err(jj)?;
    }
    ignores
        .chain_with_file(
            RepoPath::root(),
            backend.git_repo_path().join("info/exclude"),
        )
        .map_err(jj)
}
