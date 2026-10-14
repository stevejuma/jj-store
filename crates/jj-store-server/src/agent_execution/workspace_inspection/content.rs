use super::*;
pub(super) struct FileSources<'a> {
    pub source: &'a source::Source,
    pub before_source: Option<&'a source::Source>,
    pub after_exists: bool,
    pub before_exists: bool,
}
use futures::{StreamExt as _, stream};
impl Supervisor {
    pub(super) async fn workspace_content(
        &self,
        after: &Workspace,
        repository: RepositoryId,
        path: &str,
        comparison: Option<&dto::Comparison>,
        sources: FileSources<'_>,
    ) -> Result<dto::FileContent> {
        let FileSources {
            source,
            before_source,
            after_exists,
            before_exists,
        } = sources;
        let mut result = dto::FileContent {
            path: path.into(),
            fingerprint: String::new(),
            contents: None,
            before: None,
            unavailable: None,
        };
        if !after_exists {
            // A verified deletion has no after-side; it is not an unreadable file.
        } else if comparison.is_none_or(|c| c.current_files) {
            let root = after.path.clone();
            let path = path.to_owned();
            match tokio::task::spawn_blocking(move || {
                jj_store_service::working_copy::read(&root, repository, 0, &path)
            })
            .await
            .map_err(|_| Error::Unavailable)?
            {
                Ok(file) => {
                    result.fingerprint = file.selection.fingerprint;
                    result.contents = Some(file.contents);
                }
                Err(_) => {
                    result.unavailable = Some(
                        "Current file is missing, unsupported, changed, or exceeds the text limit."
                            .into(),
                    )
                }
            }
        } else {
            match source.file(&after.commit, path).await {
                Ok(text) if !text.contains('\0') && text.len() <= 256 * 1024 => {
                    result.fingerprint = hex::encode(Sha256::digest(text.as_bytes()));
                    result.contents = Some(text);
                }
                _ => {
                    result.unavailable =
                        Some("Saved file content is unavailable or exceeds the text limit.".into())
                }
            }
        }
        if let Some(c) = comparison
            && let (Some(operation), Some(commit)) = (c.before_operation, &c.before_commit)
        {
            let mut before = after.clone();
            before.operation = operation;
            before.commit = commit.clone();
            if before_exists {
                match before_source
                    .ok_or(Error::Unavailable)?
                    .file(commit, path)
                    .await
                {
                    Ok(text) if !text.contains('\0') && text.len() <= 256 * 1024 => {
                        result.before = Some(text)
                    }
                    _ => {
                        result.unavailable = Some(
                            "The original file content is unavailable or exceeds the text limit."
                                .into(),
                        )
                    }
                }
            }
        }
        Ok(result)
    }
    pub(super) async fn workspace_changed_paths(
        &self,
        after: &Workspace,
        repository: RepositoryId,
        comparison: &dto::Comparison,
        entries: Vec<dto::FileEntry>,
        source: &source::Source,
        before_source: &source::Source,
    ) -> Result<(Vec<dto::FileEntry>, Option<String>)> {
        let mut before = after.clone();
        before.operation = comparison.before_operation.ok_or(Error::Unavailable)?;
        before.commit = comparison.before_commit.clone().ok_or(Error::Unavailable)?;
        let original: BTreeSet<_> = before_source
            .files(&before.commit)
            .await?
            .iter()
            .map(|e| e.path.clone())
            .collect();
        let mut paths: BTreeMap<_, _> = entries.into_iter().map(|e| (e.path.clone(), e)).collect();
        let current: BTreeSet<_> = paths.keys().cloned().collect();
        for path in &original {
            paths.entry(path.clone()).or_insert_with(|| dto::FileEntry {
                path: path.clone(),
                kind: "file".into(),
                bytes: 0,
                fingerprint: String::new(),
                status: Some("deleted".into()),
                additions: None,
                deletions: None,
            });
        }
        // Bound expensive content comparison separately from file-tree paging.
        if paths.len() > 4096 {
            return Err(Error::Limit);
        }
        let compared = tokio::time::timeout(
            Duration::from_secs(20),
            stream::iter(paths.into_values().map(|mut entry| {
                let original = &original;
                let current = &current;
                async move {
                    let before_exists = original.contains(&entry.path);
                    let after_exists = current.contains(&entry.path);
                    if !before_exists {
                        entry.status = Some("added".into());
                    }
                    let content = self
                        .workspace_content(
                            after,
                            repository,
                            &entry.path,
                            Some(comparison),
                            FileSources {
                                source,
                                before_source: Some(before_source),
                                after_exists,
                                before_exists,
                            },
                        )
                        .await?;
                    if content.unavailable.is_some() && before_exists && after_exists {
                        entry.status = Some("unavailable".into());
                    } else if before_exists && after_exists && content.contents != content.before {
                        entry.status = Some("modified".into());
                    } else if before_exists && after_exists && content.unavailable.is_none() {
                        return Ok(None);
                    }
                    if content.unavailable.is_none() {
                        let counts = tokio::task::spawn_blocking(move || {
                            jj_store_jj::agent_workspaces::inspection::text_change_counts(
                                content.before.as_deref().unwrap_or(""),
                                content.contents.as_deref().unwrap_or(""),
                            )
                        })
                        .await
                        .map_err(|_| Error::Unavailable)?;
                        if let Some((additions, deletions)) = counts {
                            entry.additions = Some(additions);
                            entry.deletions = Some(deletions);
                        }
                    }
                    Ok::<_, Error>(Some(entry))
                }
            }))
            .buffered(4)
            .collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| Error::Limit)?;
        let incomplete = compared
            .iter()
            .any(|v| matches!(v, Ok(Some(e)) if e.status.as_deref() == Some("unavailable")));
        let entries = compared
            .into_iter()
            .filter_map(|v| match v {
                Ok(Some(e)) if e.status.as_deref() == Some("unavailable") => None,
                Ok(Some(e)) => Some(Ok(e)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((
            entries,
            incomplete.then(|| {
                "Some paths could not be compared within the supported content limits.".into()
            }),
        ))
    }
}
