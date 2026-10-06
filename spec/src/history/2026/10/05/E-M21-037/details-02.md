---
schema: 1
id: E-M21-037
---

# E-M21-037 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_contracts
```

**Notes**

public-command-before.log: Before state: 9 pass.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_command_recovery_contracts
```

**Notes**

public-command-red.log: 17 compiler diagnostics for missing public contracts; no test body ran.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal public::
```

**Notes**

public-command-server-red.log: 12 compiler diagnostics for missing projection methods; no test body ran.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_command_recovery_contracts --test stack_workspace_contracts
```

**Notes**

public-command-service.log: 5 recovery parents pass, 1 schema lookup fails; schema assertions retained.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_command_recovery_contracts public_schema_excludes_private_execution_and_current_snapshots -- --exact
```

**Notes**

public-command-schema-exact.log: Exact original runtime regression passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_command_recovery_contracts --test stack_workspace_contracts
```

**Notes**

public-command-service-after.log: 6 recovery and 9 protocol parents pass; valid oversized preview is rejected without partial output.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal public::
```

**Notes**

public-command-server-exact.log: All 3 exact projection parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal -- --test-threads=1
```

**Notes**

public-command-journal-family.log: All 16 affected journal parents pass using explicit serial fixture lane.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout -- --exact
```

**Notes**

public-command-registered-exact.log: Actual registered setup with retained archive, safe projections and unchanged disk/code reads passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation -- --test-threads=1
```

**Notes**

public-command-registered-family.log: All 19 actual registered authorization/preparation/execution/recovery parents pass.
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
pnpm check:boundaries
```

**Notes**

public-command-boundaries.log: 17 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

public-command-architecture.log: 1090 modules pass.
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
pnpm check:ui-ownership
```

**Notes**

public-command-ownership.log: Owned UI boundary passes.
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
pnpm test:governance
```

**Notes**

public-command-governance.log: 50 governance parents pass.
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
pnpm docs:check
```

**Notes**

public-command-docs.log: 139 documentation files, source reviews, build and links/anchors pass.
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
pnpm docs:review services "Reviewed stable original command recovery scope, validated preview and historical receipt linkage, bounded actor-scoped pending discovery and cancellation identity inputs. The transport reference distinguishes these service contracts from unimplemented routes and coordinated cancellation."
```

**Notes**

tool result: Owning source fingerprint refreshed after page review.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review configuration "Reviewed existing-state journal projections for original command status and pending requests, actor/scope checks, terminal history and private execution exclusion. Lifecycle documentation keeps registration admission, coordinated cancellation and HTTP composition outside this primitive."
```

**Notes**

tool result: Owning source fingerprint refreshed after page review.
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
cargo fmt --all
```

**Notes**

tool result: Rust formatting applied.
<!-- /run -->
<!-- /evidence-field -->

