---
schema: 1
id: E-LOCAL-SERVICE-ICONS-001
---

# E-LOCAL-SERVICE-ICONS-001 details 2

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Five inherited E-NPM-REGISTRY-001 governance diagnostics: missing
  .pnpmfile.cjs, missing prose pseudo-file, invalid named test reference,
  invalid run result and invalid result. These are unchanged from parent
  c9c16fc2 and prior E-GLOBAL-SETTINGS-NAV-001 evidence."
- Visual baselines, release qualification, native builds, repository migrations
  and aggregate suites are out of scope for this private presentation-only
  change. Existing development data and published dependencies are preserved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; Lucide Svelte 1.48.0; existing pnpm 10.32.1, Storybook 10.5.7 and Vitest 4.1.11. Released Design Core and Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No URL, API, persisted configuration, server authorization or credential format changes. Copy eligibility and exact values are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Only Local service Launch and Storage inventory row icon selection changes. Before: the copyable-path classification assigned folders to catalogs, identities, policy, socket, executable, volume, credentials and ownership. After: semantic Lucide icons match each setting, with sensible credential, network, bucket, region and backup handling for flattened inventory. State directory retains its folder. Existing shared rows keep their size, alignment, subtle backgrounds and aria-hidden decoration. No new tests mirror the icon map; existing copy and interaction/Axe stories cover behavior. Community presentation and released Design Core were inspected; no code or dependency copied or patched.
<!-- /evidence-field -->

<!-- evidence-field:live_acceptance markdown -->
## Live acceptance

CUA on the existing Vite development app at 127.0.0.1:7460. Observed folder overuse before editing. New icon imports initially triggered dependency optimization and failed HMR; a fresh document then rendered correctly. Verified actual Launch and Storage row SVG classes against their labels and visually checked both tabs. Launch: book catalog, fingerprint identity, plug RPC socket and shield-check policy. Storage: terminal executable, hard-drive volume, key credentials, fingerprint installation, shield-user ownership and archive backup. Copy controls and values remain present. Screenshots: .local/settings-icons/launch.png and storage.png. No live configuration saved, native services restarted or fixture data reset.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain semantic inventory icons and existing copy regressions. Inherited governance diagnostics and independent release/visual gates remain separate. Verified slice committed in the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-016 already requires relevant, restrained icons for read-only
  settings rows. This corrects the private icon mapping to comply with that
  guidance; labels, grouping, Copy behavior, authorization and values remain
  unchanged. JST-UI-012 navigation, owners, refresh, drafts and pending command
  behavior are untouched.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M14.2 remains Done. The enclosing Jujutsu change is the evidence
  revision; this scoped icon correction promotes no lifecycle, platform, release
  or visual-baseline gate.

```
<!-- /evidence-field -->

<!-- evidence-field:acceptance_target markdown -->
## Acceptance target

Existing StoreServerSettings stories: Launch inventory groups paths and retains exact copy values; Storage inventory separates connection paths credentials and backup. All 15 stories in the direct consumer file passed, including interaction/Axe checks. Story display names are recorded here because the named-test ledger validator only recognizes source test() or function definitions.
<!-- /evidence-field -->

