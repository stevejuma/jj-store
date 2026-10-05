use super::*;
impl Context {
    pub(super) async fn dispatch(
        self,
        operation: m::OperationId,
        request: Request,
        operation_only: bool,
    ) -> Result<Response> {
        let context = self;
        let value = match request {
            Request::CodeSummary { limit } => {
                Response::CodeSummary(code_summary::read(context, operation, limit).await?)
            }
            Request::QueryInfo => {
                let mut info = crate::query::language::info(true);
                let git = context
                    .scoped
                    .as_ref()
                    .map_or(&context.store, |guard| &guard.source)
                    .backend()
                    .downcast_ref::<jj_lib::git_backend::GitBackend>()
                    .ok_or(Error::Unavailable)?
                    .git_repo();
                info.default_user_email = crate::query::language::identity::local_email(
                    context.access.binding.metadata_path(),
                    &git,
                );
                Response::QueryInfo(info)
            }
            Request::Graph(request, page) => {
                graphs::validate(
                    &request,
                    &page,
                    context.access.commits.len(),
                    &context.budget,
                )?;
                let (data, next) = if operation_only {
                    let selected = operation_store::selected(&context, operation).await?;
                    graphs::run_operations(
                        &selected,
                        request,
                        page,
                        &context.access,
                        &context.budget,
                    )
                    .await?
                } else {
                    graphs::run(
                        context.repo.as_deref().ok_or(Error::Unavailable)?,
                        request,
                        page,
                        &context.access,
                        &context.budget,
                    )
                    .await?
                };
                Response::Graph(GraphPage {
                    version: 1,
                    operation,
                    data,
                    next,
                    stats: context.budget.stats(),
                })
            }
            Request::StackHistory(request) => {
                Response::StackHistory(stacks::read(context, operation, request).await?)
            }
            Request::Commit { commit } => {
                let source = context.commit_record(commit).await?;
                let native = context.commit(&context.store, commit).await?;
                let card = graphs::describe(
                    context.repo.as_deref().ok_or(Error::Unavailable)?,
                    &native,
                    &context.access,
                    &context.budget,
                )
                .await?;
                let object = context.object(ObjectKey::Commit(commit), ObjectData::Commit(source));
                Response::Commit(Box::new(CommitDetail {
                    object,
                    commit: card,
                }))
            }
            Request::CommitSummary { commit } => {
                let data = context.commit_record(commit).await?;
                Response::CommitSummary(
                    context.object(ObjectKey::Commit(commit), ObjectData::Commit(data)),
                )
            }
            Request::FileAt { commit, path, file } => {
                let path = native::path(&path)?;
                let commit = context.commit(&context.store, commit).await?;
                let value = commit
                    .tree()
                    .path_value(&path)
                    .await
                    .map_err(native::map_backend)?;
                if !value.iter().flatten().any(|term| matches!(term, b::TreeValue::File { id, .. } if id.as_bytes() == file.as_bytes())) {
                            return Err(Error::NotCommitted);
                        }
                Response::FileAt(context.file_content(file).await?)
            }
            Request::File { file } => Response::File(context.file_content(file).await?),
            Request::Paths(request) => Response::Paths(paths::read(context, request).await?),
            Request::References(request) => {
                Response::References(references::read(context, request).await?)
            }
            Request::Operation { operation } => {
                let data = context.operation(operation).await?;
                let view = context.operation_view(operation, &data).await?;
                let result = OperationDetail {
                    view: context.object(ObjectKey::View(data.view_id), ObjectData::View(view)),
                    operation: context
                        .object(ObjectKey::Operation(operation), ObjectData::Operation(data)),
                };
                Response::Operation(Box::new(result))
            }
            Request::OperationDiff(request) => {
                Response::OperationDiff(operations::read(context, request).await?)
            }
            Request::OperationChanges(request) => {
                Response::OperationChanges(operation_changes::read(context, request).await?)
            }
            Request::Diff(request) => Response::Diff(diff::read(context, request).await?),
            Request::DiffPaths(request) => {
                Response::DiffPaths(diff_summary::read(context, request).await?)
            }
            Request::Conflicts(request) => {
                Response::Conflicts(conflicts::read(context, request).await?)
            }
        };
        Ok(value)
    }
}
