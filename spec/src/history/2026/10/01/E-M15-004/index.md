---
schema: 1
id: E-M15-004
---

# E-M15-004

<!-- evidence-field:id markdown -->
## Id

E-M15-004
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

focused-local-provider
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kpuymtxs
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at a435f160. cargo test -p jj-store-service --test local_collab --locked fails because target is absent. Local carrier has batch envelope support but no typed canonical Thread/Reaction provider.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git SHA-1 companion profile; image 0.25.10; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local companion metadata 2 under unchanged refs/lapis/state/v1; independent descriptor V1 Thread/Reaction graphs with sorted canonical-ID/Git-blob inventories and existing repository-bound Agent/Review envelope bytes. Canonical reaction results and custom-library commands V1.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/reaction_state.rs
- crates/jj-collab/src/resolver.rs
- crates/jj-collab/tests/domain.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository/collab_external.rs
- crates/jj-store-service/src/local_repository/collab_graph.rs
- crates/jj-store-service/src/local_repository/collab_store.rs
- crates/jj-store-service/src/local_repository/collab_types.rs
- crates/jj-store-service/tests/local_collab.rs
- scripts/check-boundaries.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-004 details 2](details-02.md)
- [E-M15-004 details 3](details-03.md)
