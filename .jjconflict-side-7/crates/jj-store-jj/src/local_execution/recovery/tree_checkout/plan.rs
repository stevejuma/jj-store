use super::*;
use crate::local_browser::working_tree::current::{MAX_CHECKOUT_BYTES, capture};
use prost::Message as _;
const MAX_PLAN_BYTES: usize = 32 * 1024 * 1024;
/// Installation-private exact inputs. Decoding this data never grants a write
/// capability or proves publication, physical installation or command success.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeCheckoutPlanV1 {
    version: u32,
    key: StackPreparationKey,
    recovery: m::RecordDigest,
    retained: GitOid,
    archive: m::RecordDigest,
    operation: m::OperationId,
    view: m::ViewId,
    physical: m::RecordDigest,
    before_commit: m::GitSha1Id,
    after_commit: m::GitSha1Id,
    before_tree: m::GitSha1Id,
    after_tree: m::GitSha1Id,
    checkout: Vec<u8>,
    checkout_fingerprint: m::RecordDigest,
    tree_state: Vec<u8>,
    tree_fingerprint: m::RecordDigest,
    pub(super) changes: Vec<Change>,
    pub(super) directories: Vec<Directory>,
}
impl TreeCheckoutPlanV1 {
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.changes.iter().map(|change| change.path.as_str())
    }
    pub fn preparation(&self) -> &StackPreparationKey {
        &self.key
    }
    pub fn recovery(&self) -> m::RecordDigest {
        self.recovery
    }
    pub fn physical(&self) -> m::RecordDigest {
        self.physical
    }
    pub fn candidate(&self) -> (m::OperationId, m::ViewId) {
        (self.operation, self.view)
    }
    pub fn retained(&self) -> (GitOid, m::RecordDigest) {
        (self.retained, self.archive)
    }
    pub fn commits(&self) -> (m::GitSha1Id, m::GitSha1Id) {
        (self.before_commit, self.after_commit)
    }
    pub fn trees(&self) -> (m::GitSha1Id, m::GitSha1Id) {
        (self.before_tree, self.after_tree)
    }
    pub fn workspace(&self) -> Result<String> {
        Ok(decode::checkout(&self.checkout, &self.tree_state)?.workspace)
    }
    pub(super) fn check_metadata(
        &self,
        recovery: &RecoveredHistory,
        budget: &Budget,
    ) -> Result<()> {
        let files =
            capture::CheckoutFiles::read(&recovery.root, recovery.access.metadata_path(), budget)?;
        if files.checkout.bytes != self.checkout
            || files.checkout.fingerprint != self.checkout_fingerprint
            || files.tree.bytes != self.tree_state
            || files.tree.fingerprint != self.tree_fingerprint
        {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_PLAN_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/tree-checkout-plan/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
    pub fn decode(expected: m::RecordDigest, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_PLAN_BYTES {
            return Err(Error::Limit);
        }
        let plan: Self = serde_json::from_slice(bytes).map_err(|_| Error::Invalid)?;
        if plan.encode()? != bytes || plan.id()? != expected {
            return Err(Error::Invalid);
        }
        Ok(plan)
    }
    fn validate(&self) -> Result<()> {
        if self.version != 1
            || self.checkout.len() > 65536
            || self.tree_state.len() > MAX_CHECKOUT_BYTES
            || self.changes.is_empty()
            || self.changes.len() > tree::MAX_ENTRIES
            || self.directories.is_empty()
            || self.directories.len() > tree::MAX_ENTRIES * 64
            || self.before_tree == self.after_tree
            || [
                self.physical,
                self.recovery,
                self.archive,
                self.checkout_fingerprint,
                self.tree_fingerprint,
            ]
            .contains(&m::RecordDigest::new([0; 32]))
        {
            return Err(Error::Invalid);
        }
        let checkout = decode::checkout(&self.checkout, &self.tree_state)?;
        if checkout.operation == self.operation
            || checkout.trees != [self.before_tree]
            || !checkout.labels.is_empty()
            || checkout.sparse != [String::new()]
            || jj_lib::protos::local_working_copy::Checkout::decode(self.checkout.as_slice())
                .map_err(|_| Error::Invalid)?
                .encode_to_vec()
                != self.checkout
            || jj_lib::protos::local_working_copy::TreeState::decode(self.tree_state.as_slice())
                .map_err(|_| Error::Invalid)?
                .encode_to_vec()
                != self.tree_state
        {
            return Err(Error::Invalid);
        }
        let mut total = 0u64;
        let mut previous: Option<&str> = None;
        for change in &self.changes {
            safe_path(&change.path)?;
            if previous.is_some_and(|p| p >= change.path.as_str())
                || change.before == change.after
                || change.before.is_some() != change.observed.is_some()
                || change.observed == Some(m::RecordDigest::new([0; 32]))
            {
                return Err(Error::Invalid);
            }
            for value in [&change.before, &change.after].into_iter().flatten() {
                if value.size > tree::MAX_FILE_BYTES as u64
                    || value.content == m::RecordDigest::new([0; 32])
                {
                    return Err(Error::Invalid);
                }
                total = total.checked_add(value.size).ok_or(Error::Limit)?;
                if total > (2 * tree::MAX_BYTES) as u64 {
                    return Err(Error::Limit);
                }
            }
            previous = Some(&change.path);
        }
        let mut previous: Option<&str> = None;
        for dir in &self.directories {
            if !dir.path.is_empty() {
                safe_path(&dir.path)?;
            }
            if previous.is_some_and(|p| p >= dir.path.as_str())
                || dir.identity == Some(m::RecordDigest::new([0; 32]))
            {
                return Err(Error::Invalid);
            }
            previous = Some(&dir.path);
        }
        if !self.directories[0].path.is_empty() || self.directories[0].identity.is_none() {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
pub(super) fn capture(
    recovery: &RecoveredHistory,
    physical: m::RecordDigest,
    budget: &Budget,
) -> Result<TreeCheckoutPlanV1> {
    let files =
        capture::CheckoutFiles::read(&recovery.root, recovery.access.metadata_path(), budget)?;
    let checkout = decode::checkout(&files.checkout.bytes, &files.tree.bytes)?;
    if checkout.sparse != [String::new()] {
        return Err(Error::Invalid);
    }
    let mut effects = effects::capture(recovery, budget)?;
    let directories = observation::capture(&recovery.root, &mut effects.changes, budget)?;
    let plan = TreeCheckoutPlanV1 {
        version: 1,
        key: recovery.key.clone(),
        recovery: recovery.witness.id()?,
        retained: recovery.retained.commit,
        archive: recovery.retained.archive,
        operation: recovery.archive.operation,
        view: recovery.retained.view,
        physical,
        before_commit: effects.before_commit,
        after_commit: effects.after_commit,
        before_tree: effects.before_tree,
        after_tree: effects.after_tree,
        checkout: files.checkout.bytes,
        checkout_fingerprint: files.checkout.fingerprint,
        tree_state: files.tree.bytes,
        tree_fingerprint: files.tree.fingerprint,
        changes: effects.changes,
        directories,
    };
    plan.encode()?;
    Ok(plan)
}
