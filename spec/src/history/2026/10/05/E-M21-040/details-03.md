---
schema: 1
id: E-M21-040
---

# E-M21-040 details 3

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
pnpm docs:review services "Documented owned local Stacks command DTOs and service reexport of the ordinary physical-folder engine, separate from manifest and retained-executor profiles."
```

**Notes**

Service source fingerprint and ordinary-local provider separation recorded.
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
pnpm docs:review http "Documented registered same-origin LocalTrusted administrative Stacks POST commands, generation and operation/config pins, and post-publication warnings."
```

**Notes**

HTTP source fingerprint reviewed with path-free admission and warnings.
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
pnpm docs:review application "Reviewed real workspace core commands, operation changes that reset lazy graph ranges, Unstaged counts and native conflict notices; later editing capabilities remain disabled."
```

**Notes**

Application fingerprint reviewed after new-operation range invalidation and real loaded counts.
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
pnpm docs:review contracts "Reviewed generated local Stacks command/state contracts including native conflict flags and exact command pins."
```

**Notes**

Generated contract fingerprint updated after native-conflict status.
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
pnpm web:build
```

**Notes**

web-build-final-cold.log: final production bundle/manifest and strict docs staging pass. Existing non-fatal Svelte/Tailwind/bundle diagnostics remain; 9 local diagrams and valid 139-file documentation closure.
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
pnpm web:check
```

**Notes**

web-check-last.log: 0 errors/0 warnings; earlier repeats in web-check.log, web-check-final.log also pass.
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
pnpm catalog:check
```

**Notes**

catalog-check-last.log: 0 errors/0 warnings; 227 catalog modules pass.
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
pnpm test:governance
```

**Notes**

governance.log: 50/50 governance tests.
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
pnpm check:architecture
```

**Notes**

architecture-last.log: 1093 production modules pass source budgets; adapter/controller split without new ownership boundary.
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
pnpm check:ui-ownership
```

**Notes**

ownership-final.log: copied UI ownership validator passes; no reference-checkout dependency or package manifest change.
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

boundaries-final.log: 17 package boundaries pass; upstream jj-lib remains in its owner.
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
pnpm spec:first
```

**Notes**

spec-first.log: mapped initial gate passes 44 protected files and 10 owning chapters; final ledger gate repeats separately.
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
pnpm spec:validate
```

**Notes**

spec-validate.log: 5 validators, 23 chapters and 173 requirements; final ledger validation repeats separately.
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
cargo fmt --all
```

**Notes**

Rust formatting applied to owning engine and direct route/reference tests; no unrelated source changed.
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
pnpm exec prettier --write web/tests/real/stacks-browser.spec.ts web/tests/vite/production-stack-adapter.test.ts web/src/lib/controller/stack-workspace/production-adapter.ts web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte spec/src/ui.md spec/src/mutations.md spec/src/spec-governance.md spec/src/compatibility.md docs/site/guides/browsing.md
```

**Notes**

format-followup.log and subsequent format-last-source/format-final-docs/format-records/format-verification.log: changed source/docs formatted; milestones ledger never formatted.
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
cargo clippy -p jj-store-jj -p jj-store-http -p jj-store-cli --all-targets --no-deps --locked -- -D warnings
```

**Notes**

clippy-final.log and clippy-post-test.log: affected roots and all targets pass strict lint; final rerun covers the corrected bookmark test.
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
cargo fmt --all -- --check
```

**Notes**

rust-format-check.log: all owning Rust formatting passes.
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
pnpm exec prettier --check catalog/fixtures/stack-workspace/production-http.ts docs/implementation-plan.md docs/maintenance/coverage.json docs/site/guides/browsing.md docs/site/reference/api/installation.md docs/site/reference/api/types.md docs/site/reference/transport.md docs/site/technical/local.md schemas/openapi.json scripts/stacks-browser-acceptance.mjs spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/api/generated.ts web/src/lib/controller/http-local-code-data-source.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/eligibility.ts web/src/lib/controller/stack-workspace/local-references.ts web/src/lib/controller/stack-workspace/local-session.ts web/src/lib/controller/stack-workspace/local-state.ts web/src/lib/controller/stack-workspace/local-transport.ts web/src/lib/controller/stack-workspace/production-adapter.ts web/src/lib/controller/stack-workspace/source-range.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/tests/real/stacks-browser.spec.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts
```

**Notes**

format-check.log: all changed supported documents/source pass; exact path list derived from jj diff --name-only, historical ledger excluded.
<!-- /run -->
<!-- /evidence-field -->

