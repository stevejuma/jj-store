# Browser appearance state

Device appearance is browser-owned presentation state under JST-UI-018. It is
independent of server settings revisions, repository operations and receipts.
Appearance changes make no repository mutation or website metadata requests.
Merge qualification retains physical working-copy actions, native tool review
reads and local repository review creation in the shared controller contract.
Confirmed WorkStore reactions reload only their exact target through existing
authorized reads; request identities and observed slot preconditions are
unchanged.

E-CHAT-CARET-001 changes only browser-owned Document caret geometry and draft
insets. Thread and agent source identities, UTF-8 selectors, retained drafts and
existing prompt/comment send commands remain unchanged. Caret and avatar
alignment introduces no API requests, publication or persistence format changes.

E-AGENT-TRANSCRIPT-VIEW-001 applies the existing device conversation preference
across recorded agent response hosts. It changes presentation and explicit
ReviewStore form placement only. Recorded activity identities, operation-pinned
reads, pagination, sanitized captured content, retained drafts, authorization,
comment requests and receipt recovery remain unchanged under JST-UI-015 and
JST-UI-018. No API, schema or persistence format changes are introduced.

Local POST `/v1/local/repos/{repo}/working-copy/read` takes a relative file
path. It returns `WorkingFile` with a device selection and bounded physical
UTF-8 text. POST `/working-copy/save` takes that exact selection and replacement
text. `working_copy_edit` capability is independent of recorded
`code_mutations`. Both routes validate current folder identity and generation;
Save revalidates the selected physical identity before replacement. Stale file
selections use `stale_working_file`; invalid, unsupported, unavailable and
oversized files retain separate responses. Physical Save has no native operation
or companion selection in its result and requires no RustFS runtime.

Local POST `/v1/local/repos/{repo}/working-tree/observe` is a bounded,
non-publishing physical observation. Version-1 input binds the expected device
binding generation and exact operation; it accepts no folder path, workspace
name, replacement contents or command intent. Composition resolves the current
registered folder and proves current WC metadata, sparse scope and actual ignore
policy around a two-pass scan. The complete response retains operation/view,
physical workspace identity, exact WC/tree identities and metadata/physical
fingerprints, changed-path summaries and the obstacle count. It never exposes
global policy paths/bytes, file contents or executable selection tokens.
Oversize observations have no partial response. Stale, unavailable, unsupported,
denied and busy reads remain distinguishable. This endpoint does not assign
changes to a named combination, snapshot code or advertise writes.

The owned client MUST validate complete observation records against the exact
requested repository, binding generation and operation. Identity widths, safe
ordered paths, tree/label terms, numeric and aggregate bounds, and coherent
added/modified/deleted states are checked before acceptance. Native integers
remain lossless; malformed and partial records cannot be promoted. Physical
reads bypass historical query caching and automatic retries. Caller cancellation
and transport disposal reject late completion, including response-body reads.

Buzz shell gradient continuity and compact navigation picker surfaces are owned
CSS presentation. The picker retains native repository/thread selection and
keyboard navigation, with app-background surface tokens and spacing independent
of browser glass. The generated palette's shared popover role supplies the same
app surface to search commands, menus and popovers, without per-component search
overrides. These adjustments add no preference fields or API requests.

The saved accent selects one circular swatch indicator in Appearance. Its ring
stays centered and circular across color modes, hover and responsive layouts;
keyboard focus uses its own visible outline. This presentation does not change
the preference format or native API.

Projects Agents tabs compose existing agent session reads from independently
authorized native/local transports. Main scope derives from unfiltered project
links with bounded project continuations, never installation membership alone.
Session pages validate their repository identity and exact retained AgentStore
snapshot. No server API or canonical record changes. The Projects route admits
`tab=agents` for both landing and selected-project views, with paired
`agent_repository` and `session` native IDs for an inline response. These keys
are valid only on Agents and retain scope through navigation history. Repository
Agents and Projects responses follow the device Thread layout: Split occupies
the standard body right sidebar and Focus uses the structural discussion sheet.
Selection, Close and layout switching preserve the loaded session directory and
the same native selected session; no request or route identity changes. The
response composes existing separately authorized session/transcript reads and
reaction/tool-review commands without switching the global code connection.
Session list and skeleton insets, centered avatars and title-row actions are
presentation only: they preserve pagination, selection identities, independently
operable code association controls and existing agent read/command contracts.

The Projects tab-row search submits the existing `q` filter explicitly on Enter.
Close clears that filter through the same route action and restores the tabs.
Clear removes draft and submitted text through the existing filter action while
retaining the open input in its current scope. The 512 UTF-8 byte bound,
independent home/repository admission and unsupported Review text-search
capability remain unchanged; displaying search controls grants no extra access.

The Threads directory shares that toolbar control while filtering its already
authorized, bounded ThreadSummary titles and directory excerpts. It retains the
existing continuation action and exact native thread selection, without a new
query parameter, indexed-search dependency, API contract or persisted query.

Focus/Split reply presentation preserves the existing native conversation, reply
and tool-call target IDs. Agent tool discussion remains in a full-width inline
row under the exact call, with independently read existing comments and retained
drafts inside either response layout. Bounded review context/detail reads never
change the selected review or route. Comment Reply/Edit selects that exact
review before activating its retained composer; native permission and unknown
request guards remain in force. Other Focus discussions use one
application-owned structural sheet. These choices do not alter API authorization
or mutation identities.

Adjacent tool-call grouping is presentation-only. AgentActivity order and exact
tool-call, event, snapshot and turn IDs remain unchanged; grouped actions still
use the selected individual native call and existing authorized review commands.
Grouped rows and their hover controls share the transcript scroll viewport;
inline call sizing and message-edge alignment do not change those command
targets or retained Focus/Split drafts.

Session comments reuse typed ReviewStore creation and reply commands with the
additive `agent_session` anchor target. It names the exact captured session
snapshot and stable session, retains the full AgentStore closure, and fails
closed on mismatched identity. Independent bounded reads for session and tool
comments do not replace the route or another selected review. Existing request,
receipt, authorization and unknown-outcome semantics apply unchanged.

Page section disclosure preferences use versioned, bounded device-local layout
storage with opaque scope/section keys and boolean flags. They restore only
presentation and do not restore repository access, entity selection, code pins,
read snapshots, drafts or commands. Rename/filter/operation changes do not reset
saved sections; native task/review identities isolate detail choices. Shell
toggle styling retains the existing accessible expansion and action contracts.
