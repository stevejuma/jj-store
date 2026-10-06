---
schema: 1
id: E-READ-090
---

# E-READ-090 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory-response-cache.test.ts
```

**Notes**

before.log initially used wrong presentation message field; corrected test fixture. before-corrected.log proves four behavior failures, three passing invalidation checks.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory-response-cache.test.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

after.log: 19 pass, one existing fixture uses returned recursive mode distinct from requested mode; enrich now uses the returned page key.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory-response-cache.test.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

after-2.log: 20 tests passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory-query-cache.test.ts web/tests/vite/directory-response-cache.test.ts web/tests/vite/production-file-reader.test.ts web/tests/vite/http-query.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

focused.log and final-unit.log: 52 tests passed; final-unit-2.log: 53 passed after canceled-quick skeleton regression.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

web-check.log: four type errors from generic-union narrowing and local read method callback compatibility; fixed.
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
pnpm web:check
```

**Notes**

web-check-2.log and web-check-final.log: zero errors and warnings.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

loaders.log: 11 tests passed, including direct page/cursor helpers.
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
pnpm exec playwright test --config .local/directory-cache/browser.config.ts directory-response-cache.spec.ts
```

**Notes**

browser.log wrong accessible folder name; browser-2.log used root table label after entering arch. Both test selectors corrected to the existing production labels.
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
pnpm exec playwright test --config .local/directory-cache/browser.config.ts directory-response-cache.spec.ts
```

**Notes**

browser-3.log: one cold Vite/browser test passed in 8.2s; keyboard directory entry, skeleton while names are pending, rows/metadata and zero extra names/metadata requests on revisiting. Uses admitted mocked native fixture; no screenshot baseline changes. Final rerun browser-final.log passed after canceled-quick restoration.
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
pnpm check:ui-ownership
```

**Notes**

ownership.log: existing boundary checker passed.
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
pnpm check:architecture
```

**Notes**

architecture.log: existing boundary checker passed.
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
python3 -u .local/directory-cache/profile-requests.py setup-profile
```

**Notes**

Eight names-only Linux arch reads at existing pinned commit/operation; data unchanged. setup-profile.json and profile-requests.log retained.
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
sample 80426 8 1 -file .local/directory-cache/setup-sample.txt
```

**Notes**

Process sample under read load: a representative read stack has 734/1342 samples in bounded_git_header -&gt; gix pack index load/validate_size; operation fingerprinting 290/1342. Approximate sampling attribution, not exclusive timing; unrelated capture work also appears. Directory reader source uses direct-child tree iteration, not recursive traversal.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial new evidence used test filenames without required named anchors; corrected.
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
pnpm spec:first
```

**Notes**

Transport cache files also require api.md; added client lifetime and transient history policy.
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
pnpm check:progress
```

**Notes**

Final changed-surface check; no aggregate suite.
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
pnpm spec:first
```

**Notes**

Final changed-surface check; no aggregate suite.
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
pnpm spec:validate
```

**Notes**

Final changed-surface check; no aggregate suite.
<!-- /run -->
<!-- /evidence-field -->

