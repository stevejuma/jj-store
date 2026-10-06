---
schema: 1
id: E-M13-018
---

# E-M13-018

<!-- evidence-field:id markdown -->
## Id

E-M13-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-browser-registration-reconnect
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-service/src/local_browser.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#registration_replays_across_restart_and_removal_preserves_metadata
- crates/jj-store-cli/tests/local_browser.rs#registration_recovers_after_initialization_and_rejects_identity_replacement
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-server --locked
```

**Notes**

Durable admission identity and reconnect boundary compile.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- registration_
```

**Notes**

Both focused registration/reconnect/recovery tests pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

All 6 tests pass after additional copied-folder and cross-repository cursor rejection; explicit initializer interruption and replay retains exact proposed identity.
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
pnpm spec:first && pnpm check:progress && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

6 protected files, 7 chapters, 111 tasks and 132 requirements pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

13 HTTP contract tests pass including the additive reconnect inventory.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --locked
```

**Notes**

8 local agent tests pass after initializer identity guard.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Additive reconnect request and response contract generated.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

115 actual HTTP/SSE responses conform; 6 contract tests and TypeScript pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all
```

**Notes**

Rust format updated.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/{local-jj,api,architecture,cli,mutations,compatibility,milestones}.md
```

**Notes**

Canonical files formatted.
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

JJ 0.45.1, Git SHA-1, macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Device journal now binds proposed_repository before first publication; additive reconnect endpoint. Companion codecs remain unchanged. Enclosing Jujutsu change qxtoormk.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified slice and integrate profile-aware frontend.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Device registration and reconnect. Shared agent initialization now has an explicit admission identity and identity guards, so retain local agent service regressions. HTTP inventory and generated schemas require retained native contract capture. Root aggregate suites and unrelated UI lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Frontend folder controls and cold mixed acceptance follow; M13.4 remains In
  progress.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


