//! Private complete path/line totals. No snapshot, writable tree or source index.
use super::*;
use jj_lib::diff::DiffHunkKind;
#[derive(Clone, Debug, Serialize)]
pub struct LineCounts {
    pub additions: usize,
    pub deletions: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct Status {
    pub version: u32,
    pub operation: m::OperationId,
    pub workspace: String,
    pub working_copy: m::GitSha1Id,
    pub commit: dto::Commit,
    pub files: usize,
    /// Binary/structural/complexity refusals never become false zero line totals.
    pub lines: Option<LineCounts>,
    pub uncounted: usize,
}
impl Executor {
    /// Trusted composition supplies a complete observed tree. The source read
    /// remains operation-pinned; the caller must bracket physical observations.
    pub async fn working_tree_status(
        &self,
        access: Arc<dyn Access>,
        baseline: Baseline,
        entries: Vec<Entry>,
    ) -> Result<Status> {
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                if baseline.repository != access.repository()
                    || baseline.generation != access.generation()
                {
                    return Err(Error::EpochChanged);
                }
                let context = Context::open(
                    access,
                    baseline.operation,
                    budget,
                    limits,
                    true,
                    vec![],
                    false,
                )
                .await?;
                let current =
                    read(&context, baseline.operation, baseline.workspace.clone()).await?;
                if current != baseline {
                    return Err(Error::EpochChanged);
                }
                let commit = context.commit_record(baseline.commit).await?;
                let [parent] = commit.parents.as_slice() else {
                    return Err(Error::Invalid);
                };
                let parent = read_commit(
                    &context,
                    baseline.operation,
                    baseline.view,
                    baseline.workspace.clone(),
                    *parent,
                )
                .await?;
                let mut bytes = 0usize;
                let mut after = BTreeMap::new();
                for entry in entries {
                    safe_path(&entry.path)?;
                    bytes = bytes.checked_add(entry.data.len()).ok_or(Error::Limit)?;
                    if entry.data.len() > MAX_FILE_BYTES
                        || bytes > MAX_BYTES
                        || after.len() >= MAX_ENTRIES
                    {
                        return Err(Error::Limit);
                    }
                    context.budget.work(entry.data.len())?;
                    if entry.bytes != entry.data.len() as u64
                        || content_digest(&entry.data) != entry.content
                    {
                        return Err(Error::Invalid);
                    }
                    if after.insert(entry.path.clone(), entry).is_some() {
                        return Err(Error::Invalid);
                    }
                }
                let mut before: BTreeMap<_, _> = parent
                    .entries
                    .into_iter()
                    .map(|e| (e.path.clone(), e))
                    .collect();
                let paths: BTreeSet<_> = before.keys().chain(after.keys()).cloned().collect();
                let mut result = Status {
                    version: 1,
                    operation: baseline.operation,
                    workspace: baseline.workspace,
                    working_copy: baseline.commit,
                    commit,
                    files: 0,
                    lines: Some(LineCounts {
                        additions: 0,
                        deletions: 0,
                    }),
                    uncounted: 0,
                };
                for path in paths {
                    context.budget.authorize(context.access.as_ref()).await?;
                    let old = before.remove(&path);
                    let new = after.remove(&path);
                    if old == new {
                        continue;
                    }
                    result.files += 1;
                    let regular = |entry: &Option<Entry>| {
                        entry
                            .as_ref()
                            .is_none_or(|e| matches!(e.kind, Kind::File { .. }))
                    };
                    let left = old.as_ref().map_or(&[][..], |e| &e.data);
                    let right = new.as_ref().map_or(&[][..], |e| &e.data);
                    let texts = std::str::from_utf8(left)
                        .ok()
                        .zip(std::str::from_utf8(right).ok());
                    let counts = if regular(&old)
                        && regular(&new)
                        && !left.contains(&0)
                        && !right.contains(&0)
                    {
                        if let Some((left, right)) = texts {
                            crate::line_diff::bounded(
                                left,
                                right,
                                context.limits.max_diff_cells,
                                |work| context.budget.work(work),
                            )?
                            .map(|spans| {
                                let mut counts = LineCounts {
                                    additions: 0,
                                    deletions: 0,
                                };
                                for span in spans {
                                    if span.kind == DiffHunkKind::Different {
                                        counts.deletions += left[span.before].lines().count();
                                        counts.additions += right[span.after].lines().count();
                                    }
                                }
                                counts
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    match counts {
                        Some(counts) => {
                            if let Some(total) = &mut result.lines {
                                total.additions += counts.additions;
                                total.deletions += counts.deletions;
                            }
                        }
                        None => {
                            result.uncounted += 1;
                            result.lines = None;
                        }
                    }
                }
                context.budget.complete_output(&result)?;
                Ok(result)
            })
        })
        .await
    }
}
