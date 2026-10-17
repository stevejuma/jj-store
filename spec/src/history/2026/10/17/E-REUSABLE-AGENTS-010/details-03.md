---
schema: 1
id: E-REUSABLE-AGENTS-010
---

# E-REUSABLE-AGENTS-010 details 3

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
node --test scripts/governance.test.mjs scripts/local-history-features.test.mjs scripts/ci*.test.mjs scripts/release.test.mjs scripts/docs.test.mjs
```

**Notes**

94governance/CI/release/docs regressions pass.
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
node scripts/check-boundaries.mjs
```

**Notes**

25package boundaries pass.
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
node scripts/package-check.mjs
```

**Notes**

26Cargo packages have contained inputs, README/license/notice and versioned registry metadata.
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
pnpm spec:first
```

**Notes**

Updated target-feature regression required its mapped governance chapter. Added explicit standalone discovery qualification note; no gate removed.
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
pnpm spec:first
```

**Notes**

10protected files/7canonical chapters pass.
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

7validators/4193chapters/207requirements pass before final evidence expansion.
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

All authored changed files pass Prettier.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- fmt -p jj-store-jj -p jj-store -- --check
```

**Notes**

Affected Rust formatting passes.
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
pnpm check:progress
```

**Notes**

171tasks/867evidence records pass; wider qualification remains pending.
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
pnpm spec:validate
```

**Notes**

7validators/4194chapters/207requirements pass after evidence expansion.
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
pnpm spec:first
```

**Notes**

Final10protected files/7canonical chapters pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Inherited full normal-parallel CLI local-browser HTTP index task fails
  Busy:61pass/1failure/3ignored on both current and parent source. Isolated
  exact HTTP test passes; source seam is qualified by its narrower checks, while
  complete family acceptance remains pending. No user waiver or release
  qualification claimed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.99.0; stock JJ and jj-lib0.45.1 onmacOSARM64; parent20c2cc98.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing LocalFolder wire fields/errors and all native/Git/companion encodings unchanged. New owned Rust WorkspaceInfo observations include canonical root/metadata/Git paths and optional companion identity; no serialization or authorization capability. Default-only metadata carrier entry point adds no dependency edge.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

M33.4 reuse the CLI's existing read-only workspace discovery in jj-store-jj, expose default-settings metadata construction to hosts without upstream JJ types, and consume it in the CLI. Includes colocated/internal Git and linked workspaces, no-init/dirty-file/config isolation, original symlink/link-length errors and observed carrier reopening. CLI local-browser consumer family and independent companion profile are the Rust impact surface. No UI, Tauri application, provider, realRustFS or release claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Integrate Notes native physical-vault/window admission; resolve the inherited CLI index-preparation qualification gap before claiming complete affected-family/release acceptance.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 adds owned reusable discovery/default construction
  interfaces. Existing JST-LOCAL-003/004 source/read and JST-JJ-003/005 pins,
  absence-of-init, no working-copy load, no reconciliation and CLI error
  categories stay intact. Observed paths grant no authority; physical and window
  admission remain host-owned. No persisted format, schema, publication or
  execution behavior change.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.4 remain In progress for native host integration. This source
  seam alone does not complete physical-vault/window admission, native WebView,
  capture ownership or Notes conversation acceptance. M33.3 adapters/UI and
  M33.5-.8 remain required. The full CLI family Busy failure is
  parent-reproduced and keeps this wider qualification record pending; no
  milestone completed or waiver inferred.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nuzvtwqz
<!-- /evidence-field -->

