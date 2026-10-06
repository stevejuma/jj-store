---
schema: 1
id: E-M14-006
---

# E-M14-006 details 3

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
cua_repl: createBrowserTab; profile menu > Settings > Repository > Browse; getAXState
```

**Notes**

Live rebuilt demo advertises Browse, shows pending system-dialog state, then returns to enabled form with empty draft and no error. No folder linked. Automation could not bind the non-bundled osascript process; no native selection/cancellation interaction is claimed. Initial connection during restart was refused; retried after HTTP readiness.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Protected changes have owning chapters; 132 requirements and 111 tasks accepted. Canonical UI/security describe explicit fixed native helper exception, capability gating and lifecycle limits.
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

<!-- evidence-field:before markdown -->
## Before

Main working copy clean at 96fedc24 above 82bacf59; Local folders only accepted manual path entry. Two new exact controller regressions fail before implementation.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One folder-picking behavior across owned service/HTTP, CLI/server composition and Settings UI. Focused checks cover each boundary and direct consumer. Full contract lane is required for additive public schemas. Root aggregates, full Rust/UI suites and release qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional local_folder_picker capability and V1 FolderSelection {version,path:null|string}; empty strict JSON request. No registry/native persistence changes. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; pnpm 10.32.1; locked Svelte/Vite/Storybook/Vitest; existing local Community/Design Core resolution.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the macOS Browse action in Repository settings; continue M14.3 agent setup and automatic recovery.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Linux/Windows native dialog adapters and headless GUI availability; manual
  path entry remains supported.
- Visual baselines remain visual-pending; no visual approval claimed.
- M14.3-M14.7 remain unfinished.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/folder-picker-*.log and web/test-results; disposable process fixtures cleaned up.
<!-- /evidence-field -->

