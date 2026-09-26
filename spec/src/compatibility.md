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
