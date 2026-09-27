use super::*;
use crate::mutations::{
    apply as mutation_apply, preview as mutation_preview, status as mutation_status,
};
use crate::{
    contract::{Document, Reply},
    dto::*,
};
use axum::{
    Json,
    extract::{DefaultBodyLimit, Query, rejection::QueryRejection},
    routing::{get, post},
};
use jj_store_service::mutations as mutation;
use jj_store_service::{pagination::directory, reads::query::content as c};
pub(super) type Input<T> = std::result::Result<Query<T>, QueryRejection>;
pub(super) fn input<T>(value: Input<T>) -> Result<T> {
    value.map(|Query(v)| v).map_err(|_| Error::invalid())
}
fn limit() -> usize {
    50
}
fn all() -> String {
    "all()".into()
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Empty {}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Selection {
    at_op: Option<m::OperationId>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Log {
    at_op: Option<m::OperationId>,
    cursor: Option<String>,
    #[serde(default = "limit")]
    limit: usize,
    #[serde(default = "all")]
    revset: String,
    workspace: Option<String>,
    #[serde(default)]
    user_email: String,
    at_time_millis: i64,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Operations {
    at_op: Option<m::OperationId>,
    heads: Option<m::OperationId>,
    cursor: Option<String>,
    #[serde(default = "limit")]
    limit: usize,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Evolution {
    at_op: Option<m::OperationId>,
    commit: m::CommitId,
    cursor: Option<String>,
    #[serde(default = "limit")]
    limit: usize,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Directory {
    at_op: Option<m::OperationId>,
    cursor: Option<String>,
    #[serde(default)]
    path: String,
    #[serde(default = "limit")]
    limit: usize,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Difference {
    at_op: Option<m::OperationId>,
    from: m::CommitId,
    to: m::CommitId,
    #[serde(default)]
    path: String,
    #[serde(default)]
    content: bool,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Conflicts {
    at_op: Option<m::OperationId>,
    #[serde(default)]
    path: String,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct OperationDiff {
    parent: Option<m::OperationId>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Prepare {
    operation: m::OperationId,
}

// One declaration registers both executable routes and their generated contracts.
macro_rules! endpoints {
    ($register:ident) => {
        $register! {
            (get, "/repos/{repo}/capabilities", capabilities, Empty, Capabilities, Reply::Json(200)),
            (get, "/repos/{repo}/snapshot", snapshot, Empty, reads::HeadSnapshot, Reply::Json(200)),
            (get, "/repos/{repo}/events", events, Events, events::Event, Reply::Events),
            (get, "/repos/{repo}/objects/{kind}/{id}", object, Selection, Envelope<c::Object>, Reply::Json(200)),
            (get, "/repos/{repo}/log", log, Log, Envelope<q::GraphPage>, Reply::Json(200)),
            (get, "/repos/{repo}/operations", operations, Operations, Envelope<q::GraphPage>, Reply::Json(200)),
            (get, "/repos/{repo}/operations/{operation}", operation, Empty, Envelope<OperationDetail>, Reply::Json(200)),
            (get, "/repos/{repo}/operations/{operation}/diff", operation_diff, OperationDiff, Envelope<c::operations::Response>, Reply::Json(200)),
            (get, "/repos/{repo}/evolution", evolution, Evolution, Envelope<q::GraphPage>, Reply::Json(200)),
            (get, "/repos/{repo}/commits/{commit}", commit, Selection, Envelope<CommitDetail>, Reply::Json(200)),
            (get, "/repos/{repo}/commits/{commit}/tree", tree, Directory, Envelope<c::TreePage>, Reply::Json(200)),
            (get, "/repos/{repo}/commits/{commit}/conflicts", conflicts, Conflicts, Envelope<c::conflicts::Response>, Reply::Json(200)),
            (get, "/repos/{repo}/files/{file}", file, Selection, String, Reply::Binary),
            (get, "/repos/{repo}/diff", diff, Difference, Envelope<c::diff::Response>, Reply::Json(200)),
            (post, "/repos/{repo}/index/tasks", prepare, Empty, index_tasks::Task, Reply::Json(202)),
            (get, "/repos/{repo}/index/tasks/{task}", task, Empty, index_tasks::Task, Reply::Json(200)),
            (post, "/repos/{repo}/index/tasks/{task}/cancel", cancel, Empty, index_tasks::Task, Reply::Json(200)),
            (post, "/repos/{repo}/mutations/preview", mutation_preview, Empty, mutation::Preview, Reply::Json(200)),
            (post, "/repos/{repo}/mutations", mutation_apply, Empty, mutation::Outcome, Reply::Json(200)),
            (get, "/repos/{repo}/mutations/{request_id}", mutation_status, Empty, mutation::Status, Reply::Json(200))
        }
    };
}
pub(super) fn document(doc: &mut Document) {
    macro_rules! register {
        ($(($method:ident, $path:literal, $handler:ident, $query:ty, $response:ty, $reply:expr)),* ) => {
            $(doc.endpoint::<$query, $response>(stringify!($method), $path, stringify!($handler), $reply);)*
        };
    }
    endpoints!(register);
    doc.body::<Prepare>("/v1/repos/{repo}/index/tasks", "post");
    doc.body::<mutation::Request>("/v1/repos/{repo}/mutations/preview", "post");
    doc.body::<mutation::Request>("/v1/repos/{repo}/mutations", "post");
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident, $path:literal, $handler:ident, $query:ty, $response:ty, $reply:expr)),* ) => {
            Router::new()$(.route($path, $method($handler).layer(DefaultBodyLimit::max(
                if $path.contains("/mutations") { crate::mutations::MAX_REQUEST_BYTES } else { 4096 }
            ))))*
        };
    }
    endpoints!(register)
        .fallback(|| async { Error::new(StatusCode::NOT_FOUND, "unknown_route") })
        .method_not_allowed_fallback(|| async {
            Error::new(StatusCode::METHOD_NOT_ALLOWED, "unsupported_method")
        })
}
async fn capabilities(
    ExtractState(state): ExtractState<Arc<State>>,
    _: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    state.json(&Capabilities::from(&state.config))
}
async fn snapshot(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let snapshot = context
        .session
        .snapshot()
        .await
        .map_err(reads::Error::from)?;
    state.json(&reads::HeadSnapshot::from(snapshot.manifest()))
}
async fn object(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Selection>,
) -> Result<Response> {
    let args = input(args)?;
    let key = match context.value("kind")? {
        "commit" => m::ObjectKey::Commit(context.id("id")?),
        "tree" => m::ObjectKey::Tree(context.id("id")?),
        "view" => m::ObjectKey::View(context.id("id")?),
        "operation" => m::ObjectKey::Operation(context.id("id")?),
        "copy" => m::ObjectKey::Copy(context.id("id")?),
        "symlink" => m::ObjectKey::Symlink(context.id("id")?),
        "file" => m::ObjectKey::File(context.id("id")?),
        _ => return Err(Error::invalid()),
    };
    let read = pinned(&context, &state, args.at_op).await?;
    let data = read.object(state.queries.content(), key).await?;
    envelope(&state, &read, data).await
}
async fn graph(
    state: &State,
    context: Context,
    at_op: Option<m::OperationId>,
    request: q::Request,
    limit: usize,
    cursor: Option<String>,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state
            .graphs
            .resume(context.session, &cursor, at_op, request, limit)
            .await?
    } else {
        match state
            .graphs
            .begin(context.session, at_op, request, limit)
            .await?
        {
            pagination::Start::SelectHead { snapshot } => return Err(Error::heads(snapshot)),
            pagination::Start::Page(page) => *page,
        }
    };
    state.json(&Envelope {
        snapshot: page.snapshot,
        data: page.graph,
        next_cursor: page.next_cursor,
    })
}
async fn log(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Log>,
) -> Result<Response> {
    let args = input(args)?;
    graph(
        &state,
        context,
        args.at_op,
        q::Request::Log {
            revset: args.revset,
            workspace: args.workspace,
            user_email: args.user_email,
            at_time_millis: args.at_time_millis,
        },
        args.limit,
        args.cursor,
    )
    .await
}
async fn operations(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Operations>,
) -> Result<Response> {
    let args = input(args)?;
    if args.at_op.zip(args.heads).is_some_and(|(a, b)| a != b) {
        return Err(Error::invalid());
    }
    graph(
        &state,
        context,
        args.at_op.or(args.heads),
        q::Request::Operations,
        args.limit,
        args.cursor,
    )
    .await
}
async fn evolution(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Evolution>,
) -> Result<Response> {
    let args = input(args)?;
    graph(
        &state,
        context,
        args.at_op,
        q::Request::Evolution {
            commit: args.commit,
        },
        args.limit,
        args.cursor,
    )
    .await
}
async fn operation(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let id = context.id("operation")?;
    let read = pinned(&context, &state, Some(id)).await?;
    let operation = read
        .object(state.queries.content(), m::ObjectKey::Operation(id))
        .await?;
    let c::ObjectData::Operation(data) = &operation.data else {
        return Err(Error::unavailable());
    };
    let view = read
        .object(state.queries.content(), m::ObjectKey::View(data.view_id))
        .await?;
    envelope(&state, &read, OperationDetail { operation, view }).await
}
async fn commit(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Selection>,
) -> Result<Response> {
    let args = input(args)?;
    let id = context.id("commit")?;
    let read = pinned(&context, &state, args.at_op).await?;
    let object = read
        .object(state.queries.content(), m::ObjectKey::Commit(id))
        .await?;
    let graph = match read
        .graph(
            state.queries.graph(),
            q::Request::Evolution { commit: id },
            q::Page {
                limit: 1,
                after: None,
            },
        )
        .await
    {
        Err(q::Error::IndexRebuildRequired) => {
            return Err(Error::rebuild(read.snapshot().await?.clone()));
        }
        result => result?,
    };
    let q::Nodes::Evolution(nodes) = graph.data else {
        return Err(Error::unavailable());
    };
    let commit = nodes
        .into_iter()
        .next()
        .ok_or_else(Error::unavailable)?
        .commit;
    envelope(&state, &read, CommitDetail { object, commit }).await
}
async fn tree(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Directory>,
) -> Result<Response> {
    let args = input(args)?;
    let request = directory::Request {
        commit: context.id("commit")?,
        path: args.path,
        limit: args.limit,
    };
    let page = if let Some(cursor) = args.cursor {
        state
            .directories
            .resume(context.session, &cursor, args.at_op, request)
            .await?
    } else {
        match state
            .directories
            .begin(context.session, args.at_op, request)
            .await?
        {
            directory::Start::SelectHead { snapshot } => return Err(Error::heads(snapshot)),
            directory::Start::Page(page) => *page,
        }
    };
    state.json(&Envelope {
        snapshot: page.snapshot,
        data: page.tree,
        next_cursor: page.next_cursor,
    })
}
async fn operation_diff(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<OperationDiff>,
) -> Result<Response> {
    let args = input(args)?;
    let operation = context.id("operation")?;
    let read = pinned(&context, &state, Some(operation)).await?;
    let data = read
        .operation_diff(
            state.queries.content(),
            c::operations::Request {
                operation,
                parent: args.parent,
            },
        )
        .await?;
    envelope(&state, &read, data).await
}
async fn diff(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Difference>,
) -> Result<Response> {
    let args = input(args)?;
    let read = pinned(&context, &state, args.at_op).await?;
    let data = read
        .diff(
            state.queries.content(),
            c::diff::Request {
                from: args.from,
                to: args.to,
                path: args.path,
                content: args.content,
            },
        )
        .await?;
    envelope(&state, &read, data).await
}
async fn conflicts(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Conflicts>,
) -> Result<Response> {
    let args = input(args)?;
    let read = pinned(&context, &state, args.at_op).await?;
    let data = read
        .conflicts(
            state.queries.content(),
            c::conflicts::Request {
                commit: context.id("commit")?,
                path: args.path,
            },
        )
        .await?;
    envelope(&state, &read, data).await
}
fn range(headers: &HeaderMap, length: u64) -> Result<Option<reads::ByteRange>> {
    let Some(header) = headers.get(header::RANGE) else {
        return Ok(None);
    };
    let invalid = || Error::range(length);
    if length == 0 || headers.get_all(header::RANGE).iter().count() != 1 {
        return Err(invalid());
    }
    let value = header
        .to_str()
        .map_err(|_| invalid())?
        .trim()
        .strip_prefix("bytes=")
        .ok_or_else(invalid)?;
    let (a, b) = value.split_once('-').ok_or_else(invalid)?;
    let number = |s: &str| -> Result<u64> {
        if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
            return Err(invalid());
        }
        s.parse().map_err(|_| invalid())
    };
    let (start, end) = if a.is_empty() {
        let count = number(b)?;
        if count == 0 {
            return Err(invalid());
        }
        (length.saturating_sub(count), length)
    } else {
        let start = number(a)?;
        let end = if b.is_empty() {
            length
        } else {
            number(b)?.min(length - 1) + 1
        };
        (start, end)
    };
    if start >= length {
        return Err(invalid());
    }
    Ok(Some(
        reads::ByteRange::new(start, end).map_err(|_| invalid())?,
    ))
}
async fn file(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Selection>,
) -> Result<Response> {
    let args = input(args)?;
    let id: m::FileId = context.id("file")?;
    let read = pinned(&context, &state, args.at_op).await?;
    let mut file = read.file(state.queries.content(), id, None).await?;
    let full = file.info().await?.full_length;
    let selection = range(&context.headers, full)?;
    if let Some(selection) = selection {
        file = file.select_range(selection).await?;
    }
    let info = file.info().await?;
    let snapshot = read.snapshot().await?.clone();
    let stream = futures::stream::try_unfold(file, |mut file| async move {
        file.next_chunk()
            .await
            .map(|next| next.map(|bytes| (bytes, file)))
            .map_err(std::io::Error::other)
    });
    let mut response = Body::from_stream(stream).into_response();
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{id}.bin\"")
            .parse()
            .unwrap(),
    );
    headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
    headers.insert(
        header::CONTENT_LENGTH,
        (info.end - info.start).to_string().parse().unwrap(),
    );
    for (name, value) in [
        (
            "x-jj-store-repository",
            snapshot.authority.repo_id.to_string(),
        ),
        ("x-jj-store-epoch", snapshot.authority.epoch.to_string()),
        ("x-jj-store-operation", snapshot.operation_id.to_string()),
        (
            "x-jj-store-position",
            snapshot.authority.published_seq.0.to_string(),
        ),
        (
            "x-jj-store-revision",
            snapshot.authority.manifest_revision.to_string(),
        ),
    ] {
        headers.insert(name, value.parse().unwrap());
    }
    if selection.is_some() {
        headers.insert(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", info.start, info.end - 1, full)
                .parse()
                .unwrap(),
        );
        *response.status_mut() = StatusCode::PARTIAL_CONTENT;
    }
    Ok(response)
}
async fn prepare(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
    body: std::result::Result<Json<Prepare>, axum::extract::rejection::JsonRejection>,
) -> Result<Response> {
    input(args)?;
    let Json(request) = body.map_err(|_| Error::invalid())?;
    let task = state
        .tasks
        .start(context.session, request.operation)
        .await?;
    let mut response = state.json(&task)?;
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(response)
}
async fn task(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let task = state
        .tasks
        .status(context.session.clone(), context.value("task")?)
        .await?;
    state.json(&task)
}
async fn cancel(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let task = state
        .tasks
        .cancel(context.session.clone(), context.value("task")?)
        .await?;
    state.json(&task)
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Events {
    after: Option<String>,
}
async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Events>,
) -> Result<Response> {
    use axum::response::sse::{Event as SseEvent, Sse};
    let args = input(args)?;
    if context.headers.get_all("last-event-id").iter().count() > 1 {
        return Err(Error::invalid());
    }
    let header = context
        .headers
        .get("last-event-id")
        .map(|value| value.to_str().map_err(|_| Error::invalid()))
        .transpose()?;
    if args
        .after
        .as_deref()
        .zip(header)
        .is_some_and(|(a, b)| a != b)
    {
        return Err(Error::invalid());
    }
    let after = args
        .after
        .as_deref()
        .or(header)
        .map(str::parse::<events::Cursor>)
        .transpose()?;
    let feed = events::Subscription::open(context.session, after, state.config.events).await?;
    let stream = futures::stream::try_unfold(feed, |mut feed| async move {
        let Some(event) = feed.next().await.map_err(std::io::Error::other)? else {
            return Ok(None);
        };
        let frame = if matches!(event.kind, events::EventKind::Heartbeat) {
            SseEvent::default().comment("keep-alive")
        } else {
            let name = match event.kind {
                events::EventKind::Ready { .. } => "ready",
                events::EventKind::Changed => "repository-changed",
                events::EventKind::Reset { .. } => "reset",
                events::EventKind::Heartbeat => unreachable!(),
            };
            let bytes = encode(&event, 64 * 1024)
                .map_err(|_| std::io::Error::other("event response limit"))?;
            let data =
                String::from_utf8(bytes).map_err(|_| std::io::Error::other("event encoding"))?;
            SseEvent::default()
                .event(name)
                .id(event.cursor.to_string())
                .data(data)
        };
        Ok::<_, std::io::Error>(Some((frame, feed)))
    });
    let mut response = Sse::new(stream).into_response();
    response
        .headers_mut()
        .insert("x-accel-buffering", "no".parse().unwrap());
    Ok(response)
}
