# Desktop presentation metadata

The native host's version-1 macOS marker is an immutable window-local
presentation hint consumed before shell construction. It selects native control
clearance and drag-region markup. It is neither an HTTP capability nor an
authorization claim, and browser clients cannot obtain additional repository or
filesystem access by setting it. The host's window-only permissions remain
restricted to the managed main window and exact LocalTrusted origin; bootstrap,
HTTP admission and browser CSP retain their existing contracts.

Review transport availability is expressed by its optional reader, rather than
being inferred from a repository profile. Unsupported catalog fixtures omit that
reader explicitly; the typed production application props do not alter HTTP
admission, read envelopes or review persistence.

QueryInfo MAY include optional `default_user_email` only for LocalTrusted
serving. The trimmed value is bounded to 1024 bytes and excludes control
characters. Missing, malformed or unavailable configuration omits the hint;
authenticated native serving MUST omit host identity. This additive metadata
never changes request identity, admission, operation pins or explicit query
context. Multiline query whitespace retains the existing 4096-byte bound.

A client awaiting query metadata MUST discard the waiting submission when its
query scope is invalidated, including view disposal. Metadata completion grants
no authority to resume a canceled query; explicit operation/admission contracts
remain unchanged.

Local Sync adds protected POST `/local/sync/barrier` with an empty body and POST
`/local/sync/wait` with the existing versioned SyncWaitRequest. Both use
LocalTrusted admin admission before control-plane access. Wait requests are
bounded to one second and half the HTTP deadline; a worker timeout returns a
LocalSyncWait report with reached=false and the unchanged barrier. Capture is
explicit and may contact the configured peer; ordinary status GETs remain local.

The Settings reconciliation entry reuses the native preview/apply/status routes
and captures a current head independently from the historical browsing pin.
Transport failures after dispatch remain unknown; closing a presentation cannot
replace its request. Confirmed reconciliation uses automatic refresh semantics.

LocalTrusted native administrators may read `/local/storage`, submit versioned
`/local/storage/operations` commands, and read a bounded operation by request
ID. Commands are check, prepare_checkpoint, publish_checkpoint and
cancel_checkpoint; no paths, shell commands or browser-authored manifests are
accepted. A private durable journal allows cancellation only before checkpoint
dispatch, retains up to 128 operation identities per repository with bounded
bytes, one unresolved operation per repository and one running maintenance
worker per service, and rejects identity reuse with different intent. GET does
not create the journal or start a check. Reports expose captured position,
limits, phase and exact/covering checkpoint selection; checkpoint authority
remains the manifest. Operations continue independently of HTTP request
cancellation.

The managed macOS appearance bridge exposes only `set_glass_background` with a
boolean `enabled` argument, admitted by host ACL for the exact managed origin
and main window. It changes native presentation only; no HTTP, repository,
credential, filesystem or publication contract changes. Browser glass remains an
application-background effect. Native composition and handshake follow
desktop.md and JST-UI-018.

Settings read-only row actions copy complete displayed identities without an API
mutation. Copy actions are opt-in for identifiers and hashes, or enabled by an
internal navigation destination; ordinary status/count values stay
non-interactive. Separate repository and operation links reuse the existing
authorized routes and retain explicit operation pins; immutable thread
revisions, manifest digests and companion metadata IDs remain copy-only.
Copy-only rows reserve the trailing link space for alignment without adding a
navigation action. Navigation is disabled while Settings drafts or pending
operations require resolution. History search visibility and query-context
disclosure are presentation state: toggling them preserves the controller-owned
draft, accepted query, session email and pinned read context. Workspace context
uses the existing explicit workspace selection action.

Application feedback changes presentation only. Toast dismissal never resets
pending mutation identities, drops a draft or retries a command. Obsolete and
cancelled reads remain silent, session changes clear protected notices, and
confirmed state recovery removes the matching condition notice.
