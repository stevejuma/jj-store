//! Physical changes relative to an exact WC tree, not active combination assignment.
use super::*;
use jj_store_jj::local_browser::working_tree as baseline;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Modified,
    Deleted,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    pub path: String,
    pub kind: ChangeKind,
    pub before: Option<baseline::Entry>,
    pub after: Option<Entry>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obstacle {
    pub entry: Entry,
    pub ignored: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection {
    pub physical: WorkingTree,
    pub baseline: baseline::Baseline,
    pub changes: Vec<Change>,
    /// Every untracked file/link, including candidate additions and ignored files.
    pub obstacles: Vec<Obstacle>,
}
/// Trusted caller supplies verified global/exclude bytes, including verified absence.
/// This comparison does not verify current WC metadata, grant tokens or permit checkout.
pub fn project(
    physical: WorkingTree,
    baseline: baseline::Baseline,
    base_ignores: Vec<Vec<u8>>,
    mut guard: impl FnMut() -> Result<()>,
) -> Result<Projection> {
    guard()?;
    if physical.version != 1
        || baseline.version != 1
        || physical.repository != baseline.repository
        || physical.generation != baseline.generation
    {
        return Err(Error::Stale);
    }
    if physical.entries.len() > baseline::MAX_ENTRIES
        || baseline.entries.len() > baseline::MAX_ENTRIES
    {
        return Err(Error::Limit);
    }
    if !ordered(physical.entries.iter().map(|e| e.path.as_str()))
        || !ordered(baseline.entries.iter().map(|e| e.path.as_str()))
    {
        return Err(Error::Invalid);
    }
    let mut ignores = Vec::new();
    for entry in &physical.entries {
        guard()?;
        if entry.path.rsplit('/').next() == Some(".gitignore") {
            if entry.kind == Kind::Symlink {
                return Err(Error::Unsupported);
            }
            let bytes = entry.ignore_bytes.as_ref().ok_or(Error::Invalid)?;
            if bytes.len() as u64 != entry.bytes || baseline::content_digest(bytes) != entry.content
            {
                return Err(Error::Stale);
            }
            ignores.push(baseline::IgnoreFile {
                path: entry.path.clone(),
                bytes: bytes.clone(),
            });
        } else if entry.ignore_bytes.is_some() {
            return Err(Error::Invalid);
        }
    }
    let mut interrupted = None;
    let rules = baseline::IgnoreRules::with_guard(base_ignores, ignores, || {
        guard().map_err(|error| {
            interrupted = Some(error);
            jj_store_jj::local_browser::Error::Canceled
        })
    })
    .map_err(|error| interrupted.unwrap_or_else(|| map(error)))?;
    let mut observed: BTreeMap<_, _> = physical
        .entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect();
    let mut changes = Vec::new();
    for before in &baseline.entries {
        guard()?;
        rules.ignored(&before.path).map_err(map)?; // validate even tracked paths; ignore never removes tracking
        match observed.remove(before.path.as_str()) {
            None => changes.push(Change {
                path: before.path.clone(),
                kind: ChangeKind::Deleted,
                before: Some(before.clone()),
                after: None,
            }),
            Some(after) if equivalent(before, after) => {}
            Some(after) => changes.push(Change {
                path: before.path.clone(),
                kind: ChangeKind::Modified,
                before: Some(before.clone()),
                after: Some(after.clone()),
            }),
        }
    }
    let mut obstacles = Vec::new();
    for after in observed.into_values() {
        guard()?;
        let ignored = rules.ignored(&after.path).map_err(map)?;
        if !ignored {
            changes.push(Change {
                path: after.path.clone(),
                kind: ChangeKind::Added,
                before: None,
                after: Some(after.clone()),
            });
        }
        obstacles.push(Obstacle {
            entry: after.clone(),
            ignored,
        });
    }
    changes.sort_by(|a, b| a.path.cmp(&b.path));
    guard()?;
    Ok(Projection {
        physical,
        baseline,
        changes,
        obstacles,
    })
}
fn ordered<'a>(mut paths: impl Iterator<Item = &'a str>) -> bool {
    let Some(mut previous) = paths.next() else {
        return true;
    };
    for path in paths {
        if path <= previous {
            return false;
        }
        previous = path;
    }
    true
}
fn equivalent(before: &baseline::Entry, after: &Entry) -> bool {
    let kind = match after.kind {
        Kind::File { executable } => baseline::Kind::File { executable },
        Kind::Symlink => baseline::Kind::Symlink,
    };
    before.kind == kind && before.bytes == after.bytes && before.content == after.content
}
fn map(error: jj_store_jj::local_browser::Error) -> Error {
    match error {
        jj_store_jj::local_browser::Error::Limit => Error::Limit,
        _ => Error::Invalid,
    }
}
