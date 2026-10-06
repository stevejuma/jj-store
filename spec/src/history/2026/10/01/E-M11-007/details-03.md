---
schema: 1
id: E-M11-007
---

# E-M11-007 details 3

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
Cold diagnostic discovery probe after exact run completion
```

**Notes**

Connection refused because the successful exact cold harness had already stopped its isolated service and removed its disposable root. This is expected cleanup, with no remaining service fault. .scratch/work-seed-cold.log retains 1/1 successful test and cleanup evidence.
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
JJ_STORE_DEV_PORT=7482 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm web:test:real
```

**Notes**

Seven of eight tests pass: all five native-repository cases, work-seed and work-store consumers pass against the physically restarted fixture. Cold startup 828 ms, prepared restart 871 ms, resident 171808 KiB, preparation 374722 ms. The unfiltered command additionally collects git-bridge.spec.ts, which fails at missing Native Git feature before fetch. That scenario does not consume dev-demo: crates/jj-store-cli/tests/server_executable/git_bridge.rs:323 supplies its separate Rust-owned Git history and JJ_STORE_GIT_CYCLE_WORKSPACE. The generic invocation lacks that fixture. Its independent GB6 gate and collector correction remain outside this seed change. No assertions weakened or required consumers skipped. Disposable service stopped and root removed. .scratch/work-seed-native-consumers.log.
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
pnpm spec:validate
```

**Notes**

Final document gate: 4 validators, 20 chapters, 125 requirements. .scratch/work-seed-spec-final.log.
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
pnpm spec:first
```

**Notes**

Final protected-change gate: 6 protected files, 5 canonical chapters. .scratch/work-seed-spec-first-final.log.
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
pnpm check:progress
```

**Notes**

Final evidence bookkeeping passes for 96 tasks. M11/M11.4 remain In progress. .scratch/work-seed-progress-final.log.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Updated only E-M11-007 and M11 evidence references; final ledger formatting.
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
pnpm exec prettier --check package.json scripts/dev-demo.mjs scripts/dev-demo-work.mjs scripts/dev-demo-work-samples.mjs scripts/dev-demo-work.test.mjs web/tests/real/work-seed.spec.ts spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

All 11 changed files conform. .scratch/work-seed-format-final.log.
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
lsof -nP -iTCP:7460 -sTCP:LISTEN -iTCP:6046 -sTCP:LISTEN
```

**Notes**

Diagnostic option error: lsof rejects duplicate TCP LISTEN inclusion. Corrected to one -sTCP:LISTEN for both ports; no service mutation.
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
lsof -nP -iTCP:7460 -iTCP:6046 -sTCP:LISTEN
```

**Notes**

Main PID 60331 still listens on 7460; Storybook PID 27372 still listens on 6046. Existing processes retained.
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
test ! -e /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-uokN86
```

**Notes**

Broader disposable fixture root removed after owned service cleanup.
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
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:7460/
```

**Notes**

Current main development UI responds HTTP 200.
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
pnpm web:check
```

**Notes**

Svelte diagnostics: 0 errors and 0 warnings. .scratch/work-seed-web-check.log.
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

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Playwright 1.63.0; existing owned embedded RustFS and native development binaries.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing owned HTTP Work V1 requests and receipt-selected native WorkStore records. Three repository-scoped stable project IDs, nine task IDs and six comment IDs; all four task states, three subtasks, three threaded replies and active home links. Exact command bytes determine stable request identities. Interrupted initial task state changes resume only with unchanged original metadata at version one. Existing edits, retired links and tombstones are preserved.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The shared development seed changes the native-browser fixture, so its direct consumers native-repository.spec.ts, work-store.spec.ts and work-seed.spec.ts are requalified after exact seed regression and isolated cold acceptance. All seven direct-consumer cases pass within the broader eight-test invocation. The independently Rust-provisioned git-bridge scenario does not consume dev-demo and its missing-fixture failure is recorded separately. Backend implementations, public contracts and UI presentation are unchanged; root aggregates, full Rust workspace, unrelated Storybook and registry release qualification are out of scope. Cold fixtures are separate from interactive data; additive interactive seeding never resets it. Stories remain visual-pending; no baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain additive seed, interrupted creation and cold native fixture consumer regressions.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Root aggregate and full Rust workspace lanes are out of scope.
- Visual baseline approval and shared package release qualification remain
  separate.
- The unfiltered real-browser command also collects the independently
  provisioned Git bridge scenario. Its fixture selection mismatch remains
  outside this seed slice; its Rust-owned GB6 gate was not rerun.
<!-- /evidence-field -->

