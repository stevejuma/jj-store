use super::*;
impl CodexCaptureSupervisor {
    pub(in crate::capture_supervisor) fn start_api_auxiliary(&mut self) -> Result<()> {
        let Some(lane) = &mut self.api else {
            return Ok(());
        };
        if lane.discovery.phase() == CaptureWorkerPhase::Ready {
            if lane.rescan {
                lane.discovery
                    .idle_mut()
                    .expect("ready API discovery")
                    .rescan();
            }
            lane.discovery.start(())?;
            lane.rescan = false;
        }
        if lane.delivery.phase() == CaptureWorkerPhase::Ready {
            lane.delivery.start(
                lane.slots
                    .iter()
                    .flat_map(|s| s.proofs.iter().cloned())
                    .collect(),
            )?;
        }
        Ok(())
    }
    pub(in crate::capture_supervisor) fn start_api_sources(
        &mut self,
        occupied: &mut usize,
        raw: &mut usize,
    ) -> Result<()> {
        let Some(lane) = &mut self.api else {
            return Ok(());
        };
        let start = lane.after;
        for offset in 0..lane.slots.len() {
            let index = (start + offset) % lane.slots.len();
            let slot = &mut lane.slots[index];
            if slot.owner.phase() != CaptureWorkerPhase::Ready {
                continue;
            }
            if slot.rescan {
                slot.owner.idle_mut().expect("ready API source").rescan();
                slot.rescan = false;
            }
            if !slot.occupied && *occupied == self.config.max_readers {
                continue;
            }
            let needs = slot.raw || slot.snapshot.needs_frame;
            let reserve = slot.raw || (needs && *raw < self.config.max_readers * 2);
            if needs && !reserve {
                continue;
            }
            if reserve && !slot.raw {
                slot.raw = true;
                *raw += 1;
            }
            if !slot.occupied {
                slot.occupied = true;
                *occupied += 1;
            }
            slot.owner.start(reserve)?;
            lane.after = (index + 1) % lane.slots.len();
        }
        Ok(())
    }
    pub(in crate::capture_supervisor) async fn poll_api(&mut self) -> bool {
        let Some(lane) = &mut self.api else {
            return false;
        };
        let mut changed = false;
        match lane.discovery.poll().await {
            Completion::Completed(output) => {
                changed = true;
                self.unavailable += usize::from(output.unavailable);
                if let Some(native_session) = output.candidate
                    && !lane
                        .slots
                        .iter()
                        .any(|s| s.native_session == native_session)
                {
                    if self.slots.len() + lane.slots.len() == self.config.max_sources {
                        self.limit_reached = true;
                    } else if !self.closed
                        && let Ok(selector) = self.selectors.allocate()
                        && let Ok(mut owner) = OwnedWorker::new(
                            ApiSourceWork::new(
                                lane.api.clone(),
                                native_session.clone(),
                                self.resources.fork(),
                                Duration::from_secs(self.config.rescan_seconds),
                            ),
                            self.deadline,
                        )
                    {
                        if let Some(storage) = &self.storage {
                            owner.retain_storage(storage.fork());
                        }
                        if self.paused {
                            owner.pause();
                        }
                        lane.slots.push(ApiSlot {
                            selector,
                            native_session,
                            owner,
                            snapshot: SourceSnapshot {
                                awaiting_activity: true,
                                needs_frame: true,
                                ..Default::default()
                            },
                            proofs: Vec::new(),
                            publication_query: None,
                            raw: false,
                            occupied: false,
                            rescan: false,
                        });
                    }
                }
            }
            Completion::Cancelled | Completion::Deadline | Completion::Lost => {
                changed = true;
                self.unavailable += 1;
            }
            _ => {}
        }
        for slot in &mut lane.slots {
            match slot.owner.poll().await {
                Completion::Completed(output) => {
                    changed = true;
                    slot.occupied = false;
                    slot.raw = !output.snapshot.compact;
                    slot.snapshot = output.snapshot;
                    slot.proofs = output.proofs;
                    slot.publication_query = output.publication_query;
                    if self.closed {
                        slot.raw = false;
                        slot.proofs.clear();
                    }
                    self.unavailable += usize::from(output.unavailable);
                    if output.enqueued || output.unavailable {
                        self.epoch += 1;
                    }
                }
                Completion::Cancelled | Completion::Deadline => {
                    changed = true;
                    self.epoch += 1;
                    if self.closed {
                        slot.raw = false;
                        slot.occupied = false;
                        slot.proofs.clear();
                    }
                }
                Completion::Lost => {
                    changed = true;
                    slot.snapshot.unavailable = true;
                    slot.snapshot.caught_up = false;
                    slot.raw = false;
                    slot.occupied = false;
                    slot.proofs.clear();
                    self.unavailable += 1;
                    self.epoch += 1;
                }
                _ => {}
            }
        }
        match lane.delivery.poll().await {
            Completion::Completed(output) => {
                changed = true;
                lane.delivery_capacity_refusal = output.capacity_refusal;
                self.unavailable += usize::from(output.unavailable);
                if output.published || output.unavailable {
                    self.epoch += 1;
                }
            }
            Completion::Cancelled | Completion::Deadline | Completion::Lost => {
                changed = true;
                self.epoch += 1;
            }
            _ => {}
        }
        changed
    }
}
