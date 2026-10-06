---
schema: 1
id: E-M14-017
---

# E-M14-017 details 2

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
jj workspace add ../jj-store-repository-conversion --name repository-conversion; jj --no-pager st; read parent and tracked guidance and owning API/UI/local-JJ/milestone chapters
```

**Notes**

Original workspace clean; isolated workspace started at rxzwlukn/b11c0486; unrelated workspaces retained. Before: local folder Add only, no native create UI.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test native_creation --locked (initial)
```

**Notes**

Test-only reqwest fixture assumed JSON feature; changed to explicit serialization/body bytes without enabling unrelated features.
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
cargo test -p jj-store-server --test native_creation --locked (first HTTP run)
```

**Notes**

Lowercase custom HTTP method caused 405; actual request now uses uppercase while capture method remains canonical lowercase.
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
pnpm exec tsx --test web/tests/native-repository-manager.test.ts (initial)
```

**Notes**

Identity mismatch test expected internal code spelling instead of existing human Error message; corrected exact expected message.
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
pnpm exec tsx --test web/tests/native-repository-manager.test.ts (final)
```

**Notes**

4 tests; exact unknown/reload/retry, mismatched receipt, known name rejection, and two already-mounted entrypoints using one saved intent.
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
cargo test -p jj-store-server --test native_creation --locked (isolated target)
```

**Notes**

native-creation-isolated.log:4 pass; unknown-before-write persists destination identity, restart status/replay, workspace-free pinned reads, name rejection, filesystem-only and LocalTrusted Admin admission.
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
cargo check -p jj-store-server -p jj-store-service -p jj-store-http --all-targets --locked (isolated target)
```

**Notes**

rust-check-isolated.log passes. Shared-target contract/native test attempts were invalidated after cached service artifacts from another workspace caused 16 compile errors; isolated target rebuilt owning crates.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries; pnpm web:build
```

**Notes**

Zero web/catalog diagnostics;924 module architecture;182 catalog modules;17 package boundaries. Production build passes with existing chunk-size warning. No baseline updates.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte (initial)
```

**Notes**

Shared node_modules symlink lay outside Vite filesystem allowance; neither test file imported. Offline fresh installation attempts rejected no-TTY removal then existing pnpmfile/lock mismatch; manifests/lockfile unchanged.
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
pnpm exec vitest run --config .local/repository-conversion/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte
```

**Notes**

13 directly affected interaction/accessibility stories pass with isolated ignored config allowing existing sibling dependency path; local validation only, no upstream defect concealed or package publication.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (before mapped chapter additions)
```

**Notes**

Added mapped CLI, compatibility, mutations and governance documentation for installation-native provisioning and transport qualification.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts native-repository-creation.spec.ts sidebar-repository-actions.spec.ts (initial)
```

**Notes**

2 pass/1 fail; Axe ran during the enabled-button opacity transition. Added explicit enabled/full-opacity readiness, retaining every accessibility rule.
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
pnpm exec playwright test --config web/playwright.config.ts native-repository-creation.spec.ts sidebar-repository-actions.spec.ts (next)
```

**Notes**

2 pass/1 fail; close selector matched frame and footer Close controls. Disambiguated the footer action.
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
pnpm web:check; pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts native-repository-creation.spec.ts sidebar-repository-actions.spec.ts (after inline validation); pnpm exec playwright test --config web/playwright.config.ts native-repository-creation.spec.ts (name rejection regression)
```

**Notes**

web-check-fields.log has zero errors/warnings; web-build-fields.log passes with existing chunk/plugin timing diagnostics. browser-fields.log:3 pass including unchanged sidebar keyboard/Axe; browser-name-validation.log:3 pass including exact retained recovery, absent-capability gating and inline name rejection/corrected new request. Cold fresh built browser services exited.4 unique affected browser scenarios qualified.
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
pnpm contracts:check (isolated target)
```

**Notes**

contracts-check-isolated.log:complete registered HTTP/SSE capture inventory passes, including new actual native creation/status/auth fixture;7 contract-tool tests and generated TypeScript consumer compile pass. Required escalation: additive bootstrap and HTTP schemas share the global route inventory.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

136 tasks;20 protected files/7 canonical chapters;5 validators/22 chapters/167 requirements. No milestone promotion. Prior raw evidence JSON preserved.
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
pnpm exec prettier --check (initial final set included native_creation.rs)
```

**Notes**

Tooling selection mistake: Prettier has no Rust parser. Corrected supported-file set and separate Rust formatter.
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
cargo test -p jj-store-server --test native_creation --locked (alias regression)
```

**Notes**

5 pass. Current bindings and pending local registrations reserve names; completed removed history does not. native-creation-alias.log.
<!-- /run -->
<!-- /evidence-field -->

