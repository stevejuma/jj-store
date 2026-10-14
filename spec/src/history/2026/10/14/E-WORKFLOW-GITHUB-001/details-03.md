---
schema: 1
id: E-WORKFLOW-GITHUB-001
---

# E-WORKFLOW-GITHUB-001 details 3

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
pnpm dev:cargo -- clippy -p jj-workflow -p jj-store-service -p jj-store-cli --lib --no-deps --locked
```

**Notes**

Final review completed with 6 existing service warnings and no CLI/domain warnings after correcting new slice diagnostics. This is not a strict lint pass.
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
pnpm contracts:generate
```

**Notes**

Generated additive schemas/OpenAPI/browser types; latest includes original event, scoped logs and bounded step_reports. First managed command post-build guard failed at 40.9GiB despite exporter completion. Final generation passed after guarded idle cleanup.
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
node scripts/docs.mjs generate
```

**Notes**

API and CLI references generated. First CLI generation lacked --before help; supplied public command doc comment and regenerated successfully.
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
pnpm dev:disk
pnpm dev:clean -- --dry-run
pnpm dev:clean -- --apply
```

**Notes**

Only after stopping owned dev service and finishing active tests, cleaned eligible root workspace outputs and recovered about 26GiB. Dependencies, .local/dev-demo, recovery data and staged resources retained. Subsequent target measured 21.9GiB and total42GiB, below limits.
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
pnpm dev:backend:build
pnpm dev:ui
pnpm dev:backend:restart
```

**Notes**

Paired native binaries built, preserved Vite fixture restored. Final backend-only restart retained saved execution settings and journals; no data reset or installed CLI replacement. Linker compact-unwind size warning retained.
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
pnpm contracts:check
```

**Notes**

Generated contract drift comparison and live response captures passed; 8 schema validation tests and TypeScript examples passed. Wider direct contract lane required by the additive shared API schema.
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
pnpm check:progress
```

**Notes**

Initial navigation refresh rewrote whitespace in unrelated historical indexes, causing fresh coverage errors. Restored only incidental generated changes and refreshed the complete slice file inventory before rerunning.
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
pnpm docs:check
```

**Notes**

Intermediate source formatting invalidated services/application review fingerprints. Refreshed owning reviews after final formatting, then reran.
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
pnpm spec:validate
```

**Notes**

7 validators passed, 3775 chapters and 196 requirements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

Final 12 settings interactions/Axe passed after widening Forgejo environment inputs.
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
pnpm check:progress
```

**Notes**

Final slice check passes:153 tasks,791 evidence records; unrelated navigation whitespace restored.
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
pnpm docs:check
```

**Notes**

Final doc build passes:9 Mermaid diagrams in both themes,143-file documentation closure,links/anchors valid; final application source fingerprint refreshed.
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
pnpm format:rust:check
```

**Notes**

Rust workspace formatting check passes.
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
pnpm exec prettier --check web/src/lib/ui/store-workflows/WorkflowFilePicker.svelte docs/maintenance/coverage.json spec/src/current/contracts/ui/41-workflows.md
```

**Notes**

Docs review serializes coverage JSON; formatter rerun after final fingerprint refresh corrects its layout. No source-contract drift.
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
pnpm exec prettier --check changed Markdown/JSON/TypeScript/Svelte/CSS files
```

**Notes**

Final complete-slice formatting passes; Rust formatting separately passes. Specification and progress checks rerun after final evidence updates.
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
pnpm spec:validate
```

**Notes**

Appending final validation created details-04.md; its local SUMMARY entry was missing. Regenerated navigation only for the new evidence folder before rerunning, preserving unrelated history.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Remote Forgejo registration/polling/ConnectRPC, live agent providers,
  additional platforms and approved visual baselines remain separate M28 gates.
- GitHub APIs/tokens, artifact/cache services, reusable workflows,
  services/container overrides, dispatch inputs and dynamic runs-on are outside
  this qualified profile.
- Strict Clippy remains blocked by existing dependency/service diagnostics. Root
  aggregate check/test/build/spec:check and unrelated lanes are out of scope
  under targeted verification.
- The private workflow name isolates Docker resources; github.workflow exposes
  the attempt-specific name. Original workflow-name context semantics remain
  unqualified, as documented in compatibility.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Custom JJ 0.45.1; Rust 1.98.1; native Go 1.27.1 darwin/arm64 Forgejo Runner 13.2.0; Docker29.8.0 Linux arm64/alpine3.23; Node24.10.0, pnpm10.32.1, Svelte5, Vitest4.1.11; pinned embedded RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive version-1 github_workflow executor, skipped outcome, optional code_context/event/log step and bounded selected/latest step_reports; host version1 and managed environment version2 add optional Forgejo capability. Absent optional fields preserve legacy bytes and event IDs. Manifest6, root/catalog versions and journal1 stay unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

App filters identify candidates; executor verifies supported YAML filters from the same exact source operation/commit. CLI exports source and supervises external local Forgejo execution with stable journals/report receipts, scoped logs, secret redaction and attempt-labelled Docker cleanup. UI reuses Settings forms, existing path/Browse control, pinned source chooser and Forgejo console; displays actual steps, nested groups and Refresh spinners. Server retains native metadata/admission rather than scheduling or running workflow content.
<!-- /evidence-field -->

