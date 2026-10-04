use super::{
    BatchUpdate, Candidate, Domain, DomainUpdate, Error, GitOid, MetadataStore, Preparation,
    Receipt, Result, RootV1, Selection, Snapshot, Update, objects,
};
use gix::objs::Write as _;
use std::collections::BTreeMap;

impl MetadataStore {
    /// Prepare immutable bytes without making them discoverable. A domain CAS
    /// race is resolved by reloading and preparing again, never replacing siblings.
    pub fn prepare(&self, base: Option<&Snapshot>, update: &Update) -> Result<Preparation> {
        self.prepare_many(
            base,
            &BatchUpdate {
                repository: update.repository,
                actor: update.actor,
                request: update.request,
                intent: update.intent,
                domains: BTreeMap::from([(
                    update.domain,
                    DomainUpdate {
                        expected: update.expected.clone(),
                        bytes: update.bytes.clone(),
                    },
                )]),
                result: update.result.clone(),
                retained: update.retained.clone(),
            },
        )
    }
    pub fn prepare_many(
        &self,
        base: Option<&Snapshot>,
        update: &BatchUpdate,
    ) -> Result<Preparation> {
        if update.domains.is_empty() || update.domains.len() > 5 {
            return Err(Error::Invalid("batch domains"));
        }
        if let Some(base) = base {
            if base.repository() != update.repository {
                return Err(Error::Invalid("repository binding"));
            }
            if let Some(receipt) = base.receipt(update.actor, update.request) {
                if receipt.intent != update.intent {
                    return Err(Error::IntentConflict);
                }
                return Ok(Preparation::Committed(receipt.clone()));
            }
        }
        for (&domain, change) in &update.domains {
            if base.and_then(|s| s.selection(domain)) != change.expected.as_ref() {
                return Err(Error::Stale);
            }
            if change.bytes.len() > self.limits.object_bytes {
                return Err(Error::Limit);
            }
        }
        if update.result.len() > 1024 * 1024 {
            return Err(Error::Limit);
        }
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        let mut root = base.map(|s| s.root.clone()).unwrap_or_else(|| RootV1 {
            version: 1,
            repository: update.repository,
            sequence: 0,
            previous: None,
            domains: BTreeMap::new(),
            receipts: BTreeMap::new(),
            retained: BTreeMap::new(),
        });
        root.sequence = root.sequence.checked_add(1).ok_or(Error::Limit)?;
        root.previous = base.map(|s| s.commit);
        if update.domains.len() > 1
            || update
                .domains
                .keys()
                .any(|d| matches!(d, Domain::Thread | Domain::Reaction))
        {
            root.version = 2;
        }
        let mut selections = BTreeMap::new();
        for (&domain, change) in &update.domains {
            let selection = Selection {
                object: objects::blob(&repo, &change.bytes)?,
                version: change
                    .expected
                    .as_ref()
                    .map_or(Ok(1), |s| s.version.checked_add(1).ok_or(Error::Limit))?,
            };
            root.domains.insert(domain, selection.clone());
            selections.insert(domain, selection);
        }
        let (&primary, selection) = selections
            .first_key_value()
            .ok_or(Error::Invalid("empty batch"))?;
        let selection = selection.clone();
        for (&id, &kind) in &update.retained {
            if root
                .retained
                .insert(id, kind)
                .is_some_and(|old| old != kind)
            {
                return Err(Error::Invalid("retained kind changed"));
            }
            objects::sync_object(&repo, id)?;
        }
        root.retained
            .extend(objects::audit(&repo, &root.retained, self.limits, true)?);
        let receipt = Receipt {
            repository: root.repository,
            actor: update.actor,
            request: update.request,
            intent: update.intent,
            sequence: root.sequence,
            domain: primary,
            selections: if selections.len() > 1 {
                selections
            } else {
                BTreeMap::new()
            },
            selection,
            result: update.result.clone(),
        };
        root.receipts
            .insert(Receipt::key(update.actor, update.request), receipt.clone());
        self.validate_root(&root)?;
        let state = serde_json::to_vec(&root).map_err(Error::other)?;
        if state.len() > self.limits.root_bytes {
            return Err(Error::Limit);
        }
        let state_id = objects::blob(&repo, &state)?;
        let mut links =
            BTreeMap::from([("state".into(), (state_id, gix::objs::tree::EntryKind::Blob))]);
        for (&domain, selection) in &root.domains {
            links.insert(
                domain.name().into(),
                (selection.object, gix::objs::tree::EntryKind::Blob),
            );
        }
        let retained = objects::retained_links(&root.retained);
        if !retained.is_empty() {
            links.insert(
                "retained".into(),
                (
                    objects::tree(&repo, retained)?,
                    gix::objs::tree::EntryKind::Tree,
                ),
            );
        }
        let tree = objects::tree(&repo, links)?;
        let mut bytes = format!("tree {tree}\n");
        for parent in objects::parents(root.previous, &root.retained) {
            bytes.push_str(&format!("parent {parent}\n"));
        }
        bytes.push_str("author jj-store <metadata@local.invalid> 0 +0000\ncommitter jj-store <metadata@local.invalid> 0 +0000\n\nLocal collaboration state v1\n");
        let id = repo
            .objects
            .write_buf(gix::objs::Kind::Commit, bytes.as_bytes())
            .map_err(Error::other)?;
        let commit = GitOid::from_git(id)?;
        objects::sync_object(&repo, commit)?;
        self.read_at(commit)?;
        Ok(Preparation::Ready(Box::new(Candidate {
            expected: base.map(|s| s.commit),
            snapshot: Snapshot { commit, root },
            receipt,
        })))
    }
}
