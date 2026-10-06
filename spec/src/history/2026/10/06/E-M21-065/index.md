---
schema: 1
id: E-M21-065
---

# E-M21-065

<!-- evidence-field:id markdown -->
## Id

E-M21-065
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

readable native stack operation descriptions and graph preview verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rxrmtyxr: Write readable operation descriptions for local Stacks
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Format new native Stacks operation metadata as single-line action text; retain native Undo/Redo and guarded legacy merge cleanup. Verify Workspace graphs still use per-row lanes and refresh port 6057 from the old worktree to main.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_stacks/cleanup.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/operation_description.rs
- crates/jj-store-jj/tests/local_stacks/operation_descriptions.rs
- crates/jj-store-jj/tests/local_stacks/topology.rs
- crates/jj-store-jj/tests/local_stacks.rs
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; macOS ARM64; pnpm 10.32.1; Storybook 10.5.7; Vitest 4.1.11; existing local Design Core validation dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No command/API/schema/TOML changes. New operation metadata descriptions change; historical operations are untouched. Native Undo/Redo reference descriptions retain their existing protocol. No sample repository commands executed.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The native writer serialized Rust Debug command fields into operation.description, including user messages/resolution content and selections. Port 6057 was serving the older jj-store-stack-workspace-ui checkout; main already had the per-row graph layout.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks/operation_descriptions.rs#stack_operations_persist_readable_descriptions
- crates/jj-store-jj/src/local_stacks/operation_description.rs#descriptions_omit_content_messages_and_selection_tokens
- crates/jj-store-jj/src/local_stacks/operation_description.rs#descriptions_normalize_labels_and_spell_out_placement
- crates/jj-store-jj/tests/local_stacks/topology.rs#reconciliation_only_retires_proven_unreferenced_legacy_heads
- crates/jj-store-jj/tests/local_stacks.rs#native_undo_redo_includes_cli_and_never_reapplies_toml_membership
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
jj --no-pager st/diff; rg local Stacks operation descriptions and graphInset; ps/lsof port 6057 ownership inspection
```

**Notes**

Started clean rxrmtyxr above 479b3a01. Source default graphInset override is absent from Workspace/Branches and target history. Main preview switch leaves the old workspace checkout untouched. Evidence under .local/stack-operation-descriptions/.
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
cargo test -p jj-store-jj --test local_stacks stack_operations_persist_readable_descriptions --locked
```

**Notes**

before.log: 1 failed as expected; persisted Setup description includes Debug braces and fields. Existing unused state assignment warning in the native fixture is unchanged.
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
cargo test -p jj-store-jj --test local_stacks stack_operations_persist_readable_descriptions --locked
```

**Notes**

after.log: 1 passed. Reads stored descriptions using stock jj op log for setup, apply, individual/grouped unapply, target, create, switch, update and delete.
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
cargo test -p jj-store-jj --lib local_stacks::operation_description --locked
```

**Notes**

formatter-tests.log: 2 passed; excludes contents, messages and opaque selection tokens; normalizes whitespace and describes placement without enum Debug.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

native-stacks.log:33 passed. Entire directly affected command module covers normal rewrites, native Undo/Redo and guarded cleanup using both old Apply/SetTarget and new Set target descriptions. No unrelated crate test suites run.
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
cargo check -p jj-store-jj --locked
```

**Notes**

check.log:affected package check passes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check
```

**Notes**

rustfmt.log:added test module was not alphabetized. Reordered only the module declarations.
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
cargo fmt --all -- --check; pnpm spec:first; pnpm spec:validate
```

**Notes**

rustfmt-final.log, spec-first.log and spec-validation.log:formatting, 6 protected files / 2 owning chapters, 5 validators / 23 chapters / 173 requirements pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceGraph.stories.svelte
```

**Notes**

graph-stories.log:3/3 cold Chromium stories pass. Workspace, Branches and target history assert 32px tip inset versus wider merge inset, and horizontal/vertical scrolling; no shared avatar alignment. No UI source edit needed.
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
Verify PID 75160 with ps/lsof; SIGTERM only the 6057 Storybook process; pnpm storybook --port 6057 --ci from main
```

**Notes**

storybook-main.log: preview restarted on the same 6057 URL from default/main, replacing the older-worktree process. Original checkout and unrelated services retained; CUA reports no accessible tab on browser 2.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/.local/dev-demo JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:build
```

**Notes**

backend-build.log: completed in 1m 18s. Linker warns that large __eh_frame exceeds compact unwind encoding capacity; build succeeds.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/.local/dev-demo JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:restart
```

**Notes**

backend-restart.log: backend replaced with main binary, fixture preserved and search ready. Runtime confirms backend PID 52020 and unchanged Vite PID 78723; HTTP GET / returns 200 on ports 7476 and 6057. Unrelated services unchanged.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Guarded ledger append and Prettier formatting using Node
```

**Notes**

Byte-preservation assertion rejected the first append before any source write: JavaScript replacement-string expansion interpreted historical dollar markers. Corrected to callback replacement, preserving literal evidence.
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
pnpm check:progress; pnpm spec:first; pnpm exec prettier --check spec/src/milestones.md spec/src/local-jj.md spec/src/jj-integration.md
```

**Notes**

progress-final.log, spec-first-final.log and format-final.log pass. Six protected files and three canonical chapters covered. Guarded append confirms all 613 previous evidence records remain byte-identical.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused affected Rust package, 33 native command scenarios, 2 formatter units and 3 existing graph interaction stories; specification/progress/format gates. No full repository suite claim.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-065 details 2](details-02.md)
