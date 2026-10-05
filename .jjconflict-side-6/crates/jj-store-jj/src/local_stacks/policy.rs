//! Configured JJ immutable policy; inspect selected ancestry without enumerating history.
use super::*;
use jj_lib::repo::Repo;
use jj_lib::{
    fileset::FilesetAliasesMap,
    revset::{
        self, RevsetAliasesMap, RevsetDiagnostics, RevsetExpression, RevsetExtensions,
        RevsetParseContext, RevsetWorkspaceContext, SymbolResolver,
    },
    ui_path::RepoPathUiConverter,
};

pub(super) fn expression(
    repo: &dyn Repo,
    workspace_name: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    text: &str,
) -> Result<Arc<revset::ResolvedRevsetExpression>> {
    let mut aliases = RevsetAliasesMap::new();
    // Upstream jj-cli 0.45.1 defaults, overridden by the user's own aliases below.
    for (name, value) in [
        (
            "trunk()",
            "latest(remote_bookmarks(exact:\"main\", exact:\"origin\") | remote_bookmarks(exact:\"master\", exact:\"origin\") | remote_bookmarks(exact:\"trunk\", exact:\"origin\") | remote_bookmarks(exact:\"main\", exact:\"upstream\") | remote_bookmarks(exact:\"master\", exact:\"upstream\") | remote_bookmarks(exact:\"trunk\", exact:\"upstream\") | root())",
        ),
        (
            "builtin_immutable_heads()",
            "trunk() | tags() | untracked_remote_bookmarks() | untracked_remote_tags()",
        ),
        ("immutable_heads()", "builtin_immutable_heads()"),
        ("immutable()", "::(immutable_heads() | root())"),
        ("mutable()", "~immutable()"),
    ] {
        aliases.insert(name, value, None).map_err(jj)?;
    }
    for key in settings.table_keys("revset-aliases") {
        aliases
            .insert(
                key,
                settings.get_string(["revset-aliases", key]).map_err(jj)?,
                None,
            )
            .map_err(jj)?;
    }
    let mut files = FilesetAliasesMap::default();
    for key in settings.table_keys("fileset-aliases") {
        files
            .insert(
                key,
                settings.get_string(["fileset-aliases", key]).map_err(jj)?,
                None,
            )
            .map_err(jj)?;
    }
    let extensions = RevsetExtensions::default();
    let converter = RepoPathUiConverter::Fs {
        cwd: PathBuf::new(),
        base: PathBuf::new(),
    };
    let context = RevsetParseContext {
        aliases_map: &aliases,
        local_variables: Default::default(),
        user_email: settings.user_email(),
        date_pattern_context: chrono::Local::now().into(),
        default_ignored_remote: Some("git".as_ref()),
        fileset_aliases_map: &files,
        extensions: &extensions,
        workspace: Some(RevsetWorkspaceContext {
            path_converter: &converter,
            workspace_name: workspace_name,
        }),
    };
    revset::parse(&mut RevsetDiagnostics::new(), text, &context)
        .map_err(jj)?
        .resolve_user_expression(
            repo,
            &SymbolResolver::new(repo, extensions.symbol_resolvers()),
        )
        .map_err(jj)
}
pub(super) fn rewritable(
    repo: &ReadonlyRepo,
    workspace_name: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    commits: Vec<jj_lib::backend::CommitId>,
) -> Result<()> {
    let immutable = expression(
        repo,
        workspace_name,
        settings,
        "::(immutable_heads() | root())",
    )?;
    let descendants = RevsetExpression::commits(commits).descendants();
    if !immutable
        .intersection(&descendants)
        .evaluate(repo)
        .map_err(jj)?
        .is_empty()
        .map_err(jj)?
    {
        return Err(Error::Invalid);
    }
    if let Some(saved) = config.active() {
        let target = resolve(repo, &saved.target)?;
        if !RevsetExpression::commits(vec![target.id().clone()])
            .ancestors()
            .intersection(&descendants)
            .evaluate(repo)
            .map_err(jj)?
            .is_empty()
            .map_err(jj)?
        {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}

/// Object presence alone never authorizes an arbitrary historical rewrite.
pub(super) fn commit(repo: &ReadonlyRepo, id: &str) -> Result<Commit> {
    let id = jj_lib::backend::CommitId::try_from_hex(id).ok_or(Error::Invalid)?;
    if id.as_bytes().len() != repo.store().commit_id_length() {
        return Err(Error::Invalid);
    }
    if RevsetExpression::all()
        .intersection(&RevsetExpression::commits(vec![id.clone()]))
        .evaluate(repo)
        .map_err(jj)?
        .is_empty()
        .map_err(jj)?
    {
        return Err(Error::Invalid);
    }
    repo.store().get_commit(&id).map_err(jj)
}
