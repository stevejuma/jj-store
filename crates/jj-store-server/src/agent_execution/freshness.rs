//! Semantic installation notices; internal retry bookkeeping never drives UI reads.
use super::*;

pub(super) fn execution_witness(state: &State) -> Result<String> {
    journal::digest(&(
        state.setup_revision,
        state.settings.revision,
        state
            .sessions
            .iter()
            .map(|(id, saved)| (id, &saved.view))
            .collect::<Vec<_>>(),
        &state.conversations,
        &state.warm_agents,
        state
            .intents
            .iter()
            .map(|(id, intent)| (id, &intent.outcome))
            .collect::<Vec<_>>(),
        &state.runtime_selection,
        &state.runtime_installs,
        state.compute_generation,
        &state.compute,
    ))
}
impl Supervisor {
    pub(super) fn execution_changed(&self) {
        let _ = self.0.changed.send(0);
        self.0.freshness.notify(
            None,
            jj_store_service::freshness::Domain::AgentExecution,
            None,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal_journal_revisions_do_not_invalidate_execution_but_visible_changes_do() {
        let mut state = State::default();
        let initial = execution_witness(&state).unwrap();
        state.revision += 1;
        assert_eq!(initial, execution_witness(&state).unwrap());
        state.warm_agents.insert("agent".into());
        let enabled = execution_witness(&state).unwrap();
        assert_ne!(initial, enabled);
        state.settings.revision += 1;
        assert_ne!(enabled, execution_witness(&state).unwrap());
    }
}
