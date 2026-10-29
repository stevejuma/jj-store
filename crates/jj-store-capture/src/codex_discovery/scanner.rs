use super::{ApprovedCodexRoot, CaptureResult};
use lapis_ai_capture::discovery::CodexDirectoryScanner as SourceScanner;
pub use lapis_ai_capture::discovery::CodexDiscoveryStep;

/// Keeps JJ's Cursor sidecar qualification around the generic metadata walker.
pub struct CodexDirectoryScanner(SourceScanner);
impl CodexDirectoryScanner {
    pub fn new(root: ApprovedCodexRoot) -> CaptureResult<Self> {
        SourceScanner::new(root).map(Self)
    }
    /// Cursor candidates require the JJ binding sidecar before header admission.
    pub fn cursor(root: ApprovedCodexRoot) -> CaptureResult<Self> {
        SourceScanner::cursor_with_hint_extension(root, "jj-store.json").map(Self)
    }
    pub fn restart(&mut self) -> CaptureResult<()> {
        self.0.restart()
    }
    pub fn step(
        &mut self,
        max_entries: usize,
        max_candidates: usize,
    ) -> CaptureResult<CodexDiscoveryStep> {
        self.0.step(max_entries, max_candidates)
    }
}
