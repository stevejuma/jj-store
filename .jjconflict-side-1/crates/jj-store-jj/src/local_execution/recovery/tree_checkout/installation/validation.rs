use super::*;
use std::{collections::BTreeSet, fs::File};
pub(super) fn effects(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    budget: &Budget,
) -> Result<()> {
    plan.check_metadata(recovery, budget)?;
    let effects = effects::capture(recovery, budget)?;
    let mut changes = plan.changes.clone();
    for change in &mut changes {
        change.observed = None;
    }
    if effects.changes != changes
        || plan.commits() != (effects.before_commit, effects.after_commit)
        || plan.trees() != (effects.before_tree, effects.after_tree)
    {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
fn target_file(root: &File, path: &str, budget: &Budget) -> Result<Option<Snapshot>> {
    parent(root, path)?
        .map(|(dir, name)| files::read(&dir, &name, budget))
        .transpose()
        .map(|v| v.flatten().map(|v| v.0))
}
fn is_file(actual: &Option<Snapshot>, expected: &Snapshot, moved: bool) -> bool {
    actual
        .as_ref()
        .is_some_and(|actual| expected.matches(actual, moved))
}
/// True means exact saved intent moved; false means exact unchanged operands.
/// Every other combination is a changed/unsafe state, never a guessed success.
pub(super) fn pending(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    state: &Progress,
    destination: &Destination,
    budget: &Budget,
) -> Result<bool> {
    let Some(intent) = state.intent else {
        return Ok(false);
    };
    let root = physical::directory(&recovery.root)?;
    let pending_path = match intent {
        Intent::Directory(i) => &plan.directories[i].path,
        Intent::Backup(i) | Intent::Install(i) => &plan.changes[i].path,
    };
    files::checked_parent(&root, pending_path, plan, state)?.ok_or(Error::EpochChanged)?;
    let (before, after) = match intent {
        Intent::Directory(i) => {
            let stage = records::destination::optional(&destination.directory, &format!("d-{i}"))?;
            let target = files::checked_parent(&root, &plan.directories[i].path, plan, state)?
                .map(|(dir, name)| records::destination::optional(&dir, &name))
                .transpose()?
                .flatten();
            let expected = state.directories[i].ok_or(Error::Corrupt)?;
            let exact = |dir: Option<&File>, moved: bool| -> Result<bool> {
                dir.map(|dir| {
                    let actual = directory_stamp(dir)?;
                    Ok((if moved {
                        actual[..6] == expected[..6]
                    } else {
                        actual == expected
                    }) && files::entries(dir, 1)?.is_empty())
                })
                .transpose()
                .map(|v| v.unwrap_or(false))
            };
            (
                exact(stage.as_ref(), false)? && target.is_none(),
                stage.is_none() && exact(target.as_ref(), true)?,
            )
        }
        Intent::Backup(i) => {
            let source = target_file(&root, &plan.changes[i].path, budget)?;
            let backup =
                files::read(&destination.directory, &format!("b-{i}"), budget)?.map(|v| v.0);
            let expected = state.originals[i].as_ref().ok_or(Error::Corrupt)?;
            (
                is_file(&source, expected, false) && backup.is_none(),
                source.is_none() && is_file(&backup, expected, true),
            )
        }
        Intent::Install(i) => {
            let source =
                files::read(&destination.directory, &format!("f-{i}"), budget)?.map(|v| v.0);
            let target = target_file(&root, &plan.changes[i].path, budget)?;
            let expected = state.candidates[i].as_ref().ok_or(Error::Corrupt)?;
            (
                is_file(&source, expected, false) && target.is_none(),
                source.is_none() && is_file(&target, expected, true),
            )
        }
    };
    match (before, after) {
        (true, false) => Ok(false),
        (false, true) => Ok(true),
        _ => Err(Error::EpochChanged),
    }
}
fn parent_path(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}
pub(super) fn namespace(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    inventory: &PhysicalInventoryV1,
    state: &Progress,
    destination: &Destination,
    current: &PhysicalInventoryV1,
    budget: &Budget,
) -> Result<()> {
    let root = physical::directory(&recovery.root)?;
    let mut expected_files: BTreeMap<_, _> = inventory
        .files()
        .map(|f| (f.path.clone(), f.fingerprint))
        .collect();
    let mut expected_dirs: BTreeMap<_, _> = inventory
        .directories()
        .map(|d| (d.path.clone(), d.stamp))
        .collect();
    let mut touched = BTreeSet::new();
    let mut private = BTreeSet::from(["progress".to_owned()]);
    for (i, dir) in plan.directories.iter().enumerate() {
        if let Some(stamp) = state.directories[i] {
            if i < state.dirs_done {
                expected_dirs.insert(dir.path.clone(), stamp);
                touched.insert(parent_path(&dir.path).to_owned());
                touched.insert(dir.path.clone());
            } else {
                let name = format!("d-{i}");
                let staged = physical::child(&destination.directory, &name)?;
                if directory_stamp(&staged)? != stamp || !files::entries(&staged, 1)?.is_empty() {
                    return Err(Error::EpochChanged);
                }
                private.insert(name);
            }
        } else {
            let stamp = expected_dirs.get(&dir.path).ok_or(Error::Corrupt)?;
            let mut hash = Sha256::new_with_prefix(b"jj-store/tree-checkout-directory/v1\0");
            hash.update(stamp[0].to_be_bytes());
            hash.update(stamp[1].to_be_bytes());
            if dir.identity != Some(m::RecordDigest::new(hash.finalize().into())) {
                return Err(Error::Corrupt);
            }
            if stamp[0] != state.private[0] {
                return Err(Error::Invalid);
            }
        }
    }
    for (i, change) in plan.changes.iter().enumerate() {
        let phase = state.phases[i];
        if phase > 0 {
            touched.insert(parent_path(&change.path).to_owned());
        }
        let original = state.originals[i].as_ref();
        let candidate = state.candidates[i].as_ref();
        if let Some(original) = original
            && phase > 0
        {
            let name = format!("b-{i}");
            let actual = files::read(&destination.directory, &name, budget)?.map(|v| v.0);
            if !is_file(&actual, original, true) {
                return Err(Error::EpochChanged);
            }
            private.insert(name);
        }
        if let Some(candidate) = candidate
            && phase < 2
        {
            let name = format!("f-{i}");
            let actual = files::read(&destination.directory, &name, budget)?.map(|v| v.0);
            if !is_file(&actual, candidate, false) {
                return Err(Error::EpochChanged);
            }
            private.insert(name);
        }
        let expected = if phase == 0 {
            original
        } else if phase == 2 {
            candidate
        } else {
            None
        };
        let actual = target_file(&root, &change.path, budget)?;
        match (expected, actual) {
            (Some(expected), Some(actual)) if expected.matches(&actual, phase == 2) => {
                expected_files.insert(change.path.clone(), actual.fingerprint());
            }
            (None, None) => {
                expected_files.remove(&change.path);
            }
            _ => return Err(Error::EpochChanged),
        }
    }
    let actual_files: BTreeMap<_, _> = current
        .files()
        .map(|f| (f.path.clone(), f.fingerprint))
        .collect();
    if expected_files != actual_files {
        return Err(Error::EpochChanged);
    }
    let actual_dirs: BTreeMap<_, _> = current
        .directories()
        .map(|d| (d.path.clone(), d.stamp))
        .collect();
    if expected_dirs.len() != actual_dirs.len() {
        return Err(Error::EpochChanged);
    }
    for (path, expected) in expected_dirs {
        let actual = actual_dirs.get(&path).ok_or(Error::EpochChanged)?;
        if if touched.contains(&path) {
            expected[..2] != actual[..2] || expected[3] != actual[3]
        } else {
            expected != *actual
        } {
            return Err(Error::EpochChanged);
        }
    }
    let entries = files::entries(
        &destination.directory,
        2 * plan.changes.len() + plan.directories.len() + 2,
    )?;
    for entry in entries {
        if entry != "next" && !private.remove(&entry) {
            return Err(Error::EpochChanged);
        }
    }
    if !private.is_empty() {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
