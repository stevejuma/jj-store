use futures::executor::block_on;
use jj_store_conversations::coordination::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
struct Turn;
impl Types for Turn {
    type Parent = u8;
    type Command = u8;
    type Frame = u8;
    type Outcome = &'static str;
    type Error = &'static str;
}
#[derive(Default)]
struct State {
    trace: Vec<String>,
    claimed: Option<(u8, Vec<u8>)>,
    outcomes: BTreeMap<u8, &'static str>,
    fail_context: Option<u8>,
    fail_prepared: bool,
    deny: bool,
    dispatch: BTreeMap<u8, Dispatch>,
}
#[derive(Default)]
struct Host(Mutex<State>);
impl Host {
    fn coordinator(self: &Arc<Self>) -> Coordinator<'_, Turn> {
        Coordinator::new(self.clone(), self.clone(), self.clone(), self.clone())
    }
}
#[async_trait::async_trait]
impl Admission<Turn> for Host {
    async fn authorize(&self, _: &u8, commands: &[u8]) -> Result<(), &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push("authorize".into());
        if s.deny || !(1..=4).contains(&commands.len()) {
            return Err("denied");
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl Context<Turn> for Host {
    async fn prepare(&self, command: &u8) -> Result<u8, &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push(format!("context:{command}"));
        if s.fail_context == Some(*command) {
            return Err("context unavailable");
        }
        Ok(*command)
    }
}
#[async_trait::async_trait]
impl Runtime<Turn> for Host {
    async fn dispatch(&self, command: &u8, frame: u8) -> Dispatch {
        assert_eq!(*command, frame);
        let mut s = self.0.lock().unwrap();
        s.trace.push(format!("dispatch:{command}"));
        let outcome = s
            .dispatch
            .get(command)
            .copied()
            .unwrap_or(Dispatch::Enqueued);
        s.outcomes.insert(*command, "queued");
        outcome
    }
}
#[async_trait::async_trait]
impl Journal<Turn> for Host {
    async fn begin(
        &self,
        parent: &u8,
        commands: &[u8],
    ) -> Result<Begin<&'static str>, &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push("begin".into());
        if let Some((old_parent, old_commands)) = &s.claimed {
            if old_parent != parent || old_commands != commands {
                return Err("intent conflict");
            }
            return Ok(Begin::Replay(
                commands.iter().map(|c| s.outcomes[c]).collect(),
            ));
        }
        s.claimed = Some((*parent, commands.to_vec()));
        for command in commands {
            s.outcomes.insert(*command, "reserved");
        }
        Ok(Begin::New)
    }
    async fn prepared(&self, _: &u8) -> Result<(), &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push("prepared".into());
        if s.fail_prepared {
            Err("journal unavailable")
        } else {
            Ok(())
        }
    }
    async fn release(&self, command: &u8, reason: Release) -> Result<(), &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push(format!("release:{command}:{reason:?}"));
        s.outcomes.insert(*command, "not_sent");
        Ok(())
    }
    async fn unknown(&self, command: &u8) -> Result<(), &'static str> {
        let mut s = self.0.lock().unwrap();
        s.trace.push(format!("unknown:{command}"));
        s.outcomes.insert(*command, "unknown");
        Ok(())
    }
    async fn outcome(&self, command: &u8) -> Result<&'static str, &'static str> {
        Ok(self.0.lock().unwrap().outcomes[command])
    }
    async fn accepted(&self, _: &u8) -> Result<(), &'static str> {
        self.0.lock().unwrap().trace.push("accepted".into());
        Ok(())
    }
}

#[test]
fn every_context_and_durable_prepared_fence_precedes_first_dispatch() {
    let host = Arc::new(Host::default());
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1, 2])).unwrap(),
        vec!["queued", "queued"]
    );
    assert_eq!(
        host.0.lock().unwrap().trace,
        [
            "authorize",
            "begin",
            "context:1",
            "context:2",
            "prepared",
            "dispatch:1",
            "dispatch:2",
            "accepted"
        ]
    );
}

#[test]
fn context_failure_releases_prior_reservations_without_any_dispatch_or_replay() {
    let host = Arc::new(Host::default());
    host.0.lock().unwrap().fail_context = Some(2);
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1, 2])),
        Err("context unavailable")
    );
    assert_eq!(
        host.0.lock().unwrap().trace,
        [
            "authorize",
            "begin",
            "context:1",
            "context:2",
            "release:1:PreparationFailed"
        ]
    );
    host.0.lock().unwrap().fail_context = None;
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1, 2])).unwrap(),
        vec!["not_sent", "reserved"]
    );
    assert!(
        !host
            .0
            .lock()
            .unwrap()
            .trace
            .iter()
            .any(|v| v.starts_with("dispatch:"))
    );
}

#[test]
fn unknown_delivery_is_retained_and_only_proven_not_sent_releases() {
    let host = Arc::new(Host::default());
    host.0.lock().unwrap().dispatch =
        BTreeMap::from([(1, Dispatch::Unknown), (2, Dispatch::NotSent)]);
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1, 2])).unwrap(),
        vec!["unknown", "not_sent"]
    );
    let first = host.0.lock().unwrap().trace.clone();
    assert!(first.contains(&"unknown:1".into()));
    assert!(first.contains(&"release:2:NotSent".into()));
    assert!(!first.iter().any(|v| v.starts_with("release:1")));
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1, 2])).unwrap(),
        vec!["unknown", "not_sent"]
    );
    assert_eq!(
        &host.0.lock().unwrap().trace[first.len()..],
        ["authorize", "begin"]
    );
}

#[test]
fn journal_failure_before_prepared_blocks_dispatch_and_recovery_returns_retained_intent() {
    let host = Arc::new(Host::default());
    host.0.lock().unwrap().fail_prepared = true;
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1])),
        Err("journal unavailable")
    );
    host.0.lock().unwrap().fail_prepared = false;
    assert_eq!(
        block_on(host.coordinator().send(&7, &[1])).unwrap(),
        vec!["reserved"]
    );
    assert!(
        !host
            .0
            .lock()
            .unwrap()
            .trace
            .iter()
            .any(|v| v.starts_with("dispatch:"))
    );
}

#[test]
fn retained_intents_recheck_admission_and_reject_changed_request_contents() {
    let host = Arc::new(Host::default());
    block_on(host.coordinator().send(&7, &[1])).unwrap();
    assert_eq!(
        block_on(host.coordinator().send(&7, &[2])),
        Err("intent conflict")
    );
    host.0.lock().unwrap().deny = true;
    assert_eq!(block_on(host.coordinator().send(&7, &[1])), Err("denied"));
    assert_eq!(
        host.0
            .lock()
            .unwrap()
            .trace
            .iter()
            .filter(|v| *v == "dispatch:1")
            .count(),
        1
    );
}
