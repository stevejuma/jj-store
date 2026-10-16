---
schema: 1
id: E-PR10-AUTOCOMPLETE-001
---

# E-PR10-AUTOCOMPLETE-001 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base @- --head @
```

**Notes**

autocomplete-spec-first-parent-100: exit 2, 5.3 seconds. 
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base aad2ad7f45f0bfdabfbb5617c77e00117a303d3d --head @
```

**Notes**

autocomplete-spec-first-pr-100: exit 2, 5.8 seconds. 
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
pnpm exec prettier --write package.json pnpm-lock.yaml spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/progress/M28/index.md spec/src/current/progress/M30/index.md spec/src/history/2026/10/16/E-PR10-AUTOCOMPLETE-001/*.md spec/src/history/2026/10/16/index.md spec/src/history/2026/10/16/SUMMARY.nav.md
```

**Notes**

autocomplete-doc-format-100: exit 0, 0.48 seconds. 
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
pnpm spec:validate
```

**Notes**

autocomplete-spec-validate-100: exit 0, 3.14 seconds. 
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
pnpm check:progress
```

**Notes**

autocomplete-progress-100: exit 0, 5.17 seconds. 
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
pnpm spec:first --base @- --head @
```

**Notes**

autocomplete-spec-first-parent-100: exit 0, 5.73 seconds. 
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
pnpm spec:first --base aad2ad7f45f0bfdabfbb5617c77e00117a303d3d --head @
```

**Notes**

autocomplete-spec-first-pr-100: exit 0, 5.95 seconds. 
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm evidence:new -- --id E-PR10-AUTOCOMPLETE-001 --task M28 --task M30 --requirement JST-UI-001 --requirement JST-ARCH-005 --requirement JST-GOV-003 --requirement JST-GOV-004 --requirement JST-GOV-011
```

**Notes**

Scaffold created. Restored 452 incidental unrelated navigation rewrites; retained new evidence and its day navigation only.
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

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Vitest 4.1.11; Design Core released registry 0.5.1. Upstream source 1ef5bd72, merged source 41efbc9b and version 9560a62d. Source and merge Git tree hashes are identical.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only the consumer dependency minimum and registry lock resolution change. No native identity, storage, API, credential, transport, draft, command or preference format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed local source-owner, packed-consumer and released-registry dependency qualification for the PR #10 workflow failure. This record covers the finished original regression, source/consumer units and types, exact packed Storybook inventory, registry integrity/provenance, fresh application/catalog builds and cold/served browser acceptance. Follow-up full released-package Storybook and fresh hosted required checks are outside this completed local slice and remain the next action; their results must be established independently. Repair the failure at its source owner. Design Core deferred input.select() could run after the first input or caret edit and overwrite it; a selection generation now invalidates stale opening selection while retaining selection of untouched initial values. Two deterministic requestAnimationFrame interleaving regressions reproduced the text and caret failures before repair and pass after it. Five original autocomplete cases passed before; all 11 affected autocomplete/chip stories pass after. The full source-owner aggregate and both protected source/version CI gates pass. The initial aggregate exposed a new regression callback return type error; it was corrected and the complete replacement aggregate passed, with the failed attempt preserved. User explicitly authorized source/version landing and public 0.5.1 publication. Source PR #29 and generated Version Packages PR #30 each passed all 11 protected checks before squash merge, without bypass. The bot-origin version workflow was approved through GitHub after reviewing its exact diff; approval did not bypass test or publication gates. Existing trusted publishing and verification workflows retain their controls. The packed consumer uses an ignored disposable source copy only; active manifests and lockfiles consume the released registry version. Packed tests prove the exact hosted 1,022-case/211-file inventory once each without failure or skip, plus 180 web and 1,193 catalog units, both type checks and 10 unchanged workflow stories. Released-package acceptance checks the original workflow scenario, direct consumers, fresh built app/catalog, and cold browser behavior. No source aliases, node_modules patches, consumer workaround, changed workflow assertions, deadlines, retries, accessibility rules, visual baselines or native rebuild were introduced. The evidence scaffold rewrote unrelated generated navigation; those incidental rewrites were immediately restored before any other tracked edit. All earlier committed evidence remains immutable. Initial evidence authoring mixed completed local proof with separate qualification gates. The rejected pass-with-gaps, unsupported partial label and pending unchanged-contract confirmation are retained in the actual command history. The final record follows the repository completed-slice format: this local qualification passes, while next action and milestone review retain fresh hosted/full released-Storybook qualification, the prior unconfirmed intermittent catalog fetch rejection, heavy release, live providers, other platforms and visual approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit and push the verified registry dependency repair to the existing PR #10 bookmark, continue complete released-package local Storybook checks, and observe every required hosted check through completion. Fix actual remaining failures; do not merge jj-store PR #10 or dispatch heavy qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-001 retains independent registry-backed catalog builds, isolated
  deterministic fixtures and blocking interaction/accessibility checks. This
  application changes only its released Design Core minimum; original workflow
  stories and assertions are unchanged. Source-first repair preserves public
  component behavior and APIs. JST-ARCH-005 retains complete protected
  source/version/publication gates and fresh jj-store required CI; upstream
  release authorization does not authorize a jj-store release.
  JST-GOV-003/004/011 retain fresh immutable evidence and explicit milestone
  boundaries. Source, packed, released-package and hosted proofs are recorded
  separately. Native code and contracts are unchanged, so repeating Rust/storage
  qualification is out of scope for this UI dependency slice; earlier completed
  local and current hosted native results remain independently identified.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Explicit M28 and M30 review retains In progress. Original workflow
  draft/runner-label/Form-YAML contracts and all existing assertions remain
  intact. Complete upstream and consumer local proof qualifies this dependency
  repair only. Current-head standard CI, heavy hosted release, live
  agents/permissions, remote Forgejo protocol, other platforms and approved
  visual baselines remain separate.

```
<!-- /evidence-field -->

