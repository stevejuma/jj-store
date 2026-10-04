use super::*;
use std::io::Write as _;
impl Root {
    pub fn sync_selected(&self, gate: &Gate, selected: Stamp) -> Result<()> {
        let sync = || -> Result<()> {
            self.check_selected(gate, Some(selected))?;
            unavailable(File::open(self.physical.join(STATE)).and_then(|f| f.sync_all()))?;
            unavailable(File::open(&self.physical).and_then(|f| f.sync_all()))?;
            self.check_selected(gate, Some(selected))
        };
        sync().map_err(|_| Error::OutcomeUnknown)
    }
    pub fn persist(&self, gate: &Gate, bytes: &[u8], expected: Option<Stamp>) -> Result<()> {
        self.persist_with(gate, bytes, expected, || Ok(()))
    }
    pub(crate) fn persist_with(
        &self,
        gate: &Gate,
        bytes: &[u8],
        expected: Option<Stamp>,
        after_replace: impl FnOnce() -> Result<()>,
    ) -> Result<()> {
        if bytes.len() > self.layout.max_bytes {
            return Err(Error::Capacity);
        }
        self.check_selected(gate, expected)?;
        for staged in self.inventory()? {
            unavailable(fs::remove_file(staged))?;
        }
        let mut temp = unavailable(
            tempfile::Builder::new()
                .prefix(self.layout.temporary)
                .tempfile_in(&self.physical),
        )?;
        self.private(&unavailable(temp.as_file().metadata())?, false)?;
        unavailable(temp.write_all(bytes))?;
        unavailable(temp.as_file().sync_all())?;
        self.check_selected(gate, expected)?;
        let published = if expected.is_some() {
            temp.persist(self.physical.join(STATE))
        } else {
            temp.persist_noclobber(self.physical.join(STATE))
        };
        published.map_err(|_| Error::OutcomeUnknown)?;
        after_replace().map_err(|_| Error::OutcomeUnknown)?;
        unavailable(File::open(&self.physical).and_then(|f| f.sync_all()))
            .map_err(|_| Error::OutcomeUnknown)?;
        let (selected, _) = self.read(gate).map_err(|_| Error::OutcomeUnknown)?;
        if selected != bytes {
            return Err(Error::OutcomeUnknown);
        }
        Ok(())
    }
}
