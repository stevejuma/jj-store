---
schema: 1
id: E-CI-HEAVY-001
---

# E-CI-HEAVY-001 details 3

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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Final evidence passed all four applicable validators, 3874 chapters and 198 requirements.
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
pnpm spec:first
```

**Notes**

One protected source file and one mapped canonical chapter passed with a qualified unchanged-contract confirmation.
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
gh run cancel 38714895280 --repo stevejuma/jj-store
```

**Notes**

Cancel request accepted while the standard archive build was running. All early gates and web checks had passed; no heavy build or test had started. Superseded by the focused repair, not counted as qualification.
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
pnpm exec prettier --check spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/progress/M30/index.md spec/src/current/progress/M13/index.md spec/src/history/2026/10/15/E-CI-HEAVY-001 spec/src/history/2026/10/15/index.md spec/src/history/2026/10/15/SUMMARY.nav.md
```

**Notes**

All modified Markdown and generated evidence/navigation passed formatting.
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

Local macOS ARM64; Rust 1.99.0, JJ 0.45.1, Node 22.23.3, pnpm 10.32.1. Hosted Ubuntu 22.04 uses Nextest 0.9.148.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only canonical Agent domain descriptor version 2 and schema-2 append-tail transcript pages are supported. Obsolete version-1 descriptors remain rejected without upgrade or rewriting. No persistence, public API, archive, test-cache or release format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Verify and submit the focused descriptor regression repair through a separate PR. Diagnose remaining heavy failures one lane at a time. Performance improvements are deferred; automatic release remains disabled.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-LOCAL-001/002/003 already require current-only decoding and
  immutable selected/pinned state; E-M17-034 intentionally removed V1 upgrade.
  Update the stale test to enforce rejection and no mutation while preserving
  supported historical reads. JST-ARCH-005 still separates the standard merge
  gate from complete qualification; wording clarifications do not change gates.
  JST-CLI-004 command and executable custody are unchanged: tests use explicit
  workspace-owned binaries and the already registered managed build target,
  without replacing installed tools or restarting services."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 and M13.3 remain In progress. PR merge and cache reuse are verified,
  but full qualification and publication gaps remain. No skipped, canceled or
  unrun lane is promoted.

```
<!-- /evidence-field -->

<!-- evidence-field:qualification_status markdown -->
## Qualification status

Full hosted qualification is not covered by this focused pass. Run 38714895280 tested merged main with the old assertion and was canceled before heavy compilation after local reproduction. A fresh complete qualification will use the follow-up PR revision. The prior heavy failure remains unresolved on main until that PR lands; no heavy, native, storage, browser or publication gate is promoted. The canceled sccache trial provides no benchmark result and is deferred.
<!-- /evidence-field -->

