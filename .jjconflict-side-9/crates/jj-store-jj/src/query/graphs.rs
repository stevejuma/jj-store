use super::*;
use futures::TryStreamExt as _;
use jj_lib::{
    backend as b, commit,
    graph::GraphEdgeType,
    object_id::ObjectId as _,
    repo::{ReadonlyRepo, Repo as _},
    revset::{self, RevsetParseContext, RevsetWorkspaceContext},
    ui_path::RepoPathUiConverter,
};

pub(super) fn validate(request: &Request, page: &Page, budget: &Budget) -> Result<()> {
    match (request, page.after) {
        (
            Request::Operations | Request::OperationsFiltered { .. },
            Some(m::ObjectKey::Operation(_)),
        )
        | (_, None) => {}
        (Request::Log { .. } | Request::Evolution { .. }, Some(m::ObjectKey::Commit(_))) => {}
        _ => return Err(Error::Invalid),
    }
    if let Request::Log {
        revset,
        workspace,
        user_email,
        at_time_millis,
    } = request
    {
        if revset.is_empty()
            || revset.len() > budget.limits.max_revset_bytes
            || user_email.len() > 1024
            || workspace
                .as_ref()
                .is_some_and(|w| w.is_empty() || w.len() > 1024 || w.chars().any(char::is_control))
            || chrono::DateTime::from_timestamp_millis(*at_time_millis).is_none()
        {
            return Err(Error::Invalid);
        }
        // Conservative admission bounds precede upstream's recursive parser.
        let mut depth = 0usize;
        let mut syntax = 0usize;
        for ch in revset.chars() {
            if "([{~".contains(ch) {
                depth += 1;
            }
            if ")]}".contains(ch) {
                depth = depth.saturating_sub(1);
            }
            if !ch.is_alphanumeric() && !ch.is_whitespace() {
                syntax += 1;
            }
            if depth > 32 || syntax > 256 {
                return Err(Error::Limit);
            }
        }
        budget.work(revset.len())?;
    }
    Ok(())
}
fn cid(id: &b::CommitId) -> Result<m::CommitId> {
    m::CommitId::from_slice(id.as_bytes()).map_err(|_| Error::Unavailable)
}
fn oid(id: &jj_lib::op_store::OperationId) -> Result<m::OperationId> {
    m::OperationId::from_slice(id.as_bytes()).map_err(|_| Error::Unavailable)
}
fn timestamp(t: &b::Timestamp) -> Timestamp {
    Timestamp {
        millis: t.timestamp.0,
        offset_minutes: t.tz_offset,
    }
}
fn signature(s: &b::Signature) -> Signature {
    Signature {
        name: s.name.clone(),
        email: s.email.clone(),
        timestamp: timestamp(&s.timestamp),
    }
}
async fn describe(
    repo: &ReadonlyRepo,
    commit: &commit::Commit,
    access: &Arc<dyn ReadAccess>,
    budget: &Budget,
) -> Result<Commit> {
    budget.authorize(access.as_ref()).await?;
    let id = cid(commit.id())?;
    if !budget
        .until(access.contains(m::ObjectKey::Commit(id)))
        .await??
    {
        return Err(Error::NotCommitted);
    }
    let change_id =
        m::ChangeId::from_slice(commit.change_id().as_bytes()).map_err(|_| Error::Unavailable)?;
    let (visible, divergent) =
        if let Some(prepared) = repo.readonly_index().downcast_ref::<projection::Reader>() {
            prepared.commit_flags(id, change_id)?
        } else {
            let targets = repo
                .resolve_change_id(commit.change_id())
                .await
                .map_err(|_| Error::Unavailable)?;
            (
                targets.as_ref().is_some_and(|t| t.has_visible(commit.id())),
                targets.as_ref().is_some_and(|t| t.is_divergent()),
            )
        };
    let (bookmarks, workspaces) =
        if let Some(reader) = repo.readonly_index().downcast_ref::<projection::Reader>() {
            reader.annotations(id)?
        } else {
            let mut bookmarks: Vec<_> = repo
                .view()
                .local_bookmarks()
                .filter_map(|(name, target)| {
                    let added = target.added_ids().any(|id| id == commit.id());
                    let removed = target.removed_ids().any(|id| id == commit.id());
                    (added || removed).then(|| Bookmark {
                        name: name.as_str().to_owned(),
                        remote: None,
                        added,
                        removed,
                        conflicted: target.has_conflict(),
                    })
                })
                .collect();
            bookmarks.extend(repo.view().all_remote_bookmarks().filter_map(
                |(symbol, remote_ref)| {
                    let target = &remote_ref.target;
                    let added = target.added_ids().any(|id| id == commit.id());
                    let removed = target.removed_ids().any(|id| id == commit.id());
                    (added || removed).then(|| Bookmark {
                        name: symbol.name.as_str().to_owned(),
                        remote: Some(symbol.remote.as_str().to_owned()),
                        added,
                        removed,
                        conflicted: target.has_conflict(),
                    })
                },
            ));
            let workspaces = repo
                .view()
                .workspaces_for_wc_commit_id(commit.id())
                .into_iter()
                .map(|n| n.as_str().to_owned())
                .collect();
            (bookmarks, workspaces)
        };
    Ok(Commit {
        id,
        change_id,
        parents: commit.parent_ids().iter().map(cid).collect::<Result<_>>()?,
        description: commit.description().to_owned(),
        author: signature(commit.author()),
        committer: signature(commit.committer()),
        tree_terms: commit
            .tree_ids()
            .iter()
            .map(|id| m::TreeId::from_slice(id.as_bytes()).map_err(|_| Error::Unavailable))
            .collect::<Result<_>>()?,
        conflicted: commit.has_conflict(),
        bookmarks,
        workspaces,
        visible,
        divergent,
    })
}
async fn describe_evolution(
    repo: &ReadonlyRepo,
    entry: &jj_lib::evolution::CommitEvolutionEntry,
    access: &Arc<dyn ReadAccess>,
    budget: &Arc<Budget>,
) -> Result<EvolutionNode> {
    Ok(EvolutionNode {
        commit: describe(repo, &entry.commit, access, budget).await?,
        operation: entry
            .operation
            .as_ref()
            .map(|op| oid(op.id()))
            .transpose()?,
        predecessors: entry
            .predecessor_ids()
            .iter()
            .map(cid)
            .collect::<Result<_>>()?,
        predecessors_known: entry.operation.is_some(),
        legacy_predecessors: entry
            .commit
            .store_commit()
            .predecessors
            .iter()
            .map(cid)
            .collect::<Result<_>>()?,
    })
}

struct Position {
    after: Option<m::ObjectKey>,
    found: bool,
    limit: usize,
    emitted: usize,
    last: Option<m::ObjectKey>,
    next: Option<m::ObjectKey>,
}
enum Action {
    Skip,
    Emit,
    Stop,
}
impl Position {
    fn new(page: Page) -> Self {
        Self {
            found: page.after.is_none(),
            after: page.after,
            limit: page.limit,
            emitted: 0,
            last: None,
            next: None,
        }
    }
    fn visit(&mut self, id: m::ObjectKey) -> Action {
        if !self.found {
            self.found = self.after == Some(id);
            return Action::Skip;
        }
        if self.emitted == self.limit {
            self.next = self.last;
            return Action::Stop;
        }
        self.emitted += 1;
        self.last = Some(id);
        Action::Emit
    }
    fn finish(self) -> Result<Option<m::ObjectKey>> {
        if self.found {
            Ok(self.next)
        } else {
            Err(Error::Invalid)
        }
    }
}
pub(super) async fn run(
    repo: &ReadonlyRepo,
    request: Request,
    page: Page,
    access: &Arc<dyn ReadAccess>,
    budget: &Arc<Budget>,
) -> Result<(Nodes, Option<m::ObjectKey>)> {
    let mut position = Position::new(page.clone());
    let nodes = match request {
        Request::Log {
            revset: expression,
            workspace,
            user_email,
            at_time_millis,
        } => {
            let workspace = workspace.map(jj_lib::ref_name::WorkspaceNameBuf::from);
            if workspace
                .as_ref()
                .is_some_and(|w| !repo.view().wc_commit_ids().contains_key(w))
            {
                return Err(Error::Invalid);
            }
            // Lexical path conversion only: no directory exists or is accessed.
            let path_converter = RepoPathUiConverter::Fs {
                cwd: PathBuf::from("/__jj_store_query_root__"),
                base: PathBuf::from("/__jj_store_query_root__"),
            };
            let extensions = revset::RevsetExtensions::default();
            let context = RevsetParseContext {
                aliases_map: &crate::query::language::standard_aliases(),
                local_variables: Default::default(),
                user_email: &user_email,
                date_pattern_context: chrono::DateTime::from_timestamp_millis(at_time_millis)
                    .ok_or(Error::Invalid)?
                    .fixed_offset()
                    .into(),
                default_ignored_remote: None,
                fileset_aliases_map: &jj_lib::fileset::FilesetAliasesMap::default(),
                extensions: &extensions,
                workspace: workspace.as_ref().map(|name| RevsetWorkspaceContext {
                    path_converter: &path_converter,
                    workspace_name: name,
                }),
            };
            crate::query::language::require_context(
                &expression,
                workspace.as_ref().map(|w| w.as_str()),
                &user_email,
            )?;
            let parsed = revset::parse(&mut Default::default(), &expression, &context)
                .map_err(|error| crate::query::language::invalid("syntax", error.to_string()))?;
            super::predicates::preflight(repo, &parsed, access, budget).await?;
            let resolved = parsed
                .resolve_user_expression(
                    repo,
                    &revset::SymbolResolver::new(repo, extensions.symbol_resolvers()),
                )
                .map_err(|_| {
                    crate::query::language::invalid(
                        "unresolved_symbol",
                        "A revision, bookmark or operation could not be resolved in this snapshot.",
                    )
                })?;
            budget.check()?;
            if expression.trim() == "all()"
                && let Some(reader) = repo.readonly_index().downcast_ref::<projection::Reader>()
            {
                budget.authorize(access.as_ref()).await?;
                let after = match page.after {
                    Some(m::ObjectKey::Commit(id)) => Some(id),
                    None => None,
                    _ => return Err(Error::Invalid),
                };
                let (ids, next) = reader.log_window(after, page.limit)?;
                let mut nodes = Vec::with_capacity(ids.len());
                for id in ids {
                    let commit = repo
                        .store()
                        .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
                        .await
                        .map_err(|_| Error::Unavailable)?;
                    let node = CommitNode {
                        commit: describe(repo, &commit, access, budget).await?,
                        edges: commit
                            .parent_ids()
                            .iter()
                            .map(|parent| {
                                Ok(Edge {
                                    target: cid(parent)?,
                                    kind: EdgeKind::Direct,
                                })
                            })
                            .collect::<Result<_>>()?,
                    };
                    budget.output(&node)?;
                    nodes.push(node);
                }
                return Ok((Nodes::Log(nodes), next.map(m::ObjectKey::Commit)));
            }
            let set = resolved.evaluate(repo).map_err(|_| Error::Unavailable)?;
            let mut stream = set.stream_graph();
            let mut nodes = vec![];
            while let Some((id, edges)) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
                budget.authorize(access.as_ref()).await?;
                match position.visit(m::ObjectKey::Commit(cid(&id)?)) {
                    Action::Skip => continue,
                    Action::Stop => break,
                    Action::Emit => {}
                }
                let commit = repo
                    .store()
                    .get_commit_async(&id)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                let node = CommitNode {
                    commit: describe(repo, &commit, access, budget).await?,
                    edges: edges
                        .into_iter()
                        .map(|edge| {
                            Ok(Edge {
                                target: cid(&edge.target)?,
                                kind: match edge.edge_type {
                                    GraphEdgeType::Direct => EdgeKind::Direct,
                                    GraphEdgeType::Indirect => EdgeKind::Indirect,
                                    GraphEdgeType::Missing => EdgeKind::Missing,
                                },
                            })
                        })
                        .collect::<Result<_>>()?,
                };
                budget.output(&node)?;
                nodes.push(node);
            }
            Nodes::Log(nodes)
        }
        Request::Operations | Request::OperationsFiltered { .. } => {
            let lookup = repo
                .readonly_index()
                .downcast_ref::<projection::Reader>()
                .and_then(|reader| reader.prepared.operations.clone());
            return run_operations(repo.operation(), request, page, access, budget, lookup).await;
        }
        Request::Evolution { commit } => {
            if !budget
                .until(access.contains(m::ObjectKey::Commit(commit)))
                .await??
            {
                return Err(Error::NotCommitted);
            }
            if let Some(graph) = repo
                .readonly_index()
                .downcast_ref::<projection::Reader>()
                .and_then(|reader| reader.prepared.operations.as_ref())
                .and_then(|lookup| lookup.history.clone())
            {
                budget.authorize(access.as_ref()).await?;
                let after = match page.after {
                    Some(m::ObjectKey::Commit(id)) => Some(b::CommitId::from_bytes(id.as_bytes())),
                    None => None,
                    _ => return Err(Error::Invalid),
                };
                let (nodes, next) = crate::query::prepared::operations::history::evolution::read(
                    repo,
                    graph,
                    b::CommitId::from_bytes(commit.as_bytes()),
                    after,
                    page.limit,
                    |amount| budget.work(amount),
                    |_| Error::Unavailable,
                    async |entry| {
                        let node = describe_evolution(repo, &entry, access, budget).await?;
                        budget.output(&node)?;
                        Ok(node)
                    },
                )
                .await?;
                return Ok((
                    Nodes::Evolution(nodes),
                    next.as_ref()
                        .map(|id| cid(id).map(m::ObjectKey::Commit))
                        .transpose()?,
                ));
            }
            let starts = [b::CommitId::from_bytes(commit.as_bytes())];
            let stream = jj_lib::evolution::walk_predecessors(repo, &starts);
            let mut stream = std::pin::pin!(stream);
            let mut nodes = vec![];
            while let Some(entry) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
                budget.authorize(access.as_ref()).await?;
                match position.visit(m::ObjectKey::Commit(cid(entry.commit.id())?)) {
                    Action::Skip => continue,
                    Action::Stop => break,
                    Action::Emit => {}
                }
                let node = describe_evolution(repo, &entry, access, budget).await?;
                budget.output(&node)?;
                nodes.push(node);
            }
            Nodes::Evolution(nodes)
        }
    };
    Ok((nodes, position.finish()?))
}

pub(super) async fn run_operations(
    head: &jj_lib::operation::Operation,
    request: Request,
    page: Page,
    access: &Arc<dyn ReadAccess>,
    budget: &Arc<Budget>,
    lookup: Option<Arc<prepared::operations::Lookup>>,
) -> Result<(Nodes, Option<m::ObjectKey>)> {
    let mut position = Position::new(page.clone());
    let filter = crate::query::language::operations::Filter::parse(match &request {
        Request::OperationsFiltered { query } => query,
        _ => "",
    })?;
    let heads = [if let Some(lookup) = lookup.clone() {
        operation_neighbors::start(head, &filter, lookup, access.clone(), budget.clone()).await?
    } else {
        filter.start_at(head, || budget.work(1)).await?
    }];
    if let Some(graph) = lookup.as_ref().and_then(|lookup| lookup.history.clone()) {
        let after = match page.after {
            Some(m::ObjectKey::Operation(id)) => Some(id),
            None => None,
            _ => return Err(Error::Invalid),
        };
        let (nodes, next) = crate::query::prepared::operations::history::read(
            &heads[0],
            graph,
            &filter,
            after,
            page.limit,
            |amount| budget.work(amount),
            |operation| {
                let node = operation_node(operation)?;
                budget.output(&node)?;
                Ok(node)
            },
        )
        .await?;
        return Ok((Nodes::Operations(nodes), next.map(m::ObjectKey::Operation)));
    }
    let stream = jj_lib::op_walk::walk_ancestors(&heads);
    let mut stream = std::pin::pin!(stream);
    let mut nodes = vec![];
    while let Some(operation) = stream
        .try_next()
        .await
        .map_err(crate::query::language::operations::store_error)?
    {
        budget.authorize(access.as_ref()).await?;
        budget.work(1)?;
        if !filter.matches(&operation) {
            continue;
        }
        let id = oid(operation.id())?;
        match position.visit(m::ObjectKey::Operation(id)) {
            Action::Skip => continue,
            Action::Stop => break,
            Action::Emit => {}
        }
        let node = operation_node(&operation)?;
        budget.output(&node)?;
        nodes.push(node);
    }
    Ok((Nodes::Operations(nodes), position.finish()?))
}

fn operation_node(operation: &jj_lib::operation::Operation) -> Result<OperationNode> {
    let id = oid(operation.id())?;
    let data = operation.store_operation();
    Ok(OperationNode {
        id,
        parents: operation
            .parent_ids()
            .iter()
            .map(oid)
            .collect::<Result<_>>()?,
        view: m::ViewId::from_slice(data.view_id.as_bytes()).map_err(|_| Error::Unavailable)?,
        description: data.metadata.description.clone(),
        user: data.metadata.username.clone(),
        host: data.metadata.hostname.clone(),
        start: timestamp(&data.metadata.time.start),
        end: timestamp(&data.metadata.time.end),
        is_snapshot: data.metadata.is_snapshot,
        workspace: data
            .metadata
            .workspace_name
            .as_ref()
            .map(|name| name.as_str().to_owned()),
        attributes: data.metadata.attributes.clone(),
        predecessor_data_present: data.commit_predecessors.is_some(),
    })
}
