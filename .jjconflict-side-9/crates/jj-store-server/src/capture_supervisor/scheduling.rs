use super::*;
impl CodexCaptureSupervisor {
    /// Dispatches available lanes without waiting for any busy or draining lane.
    pub fn start_step(&mut self) -> Result<()> {
        if self.paused || self.closed {
            return Err(Error::Service("Codex supervisor paused or stopped"));
        }
        self.dispatched = true;
        self.start_recovery()?;
        self.start_api_auxiliary()?;
        if self.discovery.phase() == CaptureWorkerPhase::Ready {
            if self.discard_rescan {
                self.discovery
                    .idle_mut()
                    .expect("ready discovery")
                    .clear_rescan();
                self.discovery_coverage.rescan_pending = false;
                self.discard_rescan = false;
            }
            if self.rescan_requested {
                self.discovery.idle_mut().expect("ready discovery").rescan();
            }
            self.discovery.start(())?;
            self.rescan_requested = false;
        }
        let mut occupied = self.slots.iter().filter(|s| s.occupied).count();
        let mut raw = self.slots.iter().filter(|s| s.raw).count();
        if let Some(api) = &self.api {
            occupied += api.occupied();
            raw += api.raw();
        }
        self.api_first = !self.api_first;
        if self.api_first {
            self.start_api_sources(&mut occupied, &mut raw)?;
        }
        let start = self.after;
        for offset in 0..self.slots.len() {
            let index = (start + offset) % self.slots.len();
            let slot = &mut self.slots[index];
            if slot.owner.phase() != CaptureWorkerPhase::Ready {
                continue;
            }
            slot.install_import();
            if !slot.occupied && occupied == self.config.max_readers {
                continue;
            }
            let needs = slot.raw || slot.snapshot.needs_frame;
            let reserve = slot.raw || (needs && raw < self.config.max_readers * 2);
            if needs && !reserve {
                continue;
            }
            if reserve && !slot.raw {
                slot.raw = true;
                raw += 1;
            }
            if !slot.occupied {
                slot.occupied = true;
                occupied += 1;
            }
            slot.owner.start(reserve)?;
            self.after = (index + 1) % self.slots.len();
        }
        if !self.api_first {
            self.start_api_sources(&mut occupied, &mut raw)?;
        }
        if self.delivery.phase() == CaptureWorkerPhase::Ready {
            let sources = self
                .slots
                .iter()
                .map(|s| DeliverySource {
                    qualified: s.qualified.clone(),
                    anchor: s.snapshot.anchor.clone(),
                    ordinary: s.ordinary.clone(),
                })
                .collect();
            self.delivery.start(DeliveryCommand {
                sources,
                epoch: self.epoch,
            })?;
        }
        Ok(())
    }
    /// Settlement restores original owners; cancelled reservations remain held.
    pub async fn poll(&mut self) -> CodexSupervisorPoll {
        let mut changed = self.poll_recovery().await;
        match self.discovery.poll().await {
            Completion::Completed(output) => {
                self.discovery_coverage = output.coverage;
                changed = true;
                self.unavailable += usize::from(output.unavailable);
                if let Some(qualified) = output.qualified {
                    let fingerprint = qualified.fingerprint();
                    if !self.fingerprints.contains(&fingerprint) {
                        if self.slots.len() + self.api.as_ref().map_or(0, |api| api.tracked())
                            == self.config.max_sources
                        {
                            self.limit_reached = true;
                        } else if !self.closed
                            && let Ok(selector) = self.selectors.allocate()
                            && let Ok(mut owner) = OwnedWorker::new(
                                SourceWork::new(qualified.clone(), self.resources.fork()),
                                self.deadline,
                            )
                        {
                            if let Some(storage) = &self.storage {
                                owner.retain_storage(storage.fork());
                            }
                            if self.paused {
                                owner.pause();
                            }
                            self.slots.push(SourceSlot {
                                selector,
                                owner,
                                qualified,
                                snapshot: SourceSnapshot {
                                    awaiting_activity: true,
                                    needs_frame: true,
                                    ..Default::default()
                                },
                                raw: false,
                                occupied: false,
                                import: None,
                                import_installed: false,
                                ordinary: Arc::new(std::sync::atomic::AtomicBool::new(true)),
                            });
                            self.fingerprints.insert(fingerprint);
                        }
                    }
                }
            }
            Completion::Cancelled | Completion::Deadline | Completion::Lost => {
                changed = true;
                self.unavailable += 1;
            }
            _ => {}
        }
        if self.discard_rescan
            && let Some(work) = self.discovery.idle_mut()
        {
            work.clear_rescan();
            self.discovery_coverage.rescan_pending = false;
            self.discard_rescan = false;
        }
        for slot in &mut self.slots {
            match slot.owner.poll().await {
                Completion::Completed(output) => {
                    changed = true;
                    slot.occupied = false;
                    slot.raw = !output.snapshot.compact;
                    slot.snapshot = output.snapshot;
                    if self.closed {
                        slot.occupied = false;
                        slot.raw = false;
                    }
                    self.unavailable += usize::from(output.unavailable);
                    if output.enqueued || output.unavailable || slot.snapshot.retired {
                        self.epoch += 1;
                    }
                }
                Completion::Cancelled | Completion::Deadline => {
                    changed = true;
                    self.epoch += 1;
                    if self.closed {
                        slot.occupied = false;
                        slot.raw = false;
                    }
                }
                Completion::Lost => {
                    slot.snapshot.unavailable = true;
                    changed = true;
                    slot.occupied = false;
                    slot.raw = false;
                    slot.snapshot.caught_up = false;
                    self.unavailable += 1;
                    self.epoch += 1;
                }
                _ => {}
            }
        }
        let mut retired = false;
        self.slots.retain_mut(|slot| {
            if !slot.snapshot.retired {
                return true;
            }
            // Retirement is reported only by a completed compact source step.
            // No busy, lost or unfinished raw owner is reclaimed here.
            slot.owner.stop();
            self.fingerprints.remove(&slot.qualified.fingerprint());
            retired = true;
            false
        });
        if retired {
            self.after %= self.slots.len().max(1);
            self.limit_reached = false;
        }
        match self.delivery.poll().await {
            Completion::Completed(output) => {
                changed = true;
                self.delivery_frames = output.frames;
                self.delivery_capacity_refusal = output.capacity_refusal;
                if output.unavailable {
                    self.queue_epoch = None;
                    self.unavailable += 1;
                } else {
                    self.pending = output.pending;
                    self.queue_epoch = Some(output.epoch);
                }
            }
            completion @ (Completion::Cancelled | Completion::Deadline | Completion::Lost) => {
                changed = true;
                self.unavailable += usize::from(matches!(completion, Completion::Lost));
                self.queue_epoch = None;
                self.delivery_frames = self
                    .delivery
                    .idle_mut()
                    .map_or(self.config.max_readers * 2, |work| work.frames());
            }
            _ => {}
        }
        changed |= self.poll_api().await;
        if changed {
            CodexSupervisorPoll::Completed(Box::new(self.status()))
        } else if matches!(
            self.phase(),
            CaptureWorkerPhase::Running
                | CaptureWorkerPhase::Overdue
                | CaptureWorkerPhase::Draining
        ) {
            CodexSupervisorPoll::Running
        } else {
            CodexSupervisorPoll::Idle
        }
    }
}
