//! Merge a frozen remote delta without rolling back later observations.
use super::*;

pub(super) async fn apply(
    ui: &Ui,
    workspace: &mut WorkspaceCommandHelper,
    context: &BridgeContext,
    intent: &GitPushIntentV1,
    remote_name: &str,
    predecessors: BTreeSet<m::RequestId>,
) -> std::result::Result<(), CommandError> {
    let observation = stage_id(intent.id, "observation");
    let id = stage_id(intent.id, "applied");
    let state = context.client.git_bridge_state().await.map_err(error)?;
    let catalog = state
        .git
        .as_ref()
        .ok_or_else(|| user_error("Git root unavailable during apply"))?;
    let source = workspace
        .repo()
        .op_store()
        .read_operation(&jj_lib::op_store::OperationId::from_bytes(
            intent.operation.as_bytes(),
        ))
        .await?;
    let frozen = jj_lib::view::View::new(
        workspace
            .repo()
            .op_store()
            .read_view(&source.view_id)
            .await?,
        false,
    );
    let aliases: BTreeSet<_> = catalog
        .values
        .values()
        .filter_map(|value| match value {
            GitRecordV1::Remote(value) if value.remote == intent.remote => {
                Some(value.name.as_str())
            }
            _ => None,
        })
        .collect();
    let mut tx = workspace.start_transaction();
    for (name, value) in &intent.refs {
        let name = RefName::new(name.strip_prefix("refs/heads/").unwrap());
        let target = value
            .desired
            .native
            .map(|v| RefTarget::normal(jj_lib::backend::CommitId::from_bytes(v.as_bytes())))
            .unwrap_or_else(RefTarget::absent);
        let base = if let Some(expected) = &value.expected {
            let candidates: BTreeSet<_> = aliases.iter().filter_map(|alias| {
                let old = frozen.get_remote_bookmark(name.to_remote_symbol(RemoteName::new(alias))).target.as_normal()?;
                catalog.values.values().any(|record| matches!(record, GitRecordV1::Representation(v) if v.native_commit.as_bytes() == old.as_bytes() && &v.git_commit == expected)).then(|| old.clone())
            }).collect();
            if candidates.len() != 1 {
                return Err(user_error(
                    "Frozen native Git remote base is missing or ambiguous; apply pending",
                ));
            }
            RefTarget::normal(candidates.into_iter().next().unwrap())
        } else {
            RefTarget::absent()
        };
        let symbol = name.to_remote_symbol(RemoteName::new(remote_name));
        let current = tx.repo().view().get_remote_bookmark(symbol).target.clone();
        let merged =
            jj_lib::refs::merge_ref_targets(tx.repo().index(), &current, &base, &target).await?;
        tx.repo_mut().set_remote_bookmark(
            symbol,
            RemoteRef {
                target: merged,
                state: RemoteRefState::Tracked,
            },
        );
    }
    accept(
        tx,
        ui,
        context,
        vec![
            GitRecordV1::Observation(GitObservationV1 {
                id: observation,
                remote: intent.remote,
                binding: intent.binding,
                operation: intent.operation,
                predecessors: intent.observations.clone(),
                complete_heads: false,
                refs: intent
                    .refs
                    .iter()
                    .map(|(name, value)| (name.clone(), value.desired.clone()))
                    .collect(),
            }),
            GitRecordV1::PushEvent(GitPushEventV1 {
                id,
                attempt: intent.id,
                predecessors,
                kind: GitPushEventKindV1::Applied { observation },
            }),
        ],
        id,
        format!("apply native Git push {} to {remote_name}", intent.id),
    )
    .await
}
