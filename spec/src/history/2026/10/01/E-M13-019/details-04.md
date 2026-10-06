---
schema: 1
id: E-M13-019
---

# E-M13-019 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Renamed native bootstrap test reference initially used the Vite path. Corrected removed/replacement references to the real native browser test; sole-repository bootstrap unit remains unchanged.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_REPOSITORY=b2ead948cbebd2512901b2f6813ae3a7 JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ pnpm exec playwright test --config web/playwright.real.config.ts native-repository.spec.ts project-polish.spec.ts projects-activity.spec.ts work-store.spec.ts
```

**Notes**

Initial native consumer run used stale fixture assumptions and controls. Updated only requested-repository bootstrap and current filter navigation; retained sole-repository bootstrap, conflict publication, timestamps and explicit-refresh assertions. .scratch/local-browser-native-consumers.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_REPOSITORY=b2ead948cbebd2512901b2f6813ae3a7 JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts native-repository.spec.ts project-polish.spec.ts projects-activity.spec.ts work-store.spec.ts
```

**Notes**

6 passed: mixed Developer experience, requested bootstrap, native divergence, atomic conflict resolution, documents and pinned Activity/reviews. Reused descriptions/fixtures made freshness and mutation retries nonrepeatable; use timestamped/unique descriptions. Accumulated test repositories exhausted the bounded directory graph reader for WorkStore. The exact freshness/mutation retry passed 2 tests and the final clean WorkStore retry passes separately; no complete native-suite pass is claimed. Intermediate consumer/focused retries are retained in .scratch/local-browser-native-{consumers-final,focused,focused-final,qualified,repeatable}.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node .scratch/reconnect-acceptance.mjs
```

**Notes**

Exact dialog retries remained locked. tick/animation-close experiments did not resolve the strict-CSP failure and were removed. CSP blocked Bits UI empty style-attribute reset while leaving CSSOM pointer-events:none. Development-annotation mixed mode had hidden it. Final fix is consumer-owned CSP integration permitting only SHA-256(empty) via unsafe-hashes; no library patch, body cleanup or broad inline permission. Logs: reconnect-{isolated,tick,close-complete,diagnostic,css,animations,settle,probe}.log.
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
JJ_STORE_REAL_REPOSITORY=f0622f8876c339fd655d620f47d26320 JJ_STORE_REAL_RUNTIME=/tmp/jlb-final-zTsELr JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7480/ JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr JJ_STORE_DEV_PORT=7480 pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts work-store.spec.ts --output .scratch/test-results-clean-mixed
```

**Notes**

Mixed navigation passed; WorkStore task creation failed with ENOSPC before its directory timestamp assertion. .scratch/local-browser-clean-mixed-work.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cp -cR /Users/stevejuma/code/lapismd/jj-store/target .scratch/cargo-csp
```

**Notes**

Full-cache clone attempts exhausted APFS metadata; removed only this task failed generated cache. Preserved binaries in a sealed task runtime. Final setup clones only deps/build/.fingerprint, excludes incremental, and disables incremental compilation. No default artifacts removed.
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
strip -x .scratch/local-browser-runtime/debug/jj
```

**Notes**

ENOSPC truncated this task temporary binary. Recovered from shared build, then replaced it with a successful source-pinned workspace build before any final cold test. No installed or default binary modified.
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
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/cargo-csp cargo test -p jj-store-http --test assets --locked
```

**Notes**

6 tests pass including the exact empty style reset allowlist, strict script policy and bounded sandboxed assets. .scratch/local-browser-csp-assets.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/cargo-csp cargo rustc -p jj-store-cli --bin jj-store --locked -- -C strip=symbols
```

**Notes**

Two private-cache attempts: inherited newer generated RPC types from default; deleting only private RPC build/fingerprint outputs regenerated this workspace contract. Subsequent CLI compilation hit ENOSPC. Serialized the build and removed only old task binaries, duplicate asset-test outputs and obsolete failed traces. Failed traces were removed for space; their error logs remain. .scratch/local-browser-csp-server-build{,-regenerated}.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/cargo-csp cargo rustc -p jj-store-cli --bin jj-store --locked -- -C strip=symbols
```

**Notes**

Source-pinned server build passes after RPC regeneration; stripped development output copied to sealed task runtime. .scratch/local-browser-csp-server-build-serial.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/cargo-csp cargo rustc -p jj-store-cli --bin jj --locked -- -C strip=symbols
```

**Notes**

Source-pinned custom JJ build passes and replaces the recovered temporary copy before final acceptance. .scratch/local-browser-csp-jj-build.log.
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

Final source: zero errors and warnings. .scratch/local-browser-web-check-resumed.log.
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
pnpm web:build
```

**Notes**

Final production bundle passes; existing dependency CSS/chunk warnings retained. .scratch/local-browser-web-build-resumed.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node .scratch/reconnect-acceptance.mjs
```

**Notes**

Exact reconnect/removal cold regression: 1 passed. The strict-CSP page remains clickable, moved folder recovery preserves code and companion state, and removal preserves contents. .scratch/local-browser-reconnect-csp-final.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/dev-demo-prebuilt.mjs restart
```

**Notes**

Cold isolated mixed restart with real embedded RustFS and final bundle/source-pinned binaries; ignored runner skips only already completed build commands. Existing fixture data reused. .scratch/local-browser-clean-mixed-csp-restart.log.
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
JJ_STORE_REAL_REPOSITORY=f0622f8876c339fd655d620f47d26320 JJ_STORE_REAL_RUNTIME=/tmp/jlb-final-zTsELr JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7480/ JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr JJ_STORE_DEV_PORT=7480 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts work-store.spec.ts --output .scratch/test-results-clean-mixed-final
```

**Notes**

2 passed: Developer experience opens linked Git code with reload, persisted disclosure and cross-profile Back/Forward; exact native WorkStore editing/receipts/filters/timestamp/mobile/accessibility regression passes on real RustFS. .scratch/local-browser-clean-mixed-work-final.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Post-WorkStore fixture has 5 projects because browser acceptance created 2 additional projects. Initial checker incorrectly required no additional work. Retain the three deterministic sample-ID count, all separate link/task assertions, and allow later projects; no fixture reseeded. .scratch/local-browser-clean-mixed-profiles-final.log.
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
JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Exact post-edit retry passes: all three sample identities/homes/links and 9 sample tasks retained alongside extra projects; native server-only checks and Git companion identity pass. .scratch/local-browser-clean-mixed-profiles-preserved.log.
<!-- /run -->
<!-- /evidence-field -->

