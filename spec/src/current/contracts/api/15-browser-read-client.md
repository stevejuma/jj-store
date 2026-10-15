# Browser read client

The served Svelte application selects a repository by full ID and authenticates
with an in-memory Bearer token. Disconnect, authorization failure and page
teardown abort outstanding reads/subscriptions and clear repository data. No
credential enters URL state, browser storage, analytics or logs. Requests stay
on the application origin and omit cookies.

Commit, operation and evolution graphs render server-provided edges without
inferring ancestry. Multiple heads require an explicit choice; revset context
and pagination retain an exact selected operation and time anchor. Live events
refresh current-head views while retaining historical selections. Reconnect
resumes only fully parsed SSE IDs; ready/reset announcements compare against the
retained head and queue automatic browser refresh.

Branch graph navigation resolves the selected bookmark or working-copy target
from the captured reference response before issuing the log request. One target
uses its exact ancestry, several conflicted targets use the union of their exact
ancestries and a deleted bookmark uses an explicitly empty revset. Paging keeps
that same target-bound revset and time anchor. The production controller retains
commit metadata for local and remote bookmarks, logical working copies, conflict
and divergence state; operation rows independently retain workspace, snapshot
and concurrent-head metadata.

Read errors retain explicit cursor reset, incomplete history and index
preparation states. Users may prepare/cancel the selected native index. Source
strings, signatures, paths, diff hunks and conflict terms render as text. File
previews have a byte cap, distinguish binary/invalid UTF-8, and never execute or
follow repository content. JSON integers outside JavaScript's safe range remain
exact through lossless parsing and bigint presentation.

Static browser assets are loaded once from a bounded, explicit manifest. Only
the listed index, JavaScript, CSS, font and fixed `agent-icons/*.svg` files are
served; repository paths cannot select filesystem content. Agent icons retain
their stable same-origin `/agent-icons/<name>.svg` URLs. The production document
permits same-origin scripts, styles, connections and images plus image loads
from the exact `https://www.gravatar.com` origin. The only admitted inline style
element is the exact SHA-256 hash of the pinned emoji-mart 5.6.0 shadow
stylesheet used by Community's emoji picker. Style attributes admit only the
SHA-256 hash of the empty string with `unsafe-hashes`, allowing dialog cleanup
to clear its temporary CSSOM lock. Nonempty inline style attributes and every
inline script remain blocked. The document also prohibits framing and plugins.
The browser read client is independent of the later typed mutation and search
capabilities.

The explicit loopback LocalTrusted development annotation setting admits inline
scripts and styles only for the static bundle. It defaults to false; the owned
interactive demo enables it, while generic cold acceptance uses the strict
policy. API responses and request admission keep their existing restrictions.
The configuration guard and development opt-out are owned by `security.md` and
`cli.md`.

Generated schemas express reference siblings through equivalent allOf
composition so client generators retain internal enum discriminators. The
TypeScript consumer checks operation-diff narrowing across every status.

The snapshot bar identifies the loaded operation independently of editable query
fields. Query retries and pages retain original inputs; index preparation must
not reset the time anchor. Event resets queue a snapshot recapture
automatically; manual refresh remains available. Browser graph rendering admits
at most 1000 nodes and 4096 edges; exceeding the display budget fails explicitly
before layout.

Preparation recovery carries the active graph's captured time and identity
through view reconstruction. Ordinary navigation and explicit snapshot recapture
retain their normal context reset; successful index preparation cannot silently
turn an exact retry into a query evaluated at a later time.

Collection controllers retain server cursors independently for every bounded
resource. Directory navigation requests only the selected directory until a tree
node is expanded; list/package modes use the recursive path route without
forcing tree requests to become recursive. Commit inspection continues changed
path pages separately from commit detail. Operation inspection continues
affected-change groups and each selected group's changed paths separately, while
preserving the explicit comparison parent and exact before/after identities.

Agent sessions, selected session detail, transcript chunks, review threads and
selected review detail use the same repository-session query cache and bounded
HTTP client. Their keys include the active access-session identity, repository
and exact stable resource identity; server continuations remain opaque and
memory-only. Review list selection opens the same main-page detail from
repositories and Projects. Fixed page composition and default comment labels
change no root, anchor, permission, command or version precondition; explicit
Reply actions retain their stable parent comment ID.

A committed review command invalidates the repository-session scope before
refetching its list and exact thread. A durable unknown outcome retains the
complete request ID for the status route. Repository SSE queues automatic
refresh. Snapshot recapture invalidates the same scope before collaboration
reads expose a newer AgentStore or ReviewStore root, including for historical
code selections.

Revision context lookup follows opaque association and review continuations
until it finds the exact selected commit; path context likewise requires an
exact code-text or code-artifact commit/path anchor. It deduplicates rows,
rejects repeated cursors and inspects no more than 4,096 rows. The resulting
stable session and thread IDs are retained together in Log or Branches routes so
Agent context and Review discussion render as sibling columns. Selecting a path
replaces the thread selection with its exact anchored review. Closing either
column clears only that route identity.
