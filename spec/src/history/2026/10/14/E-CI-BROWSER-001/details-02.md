---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 2

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
JJ_STORE_DOC_REFERENCE=<isolated current renderer> pnpm web:build
```

**Notes**

Browser assets built; Help staging failed closed because the application
source-review fingerprint needed its owning review.
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
pnpm docs:review application "Reviewed idle reload observation and home-scoped task reaction freshness against the browser and settings guides. Working-copy action extraction and project helper imports preserve capability and session guards, explicit operations, drafts, current menus and native storage authority."; JJ_STORE_DOC_REFERENCE=<isolated current renderer> pnpm docs:build; node scripts/docs.mjs stage
```

**Notes**

Documentation was reviewed against current sources and complete Help
built/staged. Fresh complete source/bundle and cold-suite qualification remain
pending.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (intermediate reaction integration)
```

**Notes**

Typed validation rejected an unsupported invented reaction domain and a missing
lifecycle hook. Source tracing confirmed native reaction roots are carried by
the existing threads witness, not a new domain; WorkController now exposes its
owned reaction invalidation hook. No API or schema is changed.
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
pnpm web:check
```

**Notes**

Corrected typed event and WorkStore lifecycle integration: 0 errors and 0
warnings.
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
pnpm dev:cargo -- clippy -p jj-store --all-targets --locked -- -D warnings
```

**Notes**

Strict lint passed in the registered target; no warnings suppressed.
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
node --test scripts/ci.test.mjs scripts/release.test.mjs scripts/docs.test.mjs
```

**Notes**

22 CI, release and documentation tests passed.
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
CI=true JJ_STORE_REAL_KEEP=1 JJ_STORE_REAL_SHARD=1/1 JJ_STORE_CI_PREBUILT=1 JJ_STORE_DEV_PORT=7530 node scripts/real-browser-acceptance.mjs (intermediate build)
```

**Notes**

Diagnostic fresh fixture prepared in 261904ms; startup 1221ms, restart 953ms,
276128KiB resident. Stopped its owned Playwright process during the idle test
because the build predated the corrected event mapping. This interrupted run is
not acceptance. Stopped only that fixture server before rebuilding current
assets.
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
pnpm exec playwright test --config web/playwright.real.config.ts <focused current UI cases> --output=/tmp/jj-store-ci-focused-round4
```

**Notes**

Settings layout passed. Recorded agent tools use details/raw-output actions
rather than the old group role; select the actual seeded agent-artifact anchor
rather than an absent whole-session/Codex anchor. Link completion exposed a real
remount: selected-project repository summary invalidation replaced the captured
project with an initial skeleton and lost the submitted dialog state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-initial-loading.test.ts
```

**Notes**

Regression proves a captured selected-project repository view survives
invalidated secondary summaries, while uncaptured project/task and aggregate
views still require their own initial placeholders. Owning JST-UI-005 startup
contract updated; no identity or publication change.
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
CI=true JJ_STORE_REAL_TEST=profiles-emoji.spec.ts JJ_STORE_CI_PREBUILT=1 JJ_STORE_DEV_RUSTFS_RUNTIME=<same-source artifact> node scripts/real-browser-acceptance.mjs (generic seeded profile fixture)
```

**Notes**

The generic seeded fixture is not the original profile acceptance topology.
Restored the existing empty real-RustFS profile parent, with fail-closed
same-run prebuilt resource admission. No general seeding or compilation is
required on this independent CI runner.
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
CI=true JJ_STORE_REAL_TEST=profiles-emoji.spec.ts JJ_STORE_CI_PREBUILT=1 JJ_STORE_DEV_RUSTFS_RUNTIME=<same-source artifact> node scripts/real-browser-acceptance.mjs (empty profile fixture)
```

**Notes**

First cold upload exceeded Playwright default 5-second polling; exact 512x369
assertion now uses the existing 15-second UI operation budget. Open browser
streams also held the five-second graceful shutdown; the test unloads the page
before terminating its owned server and allows the bounded shutdown to complete.
No server deadline or policy is relaxed.
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
CI=true JJ_STORE_REAL_TEST=profiles-emoji.spec.ts JJ_STORE_CI_PREBUILT=1 JJ_STORE_DEV_RUSTFS_RUNTIME=<same-source artifact> node scripts/real-browser-acceptance.mjs (/tmp/jj-store-ci-profile-empty-final-4.log)
```

**Notes**

Original empty real-RustFS local/enrolled profile and emoji restart acceptance
passed in 25.1 seconds. Exact avatar/emoji normalization, object-reference, cold
restart, private email, enrollment, account-switch draft clearing and
accessibility assertions retained.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output=/tmp/jj-store-ci-focused-round8
```

**Notes**

Exact core preparation passed; WorkStore reached task filtering and found the
current collapsed toolbar search control instead of the older field. Updated the
test to open Search tasks and submit with Enter.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output=/tmp/jj-store-ci-focused-round9
```

**Notes**

Search submission passed. Review Back canonicalized the default state=active;
the comparison now retains all nondefault parameters and sorts query order
consistently.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output=/tmp/jj-store-ci-focused-round10
```

**Notes**

Task mutation, persistence, unknown-outcome recovery, links, filtering and Back
passed. The old retained fixture exhausted all 16 automatic code-index cache
slots; ordinary graph and repository summary reads returned resource limits
while direct object membership succeeded.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.local/build-targets/public-release pnpm dev:cargo -- test -p jj-store-jj --lib ordinary_reservations_evict --locked
```

**Notes**

Both new exact regressions reproduced resource-limit failure: ordinary
reservations could not evict idle managed code namespaces at the shared slot
limit or same-repository byte limit.
<!-- /run -->
<!-- /evidence-field -->
