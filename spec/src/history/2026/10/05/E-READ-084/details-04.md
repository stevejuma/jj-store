---
schema: 1
id: E-READ-084
---

# E-READ-084 details 4

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
pnpm exec spec-validator first
```

**Notes**

Final: 19 protected files mapped to six owning canonical chapters.
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
pnpm exec prettier --check spec/src/milestones.md spec/src/cli.md spec/src/local-jj.md
```

**Notes**

Final canonical formatting passed; formatter output was checked to preserve all preceding ledger text byte-for-byte.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Package Clippy remains red with 17 reproduced pre-existing diagnostics in
  untouched modules; not fixed or suppressed by this slice.
- Aggregate/root, full scale/RSS, RustFS, release and visual-baseline
  qualification remain outside this targeted slice; focused passes are not
  full-suite qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified narrow behavior slice with Jujutsu and retain the temporary workspace. Preview serves this slice at 7470; main and installed executables remain unchanged. Address existing package lint separately if requested.
<!-- /evidence-field -->

