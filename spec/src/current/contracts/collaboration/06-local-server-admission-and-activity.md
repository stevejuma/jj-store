# Local server admission and activity

The first automatic worker serves LocalTrusted native repositories. Capture is
disabled unless agent_capture.enabled and an explicit harness list are set.
Selecting a harness authorizes its version-qualified locations for the server's
user account; explicit roots override conventions and supported home/data-root
overrides are honored. Configuration applies on restart; runtime resume cannot
override disabled configuration. Enablement requires write authority and a
selected current AgentStore root. Capture does not initialize missing legacy
domains or migrate old AgentStore encodings. Unsupported roots fail closed.

The format-2 agent_capture table defaults to disabled, four readers, a 30-second
rescan, at most 1024 tracked sources and a 64 MiB pending spool. Configured
bounds may reduce concurrency to one through four, rescan to one through 30
seconds, sources to one through 4096 and pending capacity to one through 64 MiB.
Each harness entry names a versioned product-surface profile and exact product
version; unknown profiles/versions, duplicate profiles/roots and unknown fields
fail configuration checks. Relative explicit transcript roots resolve against
the configuration file; absent roots use qualified conventions at worker
preparation. A read-only OpenCode API profile requires an explicit
credential-free loopback endpoint and cannot also name filesystem roots. Native
automatic capture is unavailable in filesystem-only serving. Configured Codex
CLI/Desktop startup uses the ordinary foreground server lifecycle; unsupported
reader profiles fail before state creation. After native resources and the
process state lock are owned, first-run startup may exclusively create an absent
private capture root, installation-bound control journal and generation-bound
import journal. It MUST sync that initialization before preparing discovery. An
existing root is opened without repair, permission changes, journal replacement
or renewed consent. Incomplete or corrupt existing policy withholds capture
while ordinary service remains available. Configuration checks and management
inspection MUST NOT initialize state. Missing optional import metadata continues
to withhold import management only. Restart MUST retain pause, generation,
source identities, accepted imports and exact pending publication requests.

Admit a registered directory or another physical JJ workspace only after bounded
discovery proves the native repository/profile and this installation's exact
Unix socket. Nested directories resolve to their actual workspace. Names,
filename patterns, path-prefix similarity and clock proximity are not identity.
Revalidate before publication; moved, missing, rebound or foreign-server
bindings are unavailable. Multiple roots of one repository are supported.
Mixed-repository content needs verified per-record partitioning; otherwise
withhold it with a gap.

Codex file batches retain one device-memory directory context. Only the exact
first qualified header and subsequent `turn_context.payload.cwd` records provide
context; message/tool content, arbitrary `cwd` fields and path similarity do
not. Directory values must be bounded absolute non-root paths without traversal
or control characters. A changed context cuts the batch before the changed row,
which begins the next batch with its own exact ending offset and prefix witness.
No batch is assigned wholesale to the original header directory after a turn
context change. The server's batch admission adapter passes this reconstructed
directory through the existing current native workspace/socket/catalog verifier;
registration membership and publication write authority remain separate checks.

Malformed complete JSON/UTF-8, oversized records and invalid/missing turn cwd
make context unavailable until another explicit valid turn context. A repeated
session header withholds the remainder of the generation, preventing foreign
session content from inheriting the original native session identity. Partial
records never change acknowledged context. Context state advances only with the
exact sanitized durable batch acknowledgement. Replaced/truncated/rewritten
generations reset context to the newly qualified header.

Restart reconstructs context from the same verified acknowledged JSONL prefix,
in the existing at-most-4-MiB verification steps with a 1-MiB partial-record
bound. This reconstruction neither emits old records nor trusts a cached
directory at the old offset. A changed prefix restarts verification or starts
the existing new-generation recovery. Raw directory histories have no
persistence or debug encoding and are not added to reader journal or AgentStore
shapes.

Native body sanitization refuses an unavailable directory context. A separate
safe-gap conversion retains only constant omissions, deterministic observation
identities and source progress, without inspecting or copying source bodies. The
worker may publish those gaps only to a separately authorized bound anchor; an
unavailable/revoked anchor withholds publication. Valid later contexts can still
be ingested after the gap. This capability is not active-session admission and
does not authorize historical import or automatic worker startup.

Capture directory discovery is a separate read-only verifier capability. It
resolves at most 64 physical ancestors to the nearest repository boundary,
rejects nested ordinary Git/unqualified JJ repositories and metadata-directory
paths, and never falls back through an invalid inner boundary. Its proof binds
the canonical workspace and repository metadata paths, physical directory
identities, validated connection bytes and native profile markers. Ordinary
working-copy edits do not alter this device binding. Admission also requires the
native repository in this installation's current published discovery catalog.
Revalidation repeats both proofs and rejects changed bindings or unregistered
repositories. It repeats discovery from the original session directory,
retaining aliases and new inner repository boundaries rather than inspecting
only the prior workspace root. Source directory paths stay in device admission
state and never become an AgentStore body. This directory proof is not write
authorization or a code checkpoint; publication must separately apply the
current service policy.

Qualified live status/hooks or new transcript records while enabled admit an
active session. Source growth alone does not assert model execution. Backfill an
admitted session from its beginning, then follow its tail. Restart resumes
admitted sources and pending tails even if they ended while offline. Existing
inactive sessions require explicit import; absent activity metadata waits for
new activity rather than bulk historical import. Transcript ingestion does not
authorize snapshots or change JJ operation heads. Checkpoints require separate
per-workspace client enablement. Ordinary Git-backed folders, remote hosts and
authenticated capture remain separately gated even where M13 serves folders.

The initial Codex activity probe arms at the exact observed file length. It
reconstructs directory context with bounded reads but ignores all records that
began before that boundary, including an already partial record completed later.
Arming completes through bounded stable-prefix steps. Changes before the initial
boundary has been read reset that unproven baseline to the current file length;
records missed during arming remain historical and wait for a later new start.
Only a new supported `event_msg.payload.type=task_started` with a verified
directory context establishes eligibility. Arbitrary growth, user/tool text,
completion records and historical unfinished starts do not. This proof means
activity was observed while monitoring; it does not assert continuing model
execution. CLI and Desktop headers qualify independently. Live status/hooks
remain separate qualification; a session whose start was missed waits for a new
start or explicit import.

Activity probing has no source IDs, registry writes, reader leases, cursor
acknowledgements or publication capability. Each step scans at most 1 MiB and
256 complete records, retaining at most a 1 MiB partial record. Metadata changes
discard partial scanning and verify the previously scanned complete prefix in
at-most-4-MiB steps. Changed prefixes, truncation, replacement and rebound roots
withhold eligibility and require fresh qualification/arming. A returned activity
proof binds the exact physical/header and file snapshot plus the event's
device-memory directory; append races invalidate that snapshot until another
verified probe step. Repository admission is checked before acquisition and
repeated before publication; write authorization remains separate.

The private V1 source catalog adds an optional, bounded set of domain-separated
native-session admission digests. Empty sets omit the new field and preserve
legacy catalog bytes; generic registration cannot create admission markers.
Active acquisition selects registration and admission in the same atomic catalog
replacement only after current activity proof and both reader leases succeed.
Previously admitted sessions may reacquire after restart without another start,
including offline completion and same-session rotation, through exact current
header qualification and unambiguous retained source aliases. New native
sessions at known locations need fresh activity. Read-only existing-registry
lookup creates no directory, entry or lock. A legacy registration without an
admission marker does not authorize automatic historical capture. At most 4096
admission digests are retained without eviction, within the existing catalog
byte bound. Admission is neither repository authority nor permission to publish
or checkpoint.
