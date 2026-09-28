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
