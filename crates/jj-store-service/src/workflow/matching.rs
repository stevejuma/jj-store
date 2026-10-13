//! Reuse the native graph executor for source author, description and revset matching.
use super::*;
use crate::{
    native::NativeSession,
    reads::{OpenRead, ReadLimits},
};
use jj_store_jj::query::{self as q, GraphExecutor};
use std::sync::Arc;
pub(super) async fn code_filters(
    session: &Arc<NativeSession>,
    graph: &GraphExecutor,
    event: &mut SourceEvent,
    filters: &[Conditions],
) -> Result<()> {
    let operation = event.source.operation.ok_or(Error::Corrupt)?;
    let commit = event.source.commit.ok_or(Error::Corrupt)?;
    let OpenRead::Pinned(read) = session
        .open_read(Some(operation), ReadLimits::default())
        .await
        .map_err(|_| Error::Unavailable)?
    else {
        return Err(Error::Corrupt);
    };
    let read = Arc::new(read);
    let request = |revset: String, at_time_millis| q::Request::Log {
        revset,
        workspace: None,
        user_email: String::new(),
        at_time_millis,
    };
    let page = read
        .graph(
            graph,
            request(commit.to_string(), 0),
            q::Page {
                limit: 1,
                after: None,
            },
        )
        .await
        .map_err(|_| Error::Unavailable)?;
    let q::Nodes::Log(nodes) = page.data else {
        return Err(Error::Corrupt);
    };
    let node = nodes.first().ok_or(Error::Corrupt)?;
    for filter in filters {
        let mut matched = filter.matches_content(
            &node.commit.author.email,
            &node.commit.description,
            None,
            None,
            None,
        );
        if matched && let Some(revset) = &filter.revset {
            let page = read
                .graph(
                    graph,
                    request(
                        format!("({revset}) & {commit}"),
                        node.commit.committer.timestamp.millis,
                    ),
                    q::Page {
                        limit: 1,
                        after: None,
                    },
                )
                .await
                .map_err(|_| Error::Invalid)?;
            let q::Nodes::Log(nodes) = page.data else {
                return Err(Error::Corrupt);
            };
            matched = !nodes.is_empty();
        }
        event
            .filter_results
            .insert(filter.id().map_err(|_| Error::Invalid)?, matched);
    }
    Ok(())
}
