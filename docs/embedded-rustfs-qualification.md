# Embedded RustFS qualification

## Pinned runtime

- Rust toolchain: 1.98.1.
- Crate: `rustfs = 1.0.0`, crates.io checksum
  `ecfecedc774cfa9f40fb0b475e2605cbff858282d18e52927a57ccb5105e5dd4`, default
  features disabled.
- Published source revision: `d47f54bfb2f39f48bd1adda334bd27e151fe85b8`.
- Runtime topology: one checkout-owned helper process on a concrete loopback
  port, one explicit persistent volume, private credentials inherited through
  its environment, and the existing signed `S3Store` in the foreground server.
- Runtime stack: the helper supplies 8 MiB Tokio worker stacks, matching
  RustFS's debug runtime policy. The macro-default 2 MiB stack overflowed while
  reopening persisted data and is covered by the restart test.
- Existing external image lane remains pinned independently to
  `rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f`.
- Generic S3 acceptance is pinned independently to the MinIO image index
  `cgr.dev/chainguard/minio@sha256:6a1d0b45c8669726bba580ced0bfa4cb9fdeed1ed636dfabd81d1577beb6937b`;
  it is not qualified until A3.

RustFS and `jj-cli` depend unconditionally on different mimalloc sys crates that
both declare `links = "mimalloc"`. Cargo rejects one linked dependency graph.
The helper preserves the released crate without a patch and lets the server own
startup, readiness, storage access, shutdown, and the child lifetime. It is part
of the jj-store distribution; no separately installed RustFS service is
required.

## A0 capability matrix

| Capability                                              | A0 state                 | Evidence or next gate                                                                                               |
| ------------------------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| Explicit persistent volume                              | Implemented              | `EmbeddedStorageConfig::volume`; persistence test                                                                   |
| Graceful shutdown                                       | Implemented              | Parent closes the private shutdown channel and waits up to ten seconds                                              |
| Persistent graceful restart                             | Qualified on macOS ARM64 | `embedded_rustfs_preserves_objects_across_graceful_restart` preserves exact bytes across owned shutdown and restart |
| Failed startup and port conflict                        | Qualified on macOS ARM64 | Managed runtime fails before the public listener and reports the startup fault                                      |
| Missing volume, ownership, identity, secret or sentinel | Qualified on macOS ARM64 | Strict format-2 restart rejects each unavailable or mismatched installation input                                   |
| Private-state permission faults                         | Qualified on macOS ARM64 | Group/world-readable internal credential state is rejected before child startup                                     |
| Conditional publication                                 | Qualified on macOS ARM64 | LocalTrusted native creation and browser mutations publish through the existing manifest CAS path                   |
| Abrupt restart                                          | Qualified on macOS ARM64 | Dropped runtime is killed; the exact volume reopens and returns previously published bytes                          |
| Resource measurements                                   | Qualified on macOS ARM64 | Debug binary 899,687,824 bytes; first start 11,808 ms at 115,696 KiB RSS; restart 107 ms at 110,880 KiB RSS         |
| Bucket replication                                      | Unsupported pending A3   | Do not enable until generic-S3 bootstrap, restart, backlog, multipart and existing-object tests pass                |

The low-level measurements come from the ignored real-runtime acceptance invoked
by `pnpm test:embedded-rustfs`; they describe an unoptimized qualification
binary. The cold application lane records the complete server plus RustFS tree
and enforces 30-second cold/restart startup and 512 MiB combined RSS ceilings.
The first start creates RustFS metadata in an empty explicit volume. The second
start reopens that volume and reads an object written through `S3Store` before
the owned graceful shutdown.

This report does not qualify Linux x86_64, MinIO, AWS, replication, backup,
restore or shared production serving. Those areas are explicitly deferred.
