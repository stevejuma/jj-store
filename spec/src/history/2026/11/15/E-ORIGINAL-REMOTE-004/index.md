---
schema: 1
id: E-ORIGINAL-REMOTE-004
---

# E-ORIGINAL-REMOTE-004

<!-- evidence-field:id markdown -->
## Id

E-ORIGINAL-REMOTE-004
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
- JST-JJ-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/original_remotes/mod.rs
- crates/jj-store-jj/src/original_remotes/fetch.rs
- crates/jj-store-jj/tests/vault_original_remotes.rs
- docs/site/technical/local.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/local-jj/33-original-repository-remotes.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
- spec/src/current/progress/M34/index.md
- spec/src/current/governance/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_original_remotes.rs#discovery_is_read_only_and_never_returns_embedded_credentials
- crates/jj-store-jj/tests/vault_original_remotes.rs#mutation_admission_and_backup_cache_isolation_refuse_before_network
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_original_remotes
```

**Notes**

Baseline reports one E0599: the immutable open_readonly constructor is missing.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_original_remotes
```

**Notes**

Initial exact pass: four tests, zero failures, two opt-in real-SSH ignores. Read-only discovery cannot save configuration even when its underlying concrete grant also implements the separate writer trait.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_counts --test vault_divergence --test vault_original_remotes --test vault_publication --lib
```

**Notes**

Strengthened regression also refuses fetch and publication review before transfer. All 71 affected cases pass; five existing opt-in cases remain ignored in this lane.
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
pnpm test:vault-ssh -- --original
pnpm test:vault-https
```

**Notes**

Two real original SSH cases and one real HTTPS case pass. Explicit admitted writers still preserve unsaved files, publish one reviewed bookmark, reject divergence and observe retained unknown outcomes without replay. Disposable endpoints, trust and credentials are retired.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- clippy --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_original_remotes -- -D warnings
```

**Notes**

Strict native source and original test lint passes.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
cargo fmt --all --check
pnpm exec prettier --check <changed authored Markdown and JSON>
```

**Notes**

All 121 governance cases pass; eight spec validators accept 4,668 chapters and 212 requirements. Spec-first passes five protected files/four chapters; progress accepts 187 tasks and 1,055 records. Strict documentation closure validates 144 files and nine diagrams. Changed-document and Rust formatting pass. Wider UI lanes are outside this Rust-only source slice.
<!-- /run -->
<!-- /evidence-field -->

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

Rust/Cargo 1.99.0; JJ library 0.46.0; disposable fixture JJ 0.45.1; Deno-free spec-validator 0.2.0; Node 24.10.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive immutable OriginalRepository constructor; existing original review, transfer and receipt formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Passive settings need only a local_browser::Access source witness. OriginalRepository retains optional explicit MutationAccess only through the existing writer constructor. Read-only construction cannot recover mutation authority by casting or by a concrete reader implementing the writer trait. Snapshot and divergence preserve existing native pins and bounds.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the qualified native source slice, verify hosted CI, then intake its merged revision into Core discovery. Guarded Core saves, transfers, host adapters and both cold application acceptances remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Add a true immutable source-inspection entry point under
  JST-LOCAL-008/JST-JJ-005. Existing writer entry point and all native mutation
  guards remain compatible.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.9 remains In progress at shared orchestration, host adapters and
  both cold application hosts; native source passes do not qualify app delivery.

```
<!-- /evidence-field -->

