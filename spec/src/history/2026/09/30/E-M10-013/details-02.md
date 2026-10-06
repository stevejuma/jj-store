---
schema: 1
id: E-M10-013
---

# E-M10-013 details 2

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
jj --no-pager st
```

**Notes**

Both parent and temporary workspaces were clean before integration; the committed verification tip was preserved.
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
jj new -m "Merge local-first replication with repository UI" default@- local-first-verification@-
```

**Notes**

A two-parent integration change was created; five shared-file conflicts were resolved in the parent workspace.
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
pnpm spec:validate
```

**Notes**

All 103 canonical requirements validate after combining the parent UI and M10 chapters.
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
pnpm format:check
```

**Notes**

Rust and tracked-text formatting pass after resolving the five conflicts.
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
pnpm check:progress
```

**Notes**

Before committing the two-parent merge, the checker could not resolve @- to a unique revision; the same check passed from the single-parent child in pnpm check.
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
pnpm spec:first
```

**Notes**

Before committing the two-parent merge, the VCS diff could not resolve @- to a unique revision; validation resumes from the single-parent child.
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
pnpm web:check
```

**Notes**

The merged production web code has zero diagnostics or warnings.
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
pnpm web:test
```

**Notes**

All 55 web unit tests pass.
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
pnpm web:test:browser
```

**Notes**

A cold preview process passes all 21 browser interaction cases, including the parent text-file edit scenario.
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
cargo test -p jj-store-replication --locked
```

**Notes**

The native envelope unit test and all 15 replication integration cases pass in the parent checkout.
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
cargo test -p jj-store-cli --test workspace_migration --locked nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline -- --exact
```

**Notes**

The migrated workspace preserves local commands with zero upstream connections.
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
pnpm check
```

**Notes**

The complete specification, boundary, progress, RustFS, contract, web, catalog and Clippy check lanes pass after the merge commit.
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
pnpm test
```

**Notes**

The first parent aggregate run passed storage and RustFS stages but stopped at one stale RepositoryFiles Storybook assertion (87 of 88 Storybook cases passed). The exact case was repaired under E-UI-029.
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
pnpm test
```

**Notes**

All governance, storage, RustFS, LocalTrusted, contract, web, catalog, cold browser and Rust workspace test lanes pass in the merged parent checkout.
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
pnpm build
```

**Notes**

Storybook, web, embedded RustFS and Rust workspace builds pass in the merged parent checkout.
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
pnpm spec:check
```

**Notes**

All 18 specification lanes pass, including Storybook interactions, cold browser acceptance, Rust workspace tests, mdbook and spec-first.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Rust 1.98.1; pnpm 10.32.1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new persistence or wire format; the integration combines the existing M10 replication profile with the parent repository UI history.
<!-- /evidence-field -->

