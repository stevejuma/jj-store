//! Rebuildable append witnesses derived only from an audited, pinned object closure.
use super::*;
use jj_agent::SourceInstanceId;

#[derive(Clone, Copy)]
pub struct TranscriptAppendBoundary {
    pub(super) source: SourceInstanceId,
    pub(super) generation: u64,
    pub(super) first: u64,
    pub(super) last: u64,
    pub(super) count: u64,
}
#[derive(Clone, Default)]
pub struct TranscriptAppendProofs(BTreeMap<AgentObjectRef, TranscriptAppendBoundary>);
impl TranscriptAppendProofs {
    pub fn boundary(&self, root: AgentObjectRef) -> Option<TranscriptAppendBoundary> {
        self.0.get(&root).copied()
    }
}
enum Node {
    Chunk(TranscriptAppendBoundary),
    Page {
        source: SourceInstanceId,
        generation: u64,
        chunks: Vec<AgentObjectRef>,
        link: Option<AgentObjectRef>,
    },
}
#[derive(Default)]
pub struct TranscriptAppendAudit {
    nodes: BTreeMap<AgentObjectRef, Node>,
    roots: BTreeSet<AgentObjectRef>,
}
impl TranscriptAppendAudit {
    /// The persistence provider must validate canonical identity and closure membership first.
    pub fn observe(&mut self, reference: AgentObjectRef, object: &AgentObjectV1) {
        let node = match object {
            AgentObjectV1::TranscriptChunk(c) => Some(Node::Chunk(TranscriptAppendBoundary {
                source: c.source_instance_id,
                generation: c.source_generation,
                first: c.sequence,
                last: c.sequence,
                count: 1,
            })),
            AgentObjectV1::TranscriptTailPage(p) => Some(Node::Page {
                source: p.source_instance_id,
                generation: p.source_generation,
                chunks: p.chunks.clone(),
                link: p.prefix,
            }),
            AgentObjectV1::TranscriptManifest(m) => {
                self.roots
                    .extend(m.streams.iter().map(|s| s.ordered_chunk_root));
                None
            }
            _ => None,
        };
        if let Some(node) = node {
            self.nodes.insert(reference, node);
        }
    }
    pub fn finish(self) -> TranscriptAppendProofs {
        let mut memo = BTreeMap::<AgentObjectRef, Option<TranscriptAppendBoundary>>::new();
        for root in &self.roots {
            let mut pending = vec![(*root, false)];
            let mut visiting = BTreeSet::new();
            while let Some((reference, expanded)) = pending.pop() {
                if memo.contains_key(&reference) {
                    continue;
                }
                let Some(Node::Page { link, .. }) = self.nodes.get(&reference) else {
                    memo.insert(reference, None);
                    continue;
                };
                if expanded {
                    memo.insert(reference, self.page_boundary(reference, &memo));
                    visiting.remove(&reference);
                } else if !visiting.insert(reference) {
                    memo.insert(reference, None);
                } else {
                    pending.push((reference, true));
                    if let Some(link) = link {
                        pending.push((*link, false));
                    }
                }
            }
        }
        TranscriptAppendProofs(
            self.roots
                .into_iter()
                .filter_map(|r| memo.get(&r).copied().flatten().map(|b| (r, b)))
                .collect(),
        )
    }
    fn page_boundary(
        &self,
        reference: AgentObjectRef,
        memo: &BTreeMap<AgentObjectRef, Option<TranscriptAppendBoundary>>,
    ) -> Option<TranscriptAppendBoundary> {
        let Node::Page {
            source,
            generation,
            chunks,
            link,
        } = self.nodes.get(&reference)?
        else {
            return None;
        };
        let mut local = None;
        for reference in chunks {
            let Node::Chunk(chunk) = self.nodes.get(reference)? else {
                return None;
            };
            if chunk.source != *source || chunk.generation != *generation {
                return None;
            }
            local = Some(match local {
                None => *chunk,
                Some(before) => combine(before, *chunk)?,
            });
        }
        let local = local?;
        let Some(link) = link else {
            return Some(local);
        };
        let other = memo.get(link).copied().flatten()?;
        combine(other, local)
    }
}
fn combine(
    before: TranscriptAppendBoundary,
    after: TranscriptAppendBoundary,
) -> Option<TranscriptAppendBoundary> {
    if before.source != after.source
        || before.generation != after.generation
        || before.last >= after.first
    {
        return None;
    }
    Some(TranscriptAppendBoundary {
        last: after.last,
        count: before.count.checked_add(after.count)?,
        ..before
    })
}
