# Compatibility and release

Rust 1.97.1 and edition 2024 are the starting toolchain. New native repositories
ship first, with repository-scoped token authentication. M8a hardens that
release; M8b handles controlled migration later. A new image or topology
invalidates old storage qualification until contract tests rerun. Read-only
proof results do not constitute deployment or publication approval.

## Requirements

| ID             | Requirement                                                                                                                                                                         |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-COMPAT-001 | Both JJ crates MUST be pinned to the same tested 0.45.1 release and upstream revision, with licenses, trait coverage, and golden identities recorded before native storage rollout. |
| JST-COMPAT-002 | M0 MUST prove compiled extension/context/query boundaries and real RustFS conditional writes before dependent storage milestones can be completed.                                  |
| JST-COMPAT-003 | Release qualification MUST include empty-cache recovery, restore drills, bounded performance measurements, security, semantic parity, and cold browser/lifecycle evidence.          |
| JST-COMPAT-004 | Migration and passive replication MUST remain deferred for the first release; failed future imports MUST leave source repositories untouched and MUST NOT silently remap IDs.       |

## M0 audit evidence

The pinned reference/license and field audit is recorded in
[upstream lock](../../docs/upstream-lock.md) and
[codec matrix](../../docs/codec-coverage.md). The single-node RustFS probe uses
a disposable volume/container with signed S3 requests, 20 rounds of 16
contenders for both create and CAS, and process-kill recovery. Harness
self-tests run in the normal test/spec lanes; real Docker conformance is an
explicit operator lane.

## M0 extension qualification

Compiled M0 probes live in jj-store-jj compatibility tests and the pinned
upstream CLI example. `scripts/smoke-upstream-cli.py` runs the latter with
isolated configuration and disposable storage. Query probes exercise a LocalSet
with a timeout and row limits; production traversal/memory budgets remain M4.

The second cold RustFS run adds an actual truncated HTTP upload; its object
stays absent. [M0 cold evidence](../../docs/evidence/m0-rustfs-cold.json)
retains all eight probe outcomes. All M0 prerequisites are now met for this
initial topology.

## M1 codec conformance

The frozen M1 profile vectors cover all seven stored kinds. Native codec tests
compare supported kinds against the pinned SimpleBackend and SimpleOpStore; copy
and submodule coverage uses the audited native ContentHash profile because
SimpleBackend cannot persist those variants. Codec conformance does not qualify
the later storage transport or publication coordinator.

## Compiled storage client qualification

The disposable RustFS harness accepts `--rust-client` to qualify the Rust
transport as well as the independent HTTP probes. The compiled lane covers
conditional-create/CAS races over independent clients, stale tokens, range
reads, 8 MiB native file upload/read/duplicate verification, metadata objects
and a proxy that drops a real successful write acknowledgement. Default unit
lanes leave this external test explicitly ignored.

The inventory-source integration is also checked by a fresh
[compiled native storage run](../../docs/evidence/m2-inventory-native-rustfs.json).
This confirms the checked metadata/file reader remains compatible; it does not
establish WAL publication or recovery correctness.

The compiled RustFS lane also publishes native operations through the manifest
coordinator. It tests independent-client sibling races, an intentionally lost
manifest acknowledgement, checkpoint/receipt persistence, missing dependency
rejection and explicit corruption detection. The harness then kills/restarts its
owned RustFS container and launches a fresh Rust process to recover selected
history, retained evolution dependencies and receipts without in-memory caches.

[M2 publication evidence](../../docs/evidence/m2-publication-rustfs.json)
records the extended 19-publication model, repeated checkpoints and
original-result retries against the pinned image, then a fresh-process
retained-data audit.

The [M3 storage regression run](../../docs/evidence/m3-auth-storage-rustfs.json)
retains the native publication/recovery gate after extracting the shared checked
manifest reader for authentication. It does not claim deployed RPC/HTTP token
acceptance, which remains a separate M3 integration gate.

The M3 native lifecycle fixture found that physical file snapshotting invokes
backend I/O on upstream worker threads without an inherited Tokio reactor.
Native factories now require a caller-owned multithread runtime and enter it per
poll on those threads. Library lifecycle, cache rebuild and policy regression
coverage is in `crates/jj-store-cli/tests/native_lifecycle.rs`.

The
[native session storage regression](../../docs/evidence/m3-native-session-rustfs.json)
retains all ten RustFS storage/publication probes after sharing the committed
inventory reader. It uses the storage harness's test principals and does not
replace authenticated RPC/HTTP or executable CLI acceptance.

## Standalone native deployment

The [cold server report](../../docs/evidence/m3-server-rustfs.json) records
`python3 scripts/rustfs-contract.py --run --server` against the pinned image. It
creates a repository through authenticated operator RPC, attaches two physical
workspaces using the custom executable, and verifies
describe/bookmark/log/op-log/ evolog and binary content after killing both
server and RustFS and deleting rebuildable process/index state. Unsnapshotted
client edits survive; repository and operator revocation take effect on later
requests. Owned processes, container and volume are removed by fixture guards.

This qualifies explicit loopback deployment; valid shared TLS, complete ordinary
command semantics, recoverable init/attach and other release platforms remain
separate gates. Standalone PTY tests cover paging, color, JSON and missing-pager
hints independently of this storage run.

The
[attachment regression report](../../docs/evidence/m3-attachment-rustfs.json)
retains the standalone cold server gate after durable attachment journals and
metadata staging. Lost-acknowledgement attachment and cache-loss retry tests use
a hosted loopback service with memory storage; they remain distinct from this
real-storage lifecycle run.

The [initialization deployment report](../../docs/evidence/m3-init-rustfs.json)
creates the repository with the actual `jj store init` command, repeats it with
the same durable identities, and retains the nine-probe cold server/revocation
gate. The OS-facility initial-admin provisioning fixture separately generates,
reuses, reads and deletes a randomly scoped credential account.

The
[token-command deployment report](../../docs/evidence/m3-token-cli-rustfs.json)
retains nine real-storage probes while granting through custom `jj`, revoking
through standalone `jj-store`, and retrying the original grant after revocation.
The retry returns the original receipt without restoring revoked access.

The standalone server harness now runs native initialization, attachment, token
administration and cache-free restart over TLS with the development override
disabled. A dedicated cold process fixture checks trust, hostname and plaintext
rejection. Earlier reports retain their original plaintext-loopback scope.

The [TLS deployment report](../../docs/evidence/m3-tls-server-rustfs.json)
retains nine probes with zero failures. The
[M3 gate audit](../../docs/evidence/m3-gate-audit.md) maps native CLI,
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
[M4 RustFS rerun](../../docs/evidence/m4-runtime-rustfs.json): all 11 harness
cases pass, including native transport, cold publication recovery and TLS
server/CLI restart and token revocation. This is storage/lifecycle evidence,
separate from the native query resource measurement.

The [metadata admission audit](../../docs/evidence/m4-query-admission-audit.md)
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

The [predicate audit](../../docs/evidence/m4-native-predicate-audit.md) retains
final release measurements and a separate eleven-case RustFS rerun after the
returned-file runtime repair. Native parity uses a direct upstream oracle;
preflight admission never substitutes its own predicate results.

Native graph resource qualification includes explicit preparation of a cold
index before prepared/warm reads. Its retained report distinguishes admission,
index construction and graph evaluation for linear and wide-head histories. The
fixture store excludes service authorization and deployment I/O costs; native
lifecycle tests cover authenticated task status, revocation and cancellation.

The M4.1 HTTP read gate is audited in
[the transport report](../../docs/evidence/m4-http-read-gate.md), with retained
[pinned RustFS results](../../docs/evidence/m4-http-rustfs.json). HTTP/CLI
history at a fixed operation, binary ranges, zero read publications, cold
cache-loss recovery and revocation pass on the production TLS listener. HTTP/1.1
and HTTP/2 negotiation, incomplete TLS handshakes and connection admission are
regular cold process/unit tests. This remains single-node storage evidence; full
release performance and browser qualification are separate gates.

The committed-event contract and admission defaults are documented in
[the SSE evidence guide](../../docs/evidence/m4-committed-events.md). E-M4-015
in the ledger distinguishes focused service/HTTP evidence from the production
TLS and real RustFS deployment fixture. A Docker availability failure cannot be
reported as a passing deployment run.

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
[the M5 preparation report](../../docs/evidence/m5-command-preparation.md).
Native merge/rewrite/reporting fixtures and in-flight authorization/cancellation
checks use test storage; they do not qualify durable public mutation endpoints
or deployed RustFS/browser behavior.

[Durable typed command evidence](../../docs/evidence/m5-durable-commands.md)
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
