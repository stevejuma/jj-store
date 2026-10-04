use super::*;
impl OpenCodeCaptureSource {
    pub(super) fn sanitizer_for(
        &mut self,
        proof: &VerifiedCaptureWorkspace,
    ) -> Result<&TranscriptSanitizer> {
        if self
            .sanitizer
            .as_ref()
            .is_none_or(|(path, _)| path != &proof.workspace.path)
        {
            self.sanitizer = Some((
                proof.workspace.path.clone(),
                TranscriptSanitizer::new(
                    std::path::Path::new(&proof.workspace.path),
                    &self.privacy.home,
                    &self.privacy.temporary,
                    &self.privacy.cache,
                )
                .map_err(|_| Error::Service("OpenCode sanitizer unavailable"))?,
            ));
        }
        Ok(&self.sanitizer.as_ref().expect("sanitizer").1)
    }
    pub(super) async fn convert(
        &mut self,
        record: &OpenCodeMessageSnapshot,
        position: &OpenCodeRecordPosition,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<(ReaderCapture, VerifiedCaptureWorkspace)> {
        self.convert_with_lifecycle(record, position, self.pass.lifecycle, context)
            .await
    }
    pub(super) async fn convert_with_lifecycle(
        &mut self,
        record: &OpenCodeMessageSnapshot,
        position: &OpenCodeRecordPosition,
        lifecycle: Option<jj_store_capture::opencode_checkpoint::OpenCodeLifecycleV1>,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<(ReaderCapture, VerifiedCaptureWorkspace)> {
        let (proof, body) = if let Some(directory) = record.directory().and_then(|p| p.to_str()) {
            match context.admission.inspect(directory).await {
                Ok(proof) => (proof, true),
                Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => (self.anchor.clone(), false),
                Err(_) => return Err(Error::Service("OpenCode record admission unavailable")),
            }
        } else {
            (self.anchor.clone(), false)
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let capture = if body {
            record.sanitize(
                proof.workspace.repository,
                position,
                record.directory().expect("admitted context"),
                self.sanitizer_for(&proof)?,
            )
        } else {
            record.sanitize_omitted(proof.workspace.repository, position)
        }
        .map_err(|_| Error::Service("OpenCode record conversion unavailable"))?;
        let capture = if let Some(lifecycle) = lifecycle {
            capture
                .with_opencode_state(
                    jj_store_capture::opencode_checkpoint::OpenCodeSnapshotStateV1 {
                        message_records: position.logical_records,
                        message_prefix: position.prefix_digest,
                        lifecycle,
                    },
                )
                .map_err(|_| Error::Service("OpenCode checkpoint conversion unavailable"))?
        } else {
            capture
        };
        self.current(context).await?;
        context
            .admission
            .revalidate(&proof)
            .await
            .map_err(|_| Error::Service("OpenCode record admission changed"))?;
        Ok((capture, proof))
    }
}
