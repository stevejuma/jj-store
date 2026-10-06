---
schema: 1
id: E-M14-009
---

# E-M14-009 details 3

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Search changes affect the adapter runner and shared pager plus its server/HTTP/UI consumers. Escalate to the full Tantivy/adapters package tests and native lifecycle search family, plus server search configuration/replication tests; no unrelated root aggregates.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Search was read-only, with a fixed process-lifetime worker/pager and no browser configuration owner.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional local_search_management and V1 Search settings/save/control schemas; optional enabled field in existing configuration format 2. Native repository authority unchanged. Enclosing Jujutsu change is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; pnpm 10.32.1; locked Svelte/Vitest/Storybook.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the main local app. Continue remaining Agent setup/recovery, Storage maintenance, Sync finite-frontier UI and integrated settings acceptance as separately tracked M14 work.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending.
- Integrated native settings persistent browser qualification is M14.7; no
  milestone-wide or real-RustFS fault qualification claim.
- Unclean process termination can leave disposable replacement directories;
  normal owner retirement is verified.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/search-*.log, .scratch/settings-dev-restart*.log and web/test-results. Test services remain isolated and are shut down.
<!-- /evidence-field -->

