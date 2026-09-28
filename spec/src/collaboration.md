# Agent activity and repository reviews

AgentStore and ReviewStore are first-party repository capabilities. They share
the existing object envelopes, WAL, receipts and manifest publication service,
but retain independent models, authorization and state versions. They are not
upstream JJ traits, Tantivy indexes, Nostr projections or agent executors.

The first release is LocalTrusted. It captures Codex, Cursor and OpenCode
through qualified integrations, stores sanitized evidence before normalized
views and supports exact discussions on immutable code, diff and agent
artifacts. Review task context is explicit and version-bound; replaying or
reading a comment never dispatches an agent.

## Requirements

| ID             | Requirement                                                                                                                                                                                                     |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-COLLAB-001 | The repository facade MUST expose independent root-pinned AgentStore and ReviewStore services backed only by the existing immutable object, WAL, receipt and manifest publication authority.                    |
| JST-COLLAB-002 | Capture MUST preserve stable session/turn/tool/checkpoint identities, sanitized native evidence, deterministic normalized activity and explicit evidence-backed JJ associations for Codex, Cursor and OpenCode. |
| JST-COLLAB-003 | Review threads MUST target exact immutable artifact versions and validated UTF-8 byte ranges, with stale-safe replies, edits, tombstones, resolve/reopen and conservative explicit re-anchoring.                |
| JST-COLLAB-004 | LocalTrusted CLI and HTTP APIs MUST provide bounded root-pinned reads, typed writes, pagination and live hints without bearer credentials, public listeners or implicit controller dispatch.                    |
| JST-COLLAB-005 | The application MUST reuse Community-owned thread/comment/agent presentation and Design Core diff extension points, with accessible interactions, virtualization and pinned-code versus live-activity state.    |
| JST-COLLAB-006 | Sanitization MUST occur before every durable capture copy through the qualified packaged redaction helper; credentials, unrelated sessions and unsafe rendered content MUST fail closed.                        |
| JST-COLLAB-007 | Cold recovery and native copy MUST verify code, agent and review closures directly; search and other projections remain rebuildable, and unsupported remote/export/backup behavior stays deferred.              |

## Initial delivery boundaries

The active sequence is publication foundation, AgentStore model and capture,
ReviewStore exact anchors, LocalTrusted API/CLI integrations, shared UI, then
cold recovery and real-harness qualification. Codex CLI 0.156.1, Cursor 3.22.7
and OpenCode 1.18.30 are the initial compatibility surfaces; each still requires
sanitized fixtures and an executed capability matrix before support is claimed.

Authenticated/shared serving, remote replication, backup/restore, Git or Nostr
export, controller job dispatch, arbitrary cross-repository targets, CRDT/edit
identity, advanced semantic re-anchoring, other harnesses and visual baselines
are deferred. Tantivy may index committed collaboration records later but never
becomes their authority.

## AgentStore object foundation

`jj-agent` owns distinct fixed-width stable IDs for sessions, attempts, turns,
messages, blocks, tool calls, terminals, checkpoints, associations,
observations, events, devices and capture sources. Content-addressed
`AgentObjectId` values remain separate from those durable entity IDs. Native JJ
operation, commit and change IDs retain their existing owned types and are not
reinterpreted as agent identities.

`AgentRootV1` selects independent persistent radix catalogs for sessions,
aliases, checkpoints, associations, relations, sources and projections. Empty
catalogs are represented by absent page roots in one canonical empty root.
Catalog pages are bounded immutable objects with canonical digest keys and typed
strong references. Every staged root and page validates its complete
repository-bound dependency closure before it can be selected; missing objects,
semantic kind mismatches, unknown fields and noncanonical bytes fail closed.
Only the publication coordinator can turn staged objects into visible state.

Sanitized observation batches and normalized event batches are distinct
immutable objects. Observation batches record integration, source instance,
source order, source kind/trust, representation and sanitizer version; their
payload is validated canonical JSON or UTF-8 text rather than an arbitrary
untyped JSON value. Event batches reference their exact evidence batch and use
stable event/entity IDs. Normalization therefore cannot replace or rename the
accepted source evidence.

Session snapshots select exact evidence, event, transcript and tool object
versions. Tool snapshots keep execution state separate from capture evidence and
reference typed sanitized input/output content. Transcript manifests keep each
source generation separate and root an ordered immutable chunk-page closure
sufficient for direct reconstruction; they have no indispensable
previous-manifest link. Checkpoint discovery remains in its root-level catalog,
so a session snapshot has no mutable or circular checkpoint reference.

Checkpoint versions pin exact session, transcript, event, tool and association
objects plus complete before/after native operation and commit anchor sets. They
distinguish changed, verified no-change and not-observed code relations; a
failed tool can still have changed code. Completeness, capture status and
execution outcome remain independent. Superseded checkpoints are immutable
strong edges and cannot point back from a session snapshot.

Code associations are many-to-many assertions with stable association, session,
optional checkpoint/turn/tool/scope identities, full native IDs, optional JJ
change IDs, repository-relative paths, evidence method, trust, coverage and
exclusivity. Correction versions must name a prior immutable association and a
reason. File claims reject absolute or traversing paths, and manual assertions
cannot masquerade as stronger evidence. Stable IDs and JJ change IDs are lookup
facts; exact operation/commit IDs and immutable evidence objects are retention
edges.

An AgentStore read is pinned to one selected `AgentRootV1`. Stable session,
checkpoint and association keys resolve through bounded persistent radix pages;
immutable object reads succeed only for references in that root's audited
transitive closure. A staged but unselected object is not readable through the
pinned facade. Catalog leaves contain at most 32 selections, branch updates copy
only the affected path, and prior roots remain readable.

Typed agent deltas carry expected and replacement selections for all seven
catalogs. They reject duplicate edits, semantic kind substitution and no-op
edits. A disjoint edit can reapply to a newer agent root after a CAS race, while
a changed expected selection is stale. The stable request intent covers the
original delta rather than generated page/root IDs, so receipt replay remains
valid after later agent publications.

Alias objects bind an integration, source instance, native entity kind and
native ID to a matching stable session, turn, message, tool, checkpoint or
association identity. These lookup facts are not strong object references.
Relation subjects are typed session, tool, operation, revision or checkpoint
identities. Their catalog selections are immutable linked pages containing only
checkpoint and association object references; each page repeats its subject so a
cursor from another relation fails closed.

Source-state objects retain the integration/source identity, generation,
versioned parser, optional native session key, source-specific accepted
position, strictly ordered non-overlapping capture-batch receipts and coverage.
Projection catalogs retain one normalizer version and activation time plus exact
selected session, normalized-event and source-state object versions. Replaying a
parser therefore publishes new selected objects rather than mutating prior
facts.

Catalog enumeration is radix ordered and root pinned, returns at most 256
selections and resumes after a stable catalog key. Relation traversal uses a
root-local page/offset cursor and rejects mismatched subjects. Transcript reads
select one source generation from an immutable manifest, verify strictly
increasing chunk sequences across linked pages and enforce both a record count
and an eight MiB caller-selected byte ceiling. Session/checkpoint enumeration,
relation traversal and transcript streaming do not depend on Tantivy.

## Capture sanitization boundary

The Node capture boundary uses `@lapismd/agent-redaction` 0.1.0 at source
revision `9583b512`. It submits the complete untrusted observation to the
package before reading capture fields, then rebuilds an allowlisted DTO from the
sanitized result. Accessors, unsupported values and unknown integration IDs are
omitted without forwarding source values or diagnostics. The returned DTO
records the package policy identity and sanitization summary; disposal is
mandatory after every observation.

This boundary currently admits only `codex`, `cursor` and `opencode` integration
identifiers and the source-kind vocabulary owned by `SanitizedObservationV1`. It
proves the pre-spool sanitization contract but does not yet qualify a harness
decoder, local discovery, durable spool or capture publication path. Those
remain M9.2 work and support is not claimed from the adapter test alone.
