---
schema: 1
id: E-M21-059
---

# E-M21-059 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
jj new xqkplmwn wuuntsnu -m "Merge local Stacks delivery with bounded repository reads"; jj st
```

**Notes**

Merge created as requested with six conflicts; original parent revisions and source workspace preserved. Direct resolution retained Main cached binding/Git handle and Stacks exact recovery/physical-folder access.
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
cargo check --locked -p jj-store-cli -p jj-store-http -p jj-store-jj -p jj-store-service
```

**Notes**

Initial automatic Cargo.toml merge duplicated prost; deduplicated the identical dependency. /tmp/stacks-merge-rust-check.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-cli -p jj-store-http -p jj-store-jj -p jj-store-service
```

**Notes**

First resolution exposed bound_folder at an overly broad visibility; kept it internal to the local-browser family. /tmp/stacks-merge-rust-check-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-cli -p jj-store-http -p jj-store-jj -p jj-store-service
```

**Notes**

Moved binding made two direct generation-field reads private; consumers now use the existing Access::generation method. /tmp/stacks-merge-rust-check-final.log.
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
cargo check --locked -p jj-store-cli -p jj-store-http -p jj-store-jj -p jj-store-service
```

**Notes**

Affected packages compile. /tmp/stacks-merge-rust-check-final2.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Both type checks report zero errors/warnings; 244 catalog modules. /tmp/stacks-merge-web-check.log and catalog-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit 'web/tests/vite/stack-workspace' 'web/tests/vite/production-stacks' web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/directory-query-cache.test.ts web/tests/vite/directory-response-cache.test.ts web/tests/vite/quick-file-metadata.test.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

155 tests in 19 directly affected controller/Files consumer files. /tmp/stacks-merge-units.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <eight Stacks/loading/navigation/pinning/conflict/graph/directory story files>
```

**Notes**

No tests ran: Main local validation package lacked panel-toggle.tokens.css. Eight suites failed at setup. /tmp/stacks-merge-stories.log.
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
pnpm install --offline --force --ignore-scripts; restore exact pre-refresh pnpm-lock.yaml
```

**Notes**

Supported local source refresh restored missing stylesheet; exact lockfile verified byte-identical. No manifest paths or dependency patches. /tmp/stacks-merge-dependency-refresh.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <same eight story files>
```

**Notes**

Fresh browser process: 39 stories passed in eight files. /tmp/stacks-merge-stories-after.log.
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
pnpm check:ui-ownership; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

Ownership, 1141 production modules and Rust package boundaries pass. /tmp/stacks-merge-ownership.log, architecture.log, boundaries.log.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes. /tmp/stacks-merge-rust-format.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm exec spec-validator first --base <JJ change ID> --head @
```

**Notes**

Default @- is ambiguous for two parents; explicit --base selects Git read-only diff and requires full commit IDs rather than JJ change IDs. No gate was disabled.
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
pnpm exec spec-validator first --base <exact parent commit ID> --head <exact merged commit ID> (each parent)
```

**Notes**

Both trustworthy JJ-emitted full commit comparisons pass: Main 472 protected files/12 chapters; Stacks 67 files/9 chapters. /tmp/stacks-merge-spec-first-main-final.log and stacks-final.log.
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
Parent-relative exclusive-change audit and historical ledger JSON equality audit
```

**Notes**

All exclusive parent changes preserved. Union retains all 533 Main and 597 Stacks evidence entries, 601 unique entries total, without changing shared payloads. Main/source workspaces and services retained.
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
cargo test --locked -p jj-store-jj --test local_stacks --test stack_source_recovery
```

**Notes**

24 ordinary local native mutation tests and 31 exact-source recovery tests passed; three isolated subprocess helper tests remain governed ignored entries. Existing unused-assignment test warning retained. /tmp/stacks-merge-jj-tests.log.
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
pnpm docs:check
```

**Notes**

Combined executable route definitions made generated installation reference stale; regenerating owned manual references. /tmp/stacks-merge-docs.log.
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
pnpm docs:review <six combined source topics>
```

**Notes**

Reviewed local, services, configuration, http, application and contracts against the current manuals. Updated source fingerprints without changing documented provider qualification.
<!-- /run -->
<!-- /evidence-field -->

