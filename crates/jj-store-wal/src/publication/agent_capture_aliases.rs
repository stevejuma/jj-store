//! Resolve exact source aliases and qualified common native harness aliases.
use super::{agent::agent_error, *};
use jj_agent::{
    AgentAliasV1, AgentCatalogKey, AgentCatalogKind, AgentEntityRefV1, AgentSessionId,
    IntegrationId, NativeEntityKind, NormalizationCandidates, NormalizationDiagnosticKind,
    NormalizationDiagnosticV1, NormalizedActivityRecordV1,
};
mod catalog;
mod remap;
use remap::{Identities, session_id};

pub(super) async fn adopt_selected_aliases(
    base: &dyn AgentSnapshotRead,
    candidates: &mut NormalizationCandidates,
) -> Result<Vec<AgentAliasV1>> {
    let mut aliases = candidates.activity.aliases.clone();
    for result in &candidates.tool_results {
        for alias in [&result.session_alias, &result.tool_alias] {
            if !aliases.contains(alias) {
                aliases.push(alias.clone());
            }
        }
    }
    let sessions: Vec<_> = aliases
        .iter()
        .filter(|a| a.native_kind == NativeEntityKind::Session)
        .collect();
    let selected = if sessions.iter().any(|a| common_session(a)) {
        catalog::selected_aliases(base).await?
    } else {
        Some(Vec::new())
    };
    let mut ids = Identities::default();
    let mut proofs = Vec::new();
    for alias in &sessions {
        let original = session_id(&alias.target).ok_or(Error::Corrupt("capture session alias"))?;
        if common_session(alias) && selected.is_none() {
            block(&mut ids, candidates, original, "capture-alias-scan-limit");
            continue;
        }
        let targets = targets(base, alias, common_session(alias), selected.as_deref()).await?;
        if targets.len() > 1 {
            block(
                &mut ids,
                candidates,
                original,
                "capture-native-alias-conflict",
            );
            continue;
        }
        if let Some(target) = targets.first() {
            if !selected_target_available(base, target, alias.integration).await? {
                block(
                    &mut ids,
                    candidates,
                    original,
                    "capture-alias-target-unavailable",
                );
                continue;
            }
            ids.adopt(&alias.target, target)?;
            let mut proof = (*alias).clone();
            proof.target = target.clone();
            proofs.push(proof);
        }
    }
    for alias in &aliases {
        if alias.native_kind == NativeEntityKind::Session {
            continue;
        }
        let Some(original) = session_id(&alias.target) else {
            continue;
        };
        if ids.blocked.contains(&original) {
            continue;
        }
        let common = sessions.iter().any(|session| {
            session_id(&session.target) == Some(original)
                && common_session(session)
                && alias.integration == session.integration
                && alias
                    .native_id
                    .strip_prefix(&session.native_id)
                    .is_some_and(|tail| tail.starts_with('/') && tail.len() > 1)
        });
        let targets = targets(base, alias, common, selected.as_deref()).await?;
        let expected_session = ids.sessions.get(&original).copied().unwrap_or(original);
        if targets.len() > 1
            || targets
                .first()
                .is_some_and(|t| session_id(t) != Some(expected_session))
        {
            block(
                &mut ids,
                candidates,
                original,
                "capture-native-alias-conflict",
            );
            continue;
        }
        if let Some(target) = targets.first() {
            if !selected_target_available(base, target, alias.integration).await? {
                block(
                    &mut ids,
                    candidates,
                    original,
                    "capture-alias-target-unavailable",
                );
                continue;
            }
            ids.adopt(&alias.target, target)?;
            let mut proof = alias.clone();
            proof.target = target.clone();
            proofs.push(proof);
        }
    }
    proofs.retain(|a| {
        !aliases.iter().any(|original| {
            original.catalog_key().ok() == a.catalog_key().ok()
                && session_id(&original.target).is_some_and(|s| ids.blocked.contains(&s))
        })
    });
    ids.apply(candidates);
    Ok(proofs)
}

async fn targets(
    base: &dyn AgentSnapshotRead,
    alias: &AgentAliasV1,
    common: bool,
    selected: Option<&[AgentAliasV1]>,
) -> Result<Vec<AgentEntityRefV1>> {
    let exact;
    let values = if common {
        selected.ok_or(Error::Corrupt("capture alias proof unavailable"))?
    } else {
        exact = catalog::exact_aliases(base, alias).await?;
        &exact
    };
    let mut targets = Vec::new();
    for value in values {
        if value.integration == alias.integration
            && value.native_kind == alias.native_kind
            && native_matches(value, alias, common)
            && (common || value.source_instance_id == alias.source_instance_id)
            && !targets.contains(&value.target)
        {
            targets.push(value.target.clone());
        }
    }
    Ok(targets)
}
async fn selected_target_available(
    base: &dyn AgentSnapshotRead,
    target: &AgentEntityRefV1,
    integration: IntegrationId,
) -> Result<bool> {
    let id = session_id(target).ok_or(Error::Corrupt("capture alias session target"))?;
    let key = AgentCatalogKey::derive("session", id.as_bytes()).map_err(agent_error)?;
    let Some(reference) = base.lookup(AgentCatalogKind::Sessions, key).await? else {
        return Ok(false);
    };
    let session = match base.read_object(reference).await? {
        AgentObjectV1::SessionSnapshot(session) => session,
        AgentObjectV1::CatalogConflict(_) => return Ok(false),
        _ => return Err(Error::Corrupt("capture alias session selection")),
    };
    if session.session_id != id || session.integration != integration {
        return Err(Error::Corrupt("capture alias session identity"));
    }
    if let AgentEntityRefV1::Tool { tool_call_id, .. } = target {
        let Some(reference) = session.tools.get(tool_call_id) else {
            return Ok(false);
        };
        let AgentObjectV1::ToolSnapshot(tool) = base.read_object(*reference).await? else {
            return Err(Error::Corrupt("capture alias tool selection"));
        };
        if tool.session_id != id || tool.tool_call_id != *tool_call_id {
            return Err(Error::Corrupt("capture alias tool identity"));
        }
    }
    Ok(true)
}
fn block(
    ids: &mut Identities,
    candidates: &mut NormalizationCandidates,
    session: AgentSessionId,
    code: &str,
) {
    if ids.blocked.insert(session) {
        candidates
            .activity
            .diagnostics
            .push(NormalizationDiagnosticV1 {
                kind: NormalizationDiagnosticKind::InvalidRecord,
                code: code.into(),
                source_sequence: None,
            });
    }
}
fn common_session(alias: &AgentAliasV1) -> bool {
    if alias.native_kind != NativeEntityKind::Session {
        return false;
    }
    match alias.integration {
        IntegrationId::Codex => uuid(&alias.native_id),
        IntegrationId::OpenCode => {
            alias.native_id.starts_with("ses")
                && alias.native_id.len() > 3
                && alias.native_id.len() <= 128
                && alias
                    .native_id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        }
        _ => false,
    }
}
pub(super) fn common_projection(aliases: &[AgentAliasV1]) -> Option<&'static str> {
    aliases
        .iter()
        .find(|a| common_session(a))
        .and_then(|a| match a.integration {
            IntegrationId::Codex => Some("codex-native-alias-join/v1"),
            IntegrationId::OpenCode => Some("opencode-native-alias-join/v1"),
            _ => None,
        })
}
fn uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}

fn native_matches(a: &AgentAliasV1, b: &AgentAliasV1, common: bool) -> bool {
    if !common || b.integration != IntegrationId::Codex {
        return a.native_id == b.native_id;
    }
    if b.native_kind == NativeEntityKind::Session {
        return a.native_id.eq_ignore_ascii_case(&b.native_id);
    }
    match (a.native_id.split_once('/'), b.native_id.split_once('/')) {
        (Some((a_session, a_id)), Some((b_session, b_id))) => {
            a_session.eq_ignore_ascii_case(b_session) && a_id == b_id
        }
        _ => false,
    }
}

impl AgentProjector<'_> {
    pub(super) async fn stage_aliases(
        &self,
        base: &dyn AgentSnapshotRead,
        aliases: &[AgentAliasV1],
    ) -> Result<Vec<AgentCatalogEditV1>> {
        let mut edits = Vec::new();
        for alias in aliases {
            let key = alias.catalog_key().map_err(agent_error)?;
            let expected = base.lookup(AgentCatalogKind::Aliases, key).await?;
            let replacement = self
                .stage_agent_object(&AgentObjectV1::Alias(alias.clone()))
                .await?;
            if expected != Some(replacement) {
                edits.push(jj_agent::AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Aliases,
                    key,
                    expected,
                    replacement: Some(replacement),
                });
            }
        }
        Ok(edits)
    }
}
