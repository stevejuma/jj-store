---
schema: 1
id: E-WORKFLOW-RUN-001
---

# E-WORKFLOW-RUN-001 details 3

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
pnpm check:progress; pnpm spec:validate
```

**Notes**

Initial progress and 7-validator spec checks pass with new evidence. Final checks follow after CLI ownership review.
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
pnpm spec:first; pnpm check:progress; node scripts/docs.mjs review; pnpm exec prettier --check [changed authored files]
```

**Notes**

Final spec-first 13 protected files/5 chapters, progress 781 evidence records, documentation fingerprints and authored formatting pass. Root aggregate suites and independent Rust lanes out of scope.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

A subsequent evidence update added details-03 without refreshing its SUMMARY navigation. Fixed scoped history navigation; validation repeated below.
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

Final 7-validator specification check passes after regenerating scoped evidence navigation.
<!-- /run -->
<!-- /evidence-field -->

