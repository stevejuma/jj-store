---
schema: 1
id: E-M15-003
---

# E-M15-003

<!-- evidence-field:id markdown -->
## Id

E-M15-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.1
- M15.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-target-resolution
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vvurrnyz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 4c6a8d24. cargo test -p jj-collab --test target_resolution --locked fails because target is absent. Native collab_persistence baseline has 9 passing tests. Existing runtime has no jj-acp dependency or executable wiring; ThreadBindingV1 is an optional contract only.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; async-trait 0.1.92; tokio 1.53.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Canonical collaboration V1 and existing repository-bound Agent/Review IDs preserved; native manifest 5 and local metadata 2 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-collab/Cargo.toml
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/resolver.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/collab_targets.rs
- crates/jj-store-wal/tests/collab_persistence.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/milestones.md
- spec/src/architecture.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-003 details 2](details-02.md)
- [E-M15-003 details 3](details-03.md)
