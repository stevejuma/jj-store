---
schema: 1
id: E-M15-005
---

# E-M15-005

<!-- evidence-field:id markdown -->
## Id

E-M15-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.1
- M15.2
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-native-commands
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lzpstlkq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 00292f68. cargo test -p jj-store-wal --test collab_commands --locked fails because target is absent. Native provider has no typed commands or cross-source manifest pin.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; image 0.25.10; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native manifest 5 unchanged. Legacy DomainRequest bytes unchanged; private captured manifest pin. Canonical native typed request/result V1. Local commands/results become byte-compatible aliases of shared domain variants.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/command_types.rs
- crates/jj-collab/src/lib.rs
- crates/jj-store-service/src/local_repository/collab_types.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/collab_commands.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-005 details 2](details-02.md)
- [E-M15-005 details 3](details-03.md)
