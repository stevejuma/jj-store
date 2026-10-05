//! Device-memory directory context; no path persistence or diagnostic rendering.
use super::MAX_RECORD_BYTES;
use serde_json::Value;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, PartialEq, Eq)]
pub enum CodexDirectoryContext {
    Known(PathBuf),
    Unavailable,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eof_compaction_releases_partial_allocation_and_keeps_complete_context() {
        let mut context = ContextReconstruction::new(Path::new("/workspace/a"));
        context.feed(b"{}\n{\"type\":\"turn_context\",\"payload\":{\"cwd\":\"/workspace/b\"}}\n");
        context.feed(&vec![b'x'; 900 * 1024]);
        assert!(context.line.capacity() >= 900 * 1024);
        context.release_eof_bytes();
        assert_eq!(context.line.capacity(), 0);
        assert_eq!(
            context.history.context.directory(),
            Some(Path::new("/workspace/b"))
        );
    }
}
impl CodexDirectoryContext {
    pub fn directory(&self) -> Option<&Path> {
        match self {
            Self::Known(path) => Some(path),
            Self::Unavailable => None,
        }
    }
}
#[derive(Clone)]
pub(crate) struct DirectoryHistory {
    pub(crate) context: CodexDirectoryContext,
    first: bool,
    rejected_session: bool,
}
impl DirectoryHistory {
    pub(crate) fn new(initial: &Path) -> Self {
        Self {
            context: directory(initial.to_str()).map_or(
                CodexDirectoryContext::Unavailable,
                CodexDirectoryContext::Known,
            ),
            first: true,
            rejected_session: false,
        }
    }
    pub(crate) fn observe(&mut self, bytes: Option<&[u8]>) {
        // The file inspector has already qualified the exact first header.
        if self.first {
            self.first = false;
            return;
        }
        if self.rejected_session {
            return;
        }
        let Some(native) = bytes
            .and_then(|v| serde_json::from_slice::<Value>(v).ok())
            .filter(Value::is_object)
        else {
            self.context = CodexDirectoryContext::Unavailable;
            return;
        };
        match native["type"].as_str() {
            Some("turn_context") => {
                self.context = directory(native["payload"]["cwd"].as_str()).map_or(
                    CodexDirectoryContext::Unavailable,
                    CodexDirectoryContext::Known,
                );
            }
            // A later header cannot silently change this generation's native session.
            Some("session_meta") => {
                self.context = CodexDirectoryContext::Unavailable;
                self.rejected_session = true;
            }
            _ => {}
        }
    }
}
fn directory(text: Option<&str>) -> Option<PathBuf> {
    let text = text?;
    let path = Path::new(text);
    if !path.is_absolute()
        || path.parent().is_none()
        || text.len() > 4096
        || text.chars().any(char::is_control)
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
    {
        return None;
    }
    Some(path.to_owned())
}
pub(crate) struct ContextReconstruction {
    pub(crate) history: DirectoryHistory,
    line: Vec<u8>,
    oversized: bool,
}
impl ContextReconstruction {
    pub(crate) fn release_eof_bytes(&mut self) {
        self.line = Vec::new();
    }
    pub(crate) fn new(initial: &Path) -> Self {
        Self {
            history: DirectoryHistory::new(initial),
            line: Vec::new(),
            oversized: false,
        }
    }
    pub(super) fn feed(&mut self, bytes: &[u8]) {
        self.feed_observed(bytes, |_| {});
    }
    pub(crate) fn feed_observed(
        &mut self,
        mut bytes: &[u8],
        mut observe: impl FnMut(&CodexDirectoryContext),
    ) {
        while !bytes.is_empty() {
            let length = bytes
                .iter()
                .position(|b| *b == b'\n')
                .map_or(bytes.len(), |i| i + 1);
            let part = &bytes[..length];
            if !self.oversized {
                if self.line.len() + part.len() > MAX_RECORD_BYTES {
                    self.line.clear();
                    self.oversized = true;
                } else {
                    self.line.extend_from_slice(part);
                }
            }
            if part.last() == Some(&b'\n') {
                self.history.observe(if self.oversized {
                    None
                } else {
                    Some(&self.line)
                });
                observe(&self.history.context);
                self.line.clear();
                self.oversized = false;
            }
            bytes = &bytes[length..];
        }
    }
    pub(crate) fn complete(&self) -> bool {
        self.line.is_empty() && !self.oversized
    }
}
