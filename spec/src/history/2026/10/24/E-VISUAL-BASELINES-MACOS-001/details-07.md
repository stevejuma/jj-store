---
schema: 1
id: E-VISUAL-BASELINES-MACOS-001
---

# E-VISUAL-BASELINES-MACOS-001 details 7

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
pnpm visual-delta test --all --dry-run --explain
```

**Notes**

Before capture: 1046 eligible stories, zero existing PNG baselines in the configured native directory.
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
pnpm visual-delta doctor --runner --json
```

**Notes**

15 pass, 1 warning for stale preview inputs, zero errors. The following capture rebuilt Storybook before generating images.
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
pnpm visual-delta update --story-id jj-store-components-chip--bookmark --approved --create-only --skip-build
```

**Notes**

The exact first story passes native baseline creation. A cold Storybook build resolves stale preview inputs; one PNG is persisted and the public tool adds only visualDelta image parameters to the story. No review tags change.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
node .local/visual-delta-baselines/capture-current.mjs .local/visual-delta-baselines/all-targets.json
```

**Notes**

Stopped the first all-story batch after 131 successful capture cases and 13 readiness failures. Error contexts prove module-level vitest/browser imports cannot execute in standalone Storybook. The native mutation transaction discards this incomplete batch; the separately completed Chip baseline remains intact. No source assertion, failure mode or capture readiness condition is weakened.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 111
hasNote: true
noteEncoding: markdown

```

```shell
node .local/visual-delta-baselines/screen-stories.mjs
```

**Notes**

Read-only screening of all 1046 original story IDs: 935 reach the same completion attribute and visible root required by the public capture suite, 110 fail outside the Vitest browser host and one exceeds its 15-second readiness window. Five isolated native Chromium contexts use the same viewport, locale, timezone and motion settings as the capture profile. The user explicitly selected capture of the available stories only. This inventory is selection evidence and does not replace the subsequent public native capture or fresh comparison.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Before baseline batch completion: the two inherited stack-workspace line-limit failures are unchanged (401/400 story and 310/300 production component). Generated source wiring is restored after capture because the public preset already injects file-discovered baseline parameters during compilation.
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
node .local/visual-delta-baselines/capture-current.mjs .local/visual-delta-baselines/ready-stories.json
```

**Notes**

The 934 missing ready-story cases completed with 932 passes and two 30-second capture timeouts: jj-store-search-repository-search-command--unavailable and jj-store-stacks-topology--shared-lower-cards-keep-one-branch-draft-across-sibling-stacks. The native transaction discarded this failed batch; these 932 successful cases did not persist baseline files. The separately completed Chip baseline is preserved. Exact retries distinguish standalone capture blockers from load-dependent timeouts.
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
node .local/visual-delta-baselines/cache-status.mjs
```

**Notes**

Read-only installed-addon diagnostic: workflow.reuseActualComparisons is true, but the native Darwin profile has fontManifestSha256=null and supportsPersistentCaptureReuse returns false. Both all-story cached-actual reuse and affected-story passing-result reuse are therefore disabled. Static build caching remains available. Unchanged native stories still recapture; real upstream host-font fingerprint support is required before safe reuse. No hash is invented and no reuse guard is bypassed.
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
node .local/visual-delta-baselines/capture-current.mjs .local/visual-delta-baselines/capture-timeouts.json
```

**Notes**

Both exact capture cases reproduce the same 30-second timeout with only two workers. Failure count stays 2 before and after isolated retry; neither baseline persists. The final available-only selection excludes these two cases in addition to the 111 standalone readiness blockers. No readiness limit or capture timeout is increased.
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
node .local/visual-delta-baselines/capture-current.mjs .local/visual-delta-baselines/final-targets.json
```

**Notes**

Final available-only batch: 932 missing-story capture cases pass in 10.0 minutes; create-only skips the separately completed Chip. The public native transaction successfully persists 932 new PNGs. The configured Darwin ARM64 profile now contains 933 PNGs. The ignored driver invokes pnpm --silent visual-delta update with --approved --create-only --skip-build and one exact --story-id per selected story. The committed PNG basenames, minus -chromium.png, identify the final selection.
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
node .local/visual-delta-baselines/inventory-baselines.mjs
```

**Notes**

933 selected stories each have one Chromium PNG, totaling 55137105 bytes. PNG signatures, dimensions and SHA-256 hashes are recorded, all selected baselines exist, and the existing Chip hash is unchanged. A copy of this inventory is saved before compare-only verification.
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
jj restore --from vtstntlp catalog; jj --no-pager diff --from vtstntlp --to @ --name-only catalog
```

**Notes**

All 191 generated story diffs were checked to contain only injected baseline image parameters and whitespace, then restored to the qualified parent. The final catalog diff is empty: story implementations, assertions and review tags are byte-identical to the parent. The released public preset discovers committed PNGs at compile time, so persisted per-story metadata is unnecessary.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
node .local/visual-delta-baselines/capture-current.mjs .local/visual-delta-baselines/final-targets.json test
```

**Notes**

Forced a cold static Storybook rebuild, then invoked public visual-delta test with one exact --story-id per persisted PNG, --fresh --failure-mode strict --rebuild. All 933 Playwright capture cases complete in 9.4 minutes with runnerStatus=passed, but strict CLI exit is 1: sidecars report 911 passed, 4 changed-within-tolerance and 18 mismatch outcomes. Thus 915 comparisons are acceptable and 18 exceed existing policy. No baseline or threshold is overwritten. Playwright completion alone is not visual-comparison acceptance; the sidecars and CLI exit are authoritative. The comparison page lists every non-identical outcome.
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
node .local/visual-delta-baselines/verify-built-baselines.mjs
```

**Notes**

After the strict run exits, all 933 persisted PNG hashes remain unchanged and all 933 static copies match. The compiled Chip bundle contains its public baseline URL despite no visualDelta metadata in source; the static URL responds HTTP 200 with the exact persisted PNG bytes. This proves public-preset file discovery, mounted image serving and compare-only immutability independently of the 18 visual mismatches.
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
pnpm spec:validate
```

**Notes**

7 validators, 4424 chapters and 208 requirements validate. Rechecked after final evidence is written.
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
JJ_STORE_CHECK_REVISION=yyyztqwn pnpm check:progress
```

**Notes**

180 tasks and 956 evidence records pass for the enclosing baseline change. Rechecked against its final immutable snapshot after the actual comparison outcomes are recorded.
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
pnpm spec:first --base d6be1ae5ef1ba9b4f5b0674b95c1f53521a88775 --head 175e35f2d42bd00dec22e80294d24c6559a03865
```

**Notes**

Initial evidence gate passes for one protected file and one canonical chapter. The gate is repeated on the final evidence snapshot; this initial hash is not the final slice hash.
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
pnpm exec prettier --write docs/catalog.md spec/src/current/contracts/ui/01-storybook-presentation-catalog.md spec/src/current/progress/M4/index.md spec/src/history/2026/10/24/E-VISUAL-BASELINES-MACOS-001 spec/src/history/2026/10/24/index.md spec/src/history/2026/10/24/SUMMARY.nav.md
```

**Notes**

Only the owning documentation and new evidence pages are formatted. Final formatting is checked before commit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

