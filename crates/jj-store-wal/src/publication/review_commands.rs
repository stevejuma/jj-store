use super::*;
use jj_review::encode_command_result;
use std::time::{SystemTime, UNIX_EPOCH};
impl Coordinator {
    pub async fn publish_review_command(
        &self,
        request: &ReviewCommandRequest,
    ) -> std::result::Result<PublishOutcome, ReviewCommandError> {
        request.validate()?;
        let intent = request.intent(self.records.repository)?;
        let accepted_at_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ReviewCommandError::Invalid("review command time"))?
            .as_millis()
            .try_into()
            .map_err(|_| ReviewCommandError::Invalid("review command time"))?;
        let unknown = || PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_review_command_inner(request, intent, accepted_at_millis),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(ReviewCommandError::Publication(jj_store_core::Error::OutcomeUnknown)))
            | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    async fn publish_review_command_inner(
        &self,
        request: &ReviewCommandRequest,
        intent: IntentDigest,
        accepted_at_millis: i64,
    ) -> std::result::Result<Receipt, ReviewCommandError> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            let current = base
                .manifest
                .domain_state(Domain::Review)
                .ok_or(Error::Corrupt("review domain root"))?;
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Review,
                current,
                current.root,
                Vec::new(),
            )?;
            self.guard.authorize_domain(&base, &probe).await?;
            if let Some(receipt) = self
                .lookup_identity(
                    &base.manifest,
                    request.principal,
                    request.request_id,
                    request.publication_id,
                    intent,
                )
                .await?
            {
                return Ok(receipt);
            }

            let pinned = self.review_store(current.root).await?;
            let applied = ReviewProjector::new(self)
                .stage_command(&pinned, request, accepted_at_millis)
                .await?;
            let result = encode_command_result(&applied.result)
                .map_err(|_| ReviewCommandError::Invalid("review command result"))?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Review,
                current,
                self.stage_review_root(&applied.root).await?,
                result,
            )?
            .with_review_facts(applied.delta)?;
            match self.publish_domain_inner(&domain, intent).await {
                Err(Error::Stale) => continue,
                Ok(receipt) => return Ok(receipt),
                Err(error) => return Err(error.into()),
            }
        }
        Err(Error::Contended.into())
    }
}
