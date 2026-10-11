---
schema: 1
id: E-REPOSITORY-RAIL-SETTINGS-001
---

# E-REPOSITORY-RAIL-SETTINGS-001 details 3

<!-- evidence-field:result markdown -->

## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->

## Remaining

- Inherited evidence-integrity and source-budget diagnostics remain outside this
  UI slice.
- Independent visual approval remains pending; no baseline updates.

<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->

## Versions

Node 26.9.0; pnpm 10.32.1; Design Core 0.3.1; Mira 0.0.3; Svelte 5.57.1;
Storybook 10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->

## Formats

Transient Settings UI only. Native repository, manifest, API/DTO, browser route
and persisted formats unchanged. Existing registry packages retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->

## Scope

Settings gear row beneath repository Details forwards the current repository
into transient Repository settings. No repository reopen; code
operation/commit/file/workspace and browser history retained. The enclosing
Jujutsu change xwzwrwmv owns this evidence. UI-only targeted verification; Rust,
root aggregate lanes, release/platform checks and visual baselines are out of
scope under the repository targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->

## Next

Retain focused repository/settings regressions and independent visual review.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->

## Contract review

```yaml
status: changed
reason: |-
  JST-UI-012 adds the direct Repository Settings destination beneath
  Details. JST-UI-014 retains existing navigation composition. JST-API-005/008
  authorization, discovery and exact snapshot contracts remain unchanged; the
  callback consumes the already selected repository and performs no reopen.
```

<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->

## Milestone review

```yaml
status: unchanged
reason: |-
  M4.18 remains Done under its original gates. This narrow shortcut
  adds desktop/mobile keyboard and live HMR regression evidence; it does not
  promote any milestone or claim release/platform/visual qualification.
  Inherited evidence/source-budget gaps remain explicit.
```

<!-- /evidence-field -->
