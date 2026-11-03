---
schema: 1
id: E-CI-COLD-RESPONSE-CAPTURE-001
---

# E-CI-COLD-RESPONSE-CAPTURE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-RESPONSE-CAPTURE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5.1
- M11.3
- M13.4
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-MUT-001
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

- web/tests/real/capture-json-response.ts
- web/tests/real/local-mixed.spec.ts
- web/tests/real/native-repository.spec.ts
- web/tests/real/work-store.spec.ts
- spec/src/current/contracts/projects/04-acceptance.md
- spec/src/current/contracts/local-jj/04-delivery-gates.md
- spec/src/current/contracts/api/17-typed-mutation-http-transport.md
- spec/src/current/contracts/api/21-workstore-service-and-transports.md
- spec/src/current/contracts/api/25-filesystem-local-repository-api.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M5/index.md
- spec/src/current/progress/M5/history/M5.1/index.md
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
- web/tests/real/native-repository.spec.ts#browser mutation previews and publishes
  one native request
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh automatically
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41604607037 --json status,conclusion,headSha,jobs,url
gh run download 41604607037 --name qualification-results-cold-cold-1-1 --name qualification-results-cold-cold-3-1 --dir /tmp/jj-store-full-41604607037.flx7V1
```

**Notes**

Complete qualification at merged SHA `a9c95dc` passed standard, build, backend,
contract, storage, native, asset and profile-restart gates. Cold shards 2/4 and
4/4 exposed three response-body losses after accepted navigation and one
independent reaction-icon geometry failure. Promise chaining from
E-CI-COLD-RESPONSE-001 was insufficient under hosted load.
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
npx --yes pnpm@10.32.1 web:check
```

**Notes**

The isolated web type check passed with zero errors and zero warnings after the
shared cold-response helper and three consumers were changed.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_SHARD=2/4 npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The original hosted cold shard passed seven tests with one fixture-dependent
skip in 37.7 seconds after 220.730 seconds of fresh fixture preparation. The
mixed Thread and native apply receipt observers both passed. Embedded resources
measured 7.630-second cold and 8.173-second restart startup with 422,204 KiB
resident memory.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=work-store.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact WorkStore cold file passed in 28.4 seconds after 209.805 seconds of
fresh fixture preparation. Embedded resources measured 8.045-second cold and
7.971-second restart startup with 423,348 KiB resident memory.
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
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Changed-file formatting, selected specification validation, spec-first
enforcement and progress-ledger checks passed.
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

Cold-browser observation of three real committed command responses. Product
commands, responses, receipts, navigation and resulting views are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this focused repair, then correct the independent reaction-icon geometry
assertion before rerunning complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The tests still send real commands and require the same successful
  committed receipts, exact request identities, automatic navigation or reload,
  resulting views and persisted state. The one-shot routes only read JSON before
  forwarding each real response to the application.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M5.1 and M11.3 remain Done after their original cold acceptance passes.
  M13.4 and M30 remain In progress because indexed/recoverable local work,
  reaction geometry and complete release qualification stay open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
