---
schema: 1
id: E-M13-002
---

# E-M13-002 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_work
```

**Notes**

Before state: local_work test target did not exist.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-service --locked
```

**Notes**

Two attempts used nonexistent StoreFactories::default; resolver replaced with pinned GitBackend::load.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_work
```

**Notes**

Initial fixture dependency missing tempfile; test-only dependency added.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-service --offline
```

**Notes**

Service check and lockfile dependency addition pass; no dependency version changes.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_work
```

**Notes**

Initial 6 service tests passed; two additional cursor/concurrency tests await expanded run.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_work
```

**Notes**

Client returned CommandError but callers wrapped it with Display-only error helper; wrappers removed.
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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-cli
```

**Notes**

Affected package formatting.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_work
```

**Notes**

2 executable tests pass across colocated/internal/external Git and shared JJ workspaces.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_work --test work_store
```

**Notes**

8 local and 3 native WorkStore service tests pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test executable_native executable_workstore_creates_tasks_comments_and_recovers_without_an_upstream
```

**Notes**

Retained native executable WorkStore regression passes.
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
pnpm exec prettier --write docs/cli/work.md docs/cli/capabilities.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md spec/src/projects.md spec/src/milestones.md
```

**Notes**

Changed documents formatted.
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
pnpm spec:validate
```

**Notes**

21 chapters, 129 requirements pass.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

15 packages pass; tempfile is test-only.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

24 protected files, 7 canonical chapters pass.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Evidence test references lacked names and used invalid result spelling; corrected to named references and pending.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test executable_native custom_help_completion_build_info_and_ordinary_git_commands
```

**Notes**

Existing ordinary Git executable superset/help regression passes.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-cli --test local_work -p jj-store-service --test local_work -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

New service receipt lookup had a collapsible if; corrected without suppressions.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-cli --test local_work -p jj-store-service --test local_work -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

Affected libraries and exact local CLI/service test targets pass with no warnings.
<!-- /run -->
<!-- /evidence-field -->

