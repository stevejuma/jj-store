# Conversation comment submissions and answer links

JST-COLLAB-034 through JST-COLLAB-036 require durable conversation annotations
and exact agent replies. Conversation comments use ThreadStore; code reviews
remain ReviewStore-owned. Typed references connect them without duplicate
comment or answer bodies. Native manifest CAS and local companion CAS remain the
only publication authorities. M24 qualification remains independent.

## Sources and submission

A conversation discussion may have an exact source selector: a committed human
message revision, AgentStore message or tool snapshot/content, and an optional
text part and UTF-8 byte selection with selected SHA-256. Old whole-message
agent discussions remain valid and retain their original identities. Selectors
validate membership, source content and byte boundaries against authorized
pinned reads. Read tools and captured patches target captured content, never a
current file fetched by matching a path. Source mapping accounts explicitly for
rendered Markdown splitting, trimmed boundaries and hidden line-number/diff
prefixes.

One bounded submission binds stable request and batch identities, the authored
message and ordered comment revisions. It permits comments-only submissions.
Within one metadata home, new discussions/comments and their batch reference
publish atomically. Existing review references remain references. An agent
prompt's authored text stays in its canonical AgentStore transcript; ordinary
human ThreadStore messages are not duplicated as synthetic agent responses.

Recipient selection and dispatch are explicit. Without addressed agents an
ordinary thread publishes normally. A managed-agent submission publishes
required comments before preparing and dispatching its frozen context.
Publication, per-recipient delivered state and linked-answer state are
independent. Exact started acknowledgements mark delivery. Unknown outcomes
retain the original request and snapshots; receipt lookup precedes retry, and
successfully delivered recipients are never implicitly resent. Failure retains
unsent drafts. Sending is not cross-repository or multi-agent atomicity.
Existing unsupported-profile refusals, authorization, retention and
origin-ledger rules remain authoritative.

## Reply protocol version 1

The typed prompt context supplies discussion and comment references resolving to
immutable revisions, exact selected quotes and bounded surrounding context.
Bootstrap instructions require a block for each discussion actually answered:

```xml
<comment-reply discussion="D1" comments="C1 C2">
Answer in Markdown.
</comment-reply>
```

The agent copies supplied references exactly and lists only comments it answers.
Several same-anchor comments may share a block. Different discussions use
separate blocks placed among normal prose and tools within the same turn.
Neither one turn per comment nor guaranteed model compliance is assumed.

Trusted code emits protocol structure from sanitized typed fields. Captured text
cannot create protocol sections. Authored text remains separate from context;
retained Sent evidence contains the same sanitized context sent over ACP.
Bootstrap and incremental revision delivery retain their existing reservation,
exact-started, proven-nondelivery and unknown-outcome rules. No instruction-only
turn or automatic replay is introduced.

Only recognized assistant-output blocks outside literal code are candidates.
Streaming chunk boundaries do not define protocol boundaries. Original captured
bytes remain immutable. Validation binds exact comment revisions delivered to
that participation and exact completed AgentStore answer content/ranges. A
receipt-backed typed ThreadStore link selects the answer; a browser parse alone
cannot mark a comment replied. Malformed, unknown or unauthorized blocks remain
ordinary readable output without quote-based repair. Multiple legitimate answers
remain distinct. No tag executes a command, grants access or resolves a review.
Historical records are not reinterpreted using current prompt context.
