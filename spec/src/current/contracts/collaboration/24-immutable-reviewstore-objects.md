# Immutable ReviewStore objects

Catalog path copying and bounded root-pinned enumeration share one owned engine
across persistence adapters. Its object interface requires canonical identity,
semantic kind and dependency validation; its snapshot interface admits only the
audited selected closure. These interfaces do not carry publication authority.
Native staging repeats existing catalog, relation, comment ownership and event
reconstruction audits before selecting the returned root. Typed command
reduction and complete cross-object review auditing MUST use the same owned
projector across adapters. It returns a root value, delta and typed result only;
authorization, external code/agent retention, receipt replay and publication
remain adapter responsibilities. Repository identity is checked before immutable
reads or staging, and no projector result alone selects state.

`ReviewRootV1` selects independent COW catalog roots for threads, comments,
thread events, original-anchor relations, artifact relations, statuses, work
requests, external aliases and accepted source positions. The canonical empty
root omits all page references. Every populated root entry must select a typed
review index page; leaves contain at most 32 stable-key selections and branches
at most sixteen lowercase-hex children.

The registered v1 object kinds are index page, anchor, comment body, comment
revision, thread event, event page and thread snapshot. Review objects use
bounded canonical encoding, typed review-object references and explicit native
or AgentStore dependencies. Anchors forward their exact external retention
edges; other review objects can only point through declared ReviewStore kinds.
Root, revision, event and snapshot content IDs hash their validated canonical
forms and remain separate from stable workflow identities.

Comment bodies are bounded safe Markdown or plain UTF-8 text with an explicit
sanitizer version. Empty bodies, disallowed control characters and oversized
content fail closed. A comment revision retains stable thread/comment IDs,
original author, acting principal and accepted time. Visible edits select a body
object; tombstones and moderated hides require a prior revision, preserving
auditable history rather than deleting prior bytes.

Thread events have repository-accepted, strictly positive per-thread sequence
numbers and authenticated actors. Creation binds the original anchor and first
comment revision. Replies and revisions bind exact selected comment objects;
resolve/reopen actions retain their stable action identity and optional bounded
reason. Event pages contain at most 256 distinct, contiguous sequence entries
and a typed continuation, avoiding an unbounded reverse-linked event chain.

A thread snapshot selects one immutable original anchor, ordered unique comment
revisions, one event-page root, the last accepted sequence, open/resolved state
and selected workflow action. Its overall thread version advances for every
accepted event, while content and workflow versions remain distinct monotonic
preconditions. Parent comments must already occur in the selected order and a
resolved snapshot must identify its workflow action, while an open snapshot has
none. Cross-object loading reconstructs the ordered comments, selected
revisions, open/resolved state and content/workflow counters from the immutable
event stream. It rejects comment revisions from another thread, invalid reply
parents or revision predecessors, repeated workflow transitions, event-page or
event thread/sequence mismatches, missing creation identity and any
reconstructed state that disagrees with the snapshot.
