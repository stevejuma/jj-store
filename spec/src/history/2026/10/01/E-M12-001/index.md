---
schema: 1
id: E-M12-001
---

# E-M12-001

<!-- evidence-field:id markdown -->
## Id

E-M12-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-dispatch
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change tunulptm.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean native WorkStore main at 90390af4. Exact existing native unsupported transport regression passed 1/1 before implementation. JJ 0.45.1 already exposes parsed dispatch hooks; native backend is jj-store. No Git converter, cache or materializer exists in production.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1; gix 0.87.1; Git 2.55.0; Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native profile jj-native-0.45.1-v1 unchanged; bridge format-4 is planned only.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/git_bridge.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/presentation.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/tests/executable_native.rs
- spec/src/git-interop.md
- spec/src/SUMMARY.md
- spec/src/cli.md
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec-validator.config.mjs
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
cargo test -p jj-store-cli --locked --test executable_native native_cli_semantic_git_transport_and_gc_fail_without_publication -- --exact
```

**Notes**

Before state 1/1; memory-backed native stack, not real RustFS qualification.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-cli
```
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
cargo test -p jj-store-cli --locked --test executable_native native_git_dispatch_honors_alias_repository_and_no_snapshot -- --exact
```

**Notes**

1/1 native alias/-R/dry-run/remote/import/export routing; unsnapshotted files and native manifest unchanged. Shared dispatch and governance map affect direct consumers: run complete governance tests and focused ordinary dispatch acceptance.
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
node --test scripts/governance.test.mjs
```

**Notes**

24/24 complete governance regressions; wider lane justified by shared canonical mapping.
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
pnpm exec prettier --write spec/src/git-interop.md spec/src/SUMMARY.md spec/src/cli.md spec/src/architecture.md spec/src/spec-governance.md spec/src/milestones.md spec/src/verification.md spec-validator.config.mjs
```

**Notes**

Canonical formatting.
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
cargo test -p jj-store-cli --locked --test executable_native custom_help_completion_build_info_and_ordinary_git_commands -- --exact
```

**Notes**

1/1 ordinary upstream Git repository creation/describe/log plus custom help/completion.
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
cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Affected CLI package and direct dependency compilation; no unrelated UI lane.
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
pnpm check:boundaries
```

**Notes**

15 owned packages.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Initial appended verification rows lacked a Markdown table header; moved rows into the canonical table.
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
pnpm check:progress
```

**Notes**

Initial M12 rows were outside the task table; moved into existing canonical task table.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md spec/src/spec-governance.md
```

**Notes**

Reformatted repaired canonical tables.
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

20 chapters, 123 unique requirements.
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
pnpm check:progress
```

**Notes**

94 tasks, retained all existing evidence.
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

6 protected files mapped to 5 changed owning chapters.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --locked --test executable_native native_cli_semantic_git_transport_and_gc_fail_without_publication -- --exact
```

**Notes**

After state 1/1; unsupported import/export and destructive GC remain non-publishing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_dispatch_honors_alias_repository_and_no_snapshot
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_git_transport_and_gc_fail_without_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M12-001 details 2](details-02.md)
