//! Readonly filter data owns a shared pin on its complete disk generation.
use super::*;
use crate::query::{cache::generation::Pin, prepared::filters};
use std::path::Path;

pub(super) struct Filter {
    pub reader: filters::Reader,
    // Field order closes the database before releasing generation ownership.
    _pin: Arc<Pin>,
}
impl Filter {
    pub fn load(
        path: &Path,
        generation: &Path,
        selected: m::OperationId,
        maximum: usize,
        budget: &Budget,
    ) -> Result<Option<Arc<Self>>> {
        if !path
            .join(filters::NAME)
            .try_exists()
            .map_err(|_| Error::Unavailable)?
        {
            return Ok(None);
        }
        budget.work(1)?;
        let pin = Arc::new(Pin::acquire(generation)?);
        let reader = filters::Reader::load(
            path,
            selected,
            maximum,
            |amount| budget.work(amount),
            |bytes| budget.read_bytes(bytes),
        )?;
        Ok(Some(Arc::new(Self { reader, _pin: pin })))
    }
}
