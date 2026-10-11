---
schema: 1
id: E-UI-UNAVAILABLE-001
---

# E-UI-UNAVAILABLE-001 details 3

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
pnpm check:progress; pnpm spec:first (final task coverage)
```

**Notes**

Both command invocations remain blocked solely by five inherited
E-NPM-REGISTRY-001 diagnostics: missing deleted hook bookkeeping, invalid test
path/reference, invalid run result and invalid entry result. No task-specific
fresh evidence or chapter error remains. Logs
/tmp/jj-unavailable-progress-final.log and
/tmp/jj-unavailable-spec-first-final.log.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits (final)
```

**Notes**

Three document consumers pass. M24 is 400 lines; no immutable evidence payload
changed. Log /tmp/jj-unavailable-docs-final.log.
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

Design Core 0.3.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; pnpm 10.32.1;
macOS ARM64. Node 26.9.0; JJ 0.45.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->

## Formats

Svelte 5 components; published Design Core shadcn Empty exports; existing API
schemas unchanged; Markdown evidence schema 1.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->

## Next

Preserve independent M24 qualification gates and visual-pending baselines.
Package-adoption owner must correct E-NPM-REGISTRY-001 before global
progress/spec-first qualification can pass.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->

## Contract review

```yaml
status: changed
reason:
  JST-UI-019 canonical notification chapter specifies icon-bearing unavailable
  content; JST-UI-005/015 retain loading and owned presentation. JST-API-001
  unchanged confirmation is recorded in the mapped API chapter. No Community
  package dependency or transport generation.
```

<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->

## Milestone review

```yaml
status: unchanged
reason:
  M24 remains In progress. This slice qualifies the six unavailable-body states
  and 18 selected direct consumer stories, not real adapter,
  permission/cancellation, native/local publication, desktop/platform or cold
  production gates. Visual baselines remain unapproved; inherited
  package-adoption evidence validation is separate.
```

<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->

## Excluded scope

Rust/storage/API generation, root aggregate check/test/build/spec:check, full
catalog/repository suites and visual baseline updates were not run under the
UI-only targeted policy.
<!-- /evidence-field -->
