//! Validated owned projections. Never serialize Record's private execution.
use super::*;

fn contract(error: s::Error) -> Error {
    match error {
        s::Error::Limit => Error::Capacity,
        s::Error::IntentMismatch => Error::IntentMismatch,
        _ => Error::Invalid,
    }
}
impl Scope {
    fn check_recovery(&self, scope: &s::RecoveryScope) -> Result<()> {
        scope.validate().map_err(contract)?;
        let profile = match self.profile {
            Profile::Native => s::CodeProfile::Native,
            Profile::GitSha1Jj045 => s::CodeProfile::GitSha1Jj045,
        };
        if scope.repository != self.repository
            || scope.physical_workspace != self.physical_workspace
            || scope.jj_workspace != self.jj_workspace
            || scope.profile != profile
        {
            return Err(Error::ScopeMismatch);
        }
        Ok(())
    }
}
impl Selection {
    /// Public preview/receipt only. Principal, archive, native progress and
    /// retention references remain exclusively in the installation journal.
    pub fn public_result(&self) -> Result<s::CommandResult> {
        let result = s::CommandResult {
            version: s::VERSION,
            preview: self.preview.clone(),
            receipt: self.receipt.clone(),
            replayed: self.replayed,
        };
        result.validate().map_err(|error| match error {
            s::Error::Limit => Error::Capacity,
            _ => Error::Invalid,
        })?;
        Ok(result)
    }
}
impl StackCommandJournal {
    /// Exact historical status, independent of live generations. Runtime
    /// composition must separately admit the principal's registered folder.
    pub fn public_status(
        &self,
        principal: m::PrincipalId,
        request: &s::StatusRequest,
    ) -> Result<s::CommandStatus> {
        request.validate().map_err(contract)?;
        self.scope.check_recovery(&request.scope)?;
        let original = self.status(principal, request.request_id, request.intent_digest)?;
        let result = s::CommandStatus {
            version: s::VERSION,
            request: request.clone(),
            result: original
                .map(|record| Selection::from_record(&record, true).public_result())
                .transpose()?,
        };
        result.validate().map_err(contract)?;
        Ok(result)
    }
    /// Complete actor-scoped bounded discovery; never creates private state or
    /// reconstructs a command. Terminal outcomes remain exact-status only.
    pub fn public_pending(
        &self,
        principal: m::PrincipalId,
        request: &s::PendingRequest,
    ) -> Result<s::PendingCommands> {
        request.validate().map_err(contract)?;
        self.scope.check_recovery(&request.scope)?;
        let commands = self
            .pending(principal)?
            .iter()
            .map(|record| Selection::from_record(record, true).public_result())
            .collect::<Result<Vec<_>>>()?;
        let result = s::PendingCommands {
            version: s::VERSION,
            request: request.clone(),
            commands,
        };
        result.validate().map_err(contract)?;
        Ok(result)
    }
}
