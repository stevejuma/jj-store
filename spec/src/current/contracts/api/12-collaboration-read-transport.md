# Collaboration read transport

`GET /v1/repos/{repo}/agent/sessions`,
`GET /v1/repos/{repo}/agent/sessions/{session}` and
`GET /v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}`, plus
`GET /v1/repos/{repo}/agent/associations`,
`GET /v1/repos/{repo}/agent/checkpoints`,
`GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}` and
`GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}`, plus
`GET /v1/repos/{repo}/reviews/threads` and
`GET /v1/repos/{repo}/reviews/threads/{thread}`, are bounded collaboration
routes. LocalTrusted admits them through the existing literal-loopback
protection and installation principal, without a bearer credential.
Authenticated deployments retain their repository bearer policy. Both modes
require current repository read permission before and after every root-pinned
read.

The JSON response contains version 1, an exact AgentStore read snapshot, bounded
session summaries and an optional continuation. The snapshot carries repository,
epoch, publication position, manifest revision, AgentStore root digest and state
version. Session rows keep full stable session and immutable snapshot
identities; integration, state, optional parent, coverage and bounded object
counts are service-owned presentation data rather than serialized WAL records.
Nullable first and last activity milliseconds are derived only from exact event
IDs matched to their selected sanitized observation evidence. Missing
observation timestamp fields may be recovered from the original RFC 3339
timestamp in the matched sanitized Codex JSONL record, only for the qualified
CLI/Desktop profile, envelope version and local-transcript source/trust. The
explicit observation timestamp takes precedence. Unrecognized or malformed
native timestamps, unmatched evidence or a session beyond the 4,096-event timing
bound yields null timing fields. This read-only projection also recovers timing
for previously published transcripts; it never rewrites evidence or requires
reimport, and filesystem modification or ingestion times are not substitutes.

Message activity and transcript chunks expose an additive optional
`occurred_at_millis` field. It is the timestamp of the exact selected message
observation, with the same qualified native timestamp recovery and explicit
observation precedence as session timing. Missing, malformed or unmatched times
are omitted. Transcript page timestamps additionally match the selected current
source-generation-1 chunk's normalized message event and content; unsupported
generations remain unknown. Recovery inspects at most 4,096 selected events and
retains the original cursor/root pin across pages. Times never derive from
sequence numbers, session start, filesystem metadata or ingestion.

Tool activity adds optional nonnegative `duration_millis`, omitted when the
exact selected start/end evidence cannot prove an ordered span. Qualified Codex
call/result timestamps and explicit timed hook lifecycle endpoints preserve
source/profile identity and the 4,096-event bound. Duration may be known while
outcome remains unknown. Native and companion pins retain the original state; no
storage, normalization or receipt encoding changes.

The detail route accepts one canonical full stable session ID. It resolves that
key only through the current selected AgentStore session catalog and returns 404
when no matching selection exists. Its response repeats the exact read snapshot
and session summary, then supplies ordered immutable IDs for evidence and event
batches, stream descriptors for each selected transcript manifest and stable
tool-call to immutable tool-snapshot links. It also resolves at most 4,096
sequence-ordered activity events, eight MiB of sanitized message/tool content
and 64 exact code associations from those selected objects. Activity variants
retain session state, message, tool, permission, observed-file or capture-gap
semantics. A final tool record carries its stable call and snapshot identities,
native name, kind, state, optional sanitized input/output, observed paths and
matching code associations. Separate truncation flags make both bounds visible.
Each transcript descriptor retains the manifest and source identities,
generation, source/decoder/sanitizer versions, representation, logical byte
count, record count and gaps. It never fetches unselected objects, returns raw
capture payloads or treats an immutable snapshot ID as a stable session route.

The transcript route requires the full stable session ID, immutable transcript
manifest ID, source instance, generation and a caller-selected byte ceiling. It
first proves that the current or cursor-pinned session selects that manifest and
that the manifest owns the requested source generation. Version 1 pages return
only service-owned sequence, media type and UTF-8 sanitized content DTOs. A page
contains at most 256 records and eight MiB; a record that cannot fit the
selected budget fails explicitly instead of being truncated. Missing sessions,
transcripts or streams return 404.

The associations route accepts exactly one typed subject: `session`,
`tool_session` together with `tool`, `operation`, `commit` or `checkpoint`.
Missing, incomplete or contradictory selectors fail as an invalid request. A
version 1 page repeats the exact typed subject and returns ordered immutable
links containing either a checkpoint-version or association object ID. An
association link additionally returns the verified stable association, session
and tool identities, relation, native before/after operation and commit anchors,
change IDs, paths and creation time needed for bounded browser context lookup. A
valid subject with no links returns an empty page rather than inventing an
association. Raw AgentStore objects are not serialized by this route.

The checkpoint collection enumerates stable checkpoint IDs and their currently
selected immutable versions from one exact AgentStore root. Summaries retain the
stable checkpoint, immutable version, session, optional turn/scope, exact native
before/after anchors, code relation, evidence/trust, coverage, capture status,
execution outcome, bounded reference counts and prior version link. The stable
checkpoint detail route returns its current selected summary plus the exact
session snapshot, transcript, event, tool and association links.

The version route accepts the stable checkpoint ID and one full immutable
checkpoint-version ID. It can return only the current selection or a version in
that selection's validated `supersedes` chain. A foreign or unselected version
returns 404 even when some other selected AgentStore object refers to it. Both
detail routes pin the current root for the whole request and never reinterpret
the stable checkpoint ID as an immutable object ID.

The review collection enumerates stable thread IDs and their current immutable
snapshots from one exact ReviewStore root. Its version 1 response carries the
repository, epoch, publication position, manifest revision, ReviewStore root
digest and state version. Thread summaries preserve exact snapshot, anchor and
event-page object IDs, the validated typed anchor, workflow state and version
counters. The collection rejects an anchor whose repository identity differs
from the pinned repository. Stable-thread detail returns the same validated
anchor value, ordered selected comment revisions with safe visible bodies or
explicit tombstone/moderation states, and the complete bounded immutable event
stream. The combined visible comment bodies in one detail response are limited
to eight MiB. Each item retains its exact object identity and authenticated
actor. Missing stable threads return 404; no route reads an arbitrary
ReviewStore object or an unvalidated historical snapshot.

`POST /v1/repos/{repo}/reviews/commands` accepts a strict version-1 request ID
and one typed ReviewStore command. The authenticated session supplies the actor
and derives a repository-bound publication ID before the existing ReviewStore
WAL publication path runs. A committed response returns the exact selected WAL
receipt and immutable result identities; a timeout or unavailable publication
returns `outcome_unknown` with the original request ID.
`GET /v1/repos/{repo}/reviews/commands/{request_id}` reauthorizes current write
access and resolves that durable receipt after server reconstruction.
`not_known` never proves rejection. Both routes reject caller-authored
principals, publication IDs and unknown fields.

`limit` defaults to the common page size and accepts 1 through 256. The opaque
continuation is bound to the entire snapshot and original page size, retains its
immutable root after later publications and is never suitable for a permanent
URL. Transcript continuations additionally bind the stable session, manifest,
source instance, source generation, byte ceiling and last returned sequence.
Association continuations additionally bind the complete typed relation subject
and the root-local relation page and offset. A continuation for one subject
cannot resume another subject. Checkpoint collection continuations bind the
radix key after the last stable checkpoint in the selected catalog. Review
thread continuations use a separate signature domain and bind the corresponding
ReviewStore root/state version and last stable thread radix key. Invalid or
query-mismatched continuations return `invalid_cursor`; expiry, process restart
or epoch replacement returns `cursor_reset`. Request concurrency, retained pins,
transitive closure objects and lifetime are bounded by the shared read limits.
The route has no request body, mutation capability, receipt or agent dispatch
behavior.
