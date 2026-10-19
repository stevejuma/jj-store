---
schema: 1
id: E-VAULT-BACKUP-004
---

# E-VAULT-BACKUP-004

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-FORGE-007
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
- crates/jj-forge/src/accounts/store.rs
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-004/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-004/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-forge/tests/accounts.rs#validated_accounts_survive_restart_without_secrets_in_metadata_or_responses
- crates/jj-forge/tests/accounts.rs#cancelling_an_inflight_device_poll_discards_its_late_token_response
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-forge --test accounts
```

**Notes**

Seven existing native account regressions pass: persisted identities without tokens, OAuth interval/expiry/slowdown/cancellation, in-flight cancellation fence, token refresh and validation. Log .local/vault-backup/forge-source-tests.log. Native default namespace stays md.lapis.jj-store.forge.v1.
<!-- /run -->

<!-- run -->
### Run 2

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

26 package boundaries and seven validators/4314chapters/208requirements passed. Progress and dependent spec-first rejected one evidence test reference because it lacked a named test anchor. Added exact existing test names; no product acceptance weakened.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Progress179tasks/914evidence records, seven validators4314chapters/208requirements and spec-first2protected files/twoowning canonical chapters pass. Consumer account compilation, two native worker tests, one command-frame test, 15renderer/governance cases and strict types pass separately in E-REPO-029. Full suites/provider/cold actual-WebView gates remain deferred.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Remote configuration, fresh-device restore, automatic two-way sync, history
  search, advanced storage/adoption and Forgejo remain required implementation.
- Full suites and complete cold actual-WebView/provider acceptance deferred by
  explicit user instruction.
- Main integration follows completion of the staged implementation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Rust1.99; Node26.9; source pnpm10.32.1; desktop pnpm10.34.5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ/Git and server storage/WAL/manifest profiles unchanged. Account metadata format unchanged. Additive native SecretStore namespace injection.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reusable forge account composition for the desktop installation. ScopedNativeSecrets accepts a host-owned OS credential namespace; existing default NativeSecrets delegates to the identical jj-store service name. The desktop reuses only the configured public OAuth client identity and supplies separate metadata, custody and native main-window admission.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish focused account source/consumer verification, then remote configuration and fresh-device restore in M34.4.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 and JST-FORGE-007 explicitly distinguish desktop-owned
  account custody/admission from jj-store LocalTrusted Admin service admission.
  The existing default jj-store credential namespace and service authorization
  are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.3 becomes Partial with focused local restore proofs and cold native
  app acceptance deferred. M34.4 is the only active delivery stage; account
  composition is one slice of its original remote setup/restore gate. No
  completion claimed from compilation or mocks.

```
<!-- /evidence-field -->

