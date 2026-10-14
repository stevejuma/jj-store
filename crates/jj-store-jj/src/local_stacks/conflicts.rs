//! Ordered native file conflict terms and explicit complete resolution in WIP.
use super::*;
use jj_lib::backend::MergedTreeValueExt as _;
use jj_lib::{backend::TreeValue, merge::Merge, merged_tree_builder::MergedTreeBuilder};
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksConflict")]
pub struct Conflict {
    pub source: String,
    pub path: String,
    pub terms: Vec<Option<String>>,
    pub reason: Option<String>,
}
pub async fn conflict(
    access: Arc<dyn Access>,
    root: PathBuf,
    request: HunkRequest,
) -> Result<Conflict> {
    let (_, repo, _, _) = preview::load(
        &access,
        &root,
        request.version,
        request.binding_generation,
        Some(request.operation),
        Some(request.revision),
    )
    .await?;
    let source = policy::commit(&repo, &request.source)?;
    let path = mutations::path(&request.path)?;
    let value = source
        .tree()
        .path_value(&path)
        .await
        .map_err(jj)?
        .simplify();
    if value.is_resolved() {
        return Err(Error::Invalid);
    }
    let mut terms = vec![];
    let mut reason = None;
    let limits = crate::local_browser::Limits::default();
    let mut text_bytes = 0usize;
    if value.to_file_merge().is_none()
        || value
            .to_executable_merge()
            .and_then(|v| jj_lib::conflicts::resolve_file_executable(&v))
            .is_none()
    {
        access.authorize().await.map_err(|_| Error::Denied)?;
        return Ok(Conflict {
            source: request.source,
            path: request.path,
            terms,
            reason: Some("This structural or mode conflict cannot be resolved as text.".into()),
        });
    }
    for term in value.iter() {
        let resolved = Merge::resolved(term.clone());
        if let Some(bytes) = hunks::bytes(repo.as_ref(), &path, &resolved).await? {
            text_bytes = text_bytes.saturating_add(bytes.len());
            if text_bytes > limits.max_text_bytes || terms.len() >= limits.max_changes {
                reason = Some("Conflict versions exceed the bounded text or version limit.".into());
                terms.clear();
                break;
            }
            terms.push(if term.is_some() {
                Some(String::from_utf8(bytes).map_err(jj)?)
            } else {
                None
            });
        } else {
            reason = Some(
                "This conflict requires native JJ resolution for its file type or size.".into(),
            );
            terms.clear();
            break;
        }
    }
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(Conflict {
        source: request.source,
        path: request.path,
        terms,
        reason,
    })
}
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    path: &str,
    content: &str,
) -> Result<Preview> {
    config.active().ok_or(Error::Invalid)?;
    if content.len() > crate::local_browser::Limits::default().max_text_bytes
        || content.contains('\0')
    {
        return Err(Error::Invalid);
    }
    let source = repo
        .store()
        .get_commit(
            repo.view()
                .get_wc_commit_id(workspace)
                .ok_or(Error::Invalid)?,
        )
        .map_err(jj)?;
    policy::rewritable(repo, workspace, settings, config, vec![source.id().clone()])?;
    let path = mutations::path(path)?;
    let value = source
        .tree()
        .path_value(&path)
        .await
        .map_err(jj)?
        .simplify();
    if value.is_resolved()
        || value.to_file_merge().is_none()
        || value
            .to_executable_merge()
            .and_then(|v| jj_lib::conflicts::resolve_file_executable(&v))
            .is_none()
    {
        return Err(Error::Invalid);
    }
    Ok(Preview{title:"Resolve file conflict".into(),summary:"Replace the complete native conflict value with the reviewed text in WIP. Applied branches and all other conflict terms/paths are retained.".into(),paths:vec![path.as_internal_file_string().into()]})
}
pub(super) async fn execute(
    mutable: &mut jj_lib::repo::MutableRepo,
    context: MutationContext<'_>,
    path: &str,
    content: &str,
) -> Result<()> {
    let MutationContext {
        repo,
        workspace,
        settings,
        config,
        wc,
    } = context;
    validate(repo, workspace, settings, config, path, content).await?;
    let path = mutations::path(path)?;
    let value = wc.tree().path_value(&path).await.map_err(jj)?;
    let executable = value
        .to_executable_merge()
        .and_then(|v| jj_lib::conflicts::resolve_file_executable(&v))
        .ok_or(Error::Invalid)?;
    let id = repo
        .store()
        .write_file(&path, &mut content.as_bytes())
        .await
        .map_err(jj)?;
    let mut tree = MergedTreeBuilder::new(wc.tree());
    tree.set_or_remove(
        path,
        Merge::resolved(Some(TreeValue::File {
            id,
            executable,
            copy_id: jj_lib::backend::CopyId::placeholder(),
        })),
    );
    mutable
        .rewrite_commit(wc)
        .set_tree(tree.write_tree().await.map_err(jj)?)
        .write()
        .await
        .map_err(jj)?;
    Ok(())
}
