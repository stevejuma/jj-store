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
| Failed startup and port conflict                        | Planned A1               | Must fail before public listeners                                                                                   |
| Missing volume, ownership, identity, secret or sentinel | Planned A1               | Format-2 initialization and fail-closed restart                                                                     |
| Disk and permission faults                              | Planned A1               | Native filesystem fault acceptance                                                                                  |
| Conditional create/CAS                                  | Planned A1               | Complete existing real-storage contract against the embedded runtime                                                |
| Abrupt restart                                          | Planned A1               | Kill helper, reopen the exact volume, recover repository publication                                                |
| Resource measurements                                   | Qualified on macOS ARM64 | Debug binary 899,687,824 bytes; first start 11,808 ms at 115,696 KiB RSS; restart 107 ms at 110,880 KiB RSS         |
| Bucket replication                                      | Unsupported pending A3   | Do not enable until generic-S3 bootstrap, restart, backlog, multipart and existing-object tests pass                |

The measurements come from the ignored real-runtime acceptance invoked by
`pnpm test:embedded-rustfs`; they describe an unoptimized qualification binary,
not a packaged release budget. The first start creates RustFS metadata in an
empty explicit volume. The second start reopens that volume and reads an object
written through `S3Store` before the owned graceful shutdown.

This A0 report does not qualify Linux x86_64, MinIO, AWS, replication, restore,
or production readiness. Those claims require their named later gates.
