---
schema: 1
id: E-M21-038
---

# E-M21-038 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_command_recovery_contracts --test stack_workspace_contracts
```

**Notes**

workspace-discovery-before.log: 15 existing contract parents pass.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 22
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_discovery_contracts
```

**Notes**

workspace-discovery-red.log: 22 missing-type compiler diagnostics; no test body ran.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_discovery_contracts
```

**Notes**

workspace-discovery-service-first.log: One test-fixture borrow compiler diagnostic; exact assertions retained.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_discovery_contracts
```

**Notes**

workspace-discovery-service-exact.log: All 5 initial discovery parents pass after fixture correction.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_discovery_contracts --test stack_command_recovery_contracts --test stack_workspace_contracts
```

**Notes**

workspace-discovery-service-after.log: 21 parents pass after native-profile and schema/empty-catalog coverage was added. Final run follows the later empty revision-history refinement.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test stack_workspace_discovery_contracts --test stack_command_recovery_contracts --test stack_workspace_contracts
```

**Notes**

workspace-discovery-service-final.log: Final 6 discovery, 6 recovery and 9 protocol parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations public::
```

**Notes**

workspace-discovery-server-red.log: Initial server target returns 2 passes after the new method was compiled; no missing-method failure is claimed despite the log filename.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations public::
```

**Notes**

workspace-discovery-server-exact.log: Both exact catalog projection parents pass with final source.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations -- --test-threads=1
```

**Notes**

workspace-discovery-server-family.log: All 10 affected catalog parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations -- --test-threads=1
```

**Notes**

workspace-discovery-server-library.log: 5 private state-owner parents pass; one process helper is intentionally ignored by ordinary listing and exercised by its parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout -- --exact
```

**Notes**

workspace-discovery-registered-exact.log: Actual registered setup validates public selected code against real observed native checkout without modifying code/disk during the projection.
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
pnpm check:boundaries
```

**Notes**

workspace-discovery-boundaries.log: 17 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

workspace-discovery-architecture.log: 1090 application modules pass.
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
pnpm check:ui-ownership
```

**Notes**

workspace-discovery-ownership.log: Ownership boundary passes.
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
pnpm test:governance
```

**Notes**

workspace-discovery-governance.log: 50 governance parents pass.
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
pnpm docs:review services "Reviewed bounded current-workspace discovery inputs and selected catalog contracts, profile and readiness linkage, terminal-history separation and strict public response limits. The manual distinguishes these records from the unimplemented registered runtime join and production transport."
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
pnpm docs:review configuration "Reviewed existing-state selected combination projection, authenticated stable scope, pending coordination without proposed-state disclosure, corruption refusal and unchanged disk/code reads. Lifecycle documentation distinguishes real setup projection checks from unimplemented registered discovery and production picker wiring."
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

