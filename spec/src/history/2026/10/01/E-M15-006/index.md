---
schema: 1
id: E-M15-006
---

# E-M15-006

<!-- evidence-field:id markdown -->
## Id

E-M15-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
- M15.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-native-service
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xunxsnqx
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 36376e4c. cargo test -p jj-store-service --test native_collab --locked fails because target is absent. Native typed provider exists, but no authenticated service DTOs or authorized captured-root read facade.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; native objects and actual access policy over disposable MemoryStore; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned native command request/receipt/status V1 with server-derived actor and publication identity. Credential-free private read pins and source-referencing Thread history. Persistence formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/collab_mutations.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/tests/native_collab.rs
- spec/src/collaboration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-006 details 2](details-02.md)
- [E-M15-006 details 3](details-03.md)
