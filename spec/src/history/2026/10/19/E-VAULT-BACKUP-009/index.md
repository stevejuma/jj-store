---
schema: 1
id: E-VAULT-BACKUP-009
---

# E-VAULT-BACKUP-009

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-003
- JST-JJ-005
- JST-ARCH-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-19
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/recovery.rs
- crates/jj-store-jj/tests/vault_publication.rs
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-009/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-009/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_publication.rs#explicit_stop_preserves_the_unknown_receipt_and_never_replays_or_clears_a_newer_push
- crates/jj-store-jj/tests/vault_publication.rs#observation_confirms_acceptance_without_publication_and_pending_recovery_follows_a_root_relink
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_publication
```

**Notes**

Before implementation, new acceptance proof cannot compile because pending publication/reconciliation/stop APIs and Stopped outcome are absent. Missing-capability compile boundary. publication-before.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_publication
```

**Notes**

Two actual native proofs pass. Local stop retains original receipt, never sends or clears a newer intent; retry remains stopped. Observation confirms exact/descendant acceptance without pushing; physical-root relink preserves original identity. publication-after.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_publication --test vault_remote --test vault_sync; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_publication --test vault_remote --test vault_sync -- -D warnings
```

**Notes**

All nine directly affected publication/remote/sync cases and strict Clippy pass. publication-surface.log and publication-clippy.log. Disposable local repositories and simulated lost receipts; no personal credentials or live provider.
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
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

26package boundaries,179tasks/919records, sevenvalidators/4324chapters/208requirements pass. First spec-first attempt returned working-copy changed during revision-pinned validation while changed-file formatting completed; rerun after formatting passes. No product assertions or contract gate changed. publication-bookkeeping and publication-bookkeeping-retry logs. Changed-file Prettier and cargo fmt --all -- --check pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold desktop/provider lifecycle acceptance and full platform/release
  qualification stay user-deferred.
- Historical search, advanced storage/adoption, Forgejo and main integration
  remain required.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Git 2.55.0; Rust 1.99.0; source parent 6942a269; desktop parent e2c7e4d8.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive PendingPublication summary and stopped terminal outcome in device-private push receipt V1. Upstream JJ/Git identities and codecs unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact original push observation, local explicit stop, retained terminal receipts and safe replays across later pushes and physical-root relink.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete desktop recovery checks then implement historical search.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 specifies explicit local stop and observe-only original
  recovery. JST-JJ-003/005 immutable read and upstream identity contracts remain
  unchanged. JST-ARCH-007 continues owned DTO re-exports without upstream
  imports outside native package.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.5 remains the only active stage during desktop recovery
  verification. Native source implementation now has focused proof; original
  cold app/provider gates and stages 6/7 remain open.

```
<!-- /evidence-field -->

