//! Selected catalog projection only. Proposed after-state and retention stay private.
use super::*;

impl CombinationStore {
    /// Reads existing authenticated state. Registered runtime must separately
    /// join this catalog with current physical observation and journal recovery.
    pub fn public_catalog(
        &self,
        principal: m::PrincipalId,
        scope: &s::RecoveryScope,
    ) -> Result<s::CombinationCatalog> {
        scope.validate().map_err(|_| Error::Invalid)?;
        let profile = match self.scope.profile {
            j::Profile::Native => s::CodeProfile::Native,
            j::Profile::GitSha1Jj045 => s::CodeProfile::GitSha1Jj045,
        };
        if principal != self.principal
            || scope.repository != self.scope.repository
            || scope.physical_workspace != self.scope.physical_workspace
            || scope.jj_workspace != self.scope.jj_workspace
            || scope.profile != profile
        {
            return Err(Error::Forbidden);
        }
        let gate = self.root.gate()?;
        let (catalog, _) = self.read(&gate)?;
        let selected = &catalog.selected;
        let result = s::CombinationCatalog {
            version: s::VERSION,
            scope: scope.clone(),
            revision: selected.revision,
            active: selected.active,
            pending: catalog.records.iter().any(|r| r.phase == Phase::Pending),
            combinations: selected
                .combinations
                .iter()
                .map(|c| s::CombinationSummary {
                    selection: c.selection.clone(),
                    name: c.name.clone(),
                    target: c.target.clone(),
                    merge: s::CommitIdentity::GitSha1(c.merge),
                    code: c.code.clone(),
                    stacks: c
                        .stacks
                        .iter()
                        .map(|stack| s::AppliedStackSummary {
                            id: stack.id,
                            bookmark: stack.bookmark.clone(),
                            head: s::CommitIdentity::GitSha1(stack.head),
                            base: s::CommitIdentity::GitSha1(stack.base),
                        })
                        .collect(),
                })
                .collect(),
        };
        result.validate().map_err(|error| match error {
            s::Error::Limit => Error::Capacity,
            _ => Error::Invalid,
        })?;
        Ok(result)
    }
}
