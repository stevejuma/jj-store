# Standalone native deployment

The [cold server report](../../../../../docs/evidence/m3-server-rustfs.json)
records `python3 scripts/rustfs-contract.py --run --server` against the pinned
image. It creates a repository through authenticated operator RPC, attaches two
physical workspaces using the custom executable, and verifies
describe/bookmark/log/op-log/ evolog and binary content after killing both
server and RustFS and deleting rebuildable process/index state. Unsnapshotted
client edits survive; repository and operator revocation take effect on later
requests. Owned processes, container and volume are removed by fixture guards.

This qualifies explicit loopback deployment; valid shared TLS, complete ordinary
command semantics, recoverable init/attach and other release platforms remain
separate gates. Standalone PTY tests cover paging, color, JSON and missing-pager
hints independently of this storage run.

The
[attachment regression report](../../../../../docs/evidence/m3-attachment-rustfs.json)
retains the standalone cold server gate after durable attachment journals and
metadata staging. Lost-acknowledgement attachment and cache-loss retry tests use
a hosted loopback service with memory storage; they remain distinct from this
real-storage lifecycle run.

The
[initialization deployment report](../../../../../docs/evidence/m3-init-rustfs.json)
creates the repository with the actual `jj store init` command, repeats it with
the same durable identities, and retains the nine-probe cold server/revocation
gate. The OS-facility initial-admin provisioning fixture separately generates,
reuses, reads and deletes a randomly scoped credential account.

The
[token-command deployment report](../../../../../docs/evidence/m3-token-cli-rustfs.json)
retains nine real-storage probes while granting through custom `jj`, revoking
through standalone `jj-store`, and retrying the original grant after revocation.
The retry returns the original receipt without restoring revoked access.

The standalone server harness now runs native initialization, attachment, token
administration and cache-free restart over TLS with the development override
disabled. A dedicated cold process fixture checks trust, hostname and plaintext
rejection. Earlier reports retain their original plaintext-loopback scope.

The
[TLS deployment report](../../../../../docs/evidence/m3-tls-server-rustfs.json)
retains nine probes with zero failures. The
[M3 gate audit](../../../../../docs/evidence/m3-gate-audit.md) maps native CLI,
concurrency, authorization, presentation and recovery evidence to the
vertical-slice gate, and records remaining M4/M5/M6/M8a work explicitly.

Native query resource qualification uses a release fixture executable and an
external process measurement harness. It records linear and wide-head graphs at
the configured commit ceiling, cold/warm/filter/operation reads, native
byte/work counts, retained cache bytes, elapsed/CPU time and peak process RSS.
Fixture storage is explicitly separate from RustFS and service authorization
costs. The report is qualification for its recorded host and inputs;
first-release whole-server/deployment performance remains an M8a requirement.

The query runtime adapter is requalified by the
[M4 RustFS rerun](../../../../../docs/evidence/m4-runtime-rustfs.json): all 11
harness cases pass, including native transport, cold publication recovery and
TLS server/CLI restart and token revocation. This is storage/lifecycle evidence,
separate from the native query resource measurement.

The
[metadata admission audit](../../../../../docs/evidence/m4-query-admission-audit.md)
records release graph and content runs after predecode shape admission,
including 50,000-entry trees, 65-term merges and 256 MiB verified file
streams/ranges. Immutable fixture measurements remain separate from
RustFS/network and browser qualification. Native synthesized merges and
aggregate worker budgets remain explicit follow-on qualification.

Native predicate qualification includes accepted byte/line boundaries, repeated
logical paths sharing immutable blobs, recursive parent merge admission,
criss-cross histories, path-depth rejection and direct pinned-upstream result
comparison. The native file-reader runtime adapter additionally requires its
bounded-poll regression and real RustFS/CLI lifecycle requalification.

The [predicate audit](../../../../../docs/evidence/m4-native-predicate-audit.md)
retains final release measurements and a separate eleven-case RustFS rerun after
the returned-file runtime repair. Native parity uses a direct upstream oracle;
preflight admission never substitutes its own predicate results.

Native graph resource qualification includes explicit preparation of a cold
index before prepared/warm reads. Its retained report distinguishes admission,
index construction and graph evaluation for linear and wide-head histories. The
fixture store excludes service authorization and deployment I/O costs; native
lifecycle tests cover authenticated task status, revocation and cancellation.

The M4.1 HTTP read gate is audited in
[the transport report](../../../../../docs/evidence/m4-http-read-gate.md), with
retained
[pinned RustFS results](../../../../../docs/evidence/m4-http-rustfs.json).
HTTP/CLI history at a fixed operation, binary ranges, zero read publications,
cold cache-loss recovery and revocation pass on the production TLS listener.
HTTP/1.1 and HTTP/2 negotiation, incomplete TLS handshakes and connection
admission are regular cold process/unit tests. This remains single-node storage
evidence; full release performance and browser qualification are separate gates.

The committed-event contract and admission defaults are documented in
[the SSE evidence guide](../../../../../docs/evidence/m4-committed-events.md).
E-M4-015 in the ledger combines focused service/HTTP evidence with the
production TLS and real RustFS deployment fixture. The full pinned-image run
passed the single-node restart, cache-loss replay, `Last-Event-ID`, live
revocation and cleanup contract. Multi-node routing, external certificate
provisioning and load balancers remain outside this qualification.

Generated read contracts live in schemas/openapi.json and
web/src/lib/api/generated.ts. Contract checks regenerate both into memory and
reject drift. Serialization fixtures verify native ID widths, tagged enums,
ordered tuple entries and nullable predecessor fields independently of browser
rendering. Existing native golden fixtures continue to qualify persistence.

Browser fixtures consume serialized native read responses and explicitly test
source escaping, snapshot selection, request cancellation and credential
lifecycle. Static serving has separate compiled-route tests. Fixture-backed
browser evidence does not substitute for real RustFS/browser deployment
qualification; the ledger retains that distinction.

Cold Playwright acceptance uses a disposable loopback Rust host with the
production static asset router and CSP. API requests in this lane are fulfilled
from freshly captured authenticated native fixtures. Production RustFS/TLS
browser acceptance remains distinct from this static/client conformance lane.

Browser acceptance builds production assets and starts the actual Rust asset
router under its CSP. Disposable captured native HTTP fixtures exercise browser
interaction and accessibility; production RustFS/TLS qualification is recorded
separately. Visual review remains visual-pending.

Typed native command preparation is qualified by
[the M5 preparation report](../../../../../docs/evidence/m5-command-preparation.md).
Native merge/rewrite/reporting fixtures and in-flight authorization/cancellation
checks use test storage; they do not qualify durable public mutation endpoints
or deployed RustFS/browser behavior.

[Durable typed command evidence](../../../../../docs/evidence/m5-durable-commands.md)
qualifies the application intent V1 digest and selected result profile while
retaining existing native/token request digests and WAL envelope versions.
Fault-store and authenticated native tests cover duplicate races, checkpoint
recovery, lost acknowledgements, authorization and selected receipt corruption;
real deployment qualification remains a separate gate. UI work is deferred.

Typed HTTP mutation conformance captures actual preview/apply/status responses
for schema validation, including committed, not-known and unknown outcomes.
Native fixtures cover all advertised commands, protected policy, revocation,
stale input, worker admission and recovery from an HTTP timeout after manifest
CAS. These are fault-injecting memory-store proofs; real RustFS/TLS deployment
evidence remains separate. Generated TypeScript declarations change with the
API; browser features and visual work remain deferred under M4.3.

ReconcileOperations is qualified with real pinned native transactions over
memory storage: multi-head preview/apply/retry, three-head recursive common
ancestors, conflicted bookmark parity, descendant/workspace rewrites,
stale/invalid base rejection, protection, limits and explicit HTTP application.
Reports compare to the selected base view. Existing request/result codecs retain
V1 compatibility; only the typed command union and advertised capability gain a
variant. Real storage and larger resource scenarios remain independent
operational evidence.

Abandon adapts the pinned CLI descendant-transform algorithm with attribution in
the third-party notice. Native fixtures cover every option combination, adjacent
and merge targets, conflicted bookmark terms, multiple workspace replacements,
binary/mode/copy/symlink/submodule preservation, retained evolution and history,
atomic policy/input failures, hidden no-op and HTTP receipts. Added report
fields are default-empty and skipped when empty; earlier V1 serialized reports
reencode without additional fields. This does not replace RustFS/deployment
acceptance.

[Atomic conflict term evidence](../../../../../docs/evidence/m5-resolve-terms.md)
covers shared read/mutation fingerprints, all native term kinds, multi-sided and
unlabeled conflicts, partial resolution, descendant/reference/evolution updates,
no-staging invalid batches, authorization, durable retry and HTTP conformance.
Native codecs, stored reports and prior V1 receipt bytes are unchanged. This
term/delete slice does not qualify uploaded replacements or deployed RustFS.

[Authorized upload evidence](../../../../../docs/evidence/m5-conflict-uploads.md)
covers binary streaming, scoped record identity, atomic file replacement and
copy-history selection, checkpoint/restart receipt recovery, lost upload
acknowledgements, corrupt/missing data, authorization changes and HTTP limits.
Native/WAL formats and older StoredResult V1 bytes remain unchanged. These
native and fault-store proofs do not replace real RustFS/TLS deployment gates.

Conflict-resolution report compatibility requires older V1 Prepared results to
decode and reencode byte-for-byte with the added optional field absent. New
previews and retained receipts enumerate the target and affected descendants
without changing native object identities or WAL publication formats.

Adapter delivery qualification combines a selected WAL-reference regression,
production-coordinator fault tests with synced local artifact files, and native
service-token source acceptance. These are independent of Tantivy durability and
real RustFS/TLS deployment qualification. Corrupt selected tails must fail
bootstrap before destination effects, and missing local files cannot be masked
by an advanced remote cursor.

Tantivy 0.26.2 is pinned with mmap and LZ4 support. Its local durable projection
fixtures use real index files plus the production adapter runner and selected
WAL reader. Synthetic extraction in these fixtures proves destination
durability, not native extraction, view visibility, HTTP search or RustFS
deployment.

Native projection acceptance additionally uses real JJ metadata, valid copy
lineages, configured bookmark/workspace scopes, retained predecessors and native
tree diffs. Incremental updates and full rebuilds must produce equivalent hits.
The generic codec fixture contains a deliberately inconsistent shared copy path;
it remains rejected by diff execution, and search uses a valid scope fixture.

Selected-view search tests compare complete commit visibility to the pinned
native graph's all() revset. They distinguish retained historical metadata from
visible commits, verify filtering before pagination, and keep both selected view
and index generation stable across later writes. Foreign repository/epoch
filters and revoked tokens fail explicitly. This is native/local-index evidence,
separate from HTTP pagination and RustFS deployment qualification.

Search serving qualification uses real native service sessions, selected WAL
history and local Tantivy files. It verifies signed pagination, scope filters,
current-view ambiguity, bounded freshness waits, admission, revocation, expiry,
serving restart, output/retention budgets and durable-versus-visible
degradation. Reader catch-up tests prepare durable artifacts separately; an
invented one-second index-build expectation is not a release performance
requirement. HTTP routes and cold server wiring are covered by
registered-handler and executable fixtures; pinned RustFS qualification is
recorded separately.

Search HTTP compatibility uses real registered Axum handlers with a native
repository and local Tantivy projection. Generated OpenAPI/client shapes are
checked against captured page, status, capability and freshness-error responses.
This local evidence does not substitute for RustFS deployment qualification.

Search configuration recovery is qualified with native source content, local
index reopen and a failed replacement commit. The evidence distinguishes
rebuildable index configuration from unchanged repository IDs and committed
positions; a restart must not activate incompatible source projections.

Server-search acceptance combines strict configuration and cold executable
checks with native repositories, real local Tantivy files and background-runner
HTTP fixtures. Cache loss/reopen and token revocation require observed status
transitions, not sleeps alone. The pinned RustFS/TLS process fixture
additionally deletes all local search state and requires exact remote-generation
restoration.

Optional adapter snapshot artifacts use a repository/epoch/full-identity
namespace and a V1 canonical descriptor and pointer. Their stream, corruption,
revocation, cancellation and competing-pointer fixtures are deterministic memory
storage evidence. Tantivy fixtures export committed generations, restore older
generations before incremental replay, validate complete payload/schema/index
readability and rebuild plus repair an unusable snapshot. The pinned RustFS/TLS
fixture then removes local server/search state, restarts storage and the server,
and restores the exact generation and results. It records a 5.436-second cold
rebuild and 2.007-second restore against a 30-second budget. Snapshot support
changes no native identity, WAL, receipt, replay cursor or HTTP format.

M4.10 adds four browsing routes for reference lookup, recursive paths,
operation-change groups and changed-path summaries. Generated contracts are
checked against 51 captured registered-handler responses; existing complete read
routes remain compatible. This local native/API evidence does not replace the
separate built-browser and pinned RustFS qualification in M4.14.

The generated graph contract requires `GraphBookmark.remote` as a nullable
string. `null` identifies a local bookmark and a string preserves the native
remote name; OpenAPI and TypeScript therefore retain local and remote bookmarks
without overloading the display label or changing commit identities.

M4.16 keeps the V1 mutation request, prepared result, committed receipt and
durable status shapes intact. Generated and captured contracts add the semantic
409 `protected_transition` error code without changing native identities, stored
receipts or WAL bytes; 401/403 remain credential failures. Captured browser
fixtures merge real registered-handler read and mutation responses, while the
separate pinned RustFS lane proves one mutation through the built application
and native repository.

EditFile additively extends the existing tagged V1 command union and advertised
capability list. Existing command encodings, intent digests, prepared results,
stored receipts, WAL records, manifests and native object codecs are unchanged.
New requests retain canonical bookmark targets, paths, UTF-8 contents and change
descriptions inside the existing mutation request and receipt workflow. Native
lifecycle, generated-contract and captured-browser fixtures qualify the new
variant; visual review remains a separate presentation gate.

Manifest format 2 adds selected AgentStore/ReviewStore roots and independent
state versions. Zero-valued extension fields and absent domain history remain
canonically omitted, so retained format-1 manifests, WAL records, receipts and
mutation/upload intent bytes still decode byte-for-byte. Native RPC and OpenAPI
snapshots add `code_state_version`; this repository is unreleased, so no public
wire downgrade is promised. The explicit collaboration-enable publication is the
only supported format-1 adoption path and never rewrites native objects.

M9.4 additively registers `GET /v1/repos/{repo}/agent/sessions` and
`GET /v1/repos/{repo}/agent/sessions/{session}` plus the bounded
`GET /v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}` range
and `GET /v1/repos/{repo}/agent/associations`. It also registers checkpoint
collection, current-detail and exact-version routes. OpenAPI and generated
TypeScript own the AgentStore snapshot, coverage, summary, detail, transcript
descriptor, chunk, typed relation subject, relation link, checkpoint summary,
code anchor and page schemas. Responses preserve native repository, stable
AgentStore and immutable object identity widths; collection, transcript and
relation ranges use opaque process-local continuations. There is no
persistence-format, WAL, receipt or native-ID change. Captured LocalTrusted
responses validate all seven handlers as part of contract drift checking. A
continuation intentionally resets across server restart, expiry and epoch
replacement and is not a durable compatibility token.

M9.4 also additively registers ReviewStore thread collection and stable-thread
detail routes. Their generated schemas own the review read snapshot, thread
summary/page/detail, anchor, selected comment content and immutable event DTOs.
Collection continuations use the distinct `jsr1` signature domain; existing
`jsa1` AgentStore continuations and every persistence format remain unchanged.
It additionally registers typed review-command apply and durable-status routes.
Their strict request omits principal/publication identity; generated outcomes
retain the exact request, derived publication, intent digest, accepted snapshot
and immutable ReviewStore result identities. Captured LocalTrusted responses
validate all four handlers, replay and `not_known` status without changing WAL,
manifest or ReviewStore object formats.
