---
schema: 1
id: E-CI-BUNDLE-001
---

# E-CI-BUNDLE-001

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

snxrpykx
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99.0; JJ 0.45.1; jj-acp 0.1.0; pnpm 10.32.1; Node 24.10.0 macOS ARM64 and 24.13 Linux container; hosted Node 22.23.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, manifest/WAL/receipts, API and companion formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local targeted evidence and CI repairs do not
  promote hosted, optimized archive, container, registry, provider or full
  release qualification.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CI-BUNDLE-001
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-010
- JST-GOV-002
- JST-GOV-007
- JST-DOC-002
- JST-DOC-004
- JST-API-001
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-prebuilt.mjs
- scripts/web-manifest.mjs
- scripts/web-manifest.test.mjs
- web/playwright.config.ts
- web/playwright.real.config.ts
- web/tests/real/event-freshness.spec.ts
- web/tests/real/git-bridge.spec.ts
- web/tests/real/work-store.spec.ts
- web/vite.config.ts
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- web/tests/real/local-browser.spec.ts
- web/tests/real/native-repository.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/web-manifest.test.mjs#browser manifest generation enforces server
  count and byte limits
- crates/jj-store-http/tests/assets.rs#browser_manifest_rejects_escape_missing_duplicate_and_oversized_assets
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/real-browser-acceptance.mjs (initial isolated asset copy)
```

**Notes**

Prebuilt admission initially rejected absent collision-safe client in the default root. Explicit binaries were built in a registered isolated target and copied into a disposable JJ workspace, preserving interactive targets.
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/real-browser-acceptance.mjs (old UI copy)
```

**Notes**

Old copied UI was dated October 13; observed timeouts/failures were not accepted as current UI proof. Stopped only this owned Playwright process, then its fixture server; rebuilt current source/assets. This interrupted run is not a pass.
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/real-browser-acceptance.mjs (first current UI build)
```

**Notes**

Current reachable inventory had 259 assets, exceeding unchanged server maximum 256. Grouped released Design Core shared helpers into one build group while retaining strict view entries and every reachable font/script/style/icon; no server bounds raised.
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
node --test scripts/web-manifest.test.mjs
```

**Notes**

Four manifest regressions passed, including 256 versus 257 entries and the 16-KiB byte bound.
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
JJ_STORE_DOC_REFERENCE=<fresh isolated renderer> pnpm web:build (disposable workspace)
```

**Notes**

Current compact source build and complete Help passed. Browser manifest has 253 assets and 11262 bytes, within unchanged 256/16384 bounds.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-http --test assets --locked -- --test-threads=1
```

**Notes**

All nine unchanged server asset/CSP/path/symlink/Help regression tests passed.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- build -p jj-store --bins --examples --locked
```

**Notes**

Current dev tools built. macOS linker warned about large debug unwind offsets; optimized/native release qualification remains separate.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- build -p jj-store-http --example browser_preview --locked
```

**Notes**

Preview executable built for artifact-consuming browser jobs.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
CI=true PATH=/Users/stevejuma/.nvm/versions/node/v24.10.0/bin:$PATH JJ_STORE_CI_PREBUILT=1 JJ_STORE_DEV_RUSTFS_RUNTIME=/tmp/jj-store-ci-cold-20261014/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_ACP_EXECUTABLE=/tmp/jj-acp-0.1.0-aarch64-apple-darwin/jj-acp JJ_STORE_DEV_PORT=7530 node scripts/real-browser-acceptance.mjs
```

**Notes**

Complete current source run: 9 passed, 15 skipped, 20 failed. Failures include older CLI paths, a dedicated Git cycle without its required fixture, administrative polling counted as repository traffic, old controls/schema assumptions and remaining functional/layout failures. Preserve as blocking evidence; source bundle now starts within unchanged bounds.
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
pnpm check:progress; pnpm spec:first --base codex/ci-portability-and-bootstrap --head @
```

**Notes**

After correcting evidence coverage and run statuses, progress and protected follow-up change map passed. API chapter explicitly confirms unchanged protocol/allowlist contracts.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete fresh cold native/browser suite and direct served-bundle browser
  suites; fresh hosted impact-planned CI is required.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Same-run prebuilt browser admission, correct collision-safe client paths, CI line reporting and bounded bundle production. Source-owned view entries, server allowlists and strict production CSP retain their existing contracts.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Record current cold/served results before merge; preserve temporary fixture and evidence until qualification/cleanup.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-005/010 owning startup chapter now governs bounded producer
  failure and shared helper grouping. App view imports remain strict and lazy.
  JST-DOC-002/004 manual and server limits remain enforced; prebuilt validation
  is CI-only and normal development still builds.

```
<!-- /evidence-field -->

