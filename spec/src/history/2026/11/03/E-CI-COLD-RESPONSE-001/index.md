---
schema: 1
id: E-CI-COLD-RESPONSE-001
---

# E-CI-COLD-RESPONSE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-RESPONSE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.3
- M13.4
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WORK-008
- JST-LOCAL-003
- JST-LOCAL-004
- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/real/local-mixed.spec.ts
- web/tests/real/work-store.spec.ts
- spec/src/current/contracts/projects/04-acceptance.md
- spec/src/current/contracts/local-jj/04-delivery-gates.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/contracts/api/21-workstore-service-and-transports.md
- spec/src/current/contracts/api/25-filesystem-local-repository-api.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M11/history/M11.3/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M13/history/M13.4/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/local-mixed.spec.ts#merged_native_threads_create_send_and_reload_independent_of_local_browsing
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh automatically
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41537333075 --json conclusion,jobs,headSha,url
gh run download 41537333075 --dir /tmp/jj-store-cold-failures.RsC1dN
```

**Notes**

Integrated full qualification at the landed stack SHA `b751c03` exposed five
cold-browser failures. Two were the same Playwright response-body race in the
local-mixed and WorkStore command paths. The external-publication refresh,
project-preview visibility and reaction-geometry failures are independent and
remain outside this repair.
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
JJ_STORE_REAL_TEST=local-mixed.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The first local cold attempt stopped in preflight because the ambient mise Cargo
shim had no default toolchain. No application acceptance ran.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=local-mixed.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The repository-managed Rust retry reached the next preflight and stopped because
the pinned `uv` 0.9.28 executable was absent. No application acceptance ran.
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
PATH="/tmp/jj-store-uv.5u5xwl:$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=local-mixed.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold local-mixed file passed both tests in 5.2 seconds after staging
the CI-pinned `uv` 0.9.28 binary. The repaired command receipt was consumed
before the click-triggered navigation could evict its response body.
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
PATH="/tmp/jj-store-uv.5u5xwl:$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=work-store.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold WorkStore file passed its one test in 29.8 seconds. Its shared
save helper now consumes and retains the committed receipt before any resulting
navigation.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

The selected specification validators passed. Spec-first and progress validation
rejected two non-exact test names and the unsupported pending run result in this
record. No product or acceptance check failed.
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
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Formatting, selected specification validation, spec-first enforcement and the
progress-ledger check passed after correcting the evidence metadata.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; Playwright
1.63.0; uv 0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted, API, receipt, request, response or navigation format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cold-browser harness timing for two committed command responses only. The
external-publication refresh, project-preview visibility and reaction-geometry
failures from the same hosted run remain separate qualification blockers.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this focused repair, then fix the three independent cold-browser failures
before rerunning complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The tests still require successful committed command receipts and the
  same resulting navigation state; they now consume response JSON before browser
  navigation can discard it. Product behavior and public contracts are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.3 remains Done. M13.4 and M30 remain In progress because their
  independent feature and complete qualification gates remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
