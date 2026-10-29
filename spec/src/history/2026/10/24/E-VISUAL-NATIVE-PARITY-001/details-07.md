---
schema: 1
id: E-VISUAL-NATIVE-PARITY-001
---

# E-VISUAL-NATIVE-PARITY-001 details 7

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
node .local/visual-delta-baselines/cache-status.mjs
```

**Notes**

Before upstream repair, workflow.reuseActualComparisons=true but fontManifestSha256=null and supportsPersistentCaptureReuse=false. Both all-story actual reuse and affected passing exclusions were disabled on native capture. The earlier baseline evidence records this exact diagnostic.
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
Upstream focused regressions and VITEST_MAX_WORKERS=4 pnpm check
```

**Notes**

Source commit 73b443ee5f6706675fb72605a60f38032017ebad fixes measured font/runtime identity, safe native worker artifact inputs and profile-aware affected cache keys. Exact native-capture-reuse regression changes from 2 failures/1 pass to all passing; native-affected-reuse changes from 3 failures/2 passes to all passing. 63 focused tests across 8 files and 617 unit tests across 130 files pass. Types, seven specification lanes, Node build and publint pass. See upstream spec/records/2026-10-24-native-reuse-parity.md.
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
Upstream final packed public-worker native acceptance; STORYBOOK_PORT=25410 pnpm test:panel
```

**Notes**

The exact archive passes Chromium and WebKit primary/mode/interaction capture sets, no-op all/affected runs, cached comparisons, independent corruption recovery, changed-baseline recomparison, fresh bypass, strict mismatch and missing outcomes. Cold panel lane: 13 passed. Existing detached panel server teardown needed owned-port cleanup after cases passed. Local Firefox launch and unavailable Docker daemon remain separate limitations.
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
GitHub pull-request workflows for 73b443ee5f6706675fb72605a60f38032017ebad
```

**Notes**

Native acceptance passes on macos-15 and ubuntu-24.04. Hosted Docker Chromium/Firefox/WebKit, manager and panel acceptance also pass. All four workflows complete successfully: Native capture 40093010904, Visual Delta CI 40093010920, Native capture stage cleanup 40093010928 and Visual Delta Spec First 40093010906. Draft PR: https://github.com/lapismd/storybook-addon-visual-delta/pull/47. No merge, release or publication is performed.
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
pnpm install through .local/native-visual-parity/pnpmfile.cjs
```

**Notes**

Temporary documented archive validation composes the pre-existing Design Core packed setup and replaces only Visual Delta with the inspected archive. 805 source/dist installed files match archive bytes. Tracked package.json and pnpm-lock.yaml are restored byte-for-byte to registry ranges; Design Core 0.6.0 validation remains intact. Archive SHA-256 d59d01236e455b4438df7ca3d14f7d6d0a9c333260ecdec334a272dbd84723f3.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node .local/native-visual-parity/capture.mjs update
```

**Notes**

First 933-story attempt with the earlier archive stopped after 480 passed, 5 failed, 5 interrupted and 443 not run. Five cold-load timeouts occurred while other local native qualification ran. SIGINT stopped the owned runner; the native transaction discarded staged outputs. All original 933 PNG hashes remained unchanged. No failed-stage output was promoted.
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
node .local/native-visual-parity/capture.mjs update .local/native-visual-parity/cold-timeout-ids.json
```

**Notes**

Using the final fast-probe archive without competing capture jobs, all five exact timeouts pass in 2.2 minutes and persist successfully. The linked regeneration page lists exact story IDs. The public driver invokes update with one exact --story-id per selected story plus --approved --allow-dirty --rebuild.
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
node .local/native-visual-parity/capture.mjs update .local/native-visual-parity/remaining-targets.json
```

**Notes**

All remaining 928 selected stories pass and persist. Combined with the five successful retries, all 933 available stories have regenerated PNGs. No create-only flag is used. Original selection remains recoverable from persisted basenames minus -chromium.png.
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
jj restore --from yyyztqwn catalog
```

**Notes**

Generated story diffs contain only injected visualDelta image parameters and whitespace. Restoring them preserves every story implementation, assertion and review tag byte-for-byte. The public preset auto-binds persisted PNGs at compilation; no metadata workaround remains.
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
node .local/native-visual-parity/verify.mjs inventory
```

**Notes**

933 PNG signatures and positive dimensions verified; 55140243 total bytes and 121 changed hashes. No missing or extra baseline file. SHA-256 inventory saved before compare-only runs.
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
Initial full fresh/cached comparison and affected dry-run
```

**Notes**

Before the staging correction: all 933 browser cases completed; outcomes were 911 identical, 4 within tolerance and 18 mismatches. Cached comparison reused all 933 actuals without changing capture IDs, timestamps or hashes. However, affected dry-run selected 1046 and excluded zero despite 915 passing cache rows. Original PNGs under tests/visual entered worker global inputs, changing host/worker fingerprints. This diagnostic is preserved in initial-parity-status.json and the initial logs; no cache row was fabricated or manually advanced.
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
pnpm exec vitest run src/node/native-affected-reuse.spec.ts src/node/affected-visual-tests.spec.ts src/node/capture-configuration.spec.ts
```

**Notes**

Upstream staged-baseline regression first reproduces 1 failure / 5 passes; corrected focused lane passes 33 tests. The first consumer-shaped packed run also exposes a CLI/Playwright cached-source failure; freezing logical source identity alongside physical reads repairs it. Existing budgets are lowered after extracting path helpers; the two intermediate growth diagnostics are repaired without raising limits. Final package checks pass 618 unit tests, types, all seven specification lanes, build and publint. The exact packed Chromium/WebKit and cold 13-case panel lanes pass.
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
pnpm install --no-frozen-lockfile --config.pnpmfile=.local/native-visual-parity/pnpmfile.cjs
```

**Notes**

Final consumer integration uses the 0.1.0 patch archive SHA-256 ba97613fa700ea602f7c14c4f7470c81272c817a1f25685aab2947e0343ad041. All 810 source/dist files match the inspected archive and the consolidated 0.1.1 candidate byte-for-byte. The latter archive is b3e3c514bfd68bc542bd900f580951f654a36bc07a37cead34539583d63e6c2b, source 1b253cf053985666923e6dbb70d6469abdd8345b, PR 49. Consumer package.json and lockfile remain byte-identical to their registry originals, and existing packed Design Core 0.6.0 remains installed. The final candidate dependency updates are qualified upstream; this consumer retains its original dependency environment.
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
Upstream combined candidate and hosted verification
```

**Notes**

PR 49 on 1b253cf053985666923e6dbb70d6469abdd8345b consolidates native staging parity, Vitest 5.0.5, Svelte 5.57.2, Testing Library React 16.3.3, Storybook Icons 2.2.0, PostCSS 8.5.31 and Changesets-generated 0.1.1. All four hosted workflows pass: Visual Delta CI 40096891059 (package, Docker Chromium/Firefox/WebKit, panel and manager); Native capture 40096890979 (macOS and Linux); Spec First 40096890960; stage cleanup 40096890935. Superseded PRs 42-46 and 48 are closed with links to PR 49. No registry publication or tag is performed.
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
node .local/native-visual-parity/verify.mjs fresh (initial inventory helper)
```

**Notes**

The first inventory helper expected bare hexadecimal font evidence, while the actual versioned profile correctly uses sha256:&lt;64 hex&gt;. The helper was corrected to assert that exact established format; it then validates all 933 sidecars. No product fingerprint, capture guard or profile was modified to satisfy the helper.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
node .local/native-visual-parity/capture.mjs fresh
```

**Notes**

Cold rebuilt public native test with exact 933 IDs, --fresh --failure-mode strict --rebuild. All 933 Playwright capture cases finish with runnerStatus=passed. Authoritative sidecars: outcomes {"passed":911,"mismatch":18,"changed-within-tolerance":4}, policy {"passed":915,"failed":18}, sources {"browser":933}. Strict CLI result follows visual policy rather than runner completion. Baseline bytes are unchanged.
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
node .local/native-visual-parity/verify.mjs fresh
```

**Notes**

All 933 sidecars have complete measured native font fingerprints and correct profile/baseline identities; actual PNG hashes match sidecars. The report saves captureOperationId, actualCapturedAt, actualHash, baselineHash and outcomes before the cached rerun.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
node .local/native-visual-parity/capture.mjs cached
```

**Notes**

Full 933-story public strict rerun without --fresh or --rebuild. Sources {"cached-actual":933}, policy {"passed":915,"failed":18}. Reuse preserves existing strict failures; no baseline is overwritten to conceal a difference.
<!-- /run -->
<!-- /evidence-field -->

