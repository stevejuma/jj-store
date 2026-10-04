//! Complete immutable-file proof under independent generation ownership.
use super::*;

pub(super) struct Pending {
    pub files: BTreeMap<String, File>,
    pub upgrade: bool,
}
pub(super) struct Seal {
    pub files: BTreeMap<String, File>,
    verification: Option<resident::Verification>,
}
impl Seal {
    fn capture(path: &Path, files: BTreeMap<String, File>, budget: &Budget) -> Result<Self> {
        let verification = match resident::Verification::capture(path, files.clone(), budget) {
            Ok(value) => Some(value),
            Err(_) => {
                budget.check()?;
                None
            }
        };
        Ok(Self {
            files,
            verification,
        })
    }
    pub fn matches(&self, path: &Path, policy: Policy, budget: &Budget) -> Result<bool> {
        match &self.verification {
            Some(verification) => verification.matches(path, policy, budget),
            // This request holds independent generation ownership and has just
            // completed another full proof before namespace reacquisition.
            None => Ok(same_sizes(
                &inventory(path, policy, budget, false)?.0,
                &self.files,
            )),
        }
    }
}
pub(super) fn same_sizes(
    actual: &BTreeMap<String, File>,
    expected: &BTreeMap<String, File>,
) -> bool {
    actual.len() == expected.len()
        && actual.iter().all(|(name, file)| {
            expected
                .get(name)
                .is_some_and(|expected| expected.bytes == file.bytes)
        })
}
impl Entry {
    /// False permits rebuilding only privately owned disposable damage. Shared
    /// damage cannot become namespace-busy fallback or delete another reader.
    pub fn verify(&mut self, policy: Policy, budget: &Budget) -> Result<bool> {
        let Some(pending) = self.pending.as_ref() else {
            return Ok(true);
        };
        if !self.private() {
            return Err(Error::Unavailable);
        }
        let valid = match inventory(&self.path, policy, budget, true) {
            Ok((files, _)) => files == pending.files,
            Err(Error::Unavailable) => false,
            Err(error) => return Err(error),
        };
        budget.check()?;
        if !valid {
            return if self.pin.is_some() {
                Err(Error::Unavailable)
            } else {
                Ok(false)
            };
        }
        if !pending.upgrade {
            self.sealed = Some(Seal::capture(&self.path, pending.files.clone(), budget)?);
        }
        self.pending = None;
        Ok(true)
    }
    pub fn seal(&mut self, policy: Policy, budget: &Budget) -> Result<()> {
        if !self.private() {
            return Ok(());
        }
        if self.pending.is_some() {
            return Err(Error::Unavailable);
        }
        if self
            .sealed
            .as_ref()
            .is_some_and(|seal| seal.verification.is_some())
        {
            return Ok(());
        }
        // New output and coarse-metadata recovery both hash outside the lock.
        let files = inventory(&self.path, policy, budget, true)?.0;
        if self.sealed.as_ref().is_some_and(|seal| seal.files != files) {
            return Err(Error::Unavailable);
        }
        self.sealed = Some(Seal::capture(&self.path, files, budget)?);
        Ok(())
    }
}
