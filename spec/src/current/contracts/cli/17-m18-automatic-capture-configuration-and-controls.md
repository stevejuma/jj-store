# M18 automatic capture configuration and controls

The owned runtime accepts coalesced discovery hints without storage ownership or
policy changes. Idle root cooldowns may be advanced; an active bounded scan
keeps its progress and schedules one fresh pass. Pause/stop discard outstanding
hints, and lost lanes cannot restart. Cached root availability and
degraded-source counts add no directory/Write authorization or activity proof.
Installation HTTP rescan binds the latest device policy generation/revision
under the existing current Admin/configuration/physical binding gate. Typed CLI
import and inspection use retained workers; browser source/import presentation
remains pending. Cached runtime diagnostics group qualified profiles and
declared product versions with root/source counts, exposing no paths, session
keys or import selectors. Current Codex group capacity is two, independent of
configured source/reader limits.

`jj agent capture --server-url <url> status|pause|resume|rescan` manages one
explicit LocalTrusted installation through its existing local Admin HTTP API.
This profile works outside a JJ workspace and MUST dispatch before opening any
repository or integration registry. It rejects `--state-dir`; status and control
MUST NOT create device state, install hooks, inspect transcripts or snapshot a
working copy. The URL MUST be credential-free HTTP with a literal loopback
address and no non-root path, query or fragment. No implicit endpoint selection,
proxy, redirect, credential lookup or automatic retry is permitted.

Pause/resume require explicit nonzero `--request-id`, `--generation` and
`--expected-revision`; rescan requires the same generation/revision without a
request identity. Generation is exactly 32 lowercase nonzero hex digits. The CLI
forwards the typed version-1 command once without prefetching/replacing policy
or selecting an actor. JSON output retains the shared typed result; text
distinguishes the original receipt, latest policy and runtime request
acceptance. A durable receipt is not completed runtime pause or drain. A control
acknowledgement MUST bind the original request identity, generation, expected
revision and action, with the next receipt revision and latest selected policy
in the same generation at that revision or later. Rescan acknowledgements bind
the exact selected generation/revision. A mismatched acknowledgement is an
unknown outcome, never success or permission to replace the original command.

The same explicit installation profile adds
`sources --limit <1..100> [--cursor <token>]`. It reads cached per-source
diagnostics and runtime-scoped selectors without opening a workspace or
integration registry. Cursor input and response selectors are bounded typed
values; a stale runtime generation requires refresh from the first page. JSON
preserves the source page; text labels owned reader phases and cached
observations. Import is a separate explicit command with independent native
Write and retained source admission.

The explicit installation command adds
`import --source <selector> --generation <control-generation> --expected-revision <revision> --request-id <request>`.
It forwards only the versioned command, without policy prefetch or automatic new
identity. Exact acknowledgement binds every original input. Acceptance is
durable intent, not successful transcript publication. A retry keeps the
original selector after restart; the server resolves selected requests before
applying new-selection stale rules. Changed reuse or a mismatched
acknowledgement withholds success. New-request reselection cannot rewrite an
unknown or accepted request. Receipt/progress inspection remains local Admin
data. Dispatch uses retained bounded source workers. The command waits for
durable acceptance separately from volatile scheduling, preserving its original
inputs when an outcome is unknown. `import-status --request-id <request>`
forwards one body-free inspection GET; it requires neither a repository nor a
state directory. JSON retains the shared typed receipt/progress. Text
distinguishes accepted history publication from continuing tail phase. Exact
accepted replay remains inspectable while paused/stopped and after transcript
removal. A mismatched receipt, unsupported version or impossible completed byte
count withholds CLI success. Neither command initializes private state, starts
an owner or retries HTTP automatically. The private server journal primitive now
retains exact version-1 import intent and source binding across reopen and
process exit. It shares synchronized private-file mechanics with pause controls
while retaining its separate 2-MiB layout and receipt order. This primitive does
not expose a command, enroll a source or authorize publication. Control layout
and policy semantics remain unchanged.

The server-side import admission guard retains manager/control coordination and
rechecks exact original intent, current LocalTrusted Admin policy and the live
runtime. It is a library primitive; public import and owned scheduling remain
separate gates. No read or guard can initialize missing private state, enable
capture or create a runtime owner.

The library's owned import settlement retains the exact control and import
selection descriptors across native admission awaits. It preserves original
receipt replay and postselection unknown outcomes; it creates neither a command
surface nor an ingestion owner. Native Write, exact source preparation and
bounded frame/storage ownership remain separate requirements. Existing control
and import formats and synchronous callbacks retain their contracts.

The native source-import library now resolves the actual retained runtime
target, prepares a bounded accepted prefix and independently admits native Write
before selecting intent and enrolling its common source lease. Replay uses the
original binding before stale selector rules and opens existing durable spool
progress. Imported pipelines retain exact receipt renewal through ingestion,
publication recovery and raw-channel parking; current consent and prefix checks
precede body work and durable enqueue. Their private runtime target view exposes
no binding in public observation pages and grants neither worker/frame budgets
nor storage ownership. Retained supervisor scheduling is qualified separately.
Automatic JSONL capture now retains receipt-backed device publication evidence
separate from reader acknowledgements. Public import progress and typed HTTP/CLI
commands and configured Codex startup use these retained resources.

The prepared foreground composition may attach one supervised accepted-import
recovery lane when management binds existing import metadata. On a new runtime,
it offers exact original accepted commands to uniquely matching retained source
slots, without another CLI/HTTP command. Original receipt/prefix, current Admin,
configuration, private binding and native Write remain required; changed
physical files cannot inherit consent. Durable pause is selected before dispatch
and keeps recovery idle until resume. Frequent progress inspection cannot starve
owned policy acquisition, ingestion or publication. Recovery shares the
supervisor's cancellation, deadlines and ordered storage lifetime; stopped/lost
owners cannot be recreated. Existing missing import state is never initialized.
This optional retained composition is shared by configured Codex startup; other
readers, browser management and real harness release remain separate gates.

Original accepted import receipts acquire a separate bounded body-free reader
association under retained original-source/native admission and the held import
selection gate. The additive private version-1 catalog records receipt ordinal,
source ID, initial numeric generation, physical/header digest and
retained-prefix verification without changing the accepted receipt. Exact
retry/reopen retain the association; later numeric continuation cannot retarget
it. Receipt-only catalogs remain readable and explicitly unqualified until
renewed admission reconstructs the link. Bounded journal lookup performs no
transcript work and grants no publication authority. Public progress uses this
private association without exposing its digests or treating it as publication
authority.

Responses MUST be bounded to 64 KiB and decoded through the exact public
response type. Unknown versions, malformed/oversized bodies, redirects,
transport failures and failed command acknowledgements withhold success and
retain the original retry identity. Diagnostics expose only constant admitted
error descriptions, never arbitrary server bodies, URLs or local paths. The
server independently revalidates Admin/configuration/installation/policy on each
call. Controls cannot enable configuration, initialize a journal, replace a
runtime owner, import history or grant native publication authority. Implicit
workspace-to-installation endpoint discovery and browser controls remain
separate qualification work.

The shared native spool has a bounded partition drain for one explicit
repository, integration and source, with one through 256 batches per call. Other
native/local partitions remain intact; unknown outcomes retain exact prepared
intent, and the supervisor owns fair scheduling and authorization. Existing CLI
drains keep their whole-spool repository/native-only preconditions. The
unchanged report counts the entire remaining shared queue.

The configured automatic capture actor uses the existing installation principal
derivation with a fixed worker purpose. Disabled authority preparation is
side-effect free and skips the identity file; enabled preparation requires the
validated LocalTrusted native writable profile. In-process capture publishers
bind that principal through the native service only after current directory,
socket and catalog admission, and reauthorize before preparation/publication.
The guarded server publisher repeats these checks with a fresh native Write
request for every preparation and publication, including durable retries; it
rejects foreign actors and returns committed receipts without a post-commit
device-admission failure. This capability remains distinct from supervised
startup and typed controls.

Codex automatic source acquisition takes a current activity snapshot and checks
the activity event's reconstructed directory through current native workspace,
socket and repository-catalog admission before taking source/physical leases.
The directory proof is repeated after acquisition and remains separate from
publication authorization. Existing-registry lookup is read-only and generic
source registration grants no automatic historical eligibility; a distinct
native-session admission digest supports offline-tail recovery. These typed
composition boundaries do not grant configuration enablement.

An explicit admitted Codex source driver composes current activity/directory and
native Write admission, source leases, latest durable source cursor, bounded
reading, transcript sanitation, spool acknowledgement and one-batch guarded
publication. It resumes admitted offline tails, refuses unadmitted history and
recovers each queued Codex range from its exact qualified prefix before new
reading. Recovery publishes one exact batch through current admission and Write
authorization; another queued partition cannot inherit that proof. Changed or
unavailable source evidence retains pending intent. Only constant omission
batches may use a separately authorized reconstructed matching anchor. Source
steps and privacy-root configuration have no raw-path/body diagnostic encoding.
This composition remains separate from supervised automatic startup and typed
controls.

Bounded admitted Codex resume preparation uses read-only retained
channel/session lookup and current complete-record directory reconstruction
before taking reader leases or enrolling rotated aliases. It does not require
the original header folder to remain linked when the verified current folder is
linked and writable. Every acquisition rechecks current native Write, file/root
and channel bindings. It does not acknowledge source progress, import unadmitted
history or authorize queued/body publication through a fallback anchor.
Scheduling and automatic server startup remain separately gated.

Native publication scheduling rotates through exact pending partition heads
using only a memory-resident turn cursor. Selection advances the turn even when
delivery is withheld or unknown, while retained prepared bytes remain unchanged.
Exact delivery rechecks the live same-partition order under its drain lease; a
newly enqueued earlier batch withholds before any publisher call. Local prepared
intent excludes its complete partition from native scheduling. These scheduling
hints supply neither device proof nor native Write authorization. Independent
ingress and supervised server composition remain separate gates.

The explicit Codex pipeline consumes the conservative source driver and offers
separate bounded ingress and exact selected-entry delivery. Sanitized durable
ingress proceeds past old withheld/prepared/unknown publication, with
independent current native admission and Write checks for every new batch.
Delivery recovers the selected entry's own exact range and retains
same-partition order. Old publication cannot rewind the reader or its current
ingress anchor. Spool refusal keeps the exact unacknowledged batch; restart
retains both the latest sanitized cursor and unresolved intent. Temporal proof
scheduling, blocking-I/O ownership and automatic server startup remain
separately gated.

An explicit pipeline-step owner keeps its leased source, private spool and
current authority context outside ordinary runtime execution during synchronous
I/O. It dispatches one bounded step, retains overdue/draining work across
deadlines and pause, and refuses concurrent replacement. Cancellation/deadlines
preserve exact pending intent; current native Write remains mandatory after
resume. Shutdown waits at most five seconds and retains an undrained owner on
timeout. Known late results retain their receipt meaning. This internal owner
closes on task loss. Supervisor resume keeps that source closed while healthy
lanes may continue; explicit worker replacement must renew current source and
repository admission before recovering the unread tail. Durable queued bytes and
cursors survive loss, revocation and pause/resume. The internal owner does not
enable automatic capture or replace the separately authorized typed management
controls and full server lifecycle gates.

An explicit configured Codex supervisor joins root discovery, genuine observed
activity, staged current-directory admission, bounded parked ingress and exact
native delivery. Its memory-only controller exposes bounded status and internal
pause/resume/shutdown, retains independently draining owners and does not
perform source I/O during disabled preparation. This API does not remove the
configured enablement requirements or qualify authenticated management, other
readers, pressure/fault coverage or cold harness/server gates.

An explicit lifecycle driver owns a prepared Codex supervisor and schedules its
lanes independently of callers. Internal pause/resume preserves the same owner;
stop is terminal. Shutdown timeout retains its draining task for a later wait,
and drop requests stop without aborting blocking source/publication owners.
Storage teardown requires completed drain. Explicit storage-backed composition
binds an opaque lifetime before first dispatch and retains it in discovery,
source and delivery blocking owners and their unconsumed completions. Storage
shutdown waits for those lifetimes before stopping embedded storage or releasing
the installation lock; dropping storage or cancelling that wait preserves the
same ownership. The lifetime grants no publication authority. This memory-only
capability adds no CLI command or configured server enablement; those gates
remain pending.

The explicit runtime's cloneable status/pause/resume handle owns only its
body-free observation and command channels. It cannot start capture, authorize
publication, retain storage or wait for drain. Control acceptance requests a
transition; its observed phase establishes whether pause/drain settled. Runtime
drop stops all handles even while they remain readable during retained drain.
Lost nonterminal observation producers report unavailable, and terminal stop
rejects pause/resume. This capability does not implement authenticated commands,
durable pause or configured enablement.

The existing runtime constructor preserves a settled supervisor pause at handoff
instead of dispatching into a paused owner and stopping on that refusal. No
source is discovered, enrolled or published until explicit resume. Activity
present before resumed arming remains historical; fresh observed activity and
current admission are still required for automatic backfill. No new startup
mode, configuration field or durable pause journal is introduced.

The explicit capture-control journal records installation/generation-bound pause
policy and exact actor/request receipts in private device state outside
checkouts. Its initial bounded profile is 4096 receipts and 1 MiB. Record
selection uses a pinned private directory/lock, bounded validated reads,
synchronized temporary replacement and directory sync. Old receipt replay
returns the latest policy without reversing later changes; missing or foreign
state never initializes on ordinary open. Post-replacement I/O failure is an
unknown outcome requiring the original request. This Rust capability grants no
authentication, runtime action, native mutation, import or startup override; its
management consumer requires separate composition. The guarded foreground
factory consumes this journal with read-only policy selection at preparation and
again at handoff; it accepts legitimate later pause/resume selections but
refuses changed generation, installation or physical binding. Explicit policy
initialization is separate from ordinary factory preparation; no startup mode or
configuration override is introduced.

The foreground local native Admin profile may advertise installation capture
management when an explicit configuration path is available. Its manager derives
the transport actor from the installation, revalidates current configuration and
identity on each request and retains one selected journal generation and runtime
control cell. Startup binding shares control channels only, not task or storage
ownership. The manager's owned bounded blocking operation selects/replays device
policy and applies its latest state under one coordination gate, surviving
request cancellation. Read-only status, source and receipt requests use a fair
one-second bounded acquisition of that same gate so continuing import work
cannot repeatedly overtake an observation. Timeout remains Busy; waiter
cancellation releases only its queue place. Once dispatched, an observation
retains its gate and current-policy checks through owned completion. No scan,
new cache authority or CLI command is introduced. Durable selection and runtime
request acceptance are reported separately. Missing or closed runtime controls
cannot start another owner; accepted policy remains available for restart. Other
readers retain their startup guard. The explicit local HTTP CLI profile is
qualified separately; source import and browser coverage presentation remain
composition work.

Discovery and delivery task panic retains that storage lifetime and process lock
until the exact running state is destroyed, including after a cancelled drain
wait or dropped controller. Settled loss closes the lane. Explicit replacement
must renew current admission to recover an unknown native receipt using its
original Prepared request without changing queue bytes, cursor or committed
evidence. This fault boundary adds no public control or startup override.

Delivery rechecks exact source qualification, private source state and spool
binding after directory admission. A root/state replacement during blocked
admission withholds the unchanged queue and cursor. Restored physical state
requires fresh qualification and current admission before explicit recovery. An
initialized source registry whose catalog disappears stays unavailable rather
than allocating a replacement source catalog.
