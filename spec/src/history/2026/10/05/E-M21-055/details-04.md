---
schema: 1
id: E-M21-055
---

# E-M21-055 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
detail: Initial evidence entry passes 142-task bookkeeping; progress.log. 593
  historical entries preserved exactly as data; final evidence recheck follows.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
detail: Focused final formatting succeeds; source files unchanged by the final
  formatter. format-final.log/rust-format.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write <37 changed non-Rust files>; cargo fmt -p jj-store-jj -- --check
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
detail: Evidence status pass incorrectly listed external qualification as
  remaining slice gaps. Move these unchanged limits to scope_limits; repeated
  evidence-only check identified the same metadata issue.
  progress-final.log/progress-done.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
detail: "Final requirements: 5 validators/23 chapters/173 requirements;
  spec-final.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
detail: Completed evidence validates 142 tasks, scoped formatting, and 32
  protected files/7 mapped chapters.
  progress-verified.log/format-done.log/spec-first-done.log. All 593 historical
  entries are identical as data.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm exec prettier --check <changed files>; pnpm spec:first
```
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

<!-- evidence-field:next markdown -->
## Next

Keep the existing local preview services available. Use shared in-app Merge Editor/context controls; qualify separately deferred provider shapes, release/pressure and visual baselines only when authorized.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Stacks had discarded matching-only diff sections and replaced presentation patches with three-line native descriptors, leaving no expandable context. Full bounded patches now feed display-only partitions. The separate workspace dialog is replaced by extracted owned native conflict UI mounted beside Main; multi-sided text versions now support reviewed complete-file in-app resolution. Drafts survive file/browser switches, stale scopes cannot execute, absent versions stay disabled, and unsupported reasons remain contextual. The initial read-only version fallback did not meet the user’s Merge Editor expectation. Multi-sided text now compares each version with the reviewed resolution in the same shared editable pane; pair completion never stages the entire native value automatically. Final functional/type/native and canonical checks have no unresolved failures; unsupported shapes and wider qualification remain explicit scope limits.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Exact UI/controller/adapter and jj-lib local provider regressions; shared editor/application/lazy/repository-diff stories, cold browser/live verification, affected types/ownership/architecture/format/docs and canonical/progress checks. Root aggregates, complete Rust/UI suites, unrelated service/contract generation, screenshot comparisons, visual baselines and publication/release qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:scope_limits list -->
## Scope limits

- Structural, mode-conflicted, binary and bounded-size/version-limit native
  values retain explicit unsupported reasons. Complete native release/pressure,
  registry foundational package acceptance and screenshot/baseline qualification
  remain separate.
<!-- /evidence-field -->

