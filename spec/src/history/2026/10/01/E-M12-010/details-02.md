---
schema: 1
id: E-M12-010
---

# E-M12-010 details 2

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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_https_uses_approved_ca_and_askpass_without_serializing_secrets -- --exact
```

**Notes**

Initial HTTPS fixture refuses fetch despite explicit CA because transport strips it.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_https_uses_approved_ca_and_askpass_without_serializing_secrets -- --exact
```

**Notes**

Actual authenticated HTTPS fetch and push pass; missing CA refuses without publication; native bearer removed from askpass; credentials absent from durable records.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_ssh_uses_approved_key_and_strict_host_trust -- --exact
```

**Notes**

Actual isolated unprivileged OpenSSH daemon with generated keys; fetch/push pass, missing host trust refuses without native effects.
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
cargo fmt --all && pnpm web:build && JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

Fresh browser bundle builds with existing shared-library CSS/chunk warnings. Unqualified Rust test name matches zero cases; not gate evidence.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

Fixture parsed JJ display alphabet as native hex ChangeId; switched to pinned normal_hex template for typed anchor.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_https_uses_approved_ca_and_askpass_without_serializing_secrets -- --exact
```

**Notes**

Expanded hard cancellation during actual credential prompt kills helper; arbitrary helper refused; generated authenticated publication still passes.
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
cargo fmt --all && JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

First full fetch after selected publication correctly adds complete feature-ref observation; fixture now checks code heads/version unchanged and second full fetch no-op.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

Cold browser freshness/explicit refresh passes. Search assertion used workspace scope, which excluded the external main merge; added explicit imported bookmark scope for main.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-cli --locked --lib --test executable_native --test native_rpc --test workspace_migration --test server_executable && cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

7 library cases pass; executable suite 21 pass, 1 fail, 3 ignored. Corruption fixture attempted to overwrite its read-only loose Git object. Changed only that owned fixture object to mode 0600 before corruption; downstream commands did not run.
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
pnpm web:check && pnpm exec prettier --write web/tests/real/git-bridge.spec.ts spec/src/api.md spec/src/cli.md spec/src/git-interop.md spec/src/security.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Web check 0 errors/0 warnings and spec validation 20 chapters/123 requirements pass. Progress rejected browser pointer lacking named test; added exact test title. Later governance commands did not run.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && pnpm exec prettier --write web/tests/real/git-bridge.spec.ts spec/src/milestones.md && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Named-test repair also inserted the title into files list; progress rejected non-file path. Removed title only from files list, retaining it in tests.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_https_uses_approved_ca_and_askpass_without_serializing_secrets -- --exact && JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

HTTPS regression now passes including corruption refusal and hard credential cancellation. Cold browser passes, but content search remains empty because merge changed only message. Added external.txt with searchable content while retaining feature tree and ordered merge parents [base, feature].
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
pnpm exec prettier --write spec/src/milestones.md && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

94 tasks, 7 protected files/5 owning chapters and 15 package boundaries pass after pointer repairs.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --lib --test executable_native --test native_rpc --test workspace_migration --test server_executable && cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

7 library, 22 executable, 7 native RPC, 14 server and 1 migration cases pass; Clippy passes with warnings denied. 3 OS credential and 3 separate real-storage cases ignored by regular suites. The new real-storage case is run separately below. Wider CLI lanes justified by shared credential/transport changes and final feature qualification.
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
cargo fmt --all && JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

Exact real persistent embedded RustFS target: 1 passed, 0 failed, 26.06 seconds. Fetch native base; edit/capture/review; publish feature with original Git parent; delete all bridge caches; cold restart; retain native CommitId/ChangeId and review anchor/agent receipt; repeated fetch no-op; actual external merge; served browser SSE freshness with graph pinned until refresh; imported content searchable; final cold retained Git facts and receipts; new/describe work with Git host offline.
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
pnpm exec playwright test --config web/playwright.real.config.ts git-bridge.spec.ts
```

**Notes**

Nested exact browser case run by cold RustFS fixture against its fresh served web build: Git import waits for refresh. Browser launches before CLI fetch, asserts freshness without graph/route replacement, then explicit refresh displays external merge. No visual baseline acceptance.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native custom_color_json_and_metadata_validation_do_not_contact_or_leak_credentials -- --exact && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Final capability/help metadata compiles and exact presentation case passes 1/1. Existing dry-run/lease/cache-loss/attempt recovery case passes 1/1 after source/destination/profile/per-ref planning output changes.
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
pnpm exec prettier --write AGENTS.md docs/cli/commands.md spec/src/api.md spec/src/cli.md spec/src/git-interop.md spec/src/milestones.md spec/src/security.md spec/src/spec-governance.md web/tests/real/git-bridge.spec.ts && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Final Done matrix: 20 chapters/123 requirements, 94 tasks, 11 protected files/6 changed owning chapters and 15 package boundaries pass. Removed stale nested Markdown fence from ledger; formatter normalized existing arrays. Every earlier evidence entry is semantically unchanged.
<!-- /run -->
<!-- /evidence-field -->

