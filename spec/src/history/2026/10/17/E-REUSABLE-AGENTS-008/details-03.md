---
schema: 1
id: E-REUSABLE-AGENTS-008
---

# E-REUSABLE-AGENTS-008 details 3

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
CARGO_INCREMENTAL=0 pnpm --silent dev:cargo -- run --quiet --locked -p jj-store-http --example openapi
```

**Notes**

Fresh full generated OpenAPI structurally equals checked-in schemas/openapi.json. No generated TypeScript or schema change.
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
pnpm spec:first
```

**Notes**

Required mapped governance chapter for the new ordinary feature regression. Added narrow unchanged gate description.
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
pnpm spec:first
```

**Notes**

30 protected files /8 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

Owning change map exceeded its existing400line cap after added prose. Condensed only the existing index-ownership introduction with the new feature-check note; no historical evidence changed.
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
pnpm spec:validate
```

**Notes**

Second draft retained401lines; final formatted source is400lines. Preserved index authority and completion gates.
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
cargo fmt --all
```

**Notes**

Owning Rust files formatted.
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
pnpm exec prettier --write scripts/local-history-features.test.mjs package.json crates/jj-store-jj/README.md docs/maintenance/coverage.json docs/site/technical/local.md .changeset/local-history-profile.md spec/src/current/contracts/architecture/10-pinned-history-query-ownership.md spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md spec/src/current/contracts/documentation/03-delivery-and-validation.md spec/src/current/progress/M33/index.md
```

**Notes**

All changed authored prose/scripts formatted; prior evidence remains immutable. Final chapter cap400lines.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- fmt -p jj-store-jj -- --check
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

All changed authored files pass Prettier.
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
pnpm check:progress
```

**Notes**

171tasks/865evidence records pass.
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
pnpm spec:validate
```

**Notes**

7validators/4186chapters/207requirements pass; change-map chapter remains400lines.
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

Rust 1.99.0; pinned jj-lib and stock JJ 0.45.1; parent f607ea0d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No code, companion, cache, request, error or output encoding changes. New optional Cargo local-history profile; native includes it by default. Working-copy and native mutation adapters remain native-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

M33.4 focused feature: existing filesystem history/content/diff and cache construction compile independently from native object/WAL/services/runtimes. Preserve original native API and all default integration/example targets. Real stock-JJ source identity/pinning and direct native/server/schema consumers form the impact surface. UI, provider, Tauri, RustFS and release acceptance remain separate and are not run for this Rust-only feature slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement admitted Tauri history bridge, then native capture ownership and Notes JJ conversation persistence.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 adds independently selected local-history construction.
  JST-JJ-003/005, JST-LOCAL-003/004, JST-API-001/004/006 and persisted
  identities/budgets keep original contracts. Owning architecture, JJ, local,
  distribution and documentation chapters describe the additive profile and
  required host authority.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33 and M33.4 remain In progress. Focused history is qualified only by
  its recorded feature and real-source tests; Tauri bridge and native event
  owner remain. M33.3 retains store/workspace and UI adoption, M33.5-.8 remain
  Planned; no first usable Notes milestone claimed.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qrsytkxk
<!-- /evidence-field -->

