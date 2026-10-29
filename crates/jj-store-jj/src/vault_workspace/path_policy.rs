//! Portable backup content, separate from hidden/private application runtime state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackupPath {
    Content,
    Transcript,
    Ancestor,
    Excluded,
}
impl BackupPath {
    pub fn file(self) -> bool {
        matches!(self, Self::Content | Self::Transcript)
    }
}
pub fn backup_path(path: &str) -> BackupPath {
    use BackupPath::*;
    if path.is_empty() || path.len() > 4096 || path.contains('\\') {
        return Excluded;
    }
    let parts: Vec<_> = path.split('/').collect();
    if parts
        .iter()
        .any(|v| matches!(*v, "" | "." | ".." | ".git" | ".jj" | ".DS_Store"))
    {
        return Excluded;
    }
    let Some(at) = parts.iter().position(|v| *v == ".lapis") else {
        return Content;
    };
    let tail = &parts[at + 1..];
    if tail.is_empty() {
        return Ancestor;
    }
    if tail[0] != "agents" {
        return Excluded;
    }
    if tail.len() == 1 {
        return Ancestor;
    }
    if tail[1] != "sessions" {
        return Excluded;
    }
    if tail.len() == 2 {
        return Ancestor;
    }
    let id = tail[2].as_bytes();
    if id.len() != 36
        || !id.iter().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                *b == b'-'
            } else {
                b.is_ascii_digit() || (b'a'..=b'f').contains(b)
            }
        })
        || id[14] != b'4'
        || !b"89ab".contains(&id[19])
    {
        return Excluded;
    }
    if tail.len() == 3 {
        return Ancestor;
    }
    if tail.len() == 4
        && matches!(
            tail[3],
            "metadata.yaml" | "agents.jsonl" | "transcript.jsonl"
        )
    {
        Transcript
    } else {
        Excluded
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn portable_files_and_only_their_ancestors_are_admitted() {
        let id = "7297795f-c00b-475e-b430-5b0e2e11a2e3";
        for scope in ["", "Notes/"] {
            let prefix = format!("{scope}.lapis/agents/sessions/{id}");
            assert_eq!(backup_path(&prefix), BackupPath::Ancestor);
            for file in ["metadata.yaml", "agents.jsonl", "transcript.jsonl"] {
                assert_eq!(
                    backup_path(&format!("{prefix}/{file}")),
                    BackupPath::Transcript
                );
            }
            for file in ["secret", "transcript.jsonl.tmp", "nested/transcript.jsonl"] {
                assert_eq!(
                    backup_path(&format!("{prefix}/{file}")),
                    BackupPath::Excluded
                );
            }
        }
        for path in [
            ".lapis/cache",
            ".lapis/plugins/ai/data.json",
            ".git/config",
            "nested/.jj/repo",
            ".lapis/agents/sessions/not-a-uuid/transcript.jsonl",
            "../Note.md",
            "/Note.md",
        ] {
            assert_eq!(backup_path(path), BackupPath::Excluded, "{path}");
        }
        assert_eq!(backup_path("Notes/normal.md"), BackupPath::Content);
    }
}
