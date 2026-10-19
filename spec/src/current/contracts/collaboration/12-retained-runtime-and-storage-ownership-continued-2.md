# Retained runtime and storage ownership (continued 2)

Native API and explicit exported JSON message snapshots share a distinct
`opencode-snapshot/v1` sanitized envelope, while existing plugin/hook semantics
remain unchanged. Exact session, message and part identities and product version
must agree before reduction. API status may project lifecycle only under
NativeApi/ApprovedHarnessApi evidence; local exported snapshots use
SessionExport/LocalTranscript and never imply running activity. User/assistant
messages preserve their complete sanitized native parts and stable message
aliases; tool parts enrich their native call with declared pending/running/
completed/error state. Unknown part kinds retain safe evidence and diagnostics
without invented tool or lifecycle transitions. Cross-source alias enrichment,
reader durability, supervision and real-edit coverage still require their own
complete acceptance; acquisition/normalization tests alone cannot enable a
runtime profile.

OpenCode API source channels use the installation-private source catalog and
existing exclusive source/physical lease classes. Their location/lease keys bind
the canonical approved endpoint and case-sensitive native session identity;
generation/admission witnesses additionally bind the exact session directory and
qualified version. Endpoint, directory and witness bindings enter the catalog
only as digests. An idle/unqualified session cannot allocate automatic
admission. Restart may reacquire a previously admitted exact witness without
requiring renewed execution; a changed directory cannot inherit that admission.
Enrollment is separate from current directory/socket, catalog membership, native
Write and current AgentStore checks.

The same atomic catalog admission MUST retain a bounded private recovery index
of validated case-sensitive native session IDs, endpoint/witness digests,
qualified version and owned source channel. This metadata contains no native
directory, endpoint URL, transcript body or acquisition cursor. It is committed
before admission returns, including sessions that have not enqueued a message.
At most 4096 index entries fit within the existing 2 MiB catalog and alias
budgets; capacity refusal MUST NOT evict prior admission or grant a new lease.
Duplicate, malformed, foreign-version or unbound index entries withhold registry
opening. Read-only endpoint-scoped enumeration allocates no identity, lease or
alias, and supplies discovery candidates only. Every candidate still requires
current API witness, directory/socket/catalog and native Write checks. Ended
sessions may be rediscovered without a fresh busy status; changed witnesses do
not inherit admission. Catalog version 1 accepts an absent optional index as
empty for older qualified state; no ID is inferred from an opaque digest.
Reacquiring an exact admitted binding may atomically populate its missing index.

Snapshot ingress partitions at each message. Sanitization requires the exact
per-record directory used for admission; missing/unbound context may retain only
a constant omission at a separately authorized anchor. The existing sanitized
pending spool is durable before its reader acknowledgement. Stable observation,
batch and snapshot cursor identities survive exact retry and reopen. API and
export trust remain distinct. No native body or path gets a durability/debug
encoding from this bridge, and no publication or runtime profile is enabled by
source registration or successful enqueue alone.

The admitted API source driver acquires an exclusive installation source after
current activity or exact prior admission, verifies its session binding again,
then reads bounded pages. It holds at most one page of raw records in memory and
spools one context-partitioned observation per step. The snapshot cursor is a
domain-separated cumulative native-record prefix digest and logical record
count; the session witness remains distinct from that mutable prefix. An opaque
API paging cursor is never a durable acknowledgement or source authority.

Reopen and explicit refresh reconstruct the selected acknowledged prefix from
new context-bound GETs before accepting another record. Unchanged reconstruction
does not enqueue duplicate observations. A rewritten/shortened prefix starts a
new monotonic generation/sequence allocation, retaining earlier exact intents;
those intents cannot borrow the new prefix. Each pending body requires matching
native prefix, sanitized observation, repository partition and current directory
proof before restart publication. Prepared requests retain their original actor,
bytes and request/publication identities. Each native preparation/publication
rechecks the API session binding, exclusive source/spool ownership and current
directory/catalog/Write policy. Spool refusal does not acknowledge a record.
Publication uses the current native AgentStore and never advances code heads.

Approved OpenCode lifecycle observations use a typed optional private checkpoint
alongside the existing reader generation witness. The checkpoint contains only
message count/prefix and the observed busy/retry/idle class; it contains no
native path, body, opaque paging cursor or execution identity. Its
domain-separated composite digest is the acknowledged snapshot position. Status
may advance that digest and source sequence without advancing message count or
replaying messages. An empty admitted session may acknowledge logical message
count zero under this explicit checkpoint, with its initial prefix bound to the
exact session witness. Message-only snapshot positions and other harness
metadata keep their existing encodings. Optional typed state and ready batches
use memory indirection to keep the shared polling enum bounded; their durable
JSON is unchanged. A pass shares one sanitizer for its current verified
workspace and releases it at idle. Dropping a lifecycle checkpoint, changing its
witness/parser/native session, decreasing its message count or changing a
fixed-count message prefix fails closed. Duplicate state cannot acknowledge
another sequence.

The source records explicit API status at its independently authorized declared
session directory. Messages keep their individual directory partitions. A status
poll is an observation of the approved API class, never a reconstructed
execution from messages, timestamps or exported history. Polling does not claim
an event stream or capture intermediate states missed between polls. Sanitized
constant status evidence and its exact typed checkpoint become durable together
before acknowledgement. Restart may reconstruct that already acknowledged
historical status from its matching private checkpoint and exact constant
evidence, with current source/session/directory/Write proof; current status
cannot replace its body or prepared request. New observations use a later
ordered source sequence. Retained message prefixes are reconstructed
independently before tail reading or new status allocation. Pending statuses do
not impersonate message records at the same logical count. Repeated status and
restart cannot enqueue duplicate messages. Rewritten message prefixes retain the
last explicit status while starting a new monotonic generation; unavailable
evidence withholds original intent. Lifecycle checkpoints supply neither source
eligibility nor directory, thread or publication authority.

Directory-aware discovery uses the pinned read-only project inventory and known
project-directory API. Global/non-Git projects deliberately omit persisted
project-directory entries, so their bounded directory candidates come from the
pinned session metadata list under `directory=/`, `scope=project`, `limit=4096`.
Metadata grants no activity or inactive-history import. Reaching that list limit
reports incomplete coverage; oversized metadata withholds the discovery step.
Project metadata and paths are memory-only candidates. Each rescan traverses at
most 4096 project IDs and 4096 distinct safe absolute non-root directories and
4096 offered native session IDs; each response retains the one MiB acquisition
bound. Invalid paths are withheld with safe unsupported coverage;
malformed/oversized inventories withhold their step. One owned discovery step
processes at most one directory or project-directory response. Candidate
directories require current native workspace/socket/catalog and Write proof
before directory-scoped status GETs, and that proof is rechecked before offering
session IDs. Duplicate directories and session IDs are offered once per cycle.
Unbound or denied directories cannot grant source admission. The default
instance status map is insufficient cross-workspace discovery. Previously
admitted session recovery remains independent of new directory enumeration
failures. No project/strategy metadata authorizes a source, executes a program,
mutates a harness workspace, or enters durable capture state.

Explicit retained supervisor preparation may select the read-only OpenCode API
alongside the qualified Codex file profiles. API discovery merges current
busy/retry candidates with the endpoint-scoped admitted-session recovery index;
all candidate acquisition still rechecks exact native witness and current
repository admission. Discovery, source reading and publication use separate
retained owned workers, with shared reader/source/raw-frame bounds and
alternating harness priority. Pause, resume, rescan, deadlines, drain and native
storage ownership apply to every API lane. API source observations join the same
generation/ordinal catalog in selector order. They expose body-free coverage
only; API discovery has no filesystem root counts. API sources are not eligible
filesystem import targets until their import semantics are qualified.

A settled API source MUST release message pages, pagination scanners/cursor sets
and sanitizers. Body-free session/witness/lease and exact publication proof
metadata may remain. Periodic following reconstructs durable prefixes without
duplicate enqueue. Publication holds independent existing lease references and
an immutable digest of the exact sanitized repository/profile/cursor/body/
metadata/Thread binding. It MUST verify that digest, API witness, spool binding
and current directory/catalog/Write before preparing or publishing. Prepared
requests keep their exact identities and bytes. Unavailable repository
partitions MUST yield to other partitions while preserving earliest intent
within each partition; a blocked publication await cannot block source ingress.

This retained library supervisor qualifies observed API lifecycle classes and
bounded directory-aware discovery; hook/API alias joins, complete streaming
export, API history/import/publication management and real harness/storage
release gates remain unqualified. Configured process startup is enabled only for
the read-only API profile under the explicit LocalTrusted integration described
below; history/export management and broader release gates remain withheld. The
driver preserves unpinned API snapshot semantics; no consistency claim spans all
OpenCode database pages.

The Codex file profiles distinguish CLI 0.156.1 (`source=cli` or `source=exec`)
and Desktop 0.159.2 (`source=vscode`, `originator=Codex Desktop`). The CLI
accepts the qualified `codex_cli_rs`, `codex_exec` and inherited `Codex Desktop`
originators; an inherited originator does not change the product surface. The
shared reader also accepts CLI 0.156.1's exact Lapis ACP pair `source=vscode`,
`originator=lapis-acp` for native file inspection and local read-only snapshot
import. This does not select the Desktop profile or qualify Lapis ACP automatic
capture/JJ publication under JST-COLLAB-026. Other adapter labels and the
observed Desktop alpha version remain excluded; broader native
source/publication gates stay open. Codex's pinned rollout recorder writes the
session source independently of the environment-overridable originator. A
complete bounded first `session_meta` record MUST match the configured
surface/version, contain a native UUID session identity and an absolute working
directory before source admission. Header validation alone does not prove
repository binding or session activity. Product surfaces sharing one sessions
directory MUST NOT admit each other's files by filename or version similarity.
Selecting Desktop 0.156.1 is unsupported; existing CLI hook/version
qualification remains unchanged.

Codex location resolution uses explicit configured roots first, then
`CODEX_HOME/sessions`, then the server user's `~/.codex/sessions`. CLI and
Desktop share this convention but retain independent header/profile
qualification. Missing roots are unavailable and never created. Explicit roots
override ambient home settings; invalid relative/default home overrides fail
closed. Other harness conventions require their own qualification rather than
reusing Codex locations.

An approved Codex directory binds its logical selection to a canonical physical
directory. Metadata-only discovery resumes directory iterators across bounded
steps instead of restarting an incomplete scan at the first entries. Each step
examines at most 4096 entries and returns at most 256 regular JSONL candidates;
depth is at most eight nested directories and at most nine directory handles
remain open. No transcript body is read, no filename proves a product/session,
and no discovery candidate is serializable or debug-renderable. Nested symlinks
and special files are skipped; changed/unavailable entries are coverage gaps.
Root rebinding and candidate replacement are unavailable. Explicit cycle restart
revalidates the same approved root and can recover additions missed by directory
notifications. These candidates grant no session/repository/activity admission,
source lease, sanitization, checkpoint or publication authority.

A bound Codex reader consumes a discovery candidate rather than accepting an
arbitrary path. Preparation revalidates the approved root and physical file
before and after exact surface/header qualification. Inspection compares the
opened file's device/inode with the candidate before parsing the header; each
later file read retains that expected identity. Poll revalidates the root/file
before and after reading, including before returning an already pending exact
batch. Replaced files, symlinks and rebound roots are unavailable rather than
permission to read a new source through the old candidate. The device-only
wrapper and its raw batches have no persistence/debug encoding.

An exact sanitized durable acknowledgement may still repair progress after the
source becomes unavailable. Explicit rediscovery with the same caller-supplied
source identity and retained cursor can start the replacement file's new
generation without resetting source sequence numbers. In-place growth/rewrite
and acknowledged-prefix verification retain the bounded reader semantics below;
the existing unbound reader's replacement semantics are unchanged. Bound
preparation does not allocate stable source identities, lease readers, match a
repository, admit activity or authorize sanitization/publication. Source
registration and worker composition are separate capabilities. Callers must
separately revalidate current native directory/catalog bindings and service
authorization before publication.

Header-only qualification consumes an approved candidate and retains its exact
header witness without allocating a source or reading transcript batches. It
exposes only device-memory session/directory context for independent admission.
The worker MUST finish repository and activity admission before acquiring the
installation-private source registry; unsupported, unbound and inactive
historical discovery MUST NOT allocate registry entries or reader locks.
Qualification and registration alone do not implement those worker checks.

The version-1 source registry binds the existing installation identity and
allocates opaque nonzero source IDs from a persisted monotonic ordinal. It
retains only source IDs, ordinals and domain-separated canonical-location,
physical-file and exact-header digests, never raw paths, headers or bodies.
Known canonical locations retain their channel ID across replacement/rotation;
exact physical/header witnesses join hard-linked locations and overlapping
approved roots. Product profiles do not create separate channels for the same
physical source. Logical native-session joins remain the independent alias
publication contract below.

Conflicting location/witness/physical mappings withhold registration without
changing the catalog. A known physical file with a new header at an unknown
location waits for rediscovery of a known location; bare device/inode reuse
never grants channel adoption. Retained physical aliases may conservatively
withhold a recycled inode, rather than silently merge source channels. Header
changes require renewed qualification and registration before a leased reader
returns new-generation batches. Prior source progress is supplied separately;
registration does not reset sequences, rewrite journals or evict history.

The owner-only registry lives outside checkouts and publishes its bounded JSON
catalog by synchronized temporary-file replacement and directory sync under a
short exclusive registry gate. Capacity is one through 4096 retained sources, at
most 4096 location aliases, 4096 header witnesses and 4096 physical aliases,
with a 2 MiB catalog limit and 8194 durable coordination/catalog files. One
additional recognized temporary file is permitted for crash recovery; exhausted
bounds stop new acquisition without evicting acknowledged identity evidence.
Unknown files, unsafe permissions, foreign owners, symlinks, state rebinding,
invalid installation/identity records and ambiguous aliases fail closed. An
initialized registry with a missing catalog is unavailable and MUST NOT restart
allocation at ordinal one. Complete private-state loss requires separately
qualified recovery; registration does not claim backup/restore authority.

Each registered reader retains both a source-channel lease and a physical-file
lease. Nonblocking OS locks exclude simultaneous readers across overlapping
roots, profiles and processes sharing this installation registry, while
independent sources can be read concurrently. Distinct installation registries
remain independent. Leases explicitly unlock on drop and are released by the OS
after process death. Reader preparation and each poll revalidate the private
directory and exact lease file identities before returning source batches;
rebound locks or source/root replacement withhold results. Durable exact
acknowledgements can still repair already-sanitized progress when unavailable.
Source registration does not authorize a repository, assert activity, publish,
checkpoint or enable automatic server startup. Configuration and whole-server
resource ownership remain independent prerequisites.

The read-only Codex source reads at most 1 MiB of new bytes and 256 complete
records per poll, keeps at most 1 MiB for a partial record and represents an
oversized complete record as an omission for subsequent sanitization. Invalid
complete UTF-8/JSON also proceeds to safe omission rather than blocking the
tail. Partial records never acknowledge offsets. Header inspection is bounded by
1 MiB. Raw records and directories remain in memory in types without persistence
serialization or debug rendering; only the sanitization stage can construct
durable observations. A batch and its acknowledgement position remain stable
until the sanitized spool acknowledges that exact identity.

Source generations bind the physical file identity and complete first record.
Replacement, truncation or contradictory acknowledged prefix bytes start a new
generation, retaining monotonically increasing source sequence numbers and prior
cursor journals. Resume and changed files verify the acknowledged SHA-256 prefix
in at most 4 MiB steps before reading new records. A metadata change between
verification steps restarts verification, and a change during a read withholds
the candidate. This is bounded work per poll, not constant total verification
cost; long prefixes can take multiple scheduling steps. Same-handle partial
bytes are discarded on file changes to prevent splicing old and new records. The
source reader has no sanitization, activity-admission, publication or checkpoint
authority. Native transcript projection and leased reader preparation are
independent capabilities; active admission/backfill and supervised worker
composition remain separate M18.2 gates.
