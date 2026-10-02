//! Server-free dispatch; native selectors and stored local intents remain disjoint.
use super::*;
use jj_store_service::local_repository::{LocalRepository, LocalReviewCommand, LocalReviewRequest};

pub(super) async fn run(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    command: ReviewCommand,
) -> Result<(), CommandError> {
    let path = helper.workspace_loader()?.repo_path();
    let store =
        jj_store_jj::local_repository::metadata_store(helper.settings(), path).map_err(error)?;
    let repository = LocalRepository::new(path, store);
    match command {
        ReviewCommand::List {
            limit,
            cursor,
            after,
            at_metadata,
        } => {
            if cursor.is_some() {
                return Err(error(
                    "native --cursor is invalid for local reviews; use --after and --at-metadata",
                ));
            }
            let page = repository
                .review_threads_at(at_metadata, limit, after)
                .await
                .map_err(error)?;
            let next = page
                .next_after
                .zip(page.snapshot.metadata_commit)
                .map(|(after, pin)| format!("--after {after} --at-metadata {pin}"));
            review_presentation::page_rows(
                ui,
                helper,
                format,
                &page,
                &page.threads,
                next.as_deref(),
            )
        }
        ReviewCommand::Show {
            thread_id,
            at_metadata,
        } => {
            let detail = repository
                .review_thread_at(at_metadata, thread_id)
                .await
                .map_err(error)?;
            review_presentation::detail_rows(
                ui,
                helper,
                format,
                &detail,
                &detail.thread,
                &detail.comments,
            )
        }
        ReviewCommand::Status { request_id } => review_presentation::result(
            ui,
            helper,
            format,
            &repository
                .review_command_status(request_id)
                .map_err(error)?,
        ),
        command => {
            let request = match command {
                ReviewCommand::Repository { request_id, body } => LocalReviewRequest {
                    version: 1,
                    request_id,
                    command: LocalReviewCommand::RepositoryThread { body: body.value() },
                },
                command => {
                    let id = repository.repository().map_err(error)?.ok_or_else(|| {
                        error("initialize local reviews with jj review repository first")
                    })?;
                    let request = super::request(id, command).map_err(error)?;
                    LocalReviewRequest {
                        version: request.version,
                        request_id: request.request_id,
                        command: LocalReviewCommand::Apply {
                            intent: Box::new(request.command),
                        },
                    }
                }
            };
            review_presentation::result(
                ui,
                helper,
                format,
                &repository
                    .apply_review_command(&request)
                    .await
                    .map_err(error)?,
            )
        }
    }
}
