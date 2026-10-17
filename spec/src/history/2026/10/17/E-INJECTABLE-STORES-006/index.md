---
schema: 1
id: E-INJECTABLE-STORES-006
---

# E-INJECTABLE-STORES-006

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M32
- M32.1
- M32.2
- M32.3
- M32.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-002
- JST-ARCH-005
- JST-ARCH-006
- JST-LOCAL-001
- JST-LOCAL-003
- JST-PUB-004
- JST-PUB-005
- JST-JJ-001
- JST-JJ-002
- JST-JJ-003
- JST-JJ-004
- JST-JJ-005
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
- JST-GOV-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

independent-store-embedding-qualification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change wmzlvsps.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Parent880afe3b has verified Git/native factory consumer migration and real cold RustFS replay, but no independent Cargo embedding fixture or full SDK guide. All existing crate-root READMEs were introduced by E-CRATE-READMES-001; native overview by E004. M32 remains In progress until external qualification.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-core/README.md
- crates/jj-store-git/README.md
- crates/jj-store-jj/README.md
- crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.lock
- crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml
- crates/jj-store-jj/tests/fixtures/store-consumer/README.md
- crates/jj-store-jj/tests/fixtures/store-consumer/fixtures/codex-hooks.json
- crates/jj-store-jj/tests/fixtures/store-consumer/src/git.rs
- crates/jj-store-jj/tests/fixtures/store-consumer/src/main.rs
- crates/jj-store-jj/tests/fixtures/store-consumer/src/native.rs
- crates/jj-store-native/README.md
- docs/injectable-stores.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- docs/site/technical/publication.md
- scripts/build-storage-inventory.mjs
- scripts/managed-cargo.test.mjs
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/jj-integration/01-native-jj-integration.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M32/history/M32/index.md
- spec/src/current/progress/M32/history/M32.1/index.md
- spec/src/current/progress/M32/history/M32.2/index.md
- spec/src/current/progress/M32/history/M32.3/index.md
- spec/src/current/progress/M32/history/M32.4/index.md
- spec/src/current/progress/M32/index.md
- spec/src/current/verification/arch.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/fixtures/store-consumer/src/main.rs#git_and_independent_custom_store
- crates/jj-store-jj/tests/fixtures/store-consumer/src/main.rs#native_without_application_services
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-INJECTABLE-STORES-006 details 2](details-02.md)
- [E-INJECTABLE-STORES-006 details 3](details-03.md)
- [E-INJECTABLE-STORES-006 details 4](details-04.md)
