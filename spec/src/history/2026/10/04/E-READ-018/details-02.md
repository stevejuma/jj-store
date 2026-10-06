---
schema: 1
id: E-READ-018
---

# E-READ-018 details 2

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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture
```

**Notes**

Initial cancel body rejected400; fixed to required bodyless POST. Rerun1passed;36actual HTTP responses captured/validated against generated contracts.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture; node scripts/contracts.mjs --write; pnpm test:contracts
```

**Notes**

Focused HTTP and generated-contract unit/type checks pass. Full contracts lane retains unrelated sanitizer timeout limitation from E-READ-005.
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
pnpm web:check; pnpm web:build
```

**Notes**

0errors/0warnings; production UI built, existing large-chunk advisory. Source-dependency setup inherited, registry qualification still pending.
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
cargo test -p jj-store-cli --test local_browser --locked ordinary_point_reads_do_not_import_missing_extra_metadata -- --nocapture
```

**Notes**

Before: ordinary point read imported missing extra metadata and created a head.
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
cargo test -p jj-store-cli --test local_browser --locked ordinary_point_reads_do_not_import_missing_extra_metadata -- --nocapture; cargo test -p jj-store-jj --lib --locked local_browser::git_read -- --nocapture
```

**Notes**

1point regression and3decoder units pass.80MiB table point lookup reads under1KiB; selected oversize and same-size mutation with restored mtime fail closed.
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
cargo test -p jj-store-jj --test local_metadata --locked identity_observation_does_not_admit_or_read_domain_payloads -- --nocapture
```

**Notes**

Initial fixture attempted overwrite of readonly loose object. Removed it before corruption; assertions unchanged.
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
cargo test -p jj-store-jj --test local_metadata --locked identity_observation_does_not_admit_or_read_domain_payloads -- --nocapture; cargo test -p jj-store-jj --test local_metadata --locked; cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

Exact identity regression1passed; full local metadata14passed and local-browser21passed/1ignored before external-preparation probe addition. Shared identity/scoped-reader consumers justify these module suites.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Affected packages/direct consumers pass after identity split. Existing linker unwind-size advisory.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo cargo test -p jj-store-cli --test local_browser --locked registered_large_repository_metadata_smoke -- --ignored --nocapture
```

**Notes**

Linux ordinary smoke: operation221.7ms/5.24MB; metadata+bookmark1.252s/2.94MB; directory876.9ms/2.94MB; historical directory1.284s/7.58MB; .clang-format24291bytes.1024history/100000work bounds; not full-history qualification.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_FILESYSTEM_PORT=18778 node scripts/local-browser-acceptance.mjs
```

**Notes**

Initial cold seeded browser2passed/3Files timeouts. Code authorization repeatedly audited companion domain/evidence closure; identity observation now validates selected root separately.
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
JJ_STORE_FILESYSTEM_PORT=18778 JJ_STORE_LOCAL_BROWSER_KEEP=1 node scripts/local-browser-acceptance.mjs
```

**Notes**

After identity split4passed/1stale working-copy toast assertion. Captured page showed correct message in Notifications. Locator fixed and recovery-cleared assertions added; no timeout/assertion weakening.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<owned-fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts --grep "physical working copy" --output .scratch/test-results-local-browser-working
```

**Notes**

1passed, unchanged recorded operations/bookmarks/companion and expected physical Save.
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
target/debug/jj-store run --config <owned-fixture>/state/server.toml --format=json; JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<owned-fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts --output .scratch/test-results-local-browser-final
```

**Notes**

Fresh server/browser using retained seeded fixture:5passed/34.7s, Files/history/Agents, Projects, Reviews, reconnect/removal and Save. Visual-pending. Logs under .local/bounded-reads/.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:architecture
```

**Notes**

Initial checks required compatibility chapter and class split at300line threshold; both corrected, no governance weakened.
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
pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/ui.md web/tests/real/local-browser.spec.ts; cargo fmt --all -- --check; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

167requirements,17packages,931web modules and ownership pass. No whole-ledger formatting.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

24passed/2files in4.39s; local task/error pins and shared preparation controller.
<!-- /run -->

<!-- run -->
### Run 17

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

Draft evidence omitted two test-name anchors; corrected named references.
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
pnpm check:progress
```

**Notes**

136task ledger bookkeeping passes with moved-file exemptions and named tests.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

