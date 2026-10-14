use super::*;
pub(super) struct Selection<'a> {
    pub range: dto::ComparisonRange,
    pub base: Option<&'a str>,
}
impl Supervisor {
    pub(super) async fn workspace_comparison(
        &self,
        saved: &SavedSession,
        repository: RepositoryId,
        selected: &Workspace,
        facts: &dto::WorkspaceRevision,
        source: &source::Source,
        selection: Selection<'_>,
    ) -> Result<dto::Comparison> {
        let Selection { range, base } = selection;
        let mut result = dto::Comparison {
            range,
            before_operation: Some(selected.operation),
            before_commit: None,
            after_operation: selected.operation,
            after_commit: selected.commit.clone(),
            current_files: true,
            request_id: None,
            unavailable: None,
        };
        match range {
            dto::ComparisonRange::SinceLastCommit => {
                if facts.parents.len() == 1 {
                    result.before_commit = facts.parents.first().cloned();
                } else {
                    result.unavailable =
                        Some("This workspace has no single parent revision.".into());
                }
            }
            dto::ComparisonRange::Branch => {
                let target = base.or(facts.default_base.as_deref());
                if let Some(target) = target {
                    // Explicit selections must be an advertised reference or the configured trunk.
                    if !facts.references.iter().any(|r| r.commit == target)
                        && facts.default_base.as_deref() != Some(target)
                    {
                        return Err(Error::Stale);
                    }
                    match source.common_ancestor(&selected.commit, target).await {
                        Ok(base) => result.before_commit = Some(base),
                        Err(_) => {
                            result.unavailable =
                                Some("A unique comparison ancestor is unavailable.".into())
                        }
                    }
                } else {
                    result.unavailable =
                        Some("Choose a comparison base. No default branch is configured.".into());
                }
            }
            dto::ComparisonRange::LastTurn => {
                result.current_files = false;
                let pair = completed_turn(&saved.events, repository);
                if let Some((start, end, request)) = pair {
                    result.before_operation = Some(
                        serde_json::from_value(start.payload["operation"].clone())
                            .map_err(|_| Error::Unavailable)?,
                    );
                    result.before_commit = start.payload["commit"].as_str().map(str::to_owned);
                    result.after_operation =
                        serde_json::from_value(end.payload["operation"].clone())
                            .map_err(|_| Error::Unavailable)?;
                    result.after_commit = end.payload["commit"]
                        .as_str()
                        .ok_or(Error::Unavailable)?
                        .into();
                    result.request_id = Some(request.into());
                } else {
                    result.unavailable =
                        Some("This turn has no verified start and completion snapshots.".into());
                }
            }
        }
        Ok(result)
    }
}

fn completed_turn(events: &[Event], repository: RepositoryId) -> Option<(&Event, &Event, &str)> {
    let record = events
        .iter()
        .rev()
        .find(|e| e.payload["type"] == "record" && e.payload["record"]["status"] == "completed")?;
    let request = record.payload["requestId"].as_str()?;
    let find = |kind: &str| {
        events.iter().find(|e| {
            e.payload["type"] == kind
                && e.payload["requestId"] == request
                && e.payload["repository"] == repository.to_string()
                && e.payload["state"] == "completed"
        })
    };
    Some((
        find("repository_turn_start")?,
        find("repository_snapshot")?,
        request,
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn event(payload: Value) -> Event {
        Event {
            sequence: 1,
            observed_at_millis: None,
            payload,
            committed: true,
        }
    }
    #[test]
    fn latest_completed_turn_requires_its_own_exact_repository_boundaries() {
        let repo = RepositoryId::new([1; 16]);
        let mut events = vec![
            event(
                json!({"type":"repository_turn_start","repository":repo,"requestId":"one","state":"completed"}),
            ),
            event(json!({"type":"record","requestId":"one","record":{"status":"completed"}})),
            event(
                json!({"type":"repository_snapshot","repository":repo,"requestId":"one","state":"completed"}),
            ),
        ];
        assert_eq!(completed_turn(&events, repo).unwrap().2, "one");
        assert!(completed_turn(&events, RepositoryId::new([2; 16])).is_none());
        events.push(event(
            json!({"type":"record","requestId":"two","record":{"status":"cancelled"}}),
        ));
        assert_eq!(completed_turn(&events, repo).unwrap().2, "one");
        events.push(event(
            json!({"type":"record","requestId":"three","record":{"status":"completed"}}),
        ));
        assert!(completed_turn(&events, repo).is_none());
    }
}
