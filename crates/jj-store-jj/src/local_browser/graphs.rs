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
use std::path::PathBuf;

pub(super) fn validate(
    request: &GraphRequest,
    page: &GraphWindow,
    _commits: usize,
    budget: &Budget,
) -> Result<()> {
    if page.limit == 0 || page.limit > budget.limits.max_page {
        return Err(Error::Limit);
    }
    match (request, page.after) {
        (
            GraphRequest::Operations | GraphRequest::OperationsFiltered { .. },
            Some(ObjectKey::Operation(_)),
        )
        | (_, None) => {}
        (GraphRequest::Log { .. } | GraphRequest::Evolution { .. }, Some(ObjectKey::Commit(_))) => {
        }
        _ => return Err(Error::Invalid),
    }
    if let GraphRequest::Log {
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
fn cid(id: &b::CommitId) -> Result<m::GitSha1Id> {
    m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Unavailable)
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
pub(super) async fn describe(
    repo: &ReadonlyRepo,
    commit: &commit::Commit,
    access: &Arc<Pin>,
    budget: &Budget,
) -> Result<Commit> {
    budget.authorize(access.as_ref()).await?;
    let id = cid(commit.id())?;
    if !access.contains_commit(id).await? {
        return Err(Error::NotCommitted);
    }
    let change_id =
        m::ChangeId::from_slice(commit.change_id().as_bytes()).map_err(|_| Error::Unavailable)?;
    let (visible, divergent) = if let Some(prepared) = access.prepared.as_ref() {
        prepared.commit_flags(id, change_id, budget)?
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
        if let Some(prepared) = &access.prepared {
            prepared.annotations(id, budget)?
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
            .map(|id| m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Unavailable))
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
    access: &Arc<Pin>,
    budget: &Budget,
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
    after: Option<ObjectKey>,
    found: bool,
    limit: usize,
    emitted: usize,
    last: Option<ObjectKey>,
    next: Option<ObjectKey>,
}
enum Action {
    Skip,
    Emit,
    Stop,
}
impl Position {
    fn new(page: GraphWindow) -> Self {
        Self {
            found: page.after.is_none(),
            after: page.after,
            limit: page.limit,
            emitted: 0,
            last: None,
            next: None,
        }
    }
    fn visit(&mut self, id: ObjectKey) -> Action {
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
    fn finish(self) -> Result<Option<ObjectKey>> {
        if self.found {
            Ok(self.next)
        } else {
            Err(Error::Invalid)
        }
    }
}
pub(super) async fn run(
    repo: &ReadonlyRepo,
    request: GraphRequest,
    page: GraphWindow,
    access: &Arc<Pin>,
    budget: &Budget,
) -> Result<(Nodes, Option<ObjectKey>)> {
    let mut position = Position::new(page.clone());
    let nodes = match request {
        GraphRequest::Log {
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
            super::native::admit_revset(&parsed, budget)?;
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
                && let Some(prepared) = &access.prepared
            {
                budget.authorize(access.as_ref()).await?;
                let after = match page.after {
                    Some(ObjectKey::Commit(id)) => Some(id),
                    None => None,
                    _ => return Err(Error::Invalid),
                };
                let (ids, next) = prepared.log_window(after, page.limit, budget)?;
                let mut nodes = Vec::with_capacity(ids.len());
                for id in ids {
                    let commit = repo
                        .store()
                        .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
                        .await
                        .map_err(native::map_backend)?;
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
                return Ok((Nodes::Log(nodes), next.map(ObjectKey::Commit)));
            }
            let set = resolved.evaluate(repo).map_err(|_| Error::Unavailable)?;
            let mut stream = set.stream_graph();
            let mut nodes = vec![];
            while let Some((id, edges)) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
                budget.authorize(access.as_ref()).await?;
                match position.visit(ObjectKey::Commit(cid(&id)?)) {
                    Action::Skip => continue,
                    Action::Stop => break,
                    Action::Emit => {}
                }
                let commit = repo
                    .store()
                    .get_commit_async(&id)
                    .await
                    .map_err(native::map_backend)?;
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
        GraphRequest::Operations | GraphRequest::OperationsFiltered { .. } => {
            return run_operations(repo.operation(), request, page, access, budget).await;
        }
        GraphRequest::Evolution { commit } => {
            if !access.contains_commit(commit).await? {
                return Err(Error::NotCommitted);
            }
            if let Some(graph) = access
                .prepared
                .as_ref()
                .and_then(|prepared| prepared.operation_history())
            {
                budget.authorize(access.as_ref()).await?;
                let after = match page.after {
                    Some(ObjectKey::Commit(id)) => Some(b::CommitId::from_bytes(id.as_bytes())),
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
                    native::map_backend,
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
                        .map(|id| cid(id).map(ObjectKey::Commit))
                        .transpose()?,
                ));
            }
            let starts = [b::CommitId::from_bytes(commit.as_bytes())];
            let stream = jj_lib::evolution::walk_predecessors(repo, &starts);
            let mut stream = std::pin::pin!(stream);
            let mut nodes = vec![];
            while let Some(entry) = stream.try_next().await.map_err(|error| match error {
                jj_lib::evolution::WalkPredecessorsError::Backend(error) => {
                    native::map_backend(error)
                }
                jj_lib::evolution::WalkPredecessorsError::OpStore(error) => {
                    crate::query::language::operations::store_error(error)
                }
                jj_lib::evolution::WalkPredecessorsError::CycleDetected(_) => Error::Corrupt,
            })? {
                budget.authorize(access.as_ref()).await?;
                match position.visit(ObjectKey::Commit(cid(entry.commit.id())?)) {
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
    request: GraphRequest,
    page: GraphWindow,
    access: &Arc<Pin>,
    budget: &Budget,
) -> Result<(Nodes, Option<ObjectKey>)> {
    let mut position = Position::new(page.clone());
    let filter = crate::query::language::operations::Filter::parse(match &request {
        GraphRequest::OperationsFiltered { query } => query,
        _ => "",
    })?;
    let heads = [if access.prepared.is_some() {
        filter
            .start_at_indexed(
                head,
                async |parent, prior| {
                    use jj_lib::object_id::PrefixResolution;
                    budget.authorize(access).await?;
                    let prepared = access.prepared.as_ref().ok_or(Error::Unavailable)?;
                    let id = match prepared.child_operation(oid(parent.id())?, budget)? {
                        PrefixResolution::SingleMatch(id) => id,
                        _ => {
                            return Err(crate::query::language::invalid(
                                "unresolved_symbol",
                                "Operation selector is missing or ambiguous in this history.",
                            ));
                        }
                    };
                    if let Some(prior) = prior.filter(|prior| prior.id() == &id) {
                        budget.work(prior.parent_ids().len())?;
                        if !prior.parent_ids().contains(parent.id()) {
                            return Err(Error::Corrupt);
                        }
                        return Ok(prior.clone());
                    }
                    let data = head
                        .op_store()
                        .read_operation(&id)
                        .await
                        .map_err(crate::query::language::operations::store_error)?;
                    budget.work(data.parents.len())?;
                    if !data.parents.contains(parent.id()) {
                        return Err(Error::Corrupt);
                    }
                    Ok(jj_lib::operation::Operation::new(
                        head.op_store().clone(),
                        id,
                        data,
                    ))
                },
                |amount| budget.work(amount),
            )
            .await?
    } else {
        filter.start_at(head, || budget.work(1)).await?
    }];
    if let Some(graph) = access
        .prepared
        .as_ref()
        .and_then(|prepared| prepared.operation_history())
    {
        let after = match page.after {
            Some(ObjectKey::Operation(id)) => Some(id),
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
        return Ok((Nodes::Operations(nodes), next.map(ObjectKey::Operation)));
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
        match position.visit(ObjectKey::Operation(id)) {
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
