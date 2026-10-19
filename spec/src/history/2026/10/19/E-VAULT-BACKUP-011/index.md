---
schema: 1
id: E-VAULT-BACKUP-011
---

# E-VAULT-BACKUP-011

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.7
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-FORGE-007
- JST-ARCH-007
- JST-JJ-003
- JST-JJ-005
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

- crates/jj-forge/README.md
- crates/jj-forge/src/accounts/forgejo.rs
- crates/jj-forge/src/accounts/http.rs
- crates/jj-forge/src/accounts/mod.rs
- crates/jj-forge/tests/forgejo_accounts.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-vault/README.md
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-011/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-011/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-forge/tests/forgejo_accounts.rs#explicit_forgejo_accounts_keep_native_custody_and_use_the_approved_instance
- crates/jj-forge/src/accounts/http.rs#provider_requests_use_their_own_token_header_without_network_or_redirects
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs#basic_credentials_keep_the_native_selected_login_and_reject_header_injection
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-forge --test forgejo_accounts
```

**Notes**

Before implementation, missing native-host add_forgejo_token produces two compile errors. forgejo-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-forge --test forgejo_accounts --test accounts; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-forge
```

**Notes**

After switching PAT to owned zeroizing String, two authored test compile attempts each had three errors from broad .into() replacements on contains/Option assertions. Correct those only; retain owned credentials at the public boundary and all assertions. forgejo-native-owned/forgejo-source-package logs.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-forge
```

**Notes**

All29package cases pass. Explicit Forgejo endpoint rejection happens before network; public identity, private secret custody, restart availability and exact instance repository calls pass. Actual request builders use token for Forgejo and Bearer for unchanged GitHub/Enterprise. Existing account/OAuth/planner/submission recovery cases pass. forgejo-source-accepted.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --lib vault_workspace::remote::credentials_tests; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_remote --test vault_publication
```

**Notes**

One new basic credential case plus six existing actual transfer/publication cases pass. Native-selected Forgejo login and unchanged GitHub username are encoded only in memory; username/header injection is rejected. Existing clone, stalled-network/capture separation, expected-head drift, original unknown receipts and no replay proofs pass. forgejo-git-credentials/forgejo-transport logs.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-forge --all-targets -- -D warnings; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_remote --test vault_publication -- -D warnings
```

**Notes**

Both focused strict Clippy lanes pass. No live forge account, personal credential or provider push. forgejo-source-clippy/forgejo-git-clippy logs.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Boundaries26packages,179tasks/921records and sevenvalidators/4328chapters/208requirements pass. Spec-first requires the mapped native JJ chapter for the additive Git credential constructor; add its explicit unchanged-identity/read/codec confirmation and rerun. forgejo-bookkeeping.log.
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
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first; cargo fmt --all -- --check
```

**Notes**

26package boundaries,179tasks/921records, sevenvalidators/4328chapters/208requirements pass. Final spec-first passes after mapped native identity/read confirmation. Rust formatting and changed Markdown formatting pass. Cold native app/provider and full platform/release lanes remain deferred. forgejo-bookkeeping-final.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete native desktop account/remote resolution and UI checks; live
  Forgejo/provider acceptance remains deferred.
- Recoverable external/root .jj migration, explicit existing-repository
  adoption/source selection and main integration remain required.
- Cold native-app and full platform/release lanes remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; source parent482807c4; desktop search04d9a446.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native-host Manager method and GitCredentials basic login. Existing default account wire commands, GitHub endpoint validation, OAuth, account record format and native JJ/Git identities are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit Forgejo PAT admission/custody and provider-specific API/Git authentication reuse. No default server provider expansion or new renderer credential authority. Official Forgejo API usage documentation confirms token header at https://forgejo.org/docs/latest/user/api-usage/; local evidence copy contains no credential.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete desktop Forgejo proof then recoverable storage migration and adoption/source selector.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 adds qualified native-host Forgejo PAT profile.
  JST-FORGE-007 retains injected secret custody, explicit HTTPS roots and no
  secret in public metadata/URLs/logs. JST-ARCH-007 preserves host admission and
  native imports; default GitHub server commands and schemas stay unchanged.
  JST-JJ-003/005 identities, reads, operation codecs and publication receipts
  remain unchanged by credential construction.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.6 becomes Partial with focused source/desktop search proof and
  original cold native gate open. M34.7 is the sole active stage; provider
  implementation does not complete storage/adoption/integration or live provider
  gates.

```
<!-- /evidence-field -->

