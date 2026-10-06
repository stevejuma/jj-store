---
schema: 1
id: E-M18-016
---

# E-M18-016

<!-- evidence-field:id markdown -->
## Id

E-M18-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.2
- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kzkswsvy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Five-profile capture settings and generated contracts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Cursor Desktop3.22.7; Cursor CLI2026.09.26-dd393fe; OpenCode1.18.30 configured profiles. Svelte5.57.1; Vitest4.1.11. Runtime qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional credential-free loopback endpoint; distinct Cursor CLI profile; five configurable profiles. Existing saved/effective configuration and restart semantics retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/capture-management.ts
- catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
- crates/jj-store-capture/src/profile.rs
- crates/jj-store-cli/src/agent_commands/capture.rs
- crates/jj-store-server/src/capture_config.rs
- crates/jj-store-server/src/capture_management/report.rs
- crates/jj-store-server/src/capture_management/settings.rs
- crates/jj-store-server/tests/capture_management/settings.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/capture_management/settings.rs
- schemas/openapi.json
- web/src/lib/api/generated.ts
- web/src/lib/controller/capture-configuration.ts
- web/src/lib/controller/capture-types.ts
- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte
- web/src/lib/ui/store-settings/capture-source-presentation.ts
- web/tests/vite/capture-settings.test.ts
- spec/src/collaboration.md
- spec/src/api.md
- spec/src/ui.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-settings.test.ts (before/after)
```

**Notes**

Before6 pass/1 expected new regression failure; after7 pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-server --test capture_management settings::
```

**Notes**

6 pass,14 filtered.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript from public Rust types.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings;183 catalog modules.
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
pnpm exec vitest run --config .local/multi-harness/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
```

**Notes**

Separate cold runs:22 existing and1 new interaction/axe pass. Local wrapper only permits symlinked dependencies; no tracked infrastructure change.
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
cargo check --locked -p jj-store-server -p jj-store-cli (shared CARGO_TARGET_DIR)
```

**Notes**

Initial exhaustive match error in CLI fixed; final check passes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --frozen-lockfile; initial Storybook config
```

**Notes**

Two installs refused pnpmfileChecksum mismatch; unchanged lockfile. Initial browser dependency request refused outside-workspace module; no tests ran. Ignored local FS allow wrapper resolves test runtime.
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
Changed-file Prettier; cargo fmt --all
```

**Notes**

Only intended tracked files changed.
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

<!-- evidence-field:scope markdown -->
## Scope

Settings/contracts slice only. Prior evidence and milestone statuses unchanged. Wider aggregate lanes out of scope under targeted policy; visual baselines pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Wire and qualify OpenCode startup, then shared bounded Cursor JSONL capture.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


