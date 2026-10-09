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

The first model slice uses an optional ConversationSourceV1 on ThreadVersionV1,
with absent fields omitted so legacy canonical encodings remain identical.
CreateCommentDiscussion admits the new source; Submit projects its bounded
comments, optional ordinary message and CommentBatch link under one receipt. The
shared projector stages on a private graph and refuses the entire batch on stale
source/discussion/message preconditions. Providers resolve every external source
and existing reference against their one captured native/local selection. JSON
captured text requires an explicit string pointer; plain content uses no
pointer. Parent batch records retain exact submitted comment revisions.

CommentReply links retain the exact delivered user capture, the completed
assistant capture and selected answer bytes, and the immutable comment
references. Both publication providers validate membership, user/assistant
roles, matching participation/turn, owned-controller delivery evidence and the
parsed answer range. The same validation applies to raw Link commands. Version-1
protocol tokens derive from typed discussion IDs and exact comment-revision
keys; captured content cannot supply its own trusted token mapping. Literal
Markdown code and incomplete or malformed blocks stay ordinary content.
Conversion refuses answer-link rewriting until its captured-context identity
mapping is qualified; no token or immutable prompt body is silently rewritten.

Captured/authored strings are sanitized independently from trusted protocol
references. The frozen context includes the exact quote and up to 512 UTF-8
bytes on either side, adjusted to character boundaries, within its aggregate 32
KiB budget. Explicit submitted comments merge with other eligible undelivered
discussion revisions; they do not replace that context. The parser accepts at
most 4 MiB, 256 candidates and 128 complete reply blocks, parsing literal-code
ranges once. These limits refuse or bound work without modifying captures.

Submitted batch links retain an optional exact reply parent even when no
ordinary message was authored. Read projections include the current comment
revision and submitted historical revisions while the current comment remains
visible. A tombstone hides every revision from active conversation feedback.
Only current revisions enter incremental prompt context; an older answer remains
associated with its submitted revision. Retained Sent context carries the
canonical batch request and parent thread identity separately from each
recipient's send request.

Recorded native and local agent details expose the same exact discussion and
answer projection as live participation. Tool rows carry a committed invocation
or output target. Permission refusals and bounded-reader failures remain
explicit unavailable metadata; they do not turn a transcript read into an
implicit write.

## Repository-qualified file reviews

The additive `repository_review_comment` reference names its owning repository,
discussion, comment and immutable revision. Existing unqualified reference bytes
and tokens remain unchanged. Qualified references do not add foreign objects to
the home repository's ReviewStore dependency graph. The code repository retains
its own original evidence through its existing publication authority.

Publication requires an explicitly supplied, independently authorized review
reader. Generic commands without that capability refuse qualified references.
The reader resolves the exact selected or historical revision and original
anchor from the owning ReviewStore, verifies the retained source bytes, and
reauthorizes after the read. Missing, hidden, tombstoned or inaccessible
comments remain unavailable. Conversation and reply publication perform this
validation on each attempt; a browser body or a copied quote never grants
authority.

Frozen context carries the exact file anchor and a bounded source quote with
surrounding text. File paths, revision IDs, diff side and repository identities
remain typed source facts. Captured strings pass through the existing sanitizer
and context framing. The version-1 reply token includes the owning repository
and discussion, preventing cross-repository ID collisions. The original
delivered context and completed answer-range proof remain required for every
reply link. Conversion refuses this carrier until qualified identity remapping
is available.

Workspace drafts are additive, omitted when empty, and cannot be projected by
ordinary ThreadStore commands. The execution controller freezes each bounded
source observation and stable draft identity when its queued prompt is eligible.
It checks every current fingerprint before snapshotting, verifies captured bytes
and UTF-8 selections afterward, then publishes per-repository reviews before the
home batch. Retrying a frozen draft reuses its exact command and receipt. A
changed source requires explicit reselection; a snapshot with unknown outcome is
retained and never repeated implicitly. Existing review continuations use
retained original evidence, including after workspace cleanup. No quote matching
is permitted.
