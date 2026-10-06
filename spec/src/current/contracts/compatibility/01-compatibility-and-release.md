# Compatibility and release

The additive Blame V1 response and native/local routes use pinned jj-lib 0.45.1
same-path, all-parent annotation semantics. Source URLs retain their behavior;
`file_view=blame` opts into attribution. Continuations are process-local and
algorithm-bound; eviction/restart requires an explicit same-selection reset.
Synchronous comparisons retain upstream latency and cooperative cancellation;
this slice makes no hard per-comparison timing or cross-version cache guarantee.

Available V1 path metadata adds `attribution: path_change | snapshot`. Snapshot
metadata describes the selected revision when bounded first-parent history finds
no change; clients identify it with an accessible Snapshot icon. Missing
attribution on older responses retains path-change presentation. Existing
unavailable reason variants remain readable. Additive `last_change_mode`
defaults to `quick`; `history` reuses bounded JJ file-history batches. Files
first shows quick values, then refines automatically while retaining values
through pending/error states. Neither mode schedules changed-path extensions. JJ
0.45.1 commit format-6 and changed-path format-0 remain standard disposable
adapters. Source indexes and authoritative repository formats remain unchanged;
no migration is required.

Managed core task routes and the optional default-off `automatic_code_index`
capability are additive to HTTP V1. They reuse native/local task report schemas
and preserve existing advanced routes and explicit advanced preparation. Private
derived reservations use version2 for profile and epoch/generation bindings;
version1 records retain their existing bytes. Standard JJ index format6 and
authoritative objects, manifests, WAL, receipts and identities stay unchanged.
No source repository migration or repair is required.

CopyFamily is an additive native RPC V1 method with owned typed seed flags,
tightening limits, complete sorted identities and traversal work. Existing
Inventory and InventoryPage wire fields, native objects, manifest, WAL and
receipts remain unchanged. Only explicit Unimplemented selects the older peer's
bounded inventory fallback; other errors remain visible. Exact protobuf
round-trip and real loopback consumers qualify this boundary separately from
complete scale, installed runtime and release acceptance.

Native kind task transport is additive to V1 and preserves existing graph task,
RPC, native object, WAL, manifest, receipt and cache formats. Its schema reports
only bounded counters, original selection, kind and task state. Registered
start/status/cancel responses and generated consumer types verify this wire
boundary separately from large-repository/RustFS/release qualification.

Native unfiltered reference windows reuse ordered prepared visibility while
preserving version-1 DTOs, lexical anchors, divergence, explicit bookmark terms
and signed cursor claims. Cold large reads reuse exact-snapshot index
preparation recovery. Native index bytes, storage/WAL/receipts and public
schemas are unchanged; complete scale and release qualification remains
separately deferred.

Prepared comparison extends the disposable local lookup to version2 records
containing both operation and view fingerprints. Generations are process-owned
and rebuilt; this artifact grants no source authority and needs no persisted
repository migration. A compact read-only decoder of the unchanged pinned
format-6 segment supplies commit/change identities and parent edges. Public
operation difference and change-page schemas and cursor profiles are unchanged.

Local prepared operation lookup uses a disposable fixed-width identity and
canonical-fingerprint artifact bound to one operation, with a whole-file
checksum and bounded loading. It is not a native, Git, companion, WAL or public
API format. Native commit-index output remains unchanged; source normalization
and pinned JJ selector semantics remain authoritative. Complete scale, cold
lifecycle and release qualification is deferred.

The additive native/local WorkStore activity routes use body-free version-1
native and version-2 local pages with independent snapshot profiles. Finite
1..256-group pages and strict local companion/query bodies accompany generated
OpenAPI/TypeScript and actual first/resumed response evidence. Native
manifest/WAL, local companion, signed cursor and domain capacities are
unchanged. Compact review activity adds two further routes with version-1
native/local summaries, strict finite windows, independent snapshot profiles and
explicit sampled-count status. Finite code-head summaries add a native route and
an operation-pinned local query variant with version-1 summaries and exact
native/Git identity widths. The generated inventory now contains 122 paths (42
local and 80 other paths). Summary DTOs carry no persisted-format change.
Focused delivery evidence does not qualify the complete combined route/browser
inventory or promote a release milestone.

The ordinary local Stacks POST command/state schema is generated alongside the
existing operation-pinned read contracts. Config version 1 uses local TOML;
native JJ operation, commit and tree formats remain unchanged. Executable route
admission and cold filesystem/browser responses qualify this core separately
from manifest-backed publication and the historical retained executor.

The local working-tree observation endpoint is an additive version-1 HTTP
contract for registered Git SHA-1/JJ 0.45.1 folders. Inputs retain exact
operation and binding generation; complete outputs retain operation/view/WC/tree
and physical/metadata witnesses. Existing route/DTO framing, native codecs,
companion records and private persisted formats remain unchanged. Generated
client/OpenAPI comparison and actual registered-server response validation
qualify the addition. Observation fingerprints issue no command or content
tokens. Linked-native, complete Unstaged assignment and writable stack lifecycle
qualification remain separate; this slice performs no package publication.

The independent conversion profile is bounded to eligible JJ 0.45.1 Git SHA-1
and current native repositories on macOS ARM64. Admin HTTP preview/start/status
contracts and their generated browser client share registered response captures.
Cold qualification uses production assets, an isolated real RustFS volume and a
restart; in-memory fault evidence remains separate. Unqualified signed,
conflicted, copy and code-anchored collaboration representations fail preflight.

AgentStore tool activity adds optional `duration_millis` to version-1 read DTOs.
Absent fields retain unknown timing; zero is a known duration. Generated
OpenAPI/TypeScript and affected native/local HTTP consumers qualify known and
absent values. Tool state and immutable native/local encodings remain unchanged.

AgentStore message activity and recorded transcript read DTOs add optional
`occurred_at_millis` fields while retaining response version 1 and the existing
route inventory. Missing fields remain compatible and mean unknown time; current
source-generation-1 chunk timing additionally requires its exact selected
normalized message and content. Generated contracts and affected native/local
HTTP responses MUST qualify both known and absent timestamps. This read-only
projection does not change native/local object encodings, capture/recovery
identities, source cursors, receipts or publication formats.

Repository-review browser/CLI integration preserves native V1 anchor, record,
receipt and manifest bytes and local companion request framing. Local read DTOs
carry their actual Git metadata selection, never synthetic native epochs or WAL
positions. Native and local CLI continuation selectors remain disjoint. The
unused local synchronization capability is removed; native M10 controls retain
their contracts.

WorkStore comment reactions add one typed target containing task identity,
comment identity and exact nonzero version. Existing target tags and canonical
bytes remain unchanged, as do native manifest/WAL and companion formats. The
collaboration read DTO adds an optional captured Work root; absent roots retain
older response framing. Generated OpenAPI and TypeScript qualify the addition.

Accepted-import recovery adds no wire or persistence format. An optional
body-free supervised lane reuses original version-1 intent, exact source
binding, reader cursor, sanitized spool, prepared request and native receipt
semantics. The foreground factory binds it only to existing private
import/control state and one retained runtime. Fair owned policy acquisition
prevents inspection from starving recovery or imported source work; current
authorization is rechecked after the queued acquisition. Synthetic Codex/native
MemoryStore cold foreground and restart evidence qualify this retained
composition. Enabled server startup, browser controls, other readers and real
harness/RustFS release remain pending.

The installation import and request-inspection routes add two local paths,
bringing the registered inventory to 91 paths: 67 native and 24 local.
Actor-free version-1 commands, exact original-input acceptance and separate
accepted/tail progress are qualified through actual HTTP responses and generated
clients. Public completion requires exact associated reader and native receipt
evidence; volatile scheduling is not acceptance. The private version-1 import
reader association adds `prefix_verified`, defaulting to false when absent.
Older links remain readable but cannot prove completion until renewed
original-source/native admission verifies retained checkpoint bytes. Verified
upgrades retain the original ordinal, source, initial generation and immutable
receipt. Older strict binaries reject the additive field rather than reinterpret
it. Repository, control, source, reader cursor, publication and AgentStore
formats retain their existing interpretation. Metadata inspection never scans
transcripts, opens pending bodies or repairs state. This progress qualification
alone does not establish enabled startup, accepted restart dispatch, browser
controls or real harness/storage release. Optional retained recovery has its own
qualification below.

The additive installation rescan route and cached discovery/degradation fields
retain device-control journal version 1 and all repository/source/spool formats.
Rescan is a transient conditional hint rather than a durable mutation or import;
its body carries no actor, source path or receipt identity. Actual response
captures and generated clients qualify it with the complete existing HTTP/SSE
inventory. Default-concurrency, real storage and live harness gates remain
separate from explicit serial focused qualification.

Additive per-harness runtime coverage retains all
journal/repository/source/spool formats and the same 88-path inventory. Typed
profile metadata and cached counts do not qualify unsupported readers or grant
import/admission authority. The current prepared Codex runtime exposes at most
two profile groups. Existing registered response captures and generated clients
validate the added fields.

Installation-local capture status and pause/resume are additive Admin HTTP
contracts with actor-free strict commands and an optional bootstrap capability.
Actual loopback responses qualify policy, durable receipts, runtime counters,
replay and failures against generated OpenAPI/TypeScript. Device-control journal
version 1 and repository/source/spool formats remain unchanged. Durable receipt
selection and runtime transition acceptance are separate; unknown outcomes
retain the original request. These contracts do not enable automatic startup or
qualify import, other harnesses, browser controls or real storage acceptance.

The additive application content-search endpoint exposes V1 results with full
native source identities, current revisions, bounded excerpts and opaque
expiring cursors. Generated OpenAPI/TypeScript and actual HTTP response captures
qualify this route alongside existing consumers. Persistence formats and
operation-pinned code search remain unchanged.

The channel UI consumes additive authorized timeline and reaction witnesses.
Messages with no reply parent retain the original canonical bytes; replies
retain exact parent revisions through edits, closure transfer and cold recovery.
Generated contracts and selected CLI/transport consumers must qualify these
additions without changing manifest or companion formats.

M15 HTTP contract samples cover Thread commands/status/detail/events, exact
human message revisions, Reaction queries/commands, custom
definitions/retirement and retained raster reads. They validate actual isolated
MemoryStore transport responses against generated OpenAPI; this is distinct from
persistent RustFS, cold browser and real runner evidence.

The additive capture_gap Thread-link variant preserves all prior link, unlinked
capture and prepared-request encodings. It retains source/event and observed
binding closures through native recovery/copy and companion Git reachability.
Older readers fail closed on the new target rather than interpreting it as a
session. Generated OpenAPI/TypeScript, direct provider/RPC consumers and exact
gap-context presentation are qualified independently of real harness and broader
release acceptance.

Rust 1.98.1 and edition 2024 are the qualified toolchain for managed RustFS. New
native repositories ship first through LocalTrusted access with no user
authentication. M8a hardens that local release. A new embedded runtime or
topology invalidates old storage qualification until contract tests rerun.
Read-only proof results do not constitute deployment or publication approval.

The Appearance slice pins Buzz ef2aa1ae, Shiki themes 4.0.2 and Inter Variable
5.2.8 with Apache-2.0, MIT and OFL attribution. Source Design Core typography
and native-overlay acceptance is distinct from registry-only release
qualification; portable manifest/lock ranges remain unchanged by the temporary
source validation hook. Native formats and transport compatibility are
unaffected.
