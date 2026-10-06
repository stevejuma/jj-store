# Root-pinned capture projection

The publication coordinator can stage one validated sanitized batch and its
normalized result against an explicitly pinned AgentStore root. It stages the
evidence batch first, then typed content, normalized event, reduced tool,
session, alias, source-receipt and projection objects. Permission and file
observations have explicit event variants; a file observation still cannot
become a tool invocation. Every dependency is audited by the existing typed
AgentStore object closure before the returned delta can select it.

The returned `AgentDeltaV1` updates session, alias, source and projection
catalogs in one existing manifest-domain publication. Session snapshots retain
prior selected evidence, event batches, transcripts and tool versions. Each
accepted batch replaces a selected transcript manifest with a cumulative,
self-contained immutable closure containing the prior ordered chunks plus the
new normalized message chunks; reconstruction never depends on an unselected
earlier manifest. Source state rejects repeated or overlapping receipts, records
the accepted native sequence and carries forward explicit coverage gaps.
Projection updates replace superseded session/source versions while preserving
prior immutable event batches. The caller must durably retain the resulting
delta and request identity before publication.
