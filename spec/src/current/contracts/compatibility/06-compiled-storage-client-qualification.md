# Compiled storage client qualification

The disposable RustFS harness accepts `--rust-client` to qualify the Rust
transport as well as the independent HTTP probes. The compiled lane covers
conditional-create/CAS races over independent clients, stale tokens, range
reads, 8 MiB native file upload/read/duplicate verification, metadata objects
and a proxy that drops a real successful write acknowledgement. Default unit
lanes leave this external test explicitly ignored.

The inventory-source integration is also checked by a fresh
[compiled native storage run](../../../../../docs/evidence/m2-inventory-native-rustfs.json).
This confirms the checked metadata/file reader remains compatible; it does not
establish WAL publication or recovery correctness.

The compiled RustFS lane also publishes native operations through the manifest
coordinator. It tests independent-client sibling races, an intentionally lost
manifest acknowledgement, checkpoint/receipt persistence, missing dependency
rejection and explicit corruption detection. The harness then kills/restarts its
owned RustFS container and launches a fresh Rust process to recover selected
history, retained evolution dependencies and receipts without in-memory caches.

[M2 publication evidence](../../../../../docs/evidence/m2-publication-rustfs.json)
records the extended 19-publication model, repeated checkpoints and
original-result retries against the pinned image, then a fresh-process
retained-data audit.

The
[M3 storage regression run](../../../../../docs/evidence/m3-auth-storage-rustfs.json)
retains the native publication/recovery gate after extracting the shared checked
manifest reader for authentication. It does not claim deployed RPC/HTTP token
acceptance, which remains a separate M3 integration gate.

The M3 native lifecycle fixture found that physical file snapshotting invokes
backend I/O on upstream worker threads without an inherited Tokio reactor.
Native factories now require a caller-owned multithread runtime and enter it per
poll on those threads. Library lifecycle, cache rebuild and policy regression
coverage is in `crates/jj-store-cli/tests/native_lifecycle.rs`.

The
[native session storage regression](../../../../../docs/evidence/m3-native-session-rustfs.json)
retains all ten RustFS storage/publication probes after sharing the committed
inventory reader. It uses the storage harness's test principals and does not
replace authenticated RPC/HTTP or executable CLI acceptance.
