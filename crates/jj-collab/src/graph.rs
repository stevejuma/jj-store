use crate::*;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_COLLAB_OBJECTS: usize = 100_000;
/// Repository-bound providers supply this bounded immutable graph. It is not a publication authority.
#[derive(Clone, Debug, Default)]
pub struct CollabGraphV1 {
    pub objects: BTreeMap<CollabObjectRef, CollabObjectV1>,
}
impl CollabGraphV1 {
    pub fn stage(&mut self, object: CollabObjectV1) -> Result<CollabObjectRef> {
        let reference = object.reference()?;
        if !self.objects.contains_key(&reference) && self.objects.len() >= MAX_COLLAB_OBJECTS {
            return Err(Error::Limit);
        }
        self.objects.insert(reference, object);
        Ok(reference)
    }
    pub fn get(&self, reference: CollabObjectRef) -> Result<&CollabObjectV1> {
        let object = self
            .objects
            .get(&reference)
            .ok_or(Error::Invalid("missing collaboration dependency"))?;
        if object.reference()? != reference {
            return Err(Error::Invalid("collaboration identity mismatch"));
        }
        Ok(object)
    }
    pub fn closure(
        &self,
        roots: impl IntoIterator<Item = CollabObjectRef>,
    ) -> Result<BTreeSet<CollabObjectRef>> {
        let mut pending: Vec<_> = roots.into_iter().collect();
        let mut seen = BTreeSet::new();
        let mut bytes = 0usize;
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len() > MAX_COLLAB_OBJECTS {
                return Err(Error::Limit);
            }
            let object = self.get(reference)?;
            bytes = bytes
                .checked_add(encode_object(object)?.len())
                .ok_or(Error::Limit)?;
            if bytes > 256 * 1024 * 1024 {
                return Err(Error::Limit);
            }
            pending.extend(object.dependencies()?);
            self.audit_object(object)?;
        }
        Ok(seen)
    }
    fn audit_object(&self, object: &CollabObjectV1) -> Result<()> {
        let target = match object {
            CollabObjectV1::ThreadVersion(v) => v.comment_source.as_ref().map(|s| &s.target),
            CollabObjectV1::Reaction(v) => Some(&v.target),
            CollabObjectV1::ReactionRetraction(v) => Some(&v.target),
            CollabObjectV1::ReactionSlot(v) => Some(&v.target),
            _ => None,
        };
        if let Some(ReactionTargetV1::HumanMessage {
            thread_id,
            message_id,
            revision,
        }) = target
        {
            let CollabObjectV1::HumanMessageRevision(message) = self.get(*revision)? else {
                return Err(Error::Invalid("human reaction target"));
            };
            if (message.thread_id, message.message_id) != (*thread_id, *message_id)
                || message.body.is_none()
            {
                return Err(Error::Invalid("human reaction target membership"));
            }
        }
        if let CollabObjectV1::Reaction(v) = object
            && let ReactionValueV1::Custom { emoji_id, version } = v.value
        {
            let CollabObjectV1::CustomEmojiVersion(emoji) = self.get(version)? else {
                return Err(Error::Invalid("custom reaction value"));
            };
            if emoji.emoji_id != emoji_id {
                return Err(Error::Invalid("custom reaction emoji identity"));
            }
        }
        match object {
            CollabObjectV1::ReactionSlot(slot) => {
                slot.resolve(&self.objects)?;
            }
            CollabObjectV1::CustomEmojiVersion(emoji) => {
                let (CollabObjectV1::ImageAsset(asset), CollabObjectV1::ImageAsset(preview)) =
                    (self.get(emoji.asset)?, self.get(emoji.preview)?)
                else {
                    return Err(Error::Invalid("emoji assets"));
                };
                if preview.frames != 1
                    || preview.media_type != ImageMediaTypeV1::Png
                    || (preview.width, preview.height) != (asset.width, asset.height)
                {
                    return Err(Error::Invalid("emoji static preview"));
                }
                let (_, expected) = ImageAssetV1::ingest(&asset.bytes)?;
                if expected != *preview {
                    return Err(Error::Invalid("emoji preview differs from first frame"));
                }
                if let Some(prior) = emoji.previous {
                    let CollabObjectV1::CustomEmojiVersion(old) = self.get(prior)? else {
                        return Err(Error::Invalid("emoji history"));
                    };
                    if old.emoji_id != emoji.emoji_id {
                        return Err(Error::Invalid("emoji history identity"));
                    }
                }
            }
            CollabObjectV1::HumanMessageRevision(message) => {
                if let Some(parent) = message.reply_to {
                    let CollabObjectV1::HumanMessageRevision(parent) = self.get(parent)? else {
                        return Err(Error::Invalid("reply parent"));
                    };
                    if parent.thread_id != message.thread_id
                        || parent.message_id == message.message_id
                        || parent.body.is_none()
                    {
                        return Err(Error::Invalid("reply parent identity"));
                    }
                }
                if let Some(prior) = message.previous {
                    let CollabObjectV1::HumanMessageRevision(old) = self.get(prior)? else {
                        return Err(Error::Invalid("message history"));
                    };
                    if (old.message_id, old.thread_id, old.actor, old.reply_to)
                        != (
                            message.message_id,
                            message.thread_id,
                            message.actor,
                            message.reply_to,
                        )
                    {
                        return Err(Error::Invalid("message history identity"));
                    }
                }
            }
            CollabObjectV1::ThreadVersion(thread) => {
                let CollabObjectV1::ThreadEvent(event) = self.get(thread.event)? else {
                    return Err(Error::Invalid("Thread event"));
                };
                if (thread.thread_id, thread.actor, thread.accepted_at_millis)
                    != (event.thread_id, event.actor, event.accepted_at_millis)
                {
                    return Err(Error::Invalid("Thread event attribution"));
                }
                match thread.previous {
                    None if thread.status != ThreadStatusV1::Created
                        || event.previous.is_some()
                        || event.event != ThreadEventKindV1::Created =>
                    {
                        return Err(Error::Invalid("Thread initial version"));
                    }
                    Some(prior) => {
                        let CollabObjectV1::ThreadVersion(old) = self.get(prior)? else {
                            return Err(Error::Invalid("Thread history"));
                        };
                        if old.thread_id != thread.thread_id
                            || event.previous != Some(old.event)
                            || old.baseline != thread.baseline
                            || old.agent_message != thread.agent_message
                            || old.comment_source != thread.comment_source
                        {
                            return Err(Error::Invalid("Thread history identity"));
                        }
                        match &event.event {
                            ThreadEventKindV1::Lifecycle {
                                previous_status,
                                action,
                            } if *previous_status == old.status
                                && old.status.transition(*action)? == thread.status
                                && old.title == thread.title => {}
                            ThreadEventKindV1::Renamed { previous_title }
                                if *previous_title == old.title && thread.status == old.status => {}
                            ThreadEventKindV1::Message { revision } => {
                                let CollabObjectV1::HumanMessageRevision(message) =
                                    self.get(*revision)?
                                else {
                                    return Err(Error::Invalid("Thread message event"));
                                };
                                if message.thread_id != thread.thread_id
                                    || thread.title != old.title
                                    || thread.status != old.status
                                {
                                    return Err(Error::Invalid("Thread message membership"));
                                }
                            }
                            ThreadEventKindV1::Link { version } => {
                                let CollabObjectV1::ThreadLink(link) = self.get(*version)? else {
                                    return Err(Error::Invalid("Thread link event"));
                                };
                                if link.thread_id != thread.thread_id
                                    || thread.title != old.title
                                    || thread.status != old.status
                                {
                                    return Err(Error::Invalid("Thread link membership"));
                                }
                            }
                            _ => return Err(Error::Invalid("Thread version event transition")),
                        }
                    }
                    _ => {}
                }
            }
            CollabObjectV1::ThreadEvent(event) => {
                if let Some(prior) = event.previous {
                    let CollabObjectV1::ThreadEvent(old) = self.get(prior)? else {
                        return Err(Error::Invalid("Thread event history"));
                    };
                    if old.thread_id != event.thread_id {
                        return Err(Error::Invalid("Thread event identity"));
                    }
                }
            }
            CollabObjectV1::ThreadLink(link) => {
                if let ThreadLinkTargetV1::CommentBatch {
                    comments,
                    message,
                    reply_to,
                    ..
                } = &link.target
                {
                    for target in comments {
                        if let ReactionTargetV1::HumanMessage {
                            thread_id,
                            message_id,
                            revision,
                        } = target
                        {
                            let CollabObjectV1::HumanMessageRevision(m) = self.get(*revision)?
                            else {
                                return Err(Error::Invalid("batch comment kind"));
                            };
                            if (m.thread_id, m.message_id) != (*thread_id, *message_id)
                                || m.body.is_none()
                            {
                                return Err(Error::Invalid("batch comment identity"));
                            }
                        }
                    }
                    if let Some(reference) = reply_to {
                        let CollabObjectV1::HumanMessageRevision(m) = self.get(*reference)? else {
                            return Err(Error::Invalid("batch reply kind"));
                        };
                        if m.thread_id != link.thread_id || m.body.is_none() {
                            return Err(Error::Invalid("batch reply identity"));
                        }
                    }
                    if let Some(reference) = message {
                        let CollabObjectV1::HumanMessageRevision(m) = self.get(*reference)? else {
                            return Err(Error::Invalid("batch message kind"));
                        };
                        if m.thread_id != link.thread_id || m.body.is_none() {
                            return Err(Error::Invalid("batch message identity"));
                        }
                    }
                }

                if let Some(prior) = link.previous {
                    let CollabObjectV1::ThreadLink(old) = self.get(prior)? else {
                        return Err(Error::Invalid("Thread link history"));
                    };
                    if old.id != link.id
                        || old.thread_id != link.thread_id
                        || old.target != link.target
                    {
                        return Err(Error::Invalid("Thread link history identity"));
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn catalog(
        &self,
        root: Option<CollabObjectRef>,
    ) -> Result<BTreeMap<CollabCatalogKey, CollabObjectRef>> {
        let mut result = BTreeMap::new();
        let mut pending: Vec<_> = root.map(|r| (String::new(), r)).into_iter().collect();
        let mut seen = BTreeSet::new();
        while let Some((expected, reference)) = pending.pop() {
            if !seen.insert(reference) || seen.len() > MAX_COLLAB_OBJECTS {
                return Err(Error::Invalid("collaboration catalog cycle or limit"));
            }
            match self.get(reference)? {
                CollabObjectV1::IndexPage(CollabIndexPageV1::Leaf { prefix, entries })
                    if *prefix == expected =>
                {
                    for (&key, &value) in entries {
                        if result.insert(key, value).is_some() {
                            return Err(Error::Invalid("duplicate collaboration catalog key"));
                        }
                    }
                }
                CollabObjectV1::IndexPage(CollabIndexPageV1::Branch { prefix, children })
                    if *prefix == expected =>
                {
                    for (nibble, &child) in children {
                        pending.push((format!("{prefix}{nibble}"), child));
                    }
                }
                _ => return Err(Error::Invalid("collaboration catalog prefix or kind")),
            }
        }
        Ok(result)
    }
    pub fn catalog_update(
        &mut self,
        root: Option<CollabObjectRef>,
        updates: BTreeMap<CollabCatalogKey, CollabObjectRef>,
    ) -> Result<Option<CollabObjectRef>> {
        let mut root = root;
        for (key, value) in updates {
            self.get(value)?;
            root = Some(self.insert(root, String::new(), key, value)?);
        }
        Ok(root)
    }
    fn insert(
        &mut self,
        root: Option<CollabObjectRef>,
        prefix: String,
        key: CollabCatalogKey,
        value: CollabObjectRef,
    ) -> Result<CollabObjectRef> {
        let page = match root {
            Some(reference) => match self.get(reference)?.clone() {
                CollabObjectV1::IndexPage(page) => page,
                _ => return Err(Error::Invalid("catalog page kind")),
            },
            None => CollabIndexPageV1::Leaf {
                prefix: prefix.clone(),
                entries: BTreeMap::new(),
            },
        };
        match page {
            CollabIndexPageV1::Leaf {
                prefix: actual,
                mut entries,
            } if actual == prefix => {
                entries.insert(key, value);
                self.pack(prefix, entries)
            }
            CollabIndexPageV1::Branch {
                prefix: actual,
                mut children,
            } if actual == prefix && prefix.len() < 64 => {
                let nibble = key.to_string()[prefix.len()..prefix.len() + 1].to_owned();
                let child = self.insert(
                    children.get(&nibble).copied(),
                    format!("{prefix}{nibble}"),
                    key,
                    value,
                )?;
                children.insert(nibble, child);
                self.stage(CollabObjectV1::IndexPage(CollabIndexPageV1::Branch {
                    prefix,
                    children,
                }))
            }
            _ => Err(Error::Invalid("catalog insertion prefix")),
        }
    }
    fn pack(
        &mut self,
        prefix: String,
        entries: BTreeMap<CollabCatalogKey, CollabObjectRef>,
    ) -> Result<CollabObjectRef> {
        if entries.len() <= 32 {
            return self.stage(CollabObjectV1::IndexPage(CollabIndexPageV1::Leaf {
                prefix,
                entries,
            }));
        }
        if prefix.len() >= 64 {
            return Err(Error::Invalid("catalog key collision"));
        }
        let mut buckets: BTreeMap<String, BTreeMap<CollabCatalogKey, CollabObjectRef>> =
            BTreeMap::new();
        for (key, value) in entries {
            buckets
                .entry(key.to_string()[prefix.len()..prefix.len() + 1].into())
                .or_default()
                .insert(key, value);
        }
        let mut children = BTreeMap::new();
        for (nibble, entries) in buckets {
            children.insert(
                nibble.clone(),
                self.pack(format!("{prefix}{nibble}"), entries)?,
            );
        }
        self.stage(CollabObjectV1::IndexPage(CollabIndexPageV1::Branch {
            prefix,
            children,
        }))
    }
}
