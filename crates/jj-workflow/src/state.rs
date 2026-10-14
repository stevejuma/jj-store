use crate::*;
use jj_store_model as m;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub workflows: BTreeMap<WorkflowId, VersionId>,
    pub definitions: BTreeMap<VersionId, DefinitionVersion>,
    pub evaluations: BTreeMap<SourceEventId, Evaluation>,
    pub runs: BTreeMap<RunId, Run>,
    pub evidence: BTreeMap<EvidenceId, Evidence>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "WorkflowCommand")]
pub enum Command {
    Put {
        workflow: WorkflowId,
        expected: Option<VersionId>,
        definition: Definition,
    },
    Remove {
        workflow: WorkflowId,
        expected: VersionId,
    },
    Evaluate {
        event: SourceEvent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        definitions: Option<BTreeMap<WorkflowId, VersionId>>,
    },
    Run {
        workflow: WorkflowId,
        expected: VersionId,
        source: Source,
    },
    Claim {
        run: RunId,
        attempt: ExecutionAttemptId,
        host: String,
    },
    Report {
        run: RunId,
        attempt: ExecutionAttemptId,
        sequence: u64,
        data: EvidenceData,
    },
    Finish {
        run: RunId,
        attempt: ExecutionAttemptId,
        status: RunStatus,
        result: String,
    },
    Heartbeat {
        run: RunId,
        attempt: ExecutionAttemptId,
    },
    Interrupt {
        run: RunId,
        attempt: ExecutionAttemptId,
    },
    Cancel {
        run: RunId,
    },
    Retry {
        run: RunId,
        expected: ExecutionAttemptId,
    },
}
impl Command {
    pub fn execution_only(&self) -> bool {
        matches!(
            self,
            Self::Evaluate { .. }
                | Self::Claim { .. }
                | Self::Report { .. }
                | Self::Finish { .. }
                | Self::Heartbeat { .. }
                | Self::Interrupt { .. }
        )
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Put { definition, .. } => definition.validate(),
            Self::Evaluate { event, definitions } => {
                if definitions.as_ref().is_some_and(|v| v.len() > MAX_ENTITIES) {
                    return Err(Error::Limit);
                }
                event.validate()
            }
            Self::Run { source, .. } => source.validate(),
            Self::Claim { host, .. } => name(host),
            Self::Report { data, .. } => data.validate(),
            Self::Finish { status, result, .. } => {
                if !status.terminal() {
                    return Err(Error::Invalid("terminal status"));
                }
                text(result, 4096)
            }
            _ => Ok(()),
        }
    }
}
/// Commands return only changed records; WAL catalogs retain immutable versions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowCommandResult")]
pub struct CommandResult {
    pub workflow: Option<WorkflowId>,
    pub definition: Option<DefinitionVersion>,
    pub evaluation: Option<Evaluation>,
    pub runs: Vec<Run>,
    pub evidence: Option<Evidence>,
    pub removed: bool,
}
impl State {
    pub fn validate(&self) -> Result<()> {
        if self.workflows.len()
            + self.definitions.len()
            + self.evaluations.len()
            + self.runs.len()
            + self.evidence.len()
            > MAX_ENTITIES
        {
            return Err(Error::Limit);
        }
        for (id, definition) in &self.definitions {
            if *id != definition.version {
                return Err(Error::Invalid("definition key"));
            }
            definition.definition.validate()?;
        }
        for (id, version) in &self.workflows {
            if self.definitions.get(version).ok_or(Error::NotFound)?.id != *id {
                return Err(Error::Invalid("workflow selection"));
            }
        }
        for (id, evaluation) in &self.evaluations {
            evaluation.event.validate()?;
            if *id != evaluation.event.id {
                return Err(Error::Invalid("evaluation key"));
            }
            for (workflow, version) in &evaluation.definitions {
                if self.definitions.get(version).ok_or(Error::NotFound)?.id != *workflow {
                    return Err(Error::Invalid("evaluation definition"));
                }
            }
            for run in &evaluation.runs {
                if self.runs.get(run).ok_or(Error::NotFound)?.event != Some(*id) {
                    return Err(Error::Invalid("evaluation run"));
                }
            }
        }
        for (id, run) in &self.runs {
            run.source.validate()?;
            let def = self.definitions.get(&run.version).ok_or(Error::NotFound)?;
            if *id != run.id
                || def.id != run.workflow
                || def.definition.repository != run.source.repository
                || run.attempts.is_empty()
                || run.attempts.len() > 64
            {
                return Err(Error::Invalid("run"));
            }
            for attempt in &run.attempts {
                if (attempt.owner.is_some() != attempt.host.is_some())
                    || (attempt.status == RunStatus::Running && attempt.owner.is_none())
                {
                    return Err(Error::Invalid("attempt owner"));
                }
            }
        }
        let mut sequences = std::collections::BTreeSet::new();
        for (id, evidence) in &self.evidence {
            evidence.data.validate()?;
            let run = self.runs.get(&evidence.run).ok_or(Error::NotFound)?;
            let attempt = run
                .attempts
                .iter()
                .find(|a| a.id == evidence.attempt)
                .ok_or(Error::NotFound)?;
            if *id != evidence.id
                || evidence.sequence >= attempt.sequence
                || !sequences.insert((evidence.run, evidence.attempt, evidence.sequence))
            {
                return Err(Error::Invalid("evidence sequence"));
            }
        }
        Ok(())
    }
    pub fn apply(
        &self,
        command: &Command,
        actor: m::PrincipalId,
        request: m::RequestId,
        at: i64,
    ) -> Result<(Self, CommandResult)> {
        command.validate()?;
        let mut next = self.clone();
        let mut result = CommandResult::default();
        match command {
            Command::Put {
                workflow,
                expected,
                definition,
            } => {
                if self.workflows.get(workflow).copied() != *expected {
                    return Err(Error::Stale);
                }
                if self
                    .definitions
                    .values()
                    .any(|d| d.id == *workflow && d.owner != actor)
                {
                    return Err(Error::Denied);
                }
                if let Some(old) = expected
                    && self.definitions.get(old).ok_or(Error::NotFound)?.owner != actor
                {
                    return Err(Error::Denied);
                }
                let version = DefinitionVersion {
                    id: *workflow,
                    version: VersionId::new(identity(
                        b"definition",
                        &[workflow.as_bytes(), request.as_bytes(), actor.as_bytes()],
                    )),
                    owner: actor,
                    created_at: at,
                    definition: definition.clone(),
                };
                next.workflows.insert(*workflow, version.version);
                next.definitions.insert(version.version, version.clone());
                result.workflow = Some(*workflow);
                result.definition = Some(version);
            }
            Command::Remove { workflow, expected } => {
                if self.workflows.get(workflow) != Some(expected) {
                    return Err(Error::Stale);
                }
                if self.definitions.get(expected).ok_or(Error::NotFound)?.owner != actor {
                    return Err(Error::Denied);
                }
                next.workflows.remove(workflow);
                result.workflow = Some(*workflow);
                result.removed = true;
            }
            Command::Evaluate { event, definitions } => {
                if let Some(evaluation) = self.evaluations.get(&event.id) {
                    if evaluation.event.publication != event.publication
                        || evaluation.event.kind != event.kind
                        || evaluation.event.source != event.source
                        || evaluation
                            .event
                            .code_context
                            .as_ref()
                            .is_some_and(|old| event.code_context.as_ref() != Some(old))
                    {
                        return Err(Error::Invalid("event identity reuse"));
                    }
                    result.evaluation = Some(evaluation.clone());
                    result.runs = evaluation
                        .runs
                        .iter()
                        .filter_map(|id| self.runs.get(id).cloned())
                        .collect();
                    return Ok((next, result));
                }
                let mut evaluation = Evaluation {
                    event: event.clone(),
                    evaluated_at: at,
                    definitions: definitions
                        .clone()
                        .unwrap_or_else(|| self.workflows.clone()),
                    runs: Vec::new(),
                };
                for (workflow, version) in &evaluation.definitions {
                    let definition = self.definitions.get(version).ok_or(Error::NotFound)?;
                    if definition.id != *workflow {
                        return Err(Error::Invalid("definition catalog identity"));
                    }
                    if !definition.definition.matches(event)? {
                        continue;
                    }
                    let run = make_run(
                        RunId::new(identity(
                            b"event-run",
                            &[workflow.as_bytes(), event.id.as_bytes()],
                        )),
                        definition,
                        event.source.clone(),
                        Some(event.id),
                        actor,
                        at,
                    );
                    evaluation.runs.push(run.id);
                    next.runs.insert(run.id, run.clone());
                    result.runs.push(run);
                }
                next.evaluations.insert(event.id, evaluation.clone());
                result.evaluation = Some(evaluation);
            }
            Command::Run {
                workflow,
                expected,
                source,
            } => {
                if self.workflows.get(workflow) != Some(expected) {
                    return Err(Error::Stale);
                }
                let definition = self.definitions.get(expected).ok_or(Error::NotFound)?;
                if definition.definition.repository != source.repository {
                    return Err(Error::Invalid("source repository"));
                }
                let run = make_run(
                    RunId::new(identity(
                        b"manual-run",
                        &[actor.as_bytes(), request.as_bytes()],
                    )),
                    definition,
                    source.clone(),
                    None,
                    actor,
                    at,
                );
                next.runs.insert(run.id, run.clone());
                result.runs.push(run);
            }
            Command::Cancel { run } => {
                let value = next.runs.get_mut(run).ok_or(Error::NotFound)?;
                let attempt = value.attempts.last_mut().ok_or(Error::NotFound)?;
                if attempt.status.terminal() {
                    return Err(Error::Stale);
                }
                if attempt.status == RunStatus::Pending {
                    attempt.status = RunStatus::Cancelled;
                    attempt.completed_at = Some(at);
                } else {
                    attempt.status = RunStatus::CancelRequested;
                }
                attempt.updated_at = at;
                result.runs.push(value.clone());
            }
            Command::Retry { run, expected } => {
                let value = next.runs.get_mut(run).ok_or(Error::NotFound)?;
                if value.current().id != *expected || !value.current().status.terminal() {
                    return Err(Error::Stale);
                }
                value.attempts.push(new_attempt(
                    ExecutionAttemptId::new(identity(
                        b"retry",
                        &[run.as_bytes(), request.as_bytes()],
                    )),
                    at,
                ));
                result.runs.push(value.clone());
            }
            _ => {
                let (run_id, attempt_id) = match command {
                    Command::Claim { run, attempt, .. }
                    | Command::Report { run, attempt, .. }
                    | Command::Finish { run, attempt, .. }
                    | Command::Heartbeat { run, attempt }
                    | Command::Interrupt { run, attempt } => (*run, *attempt),
                    _ => return Err(Error::Invalid("command")),
                };
                let value = next.runs.get_mut(&run_id).ok_or(Error::NotFound)?;
                let attempt = value.attempts.last_mut().ok_or(Error::NotFound)?;
                if attempt.id != attempt_id {
                    return Err(Error::Stale);
                }
                if let Command::Claim { host, .. } = command {
                    if attempt.status != RunStatus::Pending || attempt.owner.is_some() {
                        return Err(Error::Stale);
                    }
                    attempt.owner = Some(actor);
                    attempt.host = Some(host.clone());
                    attempt.status = RunStatus::Running;
                    attempt.started_at = Some(at);
                } else {
                    if attempt.owner != Some(actor) {
                        return Err(Error::Denied);
                    }
                    if attempt.status.terminal() {
                        return Err(Error::Stale);
                    }
                    match command {
                        Command::Report { sequence, data, .. } => {
                            if *sequence != attempt.sequence {
                                return Err(Error::Stale);
                            }
                            let evidence = Evidence {
                                id: EvidenceId::new(identity(
                                    b"evidence",
                                    &[
                                        run_id.as_bytes(),
                                        attempt_id.as_bytes(),
                                        &sequence.to_be_bytes(),
                                    ],
                                )),
                                run: run_id,
                                attempt: attempt_id,
                                sequence: *sequence,
                                at,
                                data: data.clone(),
                            };
                            attempt.sequence =
                                attempt.sequence.checked_add(1).ok_or(Error::Limit)?;
                            next.evidence.insert(evidence.id, evidence.clone());
                            result.evidence = Some(evidence);
                        }
                        Command::Finish {
                            status,
                            result: message,
                            ..
                        } => {
                            if attempt.status == RunStatus::CancelRequested
                                && *status != RunStatus::Cancelled
                            {
                                return Err(Error::Stale);
                            }
                            attempt.status = status.clone();
                            attempt.result = message.clone();
                            attempt.completed_at = Some(at);
                        }
                        Command::Interrupt { .. } => attempt.status = RunStatus::Unknown,
                        Command::Heartbeat { .. } => {}
                        _ => return Err(Error::Invalid("attempt command")),
                    }
                }
                attempt.updated_at = at;
                result.runs.push(value.clone());
            }
        }
        next.validate()?;
        Ok((next, result))
    }
}
fn new_attempt(id: ExecutionAttemptId, at: i64) -> Attempt {
    Attempt {
        id,
        owner: None,
        host: None,
        status: RunStatus::Pending,
        sequence: 0,
        started_at: None,
        updated_at: at,
        completed_at: None,
        result: String::new(),
    }
}
fn make_run(
    id: RunId,
    definition: &DefinitionVersion,
    source: Source,
    event: Option<SourceEventId>,
    actor: m::PrincipalId,
    at: i64,
) -> Run {
    Run {
        id,
        workflow: definition.id,
        version: definition.version,
        event,
        source,
        requester: actor,
        created_at: at,
        attempts: vec![new_attempt(
            ExecutionAttemptId::new(identity(b"attempt", &[id.as_bytes()])),
            at,
        )],
    }
}
