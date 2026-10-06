---
schema: 1
id: E-M21-054
---

# E-M21-054 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
detail: "Final completed evidence: 5 validators/23 chapters/173 requirements;
  spec-evidence-complete.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
detail: "Final completed evidence: 142 tasks; progress-evidence-complete.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
detail: Final completed source/ledger formatting passes; format-evidence-check.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <23 changed files>
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-resolved foundational package acceptance, screenshot
  comparisons/baselines and broader local-browser pressure/release qualification
  remain separately deferred; this UI-only slice runs no Rust or contracts lane.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the local preview services available. Qualify a published Design Core release and registry-resolved consumer separately; screenshot comparisons/baselines and broader release/pressure gates remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Conflict markers use unresolved target-side terms and retain the warning surface during selection/hover. The rail and toast share a capability/lock/owner-guarded editor, and resolved conditions withdraw their notice. The optional secondary toast action is fixed upstream, leaving existing ghost actions unchanged. Banner and unavailable diff explanations no longer spend an extra title row. Final source nonvisual and consumer focused checks pass; the initial readiness, harness, specification and formatting failures are resolved and retained in the evidence.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

UI-only: exact conflict regressions, direct application/lazy/diff/notification consumers and focused controller/lifecycle/notification units, web/catalog types, ownership, architecture, documentation and spec/progress checks. Design Core owns the generic optional presentation contract and runs its required nonvisual aggregate. Root aggregates, Rust builds/tests, contract generation, screenshot comparisons, baselines and package publication are out of scope.
<!-- /evidence-field -->

