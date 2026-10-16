---
schema: 1
id: E-FORGE-AUTH-LIVE-001
---

# E-FORGE-AUTH-LIVE-001 details 2

<!-- evidence-field:versions markdown -->
## Versions

macOS 27.0 ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11/Chromium; Storybook 10.5.7; native macOS Keychain; registered jj-store OAuth app 3935935.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No schema, HTTP/CLI payload, credential-store format, endpoint identity, native JJ identity, manifest or WAL change. The existing optional saved-account binding is qualified against a live GitHub account. Only controller request scheduling changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Complete approved GitHub sign-in and cold macOS credential/repository read qualification. Serialize per-repository Forge transport calls so automatic discovery/polling and selected PR/check reads share the existing journal lease without collisions. Failure releases the queue; queued work rechecks view/access and never retries external effects. Preserve account selection, request identities, exact pending intents and revision guards. Tests are limited to the owning controller and direct Forge consumers; no wider suite repair, Rust rebuild or baseline update for this UI slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M31 In progress and qualify live Enterprise/PAT, other platforms, packaged deployment and disposable mutation flows separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning forge/workspace chapters specify serialized observation and
  closed-view dispatch guards. JST-API-001/002 wire contracts,
  JST-FORGE-002/003/007 account/intent/credential authority, native storage,
  manifest and WAL remain unchanged; this prevents an observed lease collision
  without bypassing admission or replaying effects.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress. User-approved GitHub OAuth, macOS Keychain
  save/read across restart, automatic existing-connection binding and cold live
  PR/check reads are now observed. Remaining provider/platform, mutation,
  native-storage, packaged and visual gates are not inferred from this profile.

```
<!-- /evidence-field -->

