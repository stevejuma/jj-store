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
