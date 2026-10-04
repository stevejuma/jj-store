//! Transient identity adoption; accepted records and requests are never rewritten.
use super::*;
use jj_agent::{MessageId, ToolCallId, TurnId};

#[derive(Default)]
pub(super) struct Identities {
    pub sessions: BTreeMap<AgentSessionId, AgentSessionId>,
    turns: BTreeMap<TurnId, TurnId>,
    messages: BTreeMap<MessageId, MessageId>,
    tools: BTreeMap<ToolCallId, ToolCallId>,
    pub blocked: BTreeSet<AgentSessionId>,
}
impl Identities {
    pub fn adopt(&mut self, from: &AgentEntityRefV1, to: &AgentEntityRefV1) -> Result<()> {
        match (from, to) {
            (
                AgentEntityRefV1::Session { session_id: a },
                AgentEntityRefV1::Session { session_id: b },
            ) => {
                self.sessions.insert(*a, *b);
            }
            (
                AgentEntityRefV1::Turn { turn_id: a, .. },
                AgentEntityRefV1::Turn { turn_id: b, .. },
            ) => {
                self.turns.insert(*a, *b);
            }
            (
                AgentEntityRefV1::Message { message_id: a, .. },
                AgentEntityRefV1::Message { message_id: b, .. },
            ) => {
                self.messages.insert(*a, *b);
            }
            (
                AgentEntityRefV1::Tool {
                    tool_call_id: a, ..
                },
                AgentEntityRefV1::Tool {
                    tool_call_id: b, ..
                },
            ) => {
                self.tools.insert(*a, *b);
            }
            _ => return Err(Error::Corrupt("capture alias adoption kind")),
        }
        Ok(())
    }
    pub fn target(&self, target: &mut AgentEntityRefV1) {
        match target {
            AgentEntityRefV1::Session { session_id } => replace(&self.sessions, session_id),
            AgentEntityRefV1::Turn {
                session_id,
                turn_id,
            } => {
                replace(&self.sessions, session_id);
                replace(&self.turns, turn_id);
            }
            AgentEntityRefV1::Message {
                session_id,
                message_id,
            } => {
                replace(&self.sessions, session_id);
                replace(&self.messages, message_id);
            }
            AgentEntityRefV1::Tool {
                session_id,
                tool_call_id,
            } => {
                replace(&self.sessions, session_id);
                replace(&self.tools, tool_call_id);
            }
            _ => {}
        }
    }
    pub fn apply(self, candidates: &mut NormalizationCandidates) {
        candidates
            .activity
            .aliases
            .retain(|a| session_id(&a.target).is_none_or(|s| !self.blocked.contains(&s)));
        for alias in &mut candidates.activity.aliases {
            self.target(&mut alias.target);
        }
        candidates
            .activity
            .records
            .retain(|r| !self.blocked.contains(&record_session(r)));
        for record in &mut candidates.activity.records {
            match record {
                NormalizedActivityRecordV1::SessionState(v) => {
                    replace(&self.sessions, &mut v.session_id)
                }
                NormalizedActivityRecordV1::Message(v) => {
                    replace(&self.sessions, &mut v.session_id);
                    replace_optional(&self.turns, &mut v.turn_id);
                    replace(&self.messages, &mut v.message_id);
                }
                NormalizedActivityRecordV1::Tool(v) => {
                    replace(&self.sessions, &mut v.session_id);
                    replace_optional(&self.turns, &mut v.turn_id);
                    replace(&self.tools, &mut v.tool_call_id);
                }
                NormalizedActivityRecordV1::Permission(v) => {
                    replace(&self.sessions, &mut v.session_id);
                    replace_optional(&self.tools, &mut v.tool_call_id);
                }
                NormalizedActivityRecordV1::FileObservation(v) => {
                    replace(&self.sessions, &mut v.session_id)
                }
            }
        }
        candidates.tool_results.retain(|r| {
            session_id(&r.session_alias.target).is_none_or(|s| !self.blocked.contains(&s))
        });
        for result in &mut candidates.tool_results {
            self.target(&mut result.session_alias.target);
            self.target(&mut result.tool_alias.target);
        }
    }
}

pub(super) fn session_id(target: &AgentEntityRefV1) -> Option<AgentSessionId> {
    match target {
        AgentEntityRefV1::Session { session_id }
        | AgentEntityRefV1::Tool { session_id, .. }
        | AgentEntityRefV1::Turn { session_id, .. }
        | AgentEntityRefV1::Message { session_id, .. } => Some(*session_id),
        _ => None,
    }
}
fn record_session(record: &NormalizedActivityRecordV1) -> AgentSessionId {
    match record {
        NormalizedActivityRecordV1::SessionState(v) => v.session_id,
        NormalizedActivityRecordV1::Message(v) => v.session_id,
        NormalizedActivityRecordV1::Tool(v) => v.session_id,
        NormalizedActivityRecordV1::Permission(v) => v.session_id,
        NormalizedActivityRecordV1::FileObservation(v) => v.session_id,
    }
}
fn replace<T: Copy + Ord>(map: &BTreeMap<T, T>, id: &mut T) {
    if let Some(next) = map.get(id) {
        *id = *next;
    }
}
fn replace_optional<T: Copy + Ord>(map: &BTreeMap<T, T>, id: &mut Option<T>) {
    if let Some(id) = id {
        replace(map, id);
    }
}
