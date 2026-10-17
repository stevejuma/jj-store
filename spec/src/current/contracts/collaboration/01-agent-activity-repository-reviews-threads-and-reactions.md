# Agent activity, repository reviews, Threads and Reactions

The Agent, Review, collaboration and capture crate overviews describe their
model, codec, transition and spool roles. E-CRATE-READMES-001 preserves
JST-COLLAB-001/002/003 exact sources, authorization and publication semantics.

Thread directory and selected conversation reads start concurrently through the
repository/session/profile query cache. Navigation cancels obsolete reads;
confirmed reactions invalidate and reload only their exact target, retaining
other reactions, messages, reply context, emoji definitions and raster URLs.
Unknown outcomes keep the original intent and receipt identity. Explicit refresh
invalidates displayed collections; SSE still only offers refresh. Reaction reads
are deduplicated across main/reply views and limited to four concurrent
requests. Reply counts and descendants use indexed passes without changing
loaded-history, exact revision, draft or scroll semantics. Unneeded JJ
log/reference reads do not run when restoring Threads. Cached reads retain
current repository-session isolation and cursor-reset behavior; no batch API or
durable read index is added.

Retention eviction selects only completed materializations; an initializing
entry remains owned by its original read.

Review activity MAY reuse only its count, partial flag and maximum accepted time
for an exact admitted root/window. Bind repository, native epoch or local
profile, root digest, finite limit and the ordered selected reference window.
Every request MUST first capture and validate its complete current provider
selection; reuse retains neither that provider nor its snapshot envelope.
Unrelated metadata publication may reuse facts while returning a fresh pin.
Coalesce duplicate active derivations; failures and cancellation admit no
result. Keep at most 128 fixed-size entries with a five-minute fixed expiry and
bounded retained memory. Evict idle entries only; when all slots are active,
derive request-locally under existing read admission instead of disabling the
query. Changed roots/windows/profiles/epochs, expired entries and restart derive
anew.

Collaboration reads share validated immutable materializations keyed by
repository, authority epoch and exact domain root references. Each request
independently captures its complete manifest envelope and rechecks current
viewer admission before disclosure. Thread directory/history/detail reads
materialize Thread state only; emoji/assets materialize Reaction state only;
reaction queries also load the owning target domain. Retained external
dependencies still undergo the provider's complete closure validation. Directory
metadata and exact-target slot indexes are derived once per root and never
become authority. Shared entries contain no request session, credentials or
viewer-filtered results, coalesce concurrent loads and obey fixed expiry, count,
object and encoded-byte bounds. When all retention slots are initializing, extra
query roots are validated request-locally under the existing per-pin and
concurrency budgets without entering the cache. Small retention capacity cannot
disable a valid query. Failed validation is never admitted;
expiry/eviction/restart rematerialize and validate from selected immutable
records. Signed cursor pins preserve their original complete snapshots
independently of cache eviction.

Application content search retains a separate bounded immutable field index for
each selected Thread/Work root. It indexes only current message/comment bodies
and excludes tombstoned Threads with their messages. Recent Thread previews
retain the existing accepted-event comparison, while nonempty queries preserve
title/body matching and archived-Thread behavior. Sorted query keys and Unicode
snippets are disposable service projections, with fresh viewer and source limits
on every page. Search field retention is separate from conversation pins and
cannot make historical bodies current or establish visibility for a new root.

Human messages optionally retain an exact same-Thread parent revision for
replies. The parent is an immutable closure dependency; self-links, foreign
Thread parents and changing reply identity during edits are rejected. An absent
parent omits the field and preserves legacy canonical bytes.

Explicit local BoundCapture carries the exact observed Thread version and
primary assignment with the original sanitized batch/profile. It retains one
real session replacement or a validated diagnostic-only capture gap, stages the
shared link without changing lifecycle and selects Agent and Thread descriptors
in one companion ref CAS. The Thread descriptor audits and retains the complete
newly staged session closure before selection. Matching receipt replay precedes
entity/source preconditions; stale bindings select neither root. The private
spool retains this original bound request across unknown outcomes. Legacy
unlinked capture tags, field order and receipts remain unchanged.

Native preparation accepts an optional exact Thread binding. The private capture
spool journals it before acknowledging source progress, compares the returned
binding without substitution and retains the complete prepared delta/time for
publication retries. Missing, injected or changed bindings fail before delivery.
Native and local capture-provider qualification remain distinct from automatic
runner selection.

An optional native prepared capture binding pins an observed Thread version and
primary assignment. It retains one real selected session edit or a validated
diagnostic-only source/projection pair and derives a stable link identity from
the authenticated actor, request and selected evidence. Agent evidence and the
Thread link publish atomically; gap links leave primary assignments untouched.
Receipt replay precedes observed preconditions; stale Thread/assignment or a
failed authorization selects neither root. Late bound capture retains the Thread
lifecycle without reopening it. Unlinked capture keeps its original canonical
request and receipt bytes. Provider atomicity and native spool/RPC behavior
retain separate evidence from the local provider and future runner integration.

HTTP keeps Thread and Reaction intent namespaces distinct. Binary custom emoji
definition and typed retirement assemble the same service commands. Empty reads
do not initialize domains; resumed pages retain signed captured selections.
Exact message revision and image reads validate retained membership. HTTP
transport reconstruction is independent of persistent restart and runner gates.

AgentStore and ReviewStore are first-party repository capabilities. They share
the existing object envelopes, WAL, receipts and manifest publication service,
but retain independent models, authorization and state versions. They are not
upstream JJ traits, Tantivy indexes, Nostr projections or agent executors.

The separately governed local Git profile uses the M13 companion publication
store and the same domain models. New versioned portable anchors distinguish
source native and Git identities and name immutable evidence capsules. Existing
native V1 anchor bytes and their interpretation remain unchanged. An evidence
reference is not a device binding or permission to open a local path.

Agent normalization, cumulative transcript construction and catalog path-copy
edits use one owned projection engine behind a repository-bound immutable object
interface. Persistence providers validate canonical records, kinds and complete
dependencies; the shared engine has no publication capability. Native adapters
retain manifest authorization and their existing record framing, while the local
provider selects its complete closure through companion ref CAS.

The complete capture release targets LocalTrusted Codex, Cursor and OpenCode
through qualified integrations, stores sanitized evidence before normalized
views and supports exact discussions on immutable code, diff and agent
artifacts. The separately scoped usable preview below ships the qualified Codex
workflow first; the Cursor/OpenCode active-capture extension below reopens those
two harnesses while extended features stay deferred. Review task context is
explicit and version-bound; replaying or reading a comment never dispatches an
agent.

Agent session list metrics align at the trailing end of the second row in
transcript-count, tool-count, duration order. Tool action toolbars remain inside
their row bounds and accessible by pointer, keyboard and touch; open menus
retain visibility.

## Public release integration

Public distribution uses released Kingfisher core 1.0.3, rules 1.2.1 and scanner
1.4.0 on Rust 1.99. Sanitization policy identities distinguish this rule set;
scanning remains offline, validation and Base64 decoding stay disabled, and
failures withhold publication. Historical sanitized native records are retained.
