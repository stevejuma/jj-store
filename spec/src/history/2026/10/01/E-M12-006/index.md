---
schema: 1
id: E-M12-006
---

# E-M12-006

<!-- evidence-field:id markdown -->
## Id

E-M12-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.3
- M12.4
- M12.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-origin-reduction
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change uxpxtwyo.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

GB3 bdb77b8c; code activation refused. Exact native activation regression failed once at existing Git reducer gate after correcting one fixture method-name compile error. New impact surface spans WAL publication/recovery, owned JJ validation, replication transfer/bootstrap and service/RPC/CLI consumers.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1;gix 0.87.1;Git 2.55.0;Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 4;origin optional Git root omitted from legacy envelopes;Git origins semantic format 2;negotiated reducer 2.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/git-interop.md
- spec/src/replication.md
- spec/src/publication.md
- spec/src/jj-integration.md
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/replication.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_protocol.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/publication/replication_transfer.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-jj/src/replication.rs
- crates/jj-store-jj/tests/git_bridge.rs
- crates/jj-store-service/src/git_bridge.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-replication/src/worker.rs
- crates/jj-store-cli/tests/executable_native.rs
- AGENTS.md
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-rpc/tests/replication.rs
- crates/jj-store-service/tests/git_bridge.rs
- crates/jj-store-wal/src/publication/git_interop/attempts.rs
- spec/src/cli.md
- spec/src/mutations.md
- docs/cli/commands.md
- spec/src/spec-governance.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M12-006 details 2](details-02.md)
- [E-M12-006 details 3](details-03.md)
