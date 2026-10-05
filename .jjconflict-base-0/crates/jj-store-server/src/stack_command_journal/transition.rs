use super::*;

pub(super) fn unresolved(state: &s::ReceiptState) -> bool {
    matches!(
        state,
        s::ReceiptState::PublicationUnknown
            | s::ReceiptState::Published {
                checkout: s::CheckoutState::Required | s::CheckoutState::Blocked { .. },
                ..
            }
    )
}
fn allowed(before: &s::ReceiptState, after: &s::ReceiptState) -> bool {
    use s::ReceiptState as R;
    match (before, after) {
        (R::Prepared, R::PublicationUnknown | R::Rejected { .. }) => true,
        (
            R::PublicationUnknown,
            R::Published {
                checkout: s::CheckoutState::Required,
                ..
            }
            | R::Retired { .. },
        ) => true,
        (R::Published { checkout, .. }, R::Published { .. }) => {
            !matches!(checkout, s::CheckoutState::Verified { .. })
        }
        (R::Published { .. }, R::Retired { .. }) => true,
        _ => false,
    }
}
impl StackCommandJournal {
    /// Caller-supplied states must come from authenticated provider evidence.
    /// This selects metadata only; upstream heads and checkout are separate.
    /// PublicationUnknown must be durable before invoking upstream publication.
    pub fn transition(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        expected: &s::ReceiptState,
        next: &s::ReceiptState,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = catalog
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        apply
            .validate(&record.preview.plan.request, record.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        let mut selected = record.clone();
        selected.receipt.state = next.clone();
        selected.validate(&self.scope)?;
        if record.receipt.state == *next {
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        if record.receipt.state != *expected {
            return Err(Error::StaleState);
        }
        if !allowed(expected, next) {
            return Err(Error::InvalidTransition);
        }
        if unresolved(next)
            && catalog
                .records
                .iter()
                .any(|r| r.receipt.request_id != apply.request_id && unresolved(&r.receipt.state))
        {
            return Err(Error::Pending);
        }
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        catalog.records[index] = selected.clone();
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(Selection::from_record(&selected, false))
    }
}
