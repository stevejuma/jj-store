use super::*;
use crate::codex_import::{CodexImportBinding, CodexImportProbe};

pub(super) struct ImportAdmission<'a> {
    pub preparation: &'a CodexImportProbe,
    pub accepted: CodexImportBinding,
    pub consent: &'a mut dyn FnMut() -> CaptureResult<()>,
}
impl ImportAdmission<'_> {
    pub(super) fn revalidate(&mut self, qualified: &QualifiedCodexSource) -> CaptureResult<()> {
        (self.consent)()?;
        if self
            .preparation
            .selected_source(self.accepted)?
            .fingerprint()
            != qualified.fingerprint()
        {
            return Err("Codex import source selection changed".into());
        }
        qualified.revalidate()
    }
}
impl SourceRegistry {
    /// Caller retains durable explicit intent and current policy/native Write gates.
    /// Recheck before allocation, immediately before selection and after it.
    /// Postselection failure is unknown; retry the original intent. This primitive
    /// neither records nor authorizes consent, nor grants ordinary active resume.
    pub fn acquire_import(
        &self,
        preparation: &CodexImportProbe,
        accepted: CodexImportBinding,
        mut consent: impl FnMut() -> CaptureResult<()>,
    ) -> CaptureResult<RegisteredCodexSource> {
        let qualified = preparation.selected_source(accepted)?;
        self.acquire_checked(
            qualified,
            Admission::Import(ImportAdmission {
                preparation,
                accepted,
                consent: &mut consent,
            }),
        )
    }
    /// Read-only import lookup still requires renewed exact-prefix preparation and
    /// retained consent. Generic active lookup cannot borrow an import-only marker.
    pub fn lookup_import_admitted(
        &self,
        preparation: &CodexImportProbe,
        accepted: CodexImportBinding,
        mut consent: impl FnMut() -> CaptureResult<()>,
    ) -> CaptureResult<Option<SourceInstanceId>> {
        let qualified = preparation.selected_source(accepted)?;
        let mut proof = ImportAdmission {
            preparation,
            accepted,
            consent: &mut consent,
        };
        proof.revalidate(&qualified)?;
        let source = self.lookup_marker(&qualified, import_key(&qualified))?;
        proof.revalidate(&qualified)?;
        Ok(source)
    }
}
