---
schema: 1
id: E-M10-002
---

# E-M10-002

<!-- evidence-field:id markdown -->
## Id

E-M10-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authored-replication-origins
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-model/src/ids.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/replication.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/native_copy.rs
- crates/jj-store-wal/tests/publication.rs
- spec/src/identity.md
- spec/src/milestones.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/publication.rs#authored_publications_atomically_advance_one_origin_stream
- crates/jj-store-wal/tests/publication.rs#failed_manifest_candidate_does_not_consume_origin_sequence
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test publication --locked authored_publications_atomically_advance_one_origin_stream -- --exact
```

**Notes**

One accepted native publication selects its origin envelope, receipt and local stream prefix at the content WAL position; receipt replay creates neither another WAL entry nor another origin, and the next publication links to the first digest and causal prefix.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test publication --locked failed_manifest_candidate_does_not_consume_origin_sequence -- --exact
```

**Notes**

A manifest write with an unknown outcome leaves the selected replication root at sequence zero; the next accepted publication receives sequence one.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test native_copy --locked native_copy_restores_code_agent_review_and_installs_manifest_last -- --exact
```

**Notes**

The bounded native-copy closure retains all three authored origin envelopes, receipts and accepted-origin catalog pages and installs the replication-aware manifest last.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --locked
```

**Notes**

All WAL unit, publication, AgentStore, ReviewStore, inventory, native-copy and documentation tests pass, including 31 publication cases.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

Specification, boundary, architecture, progress, formatting and the ten-minute embedded RustFS clippy lane passed. The following contracts check stopped while building Kingfisher Vectorscan because cmake is not installed on this host; this is the same environment limitation recorded by E-M10-001.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M10.3 must preserve imported origin keys and digests, atomically reduce unseen
  content and return durable per-origin receipts.
- Format-2 repositories without a replication root require the M10.6 seed
  upgrade before authored origins are enabled.
- The repository-wide contracts, check, test, build and specification lanes that
  compile Kingfisher Vectorscan require cmake on this host.
- Real RustFS, worker, outage and foreground zero-upstream-call qualification
  remain M10.7 work.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; jj-store-model and jj-store-wal 0.1.0; manifest format 2; replication root, origin envelope and origin receipt schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest format 2 gains an optional replication_root reference. Canonical control records add replication-root, origin and origin-receipt kinds. ReplicaId and WriterEpoch are distinct 128-bit identifiers; OriginDigest is a distinct 32-byte SHA-256 digest. Replication-disabled format-2 repositories remain readable and preserve their previous publication behavior.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


