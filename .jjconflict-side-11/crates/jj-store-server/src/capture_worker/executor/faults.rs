//! Test-only synchronous task loss after a real discovery/recovery boundary.
use std::{sync::mpsc, time::Duration};
use tokio::sync::oneshot;

pub(crate) struct FaultGate {
    entered: oneshot::Sender<()>,
    release: mpsc::Receiver<()>,
}
pub(crate) struct Release(mpsc::Sender<()>);
impl FaultGate {
    pub(crate) fn new() -> (Self, oneshot::Receiver<()>, Release) {
        let (entered, observed) = oneshot::channel();
        let (release, released) = mpsc::channel();
        (
            Self {
                entered,
                release: released,
            },
            observed,
            Release(release),
        )
    }
    pub(crate) fn fail(self) -> ! {
        self.entered.send(()).expect("running fault observed");
        self.release
            .recv_timeout(Duration::from_secs(30))
            .expect("running fault released");
        panic!("injected running capture task loss");
    }
}
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}
