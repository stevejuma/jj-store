---
schema: 1
id: E-READ-087
---

# E-READ-087 details 5

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
pnpm web:check
```

**Notes**

web-eviction-final.log: final UI source has zero errors and warnings.
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
python3 -u .local/path-history-fix/linux-prefilter-after.py
```

**Notes**

linux-prefilter-ready-after.log/json: final-source ready index, same fixed Linux operation/commit and limit 50. 60 finite responses all 200, maximum 12.621s. Block resolves blk-mq.c beyond the old cap after 36 batches (1 Available/49 Pending); root ends 20 Available/21 Pending; arch 5 Available/18 Pending. No exhaustive metadata or cold/pressure qualification claim. The preceding cold 413 episode remains an explicit gap.
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
CUA fresh Linux root navigation, arch folder click and request observation through 7470
```

**Notes**

Fresh UI visibly loads filenames, then Description/Updated progressively. linux-root-skeleton-final.png, linux-root-metadata-final.png and linux-folder-navigation-final.png show metadata skeletons and folder-navigation skeleton. Navigation cancels obsolete request (net::ERR_ABORTED); observed metadata requests return 200. This is focused browser evidence, not visual baseline approval or complete large-repo acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

cold-directory-final.log: final UI source, one fresh browser/component interaction passes; directory-navigation skeleton preserved. No visual baseline update.
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
pnpm check:progress
```

**Notes**

progress-final.log: 139 tasks, evidence bookkeeping passes; task status rows unchanged.
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
pnpm spec:validate
```

**Notes**

canonical-final-complete.log: final UI/Rust contracts validate with five validators, 23 chapters and 172 requirements.
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
pnpm spec:first
```

**Notes**

spec-first-final-complete.log: final source passes 33 protected files and eight canonical chapters.
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
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/ui.md web/src/lib/api/generated.ts web/src/lib/controller/native-presentation.ts web/src/lib/controller/production-file-metadata.ts web/src/lib/controller/production-file-reader.svelte.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

format-check-final-complete.log: all changed canonical and UI files are formatted.
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
node .local/path-history-fix/format-ledger.mjs
```

**Notes**

ledger-format-final.log: only E-READ-087 formatted; all 529 previous evidence entries preserved byte-for-byte in the historical text prefix and unchanged as parsed JSON.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Implementation and focused regression targets are verified; no milestone
  promotion. Exhaustive Linux row coverage is unqualified: after the finite
  final acceptance, block has 49 Pending rows, root 21 and arch 18. Cold very
  old history can require many batches; reducing all remaining cold-history cost
  is deferred."
- One final cold graph-preparation episode returned resource_limit 413; explicit
  existing preparation retry recovered to Ready and subsequent finite requests
  passed. Large-repository cold/preparation pressure remains a qualification
  gap.
- Complete release, scale/RSS, fault/RustFS, registry-resolved foundational
  packages and visual baselines remain deferred. Existing E-READ-084 Clippy
  diagnostics are unchanged and not rerun. Root aggregate spec/check/test/build
  lanes are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this verified implementation slice on clean main with Jujutsu and retain the temporary workspace and 7470 preview. Qualify cold large-repository preparation and complete older-row coverage in a separately bounded follow-up; no broader search/evolution/copy optimization in this slice.
<!-- /evidence-field -->

