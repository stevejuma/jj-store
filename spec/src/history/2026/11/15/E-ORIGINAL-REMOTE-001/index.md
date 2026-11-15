---
schema: 1
id: E-ORIGINAL-REMOTE-001
---

# E-ORIGINAL-REMOTE-001

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust/Cargo 1.99.0; native JJ engine 0.46.0 and gix 0.88.0; Docker fixture JJ 0.45.1 and Git 2.55.0; root Deno-free spec-validator 0.2.0. Remote Docker worker uses Node 22.22.0; local tools Node 24.10.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native original mutation capability; bounded SHA-1 raw object closures; opaque private push reviews; existing durable publication intent/receipt format retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Manual existing Git-backed JJ originals only. Discovery/save/fetch/one reviewed bookmark publication with distinct transport binding, no snapshot, checkout, all-bookmark or force publication. Real HTTPS, host adoption and cold application gates remain separately unfinished.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Open the native source PR and verify its exact-head hosted checks; then consume the qualified revision in Core/host adapters. HTTPS and cold Desktop/hosted application acceptance remain outside this source proof.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-008 adds explicit original writer admission, guarded remote
  save, tracking import and reviewed exact-branch publication. Immutable reader
  authority is unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "M34.9 remains In progress: real source transport cannot qualify either
  cold application host or provider rollout."

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-ORIGINAL-REMOTE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-008
- JST-DOC-002
- JST-ARCH-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/original_remotes/fetch.rs
- crates/jj-store-jj/src/original_remotes/mod.rs
- crates/jj-store-jj/src/original_remotes/objects.rs
- crates/jj-store-jj/src/original_remotes/push.rs
- crates/jj-store-jj/src/original_remotes/settings.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/selected.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/tests/original_remotes.rs
- docs/site/technical/local.md
- package.json
- scripts/test-vault-ssh.mjs
- scripts/validation/lanes.json
- scripts/validation.test.mjs
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/02-requirements.md
- spec/src/current/contracts/local-jj/33-original-repository-remotes.md
- spec/src/current/contracts/local-jj/SUMMARY.nav.md
- spec/src/current/contracts/local-jj/index.md
- spec/src/current/governance/qualification-change-reviews.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/architecture/08-git-bridge-ownership.md
- spec/src/current/contracts/architecture/23-public-distribution.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/original_remotes.rs#discovery_is_read_only_and_never_returns_embedded_credentials
- crates/jj-store-jj/tests/original_remotes.rs#mutation_admission_and_backup_cache_isolation_refuse_before_network
- crates/jj-store-jj/tests/original_remotes.rs#an_externally_changed_endpoint_refuses_the_stale_transfer
- crates/jj-store-jj/tests/original_remotes.rs#explicit_remote_save_preserves_origin_and_rejects_stale_configuration_without_transfer
- crates/jj-store-jj/tests/original_remotes.rs#original_fetch_moves_tracked_bookmarks_without_capturing_or_writing_files
- crates/jj-store-jj/tests/original_remotes.rs#original_push_publishes_one_reviewed_bookmark_and_recovers_without_replay
<!-- /evidence-field -->

