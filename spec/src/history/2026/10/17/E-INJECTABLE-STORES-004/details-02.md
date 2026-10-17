---
schema: 1
id: E-INJECTABLE-STORES-004
---

# E-INJECTABLE-STORES-004 details 2

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
pnpm dev:cargo -- test -p jj-store-wal --test bound_capture bound_capture_selects_both_roots_once_preserves_lifecycle_and_replays_after_recovery --locked -- --exact
```

**Notes**

Before: exact original native atomic-capture regression passed1.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-native --offline --message-format=short
```

**Notes**

First new native adapter compile found2WalPosition API assumptions and private semantic_kind. Corrected to the owned tuple identity and crate-private semantic helper.
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
pnpm dev:cargo -- check -p jj-store-native --locked --message-format=short
```

**Notes**

Disambiguated ThreadStore/ReactionStore execute_with_reviews methods.
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
pnpm dev:cargo -- test -p jj-store-native --locked --message-format=short
```

**Notes**

Test fixture omitted body schema_version; corrected it and the unused import.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-native --locked --message-format=short
```

**Notes**

3passed/2failed. Historical bound capture uses the existing DomainsStateChanged semantic kind; receipt lookup now admits its qualified Agent+Thread version transition. The after-apply fault needed unavailable recovery reads to establish an actual unknown outcome; enhanced the fault to cover both unknown CAS and known CAS with failed receipt reads.
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
pnpm dev:cargo -- test -p jj-store-native --locked --message-format=short
```

**Notes**

5native tests passed, covering load/read no staging, exact historical roots/versions, native request round-trip, atomic bound capture, source admission and both after-apply fault paths.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-native -p jj-store-wal -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

First wider compile exposed2error-variant arms inserted into the metadata conversion instead of the core conversion; moved them to their owner.
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
pnpm dev:cargo -- test -p jj-store-native -p jj-store-wal -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

Restored the two original ReviewCommandError state arms after correcting the core conversion.
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
pnpm dev:cargo -- test -p jj-store-native -p jj-store-wal -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

161passed,0failed,0ignored across the complete4affected crate suites. Escalated for shared typed clock entry points, original-receipt lookup and provider error handling across independent stores. native-affected-tests-3.log.
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
node scripts/package-check.mjs
```

**Notes**

24coordinated packages retain portable registry metadata and contained source inputs.
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
pnpm check:boundaries
```

**Notes**

23workspace packages preserve ownership, including the new core/model/WAL native adapter.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

30crate roots,0missing READMEs and0broken first-party README links.
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
pnpm dev:cargo -- test -p jj-store-native -p jj-store-git --test factories review_state_errors --locked --message-format=short
```

**Notes**

Two new end-to-end provider regressions retain ThreadOpen/ThreadResolved as typed public errors.
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
pnpm dev:cargo -- clippy -p jj-store-native -p jj-store-core -p jj-store-git -p jj-store-wal -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

All affected packages and the direct service consumer passed strict lint.
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
pnpm check:progress
```

**Notes**

M32.2 selected the new evidence but was absent from its task references. Included the direct Git error-conversion qualification and refreshed task-history coverage.
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
pnpm dev:cargo -- clippy -p jj-store-native -p jj-store-git --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Repeated strict lint after both review-state regressions were added; all affected new test code is checked.
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
node --test scripts/governance.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

39governance/navigation/progress helper tests passed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

Progress162/854 and protected source gate29files/4chapters passed; governance Markdown exceeded400physical lines by1. Condensed its ownership sentence.
<!-- /run -->
<!-- /evidence-field -->

