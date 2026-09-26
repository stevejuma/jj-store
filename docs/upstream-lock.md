# Upstream lock and reference audit

| Component         | Pin                                                                                     | Role                                                | Evidence state                                          |
| ----------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------------------------------------- |
| jj-cli and jj-lib | 0.45.1; `7c41cdeb16b6b321c64e789a966b6adf723816a5`                                      | CLI extension, native traits and semantic reference | Compiled native and CLI probes passed                   |
| Rust              | 1.97.1                                                                                  | Workspace toolchain                                 | Locally available                                       |
| spec-validator    | Released npm range ^0.1.0; exact resolution in pnpm lock                                | Canonical specification tooling                     | Bootstrap checks                                        |
| RustFS            | `rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f` | Initial single-node contract candidate              | Single-node probes passed; other topologies unqualified |
| Commit Cloud POC  | `4b1c77b9db365e426e49846669a1626698fd5fa6`                                              | Factory/RPC shape only                              | Audit in M0                                             |
| WalGit            | `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`                                              | Publication and durable consumer design             | Audit in M0                                             |
| Tantivy           | Pending M6 lock                                                                         | Rebuildable search                                  | Deferred dependency introduction                        |

The JJ source pin is recorded by the published crate's `.cargo_vcs_info.json`.
Both JJ crates stay on the same tested release. A RustFS image being available
is not conformance evidence. Other topologies require their own qualification.

Keep adopted source attribution in [NOTICE](licenses/NOTICE.md). References are
not runtime dependencies. Never copy their repository-level agent instructions.

## M0 source audit

Commit Cloud and JJ are Apache-2.0; WalGit is MIT. Pinned raw-source files were
inspected in ignored scratch storage. Commit Cloud uses jj-lib 0.43.0 and is not
a direct dependency. Its operation schema lacks time-zone offsets; its view
schema omits local/remote tags, remote-name nesting, Git refs, and Git heads.
Its tree schema lacks submodules and its backend RPC lacks copy-history methods.
The POC preserves predecessor presence but its custom hashes are not the chosen
identity profile. WalGit uses sequence-slot claims/burning; jj-store instead
uses attempt-unique immutable candidates, retaining its unknown-outcome
invariant.

The selected RustFS image passed seven single-node probes; see
[evidence](evidence/m0-rustfs-single-node.json). This does not qualify a load
balancer or multi-node deployment, nor implement the publication protocol.

A second cold run includes interrupted body transfer:
[eight-probe report](evidence/m0-rustfs-cold.json). The native compatibility
tests pass for exact hash vectors, read-only divergent operations, and
request-scoped OpHeadsStore gRPC. Product storage/server/API implementation
remains in M1 onward.
