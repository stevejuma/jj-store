---
schema: 1
id: E-CLI-GIT-CRATES-001
---

# E-CLI-GIT-CRATES-001

<!-- evidence-field:id markdown -->
## Id

E-CLI-GIT-CRATES-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-agent/Cargo.toml
- crates/jj-collab/Cargo.toml
- crates/jj-forge/Cargo.toml
- crates/jj-history-service/Cargo.toml
- crates/jj-review/Cargo.toml
- crates/jj-store-adapters/Cargo.toml
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-http/Cargo.toml
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-model/Cargo.toml
- crates/jj-store-object/Cargo.toml
- crates/jj-store-read-index/Cargo.toml
- crates/jj-store-replication/Cargo.toml
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-server/Cargo.toml
- crates/jj-store-service/Cargo.toml
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-wal/Cargo.toml
- crates/jj-vault/Cargo.toml
- crates/jj-workflow/Cargo.toml
- crates/tauri-plugin-jj-store/Cargo.toml
- runtime/embedded-rustfs/Cargo.lock
- runtime/embedded-rustfs/Cargo.toml
- scripts/package-check.mjs
- scripts/release-metadata.mjs
- scripts/release-registry.mjs
- scripts/release.test.mjs
- spec/src/current/contracts/adapters/01-adapters-and-tantivy.md
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/cli/40-public-installation.md
- spec/src/current/contracts/collaboration/21-capture-sanitization-boundary.md
- spec/src/current/contracts/compatibility/01-compatibility-and-release.md
- spec/src/current/contracts/identity/01-identity-and-native-codecs.md
- spec/src/current/contracts/jj-integration/01-native-jj-integration.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/mutations/01-typed-mutations.md
- spec/src/current/contracts/object-store/01-object-storage.md
- spec/src/current/contracts/publication/01-publication-and-recovery.md
- spec/src/current/contracts/replication/01-local-first-writable-replication.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
PATH="/opt/homebrew/opt/python@3.11/bin:$PATH" node --test scripts/release.test.mjs
```

**Notes**

Release graph test confirms the four git-only crates are excluded and jj-store stays publishable.
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

jj-lib and jj-cli 0.46.0; embedded rustfs 1.0.1; embedded tokio 1.54.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. Git-only crates set publish to false.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Publish the jj-store CLI and the crates it links. Keep the other crates on git. Combine the open Dependabot Cargo updates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Close the separate Dependabot pull requests after this combined change is open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now publishes the jj-store command and the crates it
  links. jj-history-service, jj-vault, tauri-plugin-jj-store, and
  jj-store-embedded-rustfs stay git-only.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This does not complete release qualification.

```
<!-- /evidence-field -->
