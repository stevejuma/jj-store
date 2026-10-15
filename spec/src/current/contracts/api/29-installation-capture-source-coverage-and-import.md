# Installation capture source coverage and import

Installation-local `GET /v1/local/capture/sources` requires current local Admin
management, accepts only bounded limit/cursor query fields and returns typed
cached source coverage. Pages are at most 100 rows; selectors/cursors bind a
fresh runtime generation and monotonic slot ordinal without paths, native
session keys or admission proofs. A last bound repository is cached metadata,
not authorization. Disabled pages are empty without capture-state I/O; an absent
or unexpectedly lost runtime is unavailable. Known completed stopped
observations remain readable. Continuations tolerate current membership
retirement/growth without reusing selectors; foreign runtime generations return
a distinct stale selection error. Status, source and receipt reads use a fair
coordination queue with a one-second acquisition bound; timeout returns the
existing Busy response. The wait does not grant authority or retain transcript
work. Current admission and private journal checks still run after acquisition.
This read endpoint cannot enroll sources, import history, advance reader
cursors, publish roots or change device policy.

Browser harness, multiple-status and multiple-repository filtering apply only to
already loaded source observations. Their combination cannot add query fields,
change page bounds or cursors, or reinterpret stable source selectors. Loaded
and observed coverage counts remain those of the accepted source pages.
Repository choices use exact cached last-bound identities; a missing last-bound
identity remains an unknown observation and cannot imply repository access or
prove that a source is unbound.

`POST /v1/local/capture/import` is an actor-free version-1 command with
request_id, generation, expected_revision and selector. Current local Admin and
independent repository Write govern historical enrollment. Acknowledgement
returns the exact durable import receipt and bounded progress, not a committed
AgentStore root. Exact replay resolves an accepted original selector after
restart without retargeting its source. Input, authority, stale/new versus
known/exact requests, private state and owner semantics follow the governed gate
in collaboration.md. Receipt/progress inspection requires current management
admission and exposes no paths, native session keys, source proofs or raw
content. The command dispatches once to an existing retained source owner. Its
response requires synchronized durable acceptance; mailbox acknowledgement alone
cannot return success. A five-second bounded acceptance observation may return
an unknown outcome, retaining every original input for retry. Cancellation
cannot release or recreate the source owner. Exact accepted replay returns its
original receipt even while paused or stopped, without transcript work or
dispatch.

`GET /v1/local/capture/imports?request_id=<request>` inspects one nonzero
request under current LocalTrusted Admin/configuration/installation admission.
Its strict query is actor-free. Missing intent returns no receipt and cached
pending or unavailable phase, never successful acceptance. Public receipts
expose original command, journal ordinal, qualified profile/version and
complete-record boundary; opaque source/reader/prefix digests and principals
remain private. Progress uses selected, pending, running, withheld, completed or
unavailable, with bounded accepted captured/published byte counts and separate
tail phase/coverage. Only exact associated reader metadata and contiguous native
receipts can establish completion. Inspection reads existing metadata without
pending bodies, recovery, source enrollment or owner creation. Known receipts
survive absent transcripts; missing/unqualified evidence withholds completion.
Disabled/revoked requests do no capture-state work. Missing import journals are
unavailable and never created by management. Configured Codex whole-server
startup creates journals only within a fresh absent private capture root; an
existing root is never repaired. A separately supervised metadata lane may
recover original accepted commands when a new retained runtime discovers their
exact physical/header bindings. Inspection itself performs no recovery dispatch.
Configured Codex startup uses these existing typed controls; browser management
uses installation-local, token-free protocol transport after advertised
LocalTrusted bootstrap admission. It preserves exact uint64 values and original
unknown control/import intent, validates response and receipt identity, and
withholds mixed runtime source pages. Browser management cannot enable capture,
enroll a changed selector on retry, or select repository roots. Other readers
remain pending.

`GET /v1/local/capture/sources` optionally includes `snapshot_publication` on
API source rows. Its closed record reports source `captured_sequence`,
`captured_logical_records`, `committed_sequence`, optional
`committed_logical_records`, `history_available`, `outstanding_ranges` (at
most 4096) and `receipts`. Missing or corrupt evidence omits the field; it
cannot be represented as successful zero coverage. Zero committed messages are
valid only with qualified receipt coverage, including empty-session lifecycle
evidence. Source sequence is a position, not a message count. Counts remain a
source publication observation, separate from accepted history-import boundaries
and current activity/eligibility. A page inspects at most its 100 selected
source associations with one metadata read under existing coordination. Current
installation Admin, configuration and control-file admission are rechecked after
inspection. No paths, native keys, reader source IDs or generation witnesses
appear in the response; the read contacts no harness and reads no pending body.

Capture management is presented in installation-scoped Agents settings. Leaving
that section cancels visible observations without replacing the original pending
command or request-ID draft; the section does not select a repository. The
browser treats `capture_control_busy` on ordinary GET observations as retryable
contention. At most five read attempts with 150/300/450/600ms delays run per
observation, with cancellation on closure. Exhaustion defers to the visible
polling cycle without an unavailable toast. Other refusals and malformed
responses remain errors; no POST request is retried by this read policy. Loading
uses Settings skeleton rows, retained data survives refresh, and Capture
controls stay visible with their existing capability/precondition guards.

E-UI-CI-002 confirms JST-API-001 unchanged: after an initial inspection finds no
receipt, browser acceptance explicitly refreshes the original import status
through the retained inspection control. A fixture without a receipt-change
event cannot rely on an unsolicited observation. The control remains valid if
background recovery has already removed the unknown-outcome action. No command
is replayed or input replaced; M30's full qualification remains incomplete.

The browser may retain the time it accepts a validated source-page observation
or continuation as session-only presentation metadata. Failed, busy, cancelled
or rejected reads and local filtering MUST NOT advance that observation time.
The source response supplies no per-source update timestamp; this metadata MUST
NOT be presented as transcript activity, file modification time or a
server-stored fact. Harness/status filtering leaves coverage, cursor identities
and exact selected import commands unchanged.

LocalTrusted Admin capture status exposes optional `configured_roots` per
qualified harness as a bounded list of startup-resolved directory paths. Each
path is an absolute UTF-8 string of at most 4096 characters, with at most
sixteen paths per harness. The server populates this from retained startup
metadata, without inspecting directories on GET. Source pages and import
receipts remain free of transcript files and native session identities. A
backend that omits the field has unavailable directory metadata, rather than an
empty configured list.

Capture notification dismissal changes presentation only. It neither clears an
unknown command nor changes its original request identity, draft or receipt.
Successful observations clear operational failure feedback only after status,
source coverage and any selected import inspection are accepted.

Agent activity and recorded transcript APIs retain their exact text content. The
browser may derive generic nested prompt-element disclosures through the safe
Markdown extension described in `ui.md`; tag names and literal attributes never
become trusted metadata or executable HTML. Preview expansion changes only
presentation, with Copy message retaining the complete original body. Compact
single-line leaf rows include the closing tag and safe inline Markdown, while
syntax coloring treats source attributes only as escaped text spans. No
transport schema, media type, stored transcript or review anchor changes.
