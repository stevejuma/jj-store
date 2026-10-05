use super::*;

impl Manager {
    pub(super) async fn request_rescan(
        &self,
        actor: PrincipalId,
        request: CaptureRescanCommand,
    ) -> Result<CaptureRescanOutcome, Error> {
        if request.version != 1
            || request.generation.len() != 32
            || request
                .generation
                .bytes()
                .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        let generation: [u8; 16] = hex::decode(&request.generation)
            .map_err(|_| Error::Invalid)?
            .try_into()
            .map_err(|_| Error::Invalid)?;
        if generation == [0; 16] {
            return Err(Error::Invalid);
        }
        let this = self.clone();
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if !this.0.authority.admit(actor)? {
                return Err(Error::Disabled);
            }
            let binding = this.binding()?;
            if generation != binding.generation {
                return Err(Error::Stale);
            }
            binding
                .journal
                .read_then(|selected, check| {
                    if selected.generation != binding.generation {
                        return Err(Error::Unavailable);
                    }
                    if selected.revision != request.expected_revision {
                        return Err(Error::Stale);
                    }
                    if selected.paused {
                        return Err(Error::Paused);
                    }
                    #[cfg(test)]
                    let hook = this.0.after_selection.lock().unwrap().take();
                    #[cfg(test)]
                    if let Some(hook) = hook {
                        hook();
                    }
                    if !this.0.authority.admit(actor)? {
                        return Err(Error::Disabled);
                    }
                    check().map_err(report::failure)?;
                    binding
                        .runtime
                        .get()
                        .ok_or(Error::Unavailable)?
                        .rescan()
                        .map_err(|_| Error::Unavailable)?;
                    Ok(CaptureRescanOutcome {
                        version: 1,
                        accepted: true,
                        status: this.status_selected(Some(selected)),
                    })
                })
                .map_err(report::failure)?
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
}
