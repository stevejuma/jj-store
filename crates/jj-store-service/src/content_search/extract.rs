use super::*;
use crate::auth::Permission;

pub(super) async fn capture(
    session: &NativeSession,
    query: &ContentQuery,
    expires: u64,
    cache: &materialization::Cache,
) -> Result<Arc<cursors::Pin>> {
    let viewer = session.principal(Permission::Read).await?.id();
    let pin = session.snapshot().await?;
    let manifest = pin.manifest();
    let snapshot = ContentSnapshot {
        repo_id: session.repository,
        epoch: manifest.epoch,
        manifest_digest: pin.digest(),
        published_seq: manifest.published,
        thread_root: manifest.thread_root.map(|r| r.digest),
        work_root: manifest.work_root.map(|r| r.digest),
    };
    let mut sources = Vec::new();
    if matches!(query.section, ContentSection::All | ContentSection::Threads)
        && query.project.is_none()
        && query.task.is_none()
        && let Some(root) = manifest.thread_root
    {
        sources.push(cache.get(session, manifest.epoch, root).await?);
    }
    if matches!(
        query.section,
        ContentSection::All | ContentSection::Projects | ContentSection::Tasks
    ) && query.thread.is_none()
        && let Some(root) = manifest.work_root
    {
        sources.push(cache.get(session, manifest.epoch, root).await?);
    }
    let filter = query.q.trim().to_lowercase();
    let selection = index::Selection::new(sources, query, &filter)?;
    if selection.objects > session.limits.inventory_nodes
        || selection.keys.len() > session.limits.inventory_nodes
    {
        return Err(Error::Limit);
    }
    Ok(Arc::new(cursors::Pin {
        expires,
        snapshot,
        viewer,
        selection,
    }))
}
pub(super) fn excerpt(body: &str, filter: &str) -> String {
    let folded = if filter.is_empty() {
        String::new()
    } else {
        body.to_lowercase()
    };
    excerpt_folded(body, &folded, filter)
}
pub(super) fn excerpt_folded(body: &str, folded: &str, filter: &str) -> String {
    let offset = if filter.is_empty() {
        0
    } else {
        folded
            .find(filter)
            .map(|byte| {
                let mut used = 0;
                body.chars()
                    .enumerate()
                    .find_map(|(index, ch)| {
                        used += ch.to_lowercase().map(char::len_utf8).sum::<usize>();
                        (used > byte).then_some(index)
                    })
                    .unwrap_or(0)
            })
            .unwrap_or(0)
            .saturating_sub(60)
    };
    let mut chars = body.chars().skip(offset);
    let mut text: String = chars.by_ref().take(240).collect();
    if offset > 0 {
        text.insert(0, '…');
    }
    if chars.next().is_some() {
        text.push('…');
    }
    text
}

#[cfg(test)]
mod tests {
    #[test]
    fn excerpt_keeps_unicode_expansion_match_in_original_text() {
        let body = format!("{}needle{}", "İ".repeat(400), "x".repeat(400));
        let excerpt = super::excerpt(&body, "needle");
        assert!(excerpt.contains("needle"));
        assert!(excerpt.starts_with('…') && excerpt.ends_with('…'));
        assert!(excerpt.chars().count() <= 242);
    }
}
