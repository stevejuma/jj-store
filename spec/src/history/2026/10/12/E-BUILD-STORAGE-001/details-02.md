---
schema: 1
id: E-BUILD-STORAGE-001
---

# E-BUILD-STORAGE-001 details 2

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
jj --no-pager st; jj --no-pager workspace list; df -h .; du -k -d 2 target runtime desktop .local
```

**Notes**

Before implementation: clean default working copy. The planning review found about 124 GiB main target, 71 GiB retained integration target, 17 GiB remote-workspace target, 30 GiB main incremental data and about 60 GiB hashed executables. At implementation start the remote workspace was already retired; main target measured 127.9 GiB, and free space was approximately 36.5 GiB. Existing root/desktop/runtime profiles already disable debug symbols.
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
node --test scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs scripts/dev-demo-processes.test.mjs
```

**Notes**

Baseline: 15 tests passed.
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
pnpm spec:search -- JST-CLI-004
```

**Notes**

The installed CLI rejects the extra -- forwarded by pnpm. Corrected discovery used pnpm exec spec-validator search JST-CLI-004 and opened the canonical requirement/topic.
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
pnpm exec spec-validator search JST-CLI-004
```

**Notes**

Located and read the owning CLI chapter, requirements and M3.3/M17.2 milestone rows.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:build-storage
```

**Notes**

Initial 8/11 pass exposed fixture cache-marker handling, empty target environment selection and Cargo package-clean behavior. Subsequent focused attempts exposed linked JJ repository path resolution, fixture revision selection and incomplete Cargo dry-run executable enumeration; all were corrected and retained in regression scenarios. Later copied-system-executable test failures were replaced with a real compiled Rust consumer, and executable identity now supplements lsof.
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
cargo clean --workspace --dry-run --offline --locked --verbose; cargo clean --workspace --locked --offline (disposable cache-proof workspace)
```

**Notes**

Read-only enumeration omits executable paths that actual Cargo package-clean removes. The implementation delegates actual package selection/removal to Cargo rather than guessing hashed artifacts. Disposable native builds and first-party cleanup preserve the external dependency rlib and remove first-party binaries.
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
pnpm test:build-storage
```

**Notes**

Final focused result: 15 tests pass, including real registered JJ sibling workspaces, unchanged-build reuse, live native executables, native locks, shared/exclusive leases, symlink/protected-state refusal, cache-marker refusal, dry-run lifecycle preservation, interrupted removal/retry and cache preservation under unfixable live pressure.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs scripts/dev-demo*.test.mjs scripts/docs.test.mjs scripts/contracts.test.mjs scripts/install-cli.test.mjs
```

**Notes**

52/54 pass; two documentation tests could not launch uv because this shell omitted its existing ~/.local/bin installation. No test assertion or implementation was weakened.
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
PATH=/Users/stevejuma/.local/bin:$PATH node --test scripts/docs.test.mjs
```

**Notes**

All 8 documentation tests pass with the existing uv available.
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
PATH=/Users/stevejuma/.local/bin:$PATH node --test scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs scripts/dev-demo*.test.mjs scripts/docs.test.mjs scripts/contracts.test.mjs scripts/install-cli.test.mjs
```

**Notes**

All 54 direct consumer tests pass. Escalation reason: shared launch/execution helpers and package build entry points affect development, desktop staging, fixture generation and documentation reference consumers.
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
pnpm test:governance
```

**Notes**

The original 67-test expanded lane passed; after the additional useful-cache preservation regression the final full relevant governance lane passes all 68 tests. Includes the new storage/managed-Cargo regressions.
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
pnpm test:dev:ui
```

**Notes**

Cold isolated native real-RustFS/Vite/browser run passed: writes via proxy, SSE, Svelte/TS/CSS HMR without reload, backend-only restart retaining Vite, agent availability, wrong Origin/Host refusal, shutdown and fixture preservation. The extracted acceptance helper omitted headers used by the runner; returning/destructuring that existing header set repairs this harness-only bug. Expected blocked avatar requests emitted ERR_FAILED console notices, with no test failure. Owned temporary processes/files were removed; no main demo restart/reset.
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
pnpm dev:target:register -- --target .local/desktop-integration-target --reason "Completed E-DESK-003 desktop qualification; retained generated cache from 2026-10-03"
```

**Notes**

Explicitly adopted the previously APFS-cloned ignored integration target using its existing Cargo cache marker.
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
pnpm dev:target:retire -- --target .local/desktop-integration-target --dry-run
```

**Notes**

Verified no artifact deletion or completion-state mutation.
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
pnpm dev:target:retire -- --target .local/desktop-integration-target --apply
```

**Notes**

Removed only the eligible completed cache, retaining Cargo lock files/cache marker. Filesystem free space increased from 36.5 to 73.8 GiB: approximately 37.3 GiB actual recovery. This is smaller than the 71 GiB directory report because APFS blocks were shared.
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
pnpm dev:clean -- --dry-run; node scripts/build-storage.mjs report --json
```

**Notes**

Reported protected application state separately and refused live main/RustFS consumers. Dry run preserved free space. Final report showed main 127.9 GiB, desktop 8.23, embedded RustFS 7.82, local-agent 1.98, compute 2.12 and retired integration cache effectively zero.
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
node scripts/managed-cargo.mjs check -p jj-store-model --locked (expected preflight refusal)
```

**Notes**

Asserted nonzero status and the explicit live-target/limit diagnostic before Cargo ran. No idle caches were removed because the live per-target excess cannot be resolved and free space is healthy. This is a negative preflight scenario, not a passing Rust package check.
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
JJ_STORE_BUILD_TARGET_GIB=200 JJ_STORE_BUILD_TOTAL_GIB=250 node --input-type=module -e 'import { references } from "./scripts/docs/references.mjs"; references(); console.log("Managed Cargo documentation references match tracked outputs");'
```

**Notes**

Actual managed Cargo reference generation matches tracked outputs. Temporary qualification overrides permit reuse of the already-live large cache without cleaning it; default budget configuration is unchanged. This validates stdout preservation through the wrapper.
<!-- /run -->
<!-- /evidence-field -->

