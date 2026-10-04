use super::*;
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
};
use std::time::Instant;
mod observations;
use observations::Availability;
pub(super) use observations::RootObservations;
struct Root {
    profile: CaptureProfile,
    path: PathBuf,
    scanner: Option<CodexDirectoryScanner>,
    next: Instant,
    availability: Availability,
    rescan: bool,
}
pub(super) struct DiscoveryWork {
    roots: Vec<Root>,
    after: usize,
    interval: Duration,
    #[cfg(test)]
    pub(super) fault: Option<crate::capture_worker::executor::faults::FaultGate>,
}
pub(super) struct DiscoveryOutput {
    pub coverage: RootObservations,
    pub qualified: Option<QualifiedCodexSource>,
    pub unavailable: bool,
}
impl DiscoveryWork {
    pub fn new(roots: Vec<(CaptureProfile, PathBuf)>, interval: Duration) -> Self {
        Self {
            roots: roots
                .into_iter()
                .map(|(profile, path)| Root {
                    profile,
                    path,
                    scanner: None,
                    next: Instant::now(),
                    availability: Availability::Unchecked,
                    rescan: false,
                })
                .collect(),
            after: 0,
            interval,
            #[cfg(test)]
            fault: None,
        }
    }
    pub fn rescan(&mut self) {
        for root in &mut self.roots {
            if root.scanner.is_none() {
                root.next = Instant::now();
            } else {
                root.rescan = true;
            }
        }
    }
    pub fn clear_rescan(&mut self) {
        for root in &mut self.roots {
            root.rescan = false;
        }
    }
    fn coverage(&self) -> RootObservations {
        let mut result = RootObservations::default();
        for root in &self.roots {
            result.observe(root.profile, root.availability, root.rescan);
        }
        result
    }
    fn advance(&mut self) -> Result<Option<QualifiedCodexSource>> {
        for _ in 0..self.roots.len() {
            let index = self.after;
            self.after = (self.after + 1) % self.roots.len();
            let root = &mut self.roots[index];
            if root.scanner.is_none() {
                if Instant::now() < root.next {
                    continue;
                }
                root.next = Instant::now() + self.interval;
                root.availability = Availability::Unavailable;
                let approved = ApprovedCodexRoot::open(&root.path)
                    .map_err(|_| Error::Service("capture discovery root unavailable"))?;
                let Some(approved) = approved else {
                    return Ok(None);
                };
                root.scanner = Some(
                    if root.profile.is_cursor() {
                        CodexDirectoryScanner::cursor(approved)
                    } else {
                        CodexDirectoryScanner::new(approved)
                    }
                    .map_err(|_| Error::Service("capture discovery scanner unavailable"))?,
                );
                root.availability = Availability::Available;
            }
            let scan = root.scanner.as_mut().expect("opened scanner").step(64, 1);
            if scan.as_ref().map_or(true, |step| step.complete) {
                root.scanner = None;
                root.next = Instant::now()
                    + if root.rescan {
                        Duration::ZERO
                    } else {
                        self.interval
                    };
                root.rescan = false;
            }
            if scan.is_err() {
                root.availability = Availability::Unavailable;
            }
            let step = scan.map_err(|_| Error::Service("capture discovery step unavailable"))?;
            return step
                .candidates
                .into_iter()
                .next()
                .map(|candidate| {
                    QualifiedCodexSource::inspect(root.profile, candidate)
                        .map_err(|_| Error::Service("capture discovery profile unavailable"))
                })
                .transpose()
                .map(Option::flatten);
        }
        Ok(None)
    }
}

#[cfg(all(test, unix))]
mod tests;
#[async_trait::async_trait]
impl OwnedWork for DiscoveryWork {
    type Command = ();
    type Output = DiscoveryOutput;
    async fn step(&mut self, _: ()) -> Self::Output {
        match self.advance() {
            Ok(qualified) => {
                #[cfg(test)]
                if qualified.is_some()
                    && let Some(gate) = self.fault.take()
                {
                    gate.fail();
                }
                DiscoveryOutput {
                    coverage: self.coverage(),
                    qualified,
                    unavailable: false,
                }
            }
            Err(_) => DiscoveryOutput {
                coverage: self.coverage(),
                qualified: None,
                unavailable: true,
            },
        }
    }
}
