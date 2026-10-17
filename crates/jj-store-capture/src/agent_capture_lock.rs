//! FIFO admission among forked in-process handles; filesystem locks stay authoritative.
use crate::CaptureResult;
#[cfg(feature = "application")]
use crate::state_lease::StateLease;
use std::sync::{Arc, Condvar, Mutex};
#[derive(Default)]
pub(crate) struct ForkMutations {
    turns: Mutex<(u64, u64)>,
    changed: Condvar,
}
impl ForkMutations {
    pub(crate) fn acquire(self: &Arc<Self>) -> CaptureResult<ForkTurn> {
        let mut turns = self
            .turns
            .lock()
            .map_err(|_| "capture fork gate unavailable")?;
        let ticket = turns.0;
        turns.0 = ticket.checked_add(1).ok_or("capture fork turn exhausted")?;
        while turns.1 != ticket {
            turns = self
                .changed
                .wait(turns)
                .map_err(|_| "capture fork gate unavailable")?;
        }
        Ok(ForkTurn(self.clone()))
    }
}
pub(crate) struct ForkTurn(Arc<ForkMutations>);
impl Drop for ForkTurn {
    fn drop(&mut self) {
        let mut turns = self
            .0
            .turns
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        turns.1 += 1;
        self.0.changed.notify_all();
    }
}
#[cfg(feature = "application")]
pub(crate) struct CaptureMutationLease {
    // Drop the authoritative OS lease before allowing the next local turn.
    pub(crate) _file: StateLease,
    pub(crate) _turn: ForkTurn,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::mpsc,
        time::{Duration, Instant},
    };
    #[test]
    fn queued_fork_mutations_settle_in_arrival_order() {
        let gate = Arc::new(ForkMutations::default());
        let first = gate.acquire().unwrap();
        let (send, receive) = mpsc::channel();
        let mut tasks = Vec::new();
        for index in 0..3 {
            let queued = gate.clone();
            let send = send.clone();
            tasks.push(std::thread::spawn(move || {
                let _turn = queued.acquire().unwrap();
                send.send(index).unwrap();
            }));
            let expires = Instant::now() + Duration::from_secs(2);
            while gate.turns.lock().unwrap().0 != index + 2 {
                assert!(Instant::now() < expires, "fork waiter did not register");
                std::thread::yield_now();
            }
        }
        assert!(receive.try_recv().is_err());
        drop(first);
        for index in 0..3 {
            assert_eq!(receive.recv_timeout(Duration::from_secs(2)).unwrap(), index);
        }
        for task in tasks {
            task.join().unwrap();
        }
    }
}
