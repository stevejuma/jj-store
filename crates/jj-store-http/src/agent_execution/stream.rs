use super::*;
use axum::response::sse::{Event, KeepAlive, Sse};
pub(super) async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<EventsQuery>,
) -> Result<Response> {
    let query = routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    let manager = manager.clone();
    // Validate repository authority before streaming and on every bounded read.
    manager
        .events(actor, query.session.clone(), query.after)
        .await
        .map_err(failure)?;
    let updates = manager.subscribe();
    let deadline = tokio::time::Instant::now() + state.config.event_lifetime;
    let stream = futures::stream::try_unfold(
        (manager, updates, query, false),
        move |(manager, mut updates, mut query, delivered)| async move {
            if tokio::time::Instant::now() >= deadline {
                return Ok::<_, std::io::Error>(None);
            }
            if delivered {
                tokio::select! { _ = updates.recv() => {}, _ = tokio::time::sleep(Duration::from_secs(2)) => {} }
            }
            let page = manager
                .events(actor, query.session.clone(), query.after)
                .await
                .map_err(|_| std::io::Error::other("agent activity unavailable"))?;
            if let Some(event) = page.events.last() {
                query.after = event.sequence;
            }
            let more = page.next.is_some();
            let frame = Event::default()
                .event("agent-activity")
                .id(query.after.to_string())
                .json_data(page)
                .map_err(std::io::Error::other)?;
            Ok(Some((frame, (manager, updates, query, !more))))
        },
    );
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response())
}
