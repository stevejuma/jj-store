---
schema: 1
id: E-CI-AFFECTED-001
---

# E-CI-AFFECTED-001 details 3

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
pnpm format:rust:check
```

**Notes**

Both Rust workspaces passed.
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
pnpm exec prettier --check .
```

**Notes**

Caught new selection test formatting; formatted with the CI-matched Node 22 runtime before final recheck.
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
pnpm release:check && actionlint
```

**Notes**

35 regressions, all 20 Cargo metadata checks and workflow lint passed after adding fail-closed missing plan outputs and read-only Actions permission on the publication caller.
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
pnpm exec prettier --check .
```

**Notes**

Complete formatter gate passed after formatting the new test.
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
pnpm check:progress && pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Final evidence checks passed: 155 tasks, 811 records, 3868 chapters and 198 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted required check and cache save/restore after this slice; manual full
  qualification after workflow is on default branch. Existing heavy shard
  failure remains unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 22.23.3; pnpm 10.32.1; Rust 1.99.0; Nextest 0.9.148; actionlint 1.7.12; JJ 0.45.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Archive inventory sidecar version 1 pins SHA, digest, package set, test identities and exact partitions. No public Rust API or persisted product format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Verify hosted standard checks and a repeated PR cache restore, then run the bounded sccache trial. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 requires conservative Cargo impact, exact inventory and
  scoped dependency-cache writes. Product/storage authority and bot exemptions
  remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; complete manual, native, storage, browser and
  release acceptance remain unqualified.

```
<!-- /evidence-field -->

