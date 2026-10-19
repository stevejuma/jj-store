//! Explicit ancestry import; source state and vault files remain untouched.
mod publish;
mod source;
use super::*;
use jj_lib::{backend::CommitId, op_store::RefTarget, ref_name::RefName};
use remote::publication::{read, request_id, save};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExistingHistory {
    pub operation: String,
    pub commit: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AdoptionRequest {
    pub request_id: String,
    pub source: ExistingHistory,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    version: u32,
    request: AdoptionRequest,
    base: Option<String>,
    candidate: Option<String>,
    result: Option<Checkpoint>,
    #[serde(default)]
    stopped: bool,
}
impl Binding {
    /// Read only the existing JJ workspace's recorded state, never disk edits.
    pub async fn existing_history(&self) -> Result<Option<ExistingHistory>> {
        self.validate()?;
        if self.state == self.root.join(".jj") || !self.root.join(".jj").exists() {
            return Ok(None);
        }
        Ok(Some(source::load(self, None).await?.history))
    }
}
impl Session {
    pub async fn adopt_history(
        &mut self,
        request: &AdoptionRequest,
        exclusions: &[String],
    ) -> Result<Checkpoint> {
        self.adopt_history_with(request, exclusions, |_| Ok(()))
            .await
    }
    pub async fn adopt_history_with(
        &mut self,
        request: &AdoptionRequest,
        exclusions: &[String],
        mut progress: impl FnMut(&str) -> Result<()>,
    ) -> Result<Checkpoint> {
        self.binding.verify_saved()?;
        request_id(&request.request_id)?;
        if self
            .binding
            .state
            .join("adoption-receipts")
            .join(format!("{}.stopped", request.request_id))
            .exists()
        {
            return Err(Error::Jj(
                "This history adoption request was stopped".into(),
            ));
        }
        let path = self
            .binding
            .state
            .join("adoption-receipts")
            .join(format!("{}.json", request.request_id));
        let mut receipt = if path.exists() {
            let receipt: Receipt = read(&path)?;
            if receipt.version != 1 || receipt.request != *request {
                return Err(Error::Binding);
            }
            receipt
        } else {
            if self.binding.pending_adoption()?.is_some() {
                return Err(Error::Busy);
            }
            source::load(&self.binding, Some(&request.source)).await?;
            let receipt = Receipt {
                version: 1,
                request: request.clone(),
                base: None,
                candidate: None,
                result: None,
                stopped: false,
            };
            save(&path, &receipt)?;
            save(
                &self.binding.state.join("adoption-receipts/active.json"),
                &request.request_id,
            )?;
            receipt
        };
        if let Some(result) = receipt.result {
            publish::clear_active(&self.binding, &request.request_id)?;
            return Ok(result);
        }
        if receipt.stopped {
            return Err(Error::Jj(
                "This history adoption request was stopped".into(),
            ));
        }
        progress("prepared")?;
        if receipt.candidate.is_none() {
            let source = source::load(&self.binding, Some(&request.source)).await?;
            self.import_existing_objects(&source.git, &request.source.commit)
                .await?;
            source.verify(&self.binding)?;
            self.capture(exclusions, "Safety checkpoint before history adoption")
                .await?;
            let before = self.checkpoint()?.ok_or(Error::Binding)?;
            let current = self
                .repo
                .store()
                .get_commit(&CommitId::try_from_hex(&before.commit).ok_or(Error::Binding)?)
                .map_err(jj)?;
            let incoming = CommitId::try_from_hex(&request.source.commit).ok_or(Error::Binding)?;
            let retained = self.repo.index().has_id(&incoming).await.map_err(jj)?
                && self
                    .repo
                    .index()
                    .is_ancestor(&incoming, current.id())
                    .await
                    .map_err(jj)?;
            if retained {
                receipt.base = Some(before.commit.clone());
                receipt.candidate = Some(before.commit);
                save(&path, &receipt)?;
                progress("retained")?;
                return self.finish_adoption(&path, receipt, &mut progress).await;
            }
            let original = self.repo.store().get_commit(&incoming).map_err(jj)?;
            let mut tx = self.repo.start_transaction();
            tx.repo_mut().add_head(&original).await.map_err(jj)?;
            let combined = tx
                .repo_mut()
                .new_commit(vec![current.id().clone(), incoming], current.tree())
                .set_description("Import existing repository history")
                .write()
                .await
                .map_err(jj)?;
            receipt.base = Some(before.commit);
            receipt.candidate = Some(combined.id().hex());
            save(&path, &receipt)?;
            progress("retained")?;
        }
        self.finish_adoption(&path, receipt, &mut progress).await
    }
}
