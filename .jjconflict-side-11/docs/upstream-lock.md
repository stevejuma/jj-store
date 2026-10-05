# Upstream lock and reference audit

| Component          | Pin                                                                                                                                                                                         | Role                                                | Evidence state                                                          |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------------------- |
| jj-cli and jj-lib  | 0.45.1; `7c41cdeb16b6b321c64e789a966b6adf723816a5`                                                                                                                                          | CLI extension, native traits and semantic reference | Compiled native and CLI probes passed                                   |
| Rust               | 1.98.1                                                                                                                                                                                      | Workspace and embedded RustFS toolchain             | Installed on qualified macOS ARM64 host                                 |
| spec-validator     | Released npm range ^0.1.0; exact resolution in pnpm lock                                                                                                                                    | Canonical specification tooling                     | Bootstrap checks                                                        |
| RustFS             | `rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f`                                                                                                     | Initial single-node contract candidate              | Single-node probes passed; other topologies unqualified                 |
| Embedded RustFS    | crate `rustfs = 1.0.0`, crates.io checksum `ecfecedc774cfa9f40fb0b475e2605cbff858282d18e52927a57ccb5105e5dd4`, source `d47f54bfb2f39f48bd1adda334bd27e151fe85b8`, default features disabled | Managed local authoritative storage runtime         | A0 macOS ARM64 compile and persistent restart qualification in progress |
| MongoDB Kingfisher | release `v2.7.0`, source `63dccba231b6b01990c84319d25f92f49428a431`, default scanner features disabled, matching Vectorscan fork                                                            | In-process local agent capture sanitization         | Focused redaction, fail-closed and cached-startup regressions pass      |
| Generic S3 target  | `cgr.dev/chainguard/minio@sha256:6a1d0b45c8669726bba580ced0bfa4cb9fdeed1ed636dfabd81d1577beb6937b`                                                                                          | Backup interoperability target                      | Immutable multi-platform index pinned; A3 qualification pending         |
| Commit Cloud POC   | `4b1c77b9db365e426e49846669a1626698fd5fa6`                                                                                                                                                  | Factory/RPC shape only                              | Audit in M0                                                             |
| WalGit             | `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`                                                                                                                                                  | Publication and durable consumer design             | Audit in M0                                                             |
| Tantivy            | 0.26.2; Cargo.lock checksum                                                                                                                                                                 | Rebuildable search                                  | Local projection durability qualification in M6                         |

The JJ source pin is recorded by the published crate's `.cargo_vcs_info.json`.
Both JJ crates stay on the same tested release. A RustFS image being available
is not conformance evidence. Other topologies require their own qualification.

The released RustFS crate and `jj-cli` use different unconditional native
mimalloc sys crates with the same Cargo `links` identity. They cannot coexist in
one Rust dependency graph. The pinned embedded API therefore runs in a
checkout-owned helper executable with its own lockfile. The foreground server
owns its credentials, readiness channel, shutdown channel, process lifetime, and
explicit volume while continuing to use the existing `S3Store`. This is a
packaging boundary, not an external RustFS deployment or container.

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

The released Tantivy 0.26.2 crate records upstream source revision
`72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d` in `.cargo_vcs_info.json`. Its
initial local schema uses mmap and LZ4 without optional language stemmers. Index
files are disposable; source/WAL format identity is unaffected by this
dependency.

MongoDB Kingfisher is consumed as unmodified Rust library crates from the pinned
release source. Agent capture uses the built-in medium-confidence rules without
validation, network calls or Base64 decoding. The compiled Vectorscan database
is cached in user-restricted device state; untrusted hook input stays in memory.
