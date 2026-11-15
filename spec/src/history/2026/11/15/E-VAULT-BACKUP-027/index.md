---
schema: 1
id: E-VAULT-BACKUP-027
---

# E-VAULT-BACKUP-027

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-027
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/process.rs
- crates/jj-store-jj/src/vault_workspace/remote/adoption.rs
- crates/jj-store-jj/tests/vault_ssh_transport.rs
- scripts/test-vault-ssh.mjs
- docs/site/technical/local.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/governance/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_ssh_transport.rs#real_ssh_preserves_files_rejects_host_key_changes_and_recovers_lost_push_responses
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
node scripts/test-vault-ssh.mjs
```

**Notes**

First attempt refused the managed 30 GiB disk reserve before executing Rust acceptance. An intermediate run isolated HOME too broadly and began downloading the toolchain; stopped that child, retired the disposable fixture and retained host Rust/Cargo cache paths explicitly.
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
node scripts/test-vault-ssh.mjs
```

**Notes**

One real loopback sshd/agent test passed. Fetch/import preserves files; wrong host key is rejected; only the selected branch is published; dropped accepted response retains the request and reconciliation completes without a second receive-pack. Disposable identities, trust files, services and HOME are retired.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_transport_validation --test vault_remote --lib
```

**Notes**

62 library tests passed, 3 existing opt-in/platform ignores; 4 local-Git remote tests and 3 endpoint/transport tests passed.
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
pnpm dev:cargo -- clippy -p jj-store-jj -p jj-vault --no-default-features --features jj-store-jj/vault-backup --lib -- -D warnings
```

**Notes**

Strict direct-source/consumer Clippy passed.
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
pnpm spec:validate; pnpm check:progress; pnpm spec:first -- --base 778933aa --head @
```

**Notes**

Eight validators pass across 4,652 chapters and 211 requirements. Progress retains 186 tasks and 1,048 receipts. Initial checks caught missing scoped SUMMARY navigation and mapped chapter/evidence coverage; corrected without regenerating historical navigation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- External HTTPS provider/token transport, shared Core/Protocol integration,
  original-repository remote operations and cold Desktop/Server qualification
  remain outstanding.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust/Cargo 1.99.0; jj-lib 0.46.0; Node 24.10.0; local macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted binding, identity or push-receipt changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Host-only existing read-only SSH trust-file selection outside the vault and real disposable SSH qualification. Renderer/repository paths cannot select trust. Strict host checks remain enforced.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete specification gates, commit source qualification, then wire and qualify both hosts.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 permits a native host-selected existing trust file outside
  vault state, without trust mutation or renderer path admission.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.5 remains Partial; real loopback SSH proof does not qualify provider
  or cold host integration.

```
<!-- /evidence-field -->

