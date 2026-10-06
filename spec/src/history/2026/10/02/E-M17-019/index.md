---
schema: 1
id: E-M17-019
---

# E-M17-019

<!-- evidence-field:id markdown -->
## Id

E-M17-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

per-action-current-directory-admission-and-native-write-publication-guard
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_authority.rs
- crates/jj-store-server/src/capture_authority/publisher.rs
- crates/jj-store-server/tests/capture_authority.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/security.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_authority.rs#guarded_publisher_rechecks_directory_and_catalog_after_preparation_and_preserves_exact_replay
- crates/jj-store-server/tests/capture_authority.rs#native_publisher_requires_current_admission_and_retains_receipt_actor_and_code_heads
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
cargo test -p jj-store-server --test capture_authority guarded_publisher_rechecks_directory_and_catalog_after_preparation_and_preserves_exact_replay --locked -- --exact > /tmp/jj-store-agent-capture-m172-guard-before.log 2>&1
```

**Notes**

Exact new regression establishes one missing publisher constructor interface.
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
cargo fmt -p jj-store-server
```

**Notes**

Repeated focused formatting after the guard and shared fixture update.
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
cargo test -p jj-store-server --test capture_authority guarded_publisher_rechecks_directory_and_catalog_after_preparation_and_preserves_exact_replay --locked -- --exact > /tmp/jj-store-agent-capture-m172-guard-after.log 2>&1
```

**Notes**

Exact guard regression passes; removed registration, changed binding and foreign actor withhold prepared publication, restored proof preserves exact replay.
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
cargo test -p jj-store-server --test capture_authority --test capture_admission --test capture_activity --locked > /tmp/jj-store-agent-capture-m172-guard-server.log 2>&1
```

**Notes**

All six direct actor/native-write/guard/activity/directory admission cases pass, including a retained committed receipt denied on replay while catalog membership is removed.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-guard-check.log 2>&1
```

**Notes**

Affected server and its direct executable consumer pass all-target checks.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-guard-clippy.log 2>&1
```

**Notes**

Strict all-target lint passes without suppression.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-guard-format.log 2>&1
```

**Notes**

Rust and embedded-runtime formatting pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-guard-boundaries.log 2>&1
```

**Notes**

17-package dependency boundaries pass; no new dependency.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-guard-doc-write.log 2>&1
```

**Notes**

Changed canonical documents formatted.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-guard-progress.log 2>&1
```

**Notes**

129 tasks validate; 274 evidence structures preserve all 273 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-guard-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-guard-first.log 2>&1
```

**Notes**

Three protected files and four canonical chapters pass spec-first.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-guard-doc-format.log 2>&1
```

**Notes**

All changed canonical document formatting passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; actual native service/WAL/receipts over MemoryStore and synthetic constant sanitized Codex gap. This is not a live harness, real RustFS or full server lifecycle proof.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive borrowed in-process publisher holds only an in-memory directory proof, without Debug/serialization. Every action binds a fresh native Write request. No credential, object/wire, pending/cursor, prepared request, receipt, sanitizer or actor format changes.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change popullqw
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One server-side publication admission boundary adds a guarded native publisher constructor distinct from the bare NativeRequest adapter. Every bound/unbound preparation and prepared publication repeats exact current directory/socket/catalog admission and native Write authorization through CaptureAuthority::bind. Preparation additionally revalidates device admission before returning its candidate; publication refuses a foreign authenticated actor and returns native committed/unknown outcomes directly without a post-commit device failure. A previously prepared request cannot bypass removed catalog membership or rebound directory proof; restored admission permits its same exact request/receipt replay, with unchanged code heads and fixed actor. Current native service authorization remains separate from device admission. Exact regression and all six direct server capture tests, affected/direct-consumer all-target checks/lint, formatting and dependency boundaries qualify this slice. No automatic source registration, activity eligibility, spool selection/fairness, checkpoint, startup or typed-control authority is added. M17.2 and aggregate M17 stay In progress; admitted reader-to-sanitizer/spool composition, fair bounded supervised workers/deadlines/backpressure/pause/restart, pending-source recovery, absent-domain initialization, Cursor/OpenCode readers, real harness/cold full-flow and full M17.3-M17.7 roadmap remain pending. Existing enabled-startup guard remains. No browser, native/WAL implementation, installed daemon, release or deployment changed; unrelated wider/root aggregate lanes are out of scope.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


