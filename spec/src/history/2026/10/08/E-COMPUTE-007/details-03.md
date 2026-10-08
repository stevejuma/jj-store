---
schema: 1
id: E-COMPUTE-007
---

# E-COMPUTE-007 details 3

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
pnpm docs:generate
```

**Notes**

Regenerated installation/type API references.
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
pnpm docs:check
```

**Notes**

First stale application source fingerprint corrected after final formatting; then uv was missing from PATH. Existing /Users/stevejuma/.local/bin/uv selected for subsequent strict docs build.
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
pnpm contracts:check
```

**Notes**

Generated contracts match; 300 actual HTTP/SSE responses conform. Seven contract tests and the contract TypeScript check passed. Public schema changes justify this focused cross-layer contract lane.
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
cargo fmt --all -- --check; cargo fmt --manifest-path runtime/local-agent/Cargo.toml -- --check
```

**Notes**

Both format checks completed successfully.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent{ProviderConfiguration,Defaults,Fleet,Execution,HarnessModels,SessionSettings,MentionSettings,Sessions,Runtimes}.stories.svelte
```

**Notes**

Fresh process after JJ Agent label/icons: nine directly affected families, 54 interaction/Axe stories passed. An earlier cold attempt was interrupted by Vite dependency optimization reload; the fresh rerun passed. No visual baselines changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

After the Advanced border follow-up, three interaction/Axe stories passed. Collapsed boundary asserts border/radius and a draft retained through collapse. The final rerun after the typed query correction also passed all three.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Latest Advanced regression used unsupported Testing Library exact option; web passed but catalog found one type error. Replaced it with an anchored accessible-name regex.
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
pnpm catalog:check
```

**Notes**

After the query correction: zero errors/warnings, 303 catalog modules. Final web check also had zero errors/warnings.
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
pnpm dev:backend:build
```

**Notes**

Rebuilt the backend after the JJ Agent label change. Existing macOS linker unwind warning did not fail the build.
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
JJ_STORE_DEV_PORT=7466 pnpm dev:restart (explicit existing resource overrides; see local setup notes)
```

**Notes**

Isolated complete native fixture bootstrapped; search eventually caught up. Cold startup 2264ms, restart 4711ms, fixture preparation 503545ms. Initial bootstrap service stop timed out and was recovered by the launcher. Main ports 7460/7462 remained untouched. Initial agent-tools/compute override paths were absent; corrected staged resources before final UI startup. No fixture reset or fabricated completion metadata.
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
stageDesktopResources / stageAgentResources via scripts/desktop-resources.mjs and scripts/agent-runtime-resources.mjs
```

**Notes**

Staged ignored task resources from existing built binaries: current backend/JJ/local-agent/tools, main ACP/embedded RustFS and existing .local/dev-resources/local-compute helper. Initial agent-tools source path was absent; corrected to runtime/local-agent/target/debug/jj-store-agent-tools and restaged. No weights downloaded, runtime started, installed executable replaced, or package published.
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
env PATH="$PATH:/Users/stevejuma/.local/bin" JJ_STORE_DEV_PORT=7466 pnpm dev:ui
```

**Notes**

Fresh Vite HMR and rebuilt backend ready on isolated ports 7466/7468 with complete preserved fixture and search ready. Bundled resource defaults resolved without the invalid earlier overrides.
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
CUA cold reload and Add agent UI inspection at http://127.0.0.1:7466/#/agents
```

**Notes**

Verified JJ Agent plus icons for Codex/Cursor/JJ Agent; eight providers plus inherited choice; Anthropic missing-auth guidance; shared compute shows no selected trusted host and unavailable catalog. Enter toggles Advanced, keeps focus, and retains DRAFT_BORDER_CHECK without saving. Collapsed group computed border 1px/radius 14px. At 390px viewport dialog is 358px wide without overflow; desktop screenshot inspected and viewport reset. Cleared temporary draft and retained preview tab. No agent prompt, secret entry or live authenticated inference.
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
pnpm spec:validate
```

**Notes**

Canonical schema validation passed before evidence finalization.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress
```

**Notes**

Intermediate evidence freshness failure after the latest label/icon files were added. Final evidence now enumerates all changed production/docs/fixtures and the M25 history. Unrelated generated Bootstrap/M24 and existing evidence navigation changes restored.
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
pnpm docs:review application "Reviewed shared bordered provider configuration, JJ Agent label and harness icons, plus collapsed Advanced boundary with retained drafts and keyboard focus."
```

**Notes**

Refreshed the owning documentation review fingerprint after final UI changes.
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
env PATH="$PATH:/Users/stevejuma/.local/bin" pnpm docs:check
```

**Notes**

Final strict documentation build passed: 141 files, local Mermaid rendering, links and anchors valid.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed non-spec Markdown/JSON/TypeScript/Svelte paths>
```

**Notes**

Only docs/maintenance/coverage.json needed formatting after the review generator. Formatted that file; all other selected paths already conformed.
<!-- /run -->
<!-- /evidence-field -->

