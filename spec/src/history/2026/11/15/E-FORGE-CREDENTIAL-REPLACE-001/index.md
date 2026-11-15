---
schema: 1
id: E-FORGE-CREDENTIAL-REPLACE-001
---

# E-FORGE-CREDENTIAL-REPLACE-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-CREDENTIAL-REPLACE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-FORGE-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-forge/src/accounts/mod.rs
- crates/jj-forge/src/accounts/replacement.rs
- crates/jj-forge/tests/credential_replacement.rs
- crates/jj-forge/README.md
- docs/site/guides/git.md
- docs/site/reference/transport.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/verification/forge.md
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
- spec/src/current/progress/M31/index.md
- spec/src/current/governance/qualification-change-reviews.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-forge/tests/credential_replacement.rs#replacement_preserves_the_selected_account_and_scope
- crates/jj-forge/tests/credential_replacement.rs#replacement_refuses_a_secret_refreshed_without_a_metadata_revision
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs test --locked -p jj-forge --test credential_replacement
```

**Notes**

Baseline: missing Manager::replace_token API fails compilation once.
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs test --locked -p jj-forge --test credential_replacement
```

**Notes**

Initial replacement scenario passes. Strengthening fixture then failed with four compiler diagnostics because derive(Default) attached to a type alias; corrected before rerunning the three guard scenarios.
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs test --locked -p jj-forge
```

**Notes**

Final 35 passed, zero failed/ignored across nine suites including doc tests. Six replacement scenarios cover GitHub, Enterprise and Forgejo, OAuth reference retention, invalid/different identity, endpoint tampering, concurrent forget/new token, locked custody, and secret refresh without metadata revision.
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs test --locked -p jj-forge --no-default-features
```

**Notes**

Final injected-only profile: same 35 passed, zero failed/ignored. No platform keyring fallback.
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs clippy --locked -p jj-forge --all-targets -- -D warnings
```

**Notes**

Strict all-target default-profile Clippy passes.
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
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs clippy --locked -p jj-forge --all-targets --no-default-features -- -D warnings
```

**Notes**

Strict injected-only Clippy passes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH node .local/native-remotes-validation.mjs build --locked -p jj-forge
```

**Notes**

Crate build passes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=$PWD/.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

Documentation closure: 144 files; links/anchors and nine Mermaid diagrams pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Initial evidence rejected a test-file reference without a named scenario and nonempty remaining list on a passing slice. Corrected to named scenarios and scoped future gates under next/milestone review.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
```

**Notes**

Initial new record had an unnecessary empty SUMMARY.nav.md, rejected as unreferenced; removed only that new file.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
```

**Notes**

121 passed, zero failed/skipped. Governance consumes the changed evidence and documentation boundary; unrelated product layers are outside the Rust-only targeted policy.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress && JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate && JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Final progress and canonical validation pass; spec-first passes seven protected files and three canonical chapters. Initial spec-first also identified the missing documentation source-qualification chapter; added the owning JST-DOC-002 mapping. Rustfmt and scoped Prettier checks pass.
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

Rust 1.99.0; Node 24.10.0; spec-validator 0.2.0. Immutable tracked archives in remote Docker image sha256:7d3aa64f81dce0988978ac830b4cc53ff5713d3acbf19163bd4987ff3e858efe; 2 CPUs/8 GiB; existing isolated cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native Manager::replace_token API; existing wire commands, account metadata and credential storage formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Pin approved provider endpoint, canonical opaque identity, account revision and saved credential before token verification. Reopen the store and compare before any replacement. Reject forgotten or changed accounts and concurrent refreshes without custody writes. Preserve the selected reference and provider method, refresh public identity fields and clear stale OAuth refresh/expiry material. Tokens remain zeroized native temporaries.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Verify required hosted CI and merge the exact checked SDK head, then intake its immutable revision into Core/server and implement host providers.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-FORGE-007 now explicitly requires identity-checked native
  replacement and concurrent metadata/credential refusal. Host admission,
  pending-publication holds and existing wire contracts are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress. Mock provider transport proves guards, not
  live-provider or packaged-app completion. M34.4 remains Partial; neither
  application host is adopted by this SDK slice.

```
<!-- /evidence-field -->

