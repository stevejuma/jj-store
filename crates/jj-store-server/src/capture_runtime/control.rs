use super::*;

/// Cloneable controls and bounded private source metadata; no task or storage ownership.
#[derive(Clone)]
pub struct CaptureRuntimeControl {
    pub(super) command: watch::Sender<Command>,
    pub(super) snapshot: watch::Receiver<CaptureRuntimeSnapshot>,
    pub(super) rescan: std::sync::Arc<rescan::RescanSignal>,
    pub(super) targets: watch::Receiver<std::sync::Arc<targets::ImportTargets>>,
    pub(super) sources:
        watch::Receiver<std::sync::Arc<crate::capture_supervisor::CodexSourceCatalog>>,
    pub(super) imports: tokio::sync::mpsc::Sender<super::imports::ImportCommand>,
}
impl CaptureRuntimeControl {
    /// Reads only the cached observation; an unexpectedly lost producer is unavailable.
    pub fn snapshot(&self) -> Result<CaptureRuntimeSnapshot> {
        let mut observed = *self.snapshot.borrow();
        observed.status.discovery.rescan_pending |= self.rescan.pending();
        if self.snapshot.has_changed().is_err() && observed.phase != CaptureWorkerPhase::Stopped {
            return Err(Error::Service("capture runtime observation unavailable"));
        }
        Ok(observed)
    }
    /// Current command and observation only; no import, frame or task ownership.
    pub(crate) fn admit_import(&self) -> Result<()> {
        let observed = self.snapshot()?;
        if self.command.is_closed()
            || self.snapshot.has_changed().is_err()
            || *self.command.borrow() != Command::Running
            || !matches!(
                observed.phase,
                CaptureWorkerPhase::Ready | CaptureWorkerPhase::Running
            )
        {
            return Err(Error::Service("capture import runtime unavailable"));
        }
        Ok(())
    }
    /// Acceptance is a requested transition, not proof that outstanding work drained.
    pub fn pause(&self) -> Result<()> {
        self.transition(Command::Paused)
    }
    pub fn resume(&self) -> Result<()> {
        self.transition(Command::Running)
    }
    fn transition(&self, command: Command) -> Result<()> {
        if self.command.is_closed() || self.snapshot.has_changed().is_err() {
            return Err(Error::Service("capture runtime stopped"));
        }
        let mut stopped = false;
        self.command.send_if_modified(|state| {
            if *state == Command::Stopped {
                stopped = true;
                return false;
            }
            let changed = *state != command;
            if command != Command::Running {
                self.rescan.clear();
            }
            *state = command;
            changed
        });
        if stopped || self.command.is_closed() || self.snapshot.has_changed().is_err() {
            Err(Error::Service("capture runtime stopped"))
        } else {
            Ok(())
        }
    }
    pub(super) fn stop(&self) {
        self.command.send_if_modified(|state| {
            self.rescan.clear();
            let changed = *state != Command::Stopped;
            *state = Command::Stopped;
            changed
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requested_pause_withholds_import_even_before_cached_observation_changes() {
        let observed = CaptureRuntimeSnapshot {
            phase: CaptureWorkerPhase::Running,
            status: Default::default(),
        };
        let (command, _receiver) = watch::channel(Command::Running);
        let (_updates, snapshot) = watch::channel(observed);
        let control = CaptureRuntimeControl {
            command,
            snapshot,
            rescan: Default::default(),
            targets: super::super::targets::empty(),
            sources: super::super::sources::test_observation(observed.phase),
            imports: super::super::imports::empty(),
        };
        assert!(control.admit_import().is_ok());
        control.command.send_replace(Command::Paused);
        assert_eq!(
            control.snapshot().unwrap().phase,
            CaptureWorkerPhase::Running
        );
        assert!(control.admit_import().is_err());
    }

    #[test]
    fn closed_nonterminal_observation_refuses_stale_caught_up_and_controls() {
        let observed = CaptureRuntimeSnapshot {
            phase: CaptureWorkerPhase::Running,
            status: CodexSupervisorStatus {
                caught_up: 3,
                ..Default::default()
            },
        };
        let (command, receiver) = watch::channel(Command::Running);
        let (updates, snapshot) = watch::channel(observed);
        let control = CaptureRuntimeControl {
            command,
            snapshot,
            rescan: Default::default(),
            targets: super::super::targets::empty(),
            sources: super::super::sources::test_observation(observed.phase),
            imports: super::super::imports::empty(),
        };
        assert_eq!(control.snapshot().unwrap(), observed);
        drop(updates);
        assert!(control.snapshot().is_err());
        assert!(control.pause().is_err() && control.resume().is_err());
        assert!(*receiver.borrow() == Command::Running);
    }

    #[test]
    fn completed_terminal_observation_remains_readable_with_closed_controls() {
        let observed = CaptureRuntimeSnapshot {
            phase: CaptureWorkerPhase::Stopped,
            status: Default::default(),
        };
        let (command, _receiver) = watch::channel(Command::Stopped);
        let (updates, snapshot) = watch::channel(observed);
        let control = CaptureRuntimeControl {
            command,
            snapshot,
            rescan: Default::default(),
            targets: super::super::targets::empty(),
            sources: super::super::sources::test_observation(observed.phase),
            imports: super::super::imports::empty(),
        };
        drop(updates);
        let terminal = control.clone();
        drop(control);
        assert_eq!(terminal.snapshot().unwrap(), observed);
        assert!(terminal.pause().is_err() && terminal.resume().is_err());
    }
}
