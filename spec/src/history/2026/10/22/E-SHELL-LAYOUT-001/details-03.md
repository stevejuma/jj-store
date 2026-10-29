---
schema: 1
id: E-SHELL-LAYOUT-001
---

# E-SHELL-LAYOUT-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Initial chapter map also requires API and governance review for UI/provenance changes. Add qualified unchanged-contract evidence for those unaffected boundaries; retain updated shell topic. /tmp/jst-shell-layout-spec-first.log.
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
pnpm exec prettier --write <existing changed files excluding spec/src/history>
```

**Notes**

Scoped formatting only; no repository-wide rewrite.
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
node scripts/web-manifest.mjs
```

**Notes**

Built UI manifest validation passes without documentation/Rust staging.
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
Changed-file architecture using architectureErrors and jj diff --name-only
```

**Notes**

Zero diagnostics in changed files; two inherited stack-workspace size diagnostics remain. No baseline or limit changes.
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
pnpm spec:first
```

**Notes**

Final API/governance confirmation accepted; 31 protected files and one owning UI topic. /tmp/jst-shell-layout-spec-first-final.log.
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
pnpm check:progress
```

**Notes**

180 tasks and 950 evidence records pass, including explicit status-unchanged milestone review and source file coverage. /tmp/jst-shell-layout-progress-final.log.
<!-- /run -->

<!-- run -->
### Run 7

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

Seven validators pass across 4396 chapters and 208 requirements. /tmp/jst-shell-layout-spec-validation.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Two inherited stack-workspace architecture size violations remain owned by the
  stack-workspace source/catalog family; no push or release qualification is
  claimed.
- Aggregate web:build documentation staging is blocked by managed Rust disk
  preflight; the focused UI bundle passes.
- Registry deployment requires a released Design Core version containing these
  APIs, normal dependency update and renewed acceptance.
- Visual approval remains separate; no screenshots or baselines were updated.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; Chromium via repository Playwright. Registry manifest remains @lapismd/design-core ^0.5.1. Local prepared archive from Design Core change kulovuqu/e6320c0d and verification ab1570ca uses a development 0.6.0 manifest; registry 0.6.0 does not contain these unpublished additions. SHA-256 272678190a387b017f73b58c46417093340d9ccc069d31240e4e857fa629817a.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, route, saved-layout, pending-command or storage format changes. Existing panel IDs and widths remain authoritative. Tracked package.json and pnpm-lock.yaml retain registry resolutions. Ignored archive and pnpm readPackage hook are documented in docs/catalog.md.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

New isolated shell-layout workspace based on f58b9363, excluding default workspace documentation edits. Opt-in public AppShell.Topbar/Target replaces duplicate header geometry; native chrome stays at top, compact browser chrome relocates into Main without remounting owners. Attached sidebars own adjacency; outer repository rail stays independent and uses Sidebar.Section. SurfaceLayer contextual Close/Footer, responsive inset tokens and explicit child-owned scrolling replace local overlay presentation. Only UI and direct consumers are in scope. Cold focused Storybook and web build cover startup/package acceptance; root aggregate, Rust, live native backend and release qualification are out of scope under the tracked targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the locally verified adoption and preserve explicit inherited architecture, documentation-build, registry-release and visual gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: UI shell composition chapter documents opt-in primitive adoption and
  temporary packed-archive gate. Preserve routing, explicit controllers,
  persisted panel identities/widths, drafts, pending commands, native geometry,
  single active layer and return focus. No backend policy change.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "M4.5, M4.20 and M4.24 retain Done for their previously qualified
  contracts. M4.13 remains Deferred: local archive acceptance cannot qualify a
  released shared dependency. The enclosing change owns fresh evidence; no
  release or visual gate is promoted."

```
<!-- /evidence-field -->

