use super::*;
pub(super) fn agent_error(error: AgentError) -> Error {
    match error {
        AgentError::Limit => Error::Limit("agent object"),
        AgentError::Invalid(_) | AgentError::Encoding(_) => Error::Corrupt("agent object"),
    }
}
