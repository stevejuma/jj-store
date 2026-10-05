//! Native unified diff groups with byte-bound selection fingerprints.
use super::*;
use futures::AsyncReadExt as _;
use jj_lib::diff_presentation::{
    LineCompareMode,
    unified::{DiffLineType, unified_diff_hunks},
};
use jj_lib::{
    backend::{MergedTreeValue, TreeValue},
    merge::{Diff, Merge},
    repo::Repo,
    repo_path::RepoPath,
};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalStacksHunkRequest")]
pub struct HunkRequest {
    pub version: u32,
    pub binding_generation: u64,
    pub operation: jj_store_model::OperationId,
    pub revision: u64,
    pub source: String,
    pub path: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksHunk")]
pub struct Hunk {
    pub id: String,
    pub label: String,
    pub patch: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksHunkResponse")]
pub struct HunkResponse {
    pub source: String,
    pub path: String,
    pub hunks: Vec<Hunk>,
    pub reason: Option<String>,
}
struct Part {
    descriptor: Hunk,
    before: std::ops::Range<usize>,
    after: std::ops::Range<usize>,
}
struct Generated {
    before: Vec<u8>,
    after: Vec<u8>,
    parts: Vec<Part>,
}
pub(super) async fn bytes(
    repo: &dyn Repo,
    path: &RepoPath,
    value: &MergedTreeValue,
) -> Result<Option<Vec<u8>>> {
    let Some(value) = value.as_resolved() else {
        return Ok(None);
    };
    let Some(value) = value else {
        return Ok(Some(vec![]));
    };
    let TreeValue::File { id, .. } = value else {
        return Ok(None);
    };
    let max = crate::local_browser::Limits::default().max_text_bytes;
    let mut contents = vec![];
    repo.store()
        .read_file(path, id)
        .await
        .map_err(jj)?
        .take(max as u64 + 1)
        .read_to_end(&mut contents)
        .await
        .map_err(jj)?;
    if contents.len() > max
        || contents.iter().take(8000).any(|b| *b == 0)
        || std::str::from_utf8(&contents).is_err()
    {
        return Ok(None);
    }
    Ok(Some(contents))
}
async fn generate(
    repo: &dyn Repo,
    source: &Commit,
    path: &RepoPath,
    before: &MergedTreeValue,
    after: &MergedTreeValue,
) -> Result<Option<Generated>> {
    let mode = |value: &MergedTreeValue| match value.as_resolved() {
        Some(Some(TreeValue::File { executable, .. })) => Some(*executable),
        _ => None,
    };
    if mode(before)
        .zip(mode(after))
        .is_some_and(|(old, new)| old != new)
    {
        return Ok(None);
    }
    let Some(old) = bytes(repo, path, before).await? else {
        return Ok(None);
    };
    let Some(new) = bytes(repo, path, after).await? else {
        return Ok(None);
    };
    let lines = |bytes: &[u8]| bytes.iter().filter(|b| **b == b'\n').count() + 1;
    if lines(&old).checked_mul(lines(&new)).unwrap_or(usize::MAX)
        > crate::local_browser::Limits::default().max_diff_cells
    {
        return Ok(None);
    }
    let groups = unified_diff_hunks(
        Diff::new(old.as_slice().into(), new.as_slice().into()),
        3,
        LineCompareMode::Exact,
    );
    if groups.len() > 500 {
        return Ok(None);
    }
    let mut parts = vec![];
    for (index, hunk) in groups.into_iter().enumerate() {
        let label = format!(
            "@@ -{},{} +{},{} @@",
            hunk.left_line_range.start + usize::from(!hunk.left_line_range.is_empty()),
            hunk.left_line_range.len(),
            hunk.right_line_range.start + usize::from(!hunk.right_line_range.is_empty()),
            hunk.right_line_range.len()
        );
        let mut patch = format!(
            "--- a/{}\n+++ b/{}\n{label}\n",
            path.as_internal_file_string(),
            path.as_internal_file_string()
        );
        for (kind, tokens) in &hunk.lines {
            patch.push(match kind {
                DiffLineType::Context => ' ',
                DiffLineType::Removed => '-',
                DiffLineType::Added => '+',
            });
            for (_, text) in tokens {
                patch.push_str(std::str::from_utf8(text).map_err(jj)?);
            }
            if !patch.ends_with('\n') {
                patch.push_str("\n\\ No newline at end of file\n");
            }
        }
        let mut hash = Sha256::new();
        hash.update(b"jj-store/local-hunk/1\0");
        hash.update(source.change_id().as_bytes());
        for bytes in [
            path.as_internal_file_string().as_bytes(),
            old.as_slice(),
            new.as_slice(),
        ] {
            hash.update((bytes.len() as u64).to_le_bytes());
            hash.update(bytes);
        }
        hash.update((index as u64).to_le_bytes());
        parts.push(Part {
            descriptor: Hunk {
                id: format!("native:{:x}", hash.finalize()),
                label,
                patch,
            },
            before: hunk.left_line_range,
            after: hunk.right_line_range,
        });
    }
    Ok(Some(Generated {
        before: old,
        after: new,
        parts,
    }))
}
pub async fn hunks(
    access: Arc<dyn Access>,
    root: PathBuf,
    request: HunkRequest,
) -> Result<HunkResponse> {
    let (_, repo, _, _) = preview::load(
        &access,
        &root,
        request.version,
        request.binding_generation,
        Some(request.operation),
        Some(request.revision),
    )
    .await?;
    let source = policy::commit(&repo, &request.source)?;
    let path = mutations::path(&request.path)?;
    if source.parent_ids().len() != 1 {
        return Err(Error::Invalid);
    }
    let before = source
        .parent_tree(repo.as_ref())
        .await
        .map_err(jj)?
        .path_value(&path)
        .await
        .map_err(jj)?;
    let after = source.tree().path_value(&path).await.map_err(jj)?;
    let generated = generate(repo.as_ref(), &source, &path, &before, &after).await?;
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(HunkResponse { source:request.source,path:request.path,reason:generated.is_none().then(||"Partial selection is unavailable for this file type, metadata change, conflict or text/diff limit. Use supported whole-file changes or resolve its conflict first.".into()),hunks:generated.map(|g|g.parts.into_iter().map(|p|p.descriptor).collect()).unwrap_or_default() })
}
pub(super) async fn selected_value(
    repo: &dyn Repo,
    source: &Commit,
    path: &RepoPath,
    before: &MergedTreeValue,
    after: &MergedTreeValue,
    selected: &[String],
) -> Result<MergedTreeValue> {
    let data = generate(repo, source, path, before, after)
        .await?
        .ok_or(Error::Invalid)?;
    let selected = selected.iter().collect::<std::collections::BTreeSet<_>>();
    if selected.is_empty()
        || selected.len() > data.parts.len()
        || selected
            .iter()
            .any(|id| !data.parts.iter().any(|p| &p.descriptor.id == *id))
    {
        return Err(Error::Invalid);
    }
    if selected.len() == data.parts.len() {
        return Ok(after.clone());
    }
    let old = data
        .before
        .split_inclusive(|b| *b == b'\n')
        .collect::<Vec<_>>();
    let new = data
        .after
        .split_inclusive(|b| *b == b'\n')
        .collect::<Vec<_>>();
    let mut content = vec![];
    let mut offset = 0;
    for part in &data.parts {
        if !selected.contains(&part.descriptor.id) {
            continue;
        }
        for line in &old[offset..part.before.start] {
            content.extend_from_slice(line);
        }
        for line in &new[part.after.clone()] {
            content.extend_from_slice(line);
        }
        offset = part.before.end;
    }
    for line in &old[offset..] {
        content.extend_from_slice(line);
    }
    let mut value = after
        .as_resolved()
        .and_then(Option::as_ref)
        .or_else(|| before.as_resolved().and_then(Option::as_ref))
        .cloned()
        .ok_or(Error::Invalid)?;
    let TreeValue::File { id, .. } = &mut value else {
        return Err(Error::Invalid);
    };
    *id = repo
        .store()
        .write_file(path, &mut content.as_slice())
        .await
        .map_err(jj)?;
    Ok(Merge::resolved(Some(value)))
}
