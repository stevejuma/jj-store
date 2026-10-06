---
schema: 1
id: E-UI-087
---

# E-UI-087

<!-- evidence-field:id markdown -->
## Id

E-UI-087
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Restore foundation tooltip sizing and qualify local navigation consumers
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change wovknnpw; foundation change quoystmy
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-018
- JST-ARCH-004
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-008
- JST-GOV-009
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate and stories stay visual-pending.
  M11.4 In progress and M16 Partial remain unchanged; source release and
  registry-only qualification remain pending.
- The earlier compare-only capture stopped at generated-cache EACCES before
  screenshot comparison. The user subsequently instructed us to ignore Design
  Core visual tests; further visual testing is out of scope, and no visual pass
  or baseline approval is claimed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User requests undoing earlier Design Core tooltip compaction. Inspect Community navigation first, foundation source/history second. Foundation change42a125503632 reduced theme small text to11px and padding from12px/6px to10px/4px. In a reused clean Design Core Appearance workspace rebased onto current clean main parent4922feea, restore theme small-text defaults and12px inline/6px block padding; retain public sizing tokens and unchanged arrow/portal behavior. No consumer tooltip workaround. Consume source via the already documented temporary /tmp/jj-appearance-pnpm.cjs hook with lockfile disabled; verify installed tooltip bytes match source. Restart only owned VitePID68050 to33733, retaining backendPID42374 and persistent main fixture. Source package full checks and compare-only visual target run because shared foundation behavior spans consumers; jj-store runs only focused direct navigation/shell interaction and type checks. Root jj-store aggregates, Rust, fixture reset, native writes, source main landing, publication and visual baseline updates are out of scope. Source acceptance is linked from DC-SHA-050 to spec/records/tooltip-sizing-restoration.md. Latest user instruction excludes further Design Core visual tests; finish with the passing nonvisual lanes and retain the earlier capture failure as historical evidence.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/reference/appearance-provenance.md
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/architecture.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1. Foundation local source0.1.2; its Svelte5.56.6, Storybook10.5.2 and Vitest4.1.11 remain unchanged. Consumer Svelte5.57.1 and Storybook10.5.7. Manifest/lockfile registry ranges unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/route/preference or public token name changes. Existing jj-store.appearance.v1 and fixture retained. Source checkout remains independently owned.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/dc-tooltip-restore-*.log; /tmp/jj-tooltip-restore-*.log; /tmp/jj-tooltip-restore-live.jpg; /tmp/dc-tooltip-restore-pack/lapismd-design-core-0.1.2.tgz
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain local source validation; further Design Core visual testing is excluded by the user. Complete separate foundation release/registry qualification only when authorized; no milestone promotion.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-087 details 2](details-02.md)
- [E-UI-087 details 3](details-03.md)
