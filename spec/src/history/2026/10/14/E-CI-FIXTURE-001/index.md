---
schema: 1
id: E-CI-FIXTURE-001
---

# E-CI-FIXTURE-001

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

snxrpykx
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99.0; JJ 0.45.1; jj-acp 0.1.0; pnpm 10.32.1; Node 24.10.0 macOS ARM64 and 24.13 Linux container; hosted Node 22.23.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, manifest/WAL/receipts, API and companion formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local targeted evidence and CI repairs do not
  promote hosted, optimized archive, container, registry, provider or full
  release qualification.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CI-FIXTURE-001
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-MUT-001
- JST-LOCAL-004
- JST-LOCAL-005
- JST-CLI-002
- JST-CLI-004
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_cli.rs
- crates/jj-store-jj/tests/conversion.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-jj/tests/local_metadata.rs
- crates/jj-store-jj/tests/local_stacks.rs
- crates/jj-store-jj/tests/stack_foreground/fixture.rs
- crates/jj-store-jj/tests/stack_foreground.rs
- crates/jj-store-jj/tests/stack_preparation_retention.rs
- crates/jj-store-server/tests/repository_conversion.rs
- crates/jj-store-server/tests/stack_combinations.rs
- crates/jj-store-service/src/local_repository/work_cache/tests.rs
- crates/jj-store-service/tests/conversion.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-service/tests/local_agent_binding.rs
- crates/jj-store-service/tests/local_agent_pages.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-service/tests/local_review.rs
- crates/jj-store-service/tests/local_work.rs
- docs/maintenance/coverage.json
- docs/site/cli/jj-store.md
- scripts/spec/progress-regression.test.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/spec/progress-regression.test.mjs#migration_preserves_original_evidence_tasks_requirements_and_byte_archives
- crates/jj-store-cli/tests/stack_cli.rs#inspection_dry_run_and_invalid_arguments_preserve_native_state
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [CI qualification evidence](details-02.md)
