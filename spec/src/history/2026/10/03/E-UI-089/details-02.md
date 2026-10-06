---
schema: 1
id: E-UI-089
---

# E-UI-089 details 2

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
jj --no-pager st; jj workspace list; jj log and diff for main/Appearance ancestry in both repositories; inspect parent/repository guides, runtime ownership/configuration and local package hook
```

**Notes**

Both main/temporary workspace pairs clean. jj-store common base nxoksqrt/ba74e1ea; complete Appearance history has 389 changed paths, main 96, with 29 overlapping paths. Unrelated registered workspaces and active source/consumer checks left intact.
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
jj new yyknxszk kltumumn -m "Merge Appearance UI into the main workspace"; jj rebase -r ltrkpyzt -d quoystmy (Design Core)
```

**Notes**

Two-parent consumer merge preserves the full stack; 11 committed merge conflicts resolved explicitly. Source default now descends unchanged tooltip fix bb40d39d. No source implementation rewrite or extra source visual test.
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
python3 /tmp/jj-land-resolve.py; jj resolve --list; pnpm exec prettier --write (11 resolved files)
```

**Notes**

295 historical evidence IDs retained. Shared E-M15-031 uses main’s later verification extension; other parent evidence entries are unchanged.108 task rows preserve both evidence lists and statuses. No conflicts remain. jj resolve --list returns code2 when there are none; the formatter was run separately after that inspection.
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
pnpm install --no-frozen-lockfile --lockfile=false; compare installed tooltip-content.svelte to landed Design Core source
```

**Notes**

Existing ignored .pnpmfile.cjs consumes main Design Core. No manifest/lockfile edits. Tooltip bytes identical, SHA256 385eef907e223c778f2d7941b16d4ddaa3481159dcf51f39b7de0dc573c88757; Bits UI2.19.4 from Appearance portable manifest retained. Registry requests warned about17s latency but install completed25.8s.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-directories.test.ts (initial integration)
```

**Notes**

3pass/1failure: old test treated local profile as unsupported even when it exposed qualified review reads. Update capability-missing fixture and add local/native capability regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-directories.test.ts (first new regression)
```

**Notes**

4pass/1fixture calibration failure: homeB intentionally has zero reviews; assert exact per-home [1,0] rows and repository snapshots rather than inventing a review.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test; pnpm catalog:test (initial combined UI unit lanes)
```

**Notes**

Web70pass. Catalog289pass/4fail across52files: review fixture count, missing native permission in navigation fixture, confirmed Work reactions discarded before UI reload, and storage-cap test exceeded5s under parallel machine load. Exact serial regressions subsequently20pass with no timeout/assertion weakening; complete serial catalog rerun follows.
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
pnpm check:architecture (initial merged contracts)
```

**Notes**

Combined controller contract301 counted lines against300. Reuse one private ReviewDetail type alias, preserving both actions and the unchanged source budget.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/projects-directories.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/work-reactions.test.ts web/tests/vite/disclosure-layout.test.ts; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

20/20 exact regressions across4files;808production modules under unchanged budget;ownership1validator/21chapters/147requirements. Final strengthened unrelated reaction-cache assertion is included in the next complete catalog rerun.
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
pnpm web:build
```

**Notes**

Fresh integrated production bundle generated for cold local-browser acceptance; existing chunk-size warning retained.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial merged types)
```

**Notes**

Web9errors/7files, catalog5errors/5files: new sidebar HTTP source constructor and native-only review-detail types. Retain repository-owned native/local ReviewDetail union in agent and review composition; initialize and close independent query scopes.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (first type correction)
```

**Notes**

4errors/2files remained: inline review-action context still native-only, and generic Thread/Reaction command union required exact command discriminator. Fixed both with owned union type and existing set/clear/resolve_conflict discriminators.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm spec:first --base kltumumn --head @
```

**Notes**

Default @- is ambiguous on a two-parent merge; explicit CI ranges use Git commit IDs rather than JJ change IDs. Both parent ranges subsequently pass using actual commit hashes. No VCS or gate weakening.
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
pnpm catalog:test --maxWorkers=1 --no-file-parallelism (first serial rerun)
```

**Notes**

292pass/1failure across52files: bounded disclosure storage exceeded5s under machine load. Immutable replacement now uses $state.raw, removing redundant deep-proxy traversals; exact storage contract unchanged and full rerun293pass. No timeout changes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook (9 direct families, first main run)
```

**Notes**

53pass/6fail with6runtime errors across9files. DisclosurePersistence failed on effect_orphan while rendering refreshed dependency modules; other8families passed. Clear only the owned generated Storybook optimizer cache and run DisclosurePersistence in a fresh process. No source baseline or assertion updates.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/work-reactions.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/disclosure-layout.test.ts (new HTTP regression)
```

**Notes**

14pass/1fixture failure: Response default text/plain was rejected by Client. Add the real application/json response header; keep strict decoder checks.
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
pnpm contracts:check (first merged contract)
```

**Notes**

Generated OpenAPI artifact drift after automatic merge; regenerate from combined owning Rust source then qualify full contract response capture.
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
python3 inline fixture/bootstrap/runtime inspection; pnpm dev:ui
```

**Notes**

Two optional header-incomplete urllib bootstrap attempts received403 and made no mutations. Existing fixture SHA256 retained. Owned app now runs from main: server95351/ui95352, same7460/7462ports. Runtime webRoot/uiRunner/workspaceRoot use main. Only old owned33733/42374 processes stopped; RustFS volume and installation identities retained.
<!-- /run -->
<!-- /evidence-field -->

