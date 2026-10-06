---
schema: 1
id: E-M14-003
---

# E-M14-003 details 2

<!-- evidence-field:scope markdown -->
## Scope

UI-only production/controller/catalog and browser slice. Focused verification includes both direct consumers. Rust/backend and generated wire contracts unchanged; E-M14-002 owns real native/RustFS restart evidence. Root aggregate, full Storybook/Rust suites and release qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 workspace links API only; no new persistence or wire format. Folder drafts remain in memory. The enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

pnpm 10.32.1; Svelte 5; Vitest 4.1.11; Storybook 10.5.7; Playwright Chromium; existing local Community/Design Core workspace resolution (not registry-only qualification).
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement M14.3 agent management and automatic queue recovery, starting with independent enqueue and drain ownership.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending.
- Git-backed browser management remains gated on M13.4.
- Agent management/recovery worker, editable Search, Storage and Sync controls
  and integrated live browser acceptance remain subsequent slices.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/workspace-ui-*.log and web/test-results/settings; response fixture SHA256 c04558ab70ded072ee171712e84c6113343f77919b83a6e6532ef443b3ae5d8a.
<!-- /evidence-field -->

