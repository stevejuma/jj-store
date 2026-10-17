---
schema: 1
id: E-REUSABLE-AGENTS-002
---

# E-REUSABLE-AGENTS-002 details 2

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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --test codex_discovery --locked
```

**Notes**

Before acquisition routing: four original discovery cases pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --test codex_discovery --offline
```

**Notes**

After acquisition routing: same four cases pass; lock adds only lapis-ai-capture0.1.1.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --locked
```

**Notes**

Complete affected capture boundary:185 parent cases plus one printed child execution pass; three parent-invoked child fixtures ignored. New exact raw-header witness regression included. Historical binding test name retained and separately verified against actual absent/looped filesystem bindings.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-capture --all-targets --locked -- -D warnings
```

**Notes**

Strict all-target capture lint passed.
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
node --test scripts/ci*.test.mjs scripts/governance.test.mjs
```

**Notes**

81 cases pass; escalation covers shared CI dependency planning and pinned-source inputs.
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
node --test scripts/acquisition-boundaries.test.mjs
```

**Notes**

Moved acquisition boundary case into its own bounded test module; only capture may import acquisition and it still cannot import runtime.
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
node scripts/check-boundaries.mjs; node scripts/package-check.mjs
```

**Notes**

23 workspace package boundaries and24 Cargo package metadata/input checks pass.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

New governance test initially exceeded500lines; moved to acquisition-boundaries.test.mjs. Remaining three unrelated pre-existing UI size failures reproduced from parent: StoreStackWorkspaceApplication401/400, StoreApplicationMain313/300, StoreStackWorkspace310/300. No limits or UI changed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-server --lib capture_supervisor --locked
```

**Notes**

12 source lifecycle, discovery, restart and unknown-receipt supervisor cases pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test capture_import_cli --test capture_cli --locked
```

**Notes**

12 CLI control cases and one explicit import/replay case pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --lib current_binding_distinguishes_absence_from_unknown_io_outcomes --locked
```

**Notes**

Preserved historical named test now checks actual filesystem absence vs symlink-loop IO using native current_binding; availability false is not retirement evidence.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

Full strict manual build and closure pass with9Mermaid diagrams in both themes; capture and local-compute source fingerprints reviewed.
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
pnpm check:progress; pnpm spec:first
```

**Notes**

Historical evidence requires the original named binding test path. Retained the name with actual native filesystem negative/unknown IO coverage; no historical evidence rewritten.
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
pnpm spec:validate
```

**Notes**

Initial fresh evidence navigation was not linked from date index. Added the owning date index and summary entries only.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-capture --all-targets --locked -- -D warnings
```

**Notes**

Final strict lint includes the retained actual-filesystem binding regression.
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
node --test scripts/acquisition-boundaries.test.mjs scripts/ci-selection.test.mjs scripts/governance.test.mjs
```

**Notes**

43 cases pass after separating the new boundary test module.
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
pnpm check:progress
```

**Notes**

171tasks/859evidence records; M33 status remains In progress.
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
pnpm spec:first
```

**Notes**

16protectedfiles/5canonicalchapters pass.
<!-- /run -->
<!-- /evidence-field -->

