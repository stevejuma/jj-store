---
schema: 1
id: E-M15-008
---

# E-M15-008

<!-- evidence-field:id markdown -->
## Id

E-M15-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-native-pagination
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qlyuwxky
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 218a59db. collab_pagination test target absent. Credential-free native read pins exist, but no signed query-bound cursor facade or retained-pin capacity/expiry/concurrency enforcement.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; native objects over disposable MemoryStore; real token policy/revocation; deterministic registry clock; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Process-local HMAC-SHA256 jsc1 tokens with instance ID, complete captured-snapshot digest, viewer, repository, exact query/page size and typed position. Owned CollabQuery/CollabPage DTOs; persistence formats unchanged. No public HTTP/RPC endpoint or generated contract changes yet.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/collab_pages.rs
- crates/jj-store-service/src/collab_pages/cursors.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/tests/collab_pagination.rs
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-008 details 2](details-02.md)
