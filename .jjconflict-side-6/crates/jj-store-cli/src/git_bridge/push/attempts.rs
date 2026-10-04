//! Durable stages and explicit status/recovery.
use super::*;
pub fn stage_id(attempt: m::RequestId, stage: &str) -> m::RequestId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/git-push-stage/v1\0");
    hash.update(attempt.as_bytes());
    hash.update(stage.as_bytes());
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash.finalize()[..16]);
    m::RequestId::new(bytes)
}
pub(super) async fn record(
    context: &BridgeContext,
    id: m::RequestId,
    records: Vec<GitRecordV1>,
) -> std::result::Result<(), CommandError> {
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: id,
        records,
        transition: None,
    };
    match context
        .client
        .accept_git_bridge(&request)
        .await
        .map_err(error)?
    {
        jj_store_service::git_bridge::Outcome::Committed { .. } => Ok(()),
        jj_store_service::git_bridge::Outcome::OutcomeUnknown { request_id } => Err(user_error(
            format!("Git native outcome unknown; request {request_id}"),
        )),
    }
}
#[derive(Clone, Debug, clap::Args)]
pub struct GitArgs {
    #[command(subcommand)]
    pub command: GitCommand,
}
#[derive(Clone, Debug, clap::Subcommand)]
pub enum GitCommand {
    /// Read retained attempts without reconciling code or contacting Git
    Status { attempt: Option<m::RequestId> },
    /// Inspect the approved destination and repair bookkeeping without pushing
    Recover { attempt: m::RequestId },
}
pub async fn run_store(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    args: GitArgs,
) -> std::result::Result<(), CommandError> {
    let format = helper
        .matches()
        .subcommand_matches("store")
        .and_then(|v| v.get_one::<crate::commands::Format>("format"))
        .copied()
        .unwrap_or_default();
    let loaded = helper.load_workspace()?;
    if loaded.repo_loader().store().backend().name() != "jj-store" {
        return Err(user_error(
            "Git attempt commands require a native repository",
        ));
    }
    let context = registry.context(loaded.repo_path()).map_err(error)?;
    let state = context.client.git_bridge_state().await.map_err(error)?;
    let git = state
        .git
        .as_ref()
        .ok_or_else(|| user_error("No Git attempts"))?;
    match args.command {
        GitCommand::Status { attempt } => {
            if attempt.is_some_and(|v| git.push_intent(v).is_none()) {
                return Err(user_error("Git attempt not found"));
            }
            let intents: Vec<_> = git
                .values
                .values()
                .filter_map(|v| match v {
                    GitRecordV1::PushIntent(v) if attempt.is_none_or(|id| id == v.id) => Some(v),
                    _ => None,
                })
                .collect();
            if intents.len() > 1024 {
                return Err(user_error(
                    "Git status list exceeds limit; select an attempt ID",
                ));
            }
            let text = intents
                .iter()
                .map(|intent| {
                    format!(
                        "{} {} {}\n",
                        intent.id,
                        git.push_status(intent.id),
                        intent.refs.keys().cloned().collect::<Vec<_>>().join(", ")
                    )
                })
                .collect::<String>();
            let attempts: Vec<_> = intents.iter().map(|intent| serde_json::json!({"intent":intent,"status":git.push_status(intent.id),"events":git.push_events(intent.id)})).collect();
            crate::presentation::result(
                ui,
                helper,
                format,
                &serde_json::json!({"version":1,"repository":context.client.repository(),"attempts":attempts}),
                &text,
            )
        }
        GitCommand::Recover { attempt } => {
            mutating(helper)?;
            context.client.authorize(true).await.map_err(error)?;
            let intent = git
                .push_intent(attempt)
                .ok_or_else(|| user_error("Git attempt not found"))?;
            if intent.executor != state.principal {
                return Err(user_error("Git attempt belongs to another executor"));
            }
            if git.push_status(attempt) == "applied" {
                return crate::presentation::result(
                    ui,
                    helper,
                    format,
                    &serde_json::json!({"version":1,"attempt":attempt,"native_apply":"applied","status":"already_applied"}),
                    &format!("Attempt {attempt} already applied.\n"),
                );
            }
            let remotes = git.remotes().map_err(error)?;
            let remote_name = remotes
                .iter()
                .find(|(_, id)| **id == intent.remote)
                .map(|(name, _)| name)
                .ok_or_else(|| user_error("Attempt remote removed or conflicted"))?;
            let device = device::Device::open(context.client.repository(), loaded.workspace_root())
                .map_err(error)?;
            let endpoint = device.endpoint(intent.remote).map_err(error)?;
            if endpoint.fingerprint != intent.binding {
                return Err(user_error(
                    "Attempt destination approval changed; recovery refused",
                ));
            }
            let cache = device.cache().map_err(error)?;
            let actual = transport::inspect(&cache.bare, &endpoint.url, intent.refs.keys())
                .await
                .map_err(error)?;
            if intent
                .refs
                .iter()
                .any(|(name, value)| actual.get(name) != value.desired.oid.as_ref())
            {
                return Err(user_error(format!(
                    "Attempt {attempt} remains unknown; current refs differ from desired. Fetch and review before replanning"
                )));
            }
            let current = git.push_events(attempt);
            let mut predecessor: BTreeSet<_> = current.iter().map(|v| v.id).collect();
            if git.push_status(attempt) != "apply_pending" {
                let result = device::id().map_err(error)?;
                record(
                    &context,
                    result,
                    vec![GitRecordV1::PushEvent(GitPushEventV1 {
                        id: result,
                        attempt,
                        predecessors: predecessor,
                        kind: GitPushEventKindV1::ResultRecorded {
                            refs: intent
                                .refs
                                .keys()
                                .map(|name| (name.clone(), GitPushRefOutcomeV1::ObservedSatisfied))
                                .collect(),
                        },
                    })],
                )
                .await?;
                predecessor = BTreeSet::from([result]);
            }
            let mut workspace = helper.workspace_helper(ui).await?;
            apply(
                ui,
                &mut workspace,
                &context,
                intent,
                remote_name,
                predecessor,
            )
            .await?;
            crate::presentation::result(
                ui,
                helper,
                format,
                &serde_json::json!({"version":1,"attempt":attempt,"native_apply":"applied","status":"observed_satisfied","authorship":"unproven_by_observation"}),
                &format!(
                    "Attempt {attempt}: observed_satisfied; native bookkeeping applied. This observation does not prove attempt authorship.\n"
                ),
            )
        }
    }
}
