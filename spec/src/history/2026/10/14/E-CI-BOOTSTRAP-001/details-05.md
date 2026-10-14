---
schema: 1
id: E-CI-BOOTSTRAP-001
---

# E-CI-BOOTSTRAP-001 details 5

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
node scripts/ci-setup-jj.mjs with isolated RUNNER_TEMP and GITHUB_PATH
```

**Notes**

Actual upstream release downloaded, matched its pinned SHA-256, extracted and verified its exact version/source. Only disposable tool paths were used and then removed.
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
pnpm check:progress
```

**Notes**

155 tasks and 801 original/new evidence records passed integrity and same-change coverage.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Four static validators passed, including complete bounded evidence navigation.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj -p jj-store-service --all-targets --locked --target-dir .local/build-targets/public-release -- -D warnings
```

**Notes**

Complete all-target strict lint passed for both direct Unix-stat consumers after normalizing every matching cast. No lint allowances were introduced.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test working_tree_inventory --test agent_workspace_files --locked --target-dir .local/build-targets/public-release -- --test-threads=1
```

**Notes**

Twelve regressions passed across both filesystem consumers: two workspace tests and ten inventory/guard tests.
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
gh api --method PUT repos/stevejuma/jj-store/branches/main/protection --input /tmp/jj-store-main-protection.json
```

**Notes**

PRs, strict required checks, conversation resolution, admin enforcement and no force-push/deletion are active. Required status was subsequently bound to GitHub Actions app 15368 and read back.
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
gh api repos/stevejuma/jj-store/rulesets --method POST --input /dev/stdin
```

**Notes**

Active immutable version tag update/deletion ruleset 25110878 applied to refs/tags/v*, with no bypass actors.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --test working_tree_current --locked --target-dir .local/build-targets/public-release -- --test-threads=1
```

**Notes**

All nine current-read and identity tests passed again after the full matching stat-cast normalization.
<!-- /run -->

<!-- run -->
### Run 9

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

All 115 protected paths and 15 canonical chapters passed, including qualified formatting-only confirmations; no protected-path exemption was added.
<!-- /run -->

<!-- run -->
### Run 10

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

Final 155-task, 801-record integrity and fresh owner coverage passed.
<!-- /run -->

<!-- run -->
### Run 11

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

Final four validators passed with bounded evidence navigation and 198 requirements.
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

Rust 1.99.0; upstream JJ 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; checksum-pinned four-platform tool archives; pnpm 10.32.1; GitHub Actions app 15368.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identity, manifest/WAL and local checkout stamp bytes are unchanged. Source formatting changes no public APIs or runtime state. GitHub protection is repository policy, not repository-state authority.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair the initial hosted CI/bootstrap failures and add the user-requested PR-only main protection. Required GitHub Actions check must pass on an up-to-date base; admins cannot bypass PRs/checks, conversations must resolve, force-push/deletion are blocked, and v* tags are immutable. No additional approving-review count is imposed on this solo-maintainer repository. Shared CI setup justified the complete affected tooling suites; native validation targets the stat/read boundary. Hosted full CI is the wider qualification gate; no product-wide local aggregate was run. Native PR CI now verifies the compiler triple and reuses the debug target across its build and tests; optimized archive builds stay release-only. Test/rule conditions and archive partitioning remain intact.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit and push the locally verified repair as a PR; hosted CI and Pages checks remain required before merge. Automatic release orchestration remains disabled.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-ARCH-005 gains enforced PR/check/tag policy and CI prerequisites.
  JST-LOCAL-004/005 behavior and binary stat fields are explicitly unchanged:
  lossless i128 widening retains original signed/unsigned u64 representations.
  Formatting does not alter current requirement/status/evidence payloads;
  immutable historical records remain untouched."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local repairs do not complete hosted CI,
  native/container or external registry qualification. Changesets orchestration
  stays disabled while the initial release remains unqualified.

```
<!-- /evidence-field -->

