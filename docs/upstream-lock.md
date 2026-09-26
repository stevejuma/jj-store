# Upstream lock and reference audit

| Component         | Pin                                                                                     | Role                                                | Evidence state                                    |
| ----------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------------------------------- |
| jj-cli and jj-lib | 0.45.1; `7c41cdeb16b6b321c64e789a966b6adf723816a5`                                      | CLI extension, native traits and semantic reference | Source inspected; compiled integration pending M0 |
| Rust              | 1.97.1                                                                                  | Workspace toolchain                                 | Locally available                                 |
| spec-validator    | Released npm range ^0.1.0; exact resolution in pnpm lock                                | Canonical specification tooling                     | Bootstrap checks                                  |
| RustFS            | `rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f` | Initial single-node contract candidate              | Locally available; not yet qualified              |
| Commit Cloud POC  | Pending immutable revision                                                              | Factory/RPC shape only                              | Audit in M0                                       |
| WalGit            | Pending immutable revision                                                              | Publication and durable consumer design             | Audit in M0                                       |
| Tantivy           | Pending M6 lock                                                                         | Rebuildable search                                  | Deferred dependency introduction                  |

The JJ source pin is recorded by the published crate's `.cargo_vcs_info.json`.
Both JJ crates stay on the same tested release. A RustFS image being available
is not conformance evidence. Other topologies require their own qualification.

Keep adopted source attribution in [NOTICE](licenses/NOTICE.md). References are
not runtime dependencies. Never copy their repository-level agent instructions.
