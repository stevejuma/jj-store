---
schema: 1
id: E-VISUAL-BASELINES-MACOS-001
---

# E-VISUAL-BASELINES-MACOS-001 details 8

<!-- evidence-field:remaining list -->
## Remaining

- Human visual review remains pending. No story review tags, screenshot
  thresholds, readiness conditions or timeout limits are changed by baseline
  creation.
- "Fresh comparison is not green: 18 visual mismatches remain from 933 completed
  captures; 915 are within the unchanged comparison policy. Several inspected
  mismatches contain random request IDs printed by existing fixture debug
  output. Other differences require separate visual/stability review. The exact
  22 non-identical outcomes, including four within tolerance, are recorded in
  comparison.md. No assertion, threshold, source implementation or baseline is
  changed to conceal them."
- "113 of the original 1046 stories have no native baselines: 110 require the
  Vitest browser host, one exceeds the 15-second readiness window, and two
  repeatedly exceed the 30-second capture test window. Their exact IDs and
  categories are listed in the linked blocked pages. The user explicitly chose
  available-story capture rather than story portability changes."
- "Unchanged native stories currently recapture in both all and affected modes
  despite workflow.reuseActualComparisons=true: the released addon does not
  fingerprint native host fonts and disables persistent capture reuse. The
  static build cache still works. A proper upstream native-font fingerprint is
  required; no reuse guard is bypassed here."
- Inherited full catalog interaction/accessibility qualification and the Design
  Core registry release gap remain open. This artifact-only slice does not rerun
  or qualify the full product, Rust or full Storybook interaction lanes. The two
  inherited architecture-budget failures are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Visual Delta 0.1.0, Playwright 1.63.0, Chromium 153.0.8010.12, native macOS ARM64 Darwin 27, Node 26.9.0, pnpm 10.32.1. The pre-existing packed Design Core 0.6.0 validation environment remains active; the manifest and lockfile still use registry ^0.5.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native PNG baseline files under the Darwin ARM64 profile path, public-preset image parameters injected only during Storybook compilation, capture-profile schema 2 and result sidecar schema 4.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The user explicitly requested current native macOS baselines and then selected capture of available stories only after the full readiness inventory identified 111 readiness blockers and subsequent bulk/isolated capture reproduced two 30-second timeouts. Scope: create missing primary PNGs for 933 available stories; preserve existing files, component source and visual-pending review tags. The public preset discovers PNGs and injects image parameters at build time, so generated CLI source wiring is restored after capture. No production, interaction, accessibility or package-version change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the native PNGs and the 18 recorded mismatches separately; stabilize story fixtures and port standalone blockers in their owning code before full visual qualification. Native font fingerprint support is separately required for persistent capture reuse.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: The native PNG set establishes current visual references for JST-UI-001.
  Its owning chapter records platform scope and public-preset discovery. Story
  implementations, play assertions, review tags, thresholds and production
  contracts remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.4 retains its current status. Baseline generation is separate from
  human visual approval, inherited story failures and the existing Design Core
  registry release gap.

```
<!-- /evidence-field -->

