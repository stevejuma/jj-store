---
schema: 1
id: E-CI-SUITES-FIX-002
---

# E-CI-SUITES-FIX-002

<!-- evidence-field:id markdown -->
## Id

E-CI-SUITES-FIX-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- scripts/validation.mjs
- scripts/validation.test.mjs
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/27/E-CI-SUITES-FIX-002/index.md
- spec/src/history/2026/10/27/E-CI-SUITES-FIX-002/SUMMARY.nav.md
- spec/src/history/2026/10/27/index.md
- spec/src/history/2026/10/27/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_execution/context_delivery.rs#markdown_deltas_and_bound_task_cli_survive_loaded_and_fresh_sessions
- scripts/validation.test.mjs#native dispatch keeps bounded full-suite shards
  and exact explicit targets
- scripts/validation.test.mjs#default browser suites run complete bounded shards
  without changing explicit targets
<!-- /evidence-field -->

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
pnpm spec:check
```

**Notes**

Before. Docker run jj-store-validation-e5b53eea-e110-418d-a1e3-9b0c4dec4ae0 (full image, source c61065b9): the catalog-interaction step ran the unfiltered Storybook suite unsharded in one container browser session. Tests 1 failed | 958 passed (959); Test Files 1 failed | 172 passed (216); one unhandled chromium page-closed error during catalog/components/store-work/ProjectsAgentActions.stories.svelte left 43 files unreported. The single failed play (StoreWorkspaces KeyboardSearchAndRepositoryOverview) showed a stuck sb-preparing-story body, then the Storybook error boundary on retry: a degraded session, not story logic.
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
pnpm spec:check
```

**Notes**

Deterministic reproduction. Docker run jj-store-validation-1681c37a-a56c-4707-87f2-9a1a93002d13 (same source): Tests 959 passed (959), Test Files 173 passed (216), and the identical unhandled page-closed crash at the same story file again left 43 files unreported. No product test assertion failed in this run; the unhandled browser crash fails the step.
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
node scripts/validation.mjs run test:storybook -- catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte
```

**Notes**

Targeted Docker rerun of both affected files, run jj-store-validation-29c25ce4-b071-4b18-828b-8213e09103e0 (ui image): 2 files / 8 tests passed, including KeyboardSearchAndRepositoryOverview in 646ms. Prior sharded full-suite runs also passed (.local/validation/storybook-full*.log), so the stories are healthy and only the unsharded single-session nested path crashed.
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
node --test scripts/validation.test.mjs
```

**Notes**

Native macOS after the fix: 11 tests / 11 passed / 0 failed, including the new regression proving unfiltered native dispatch runs all four shards even when shard 2 fails and rethrows the first failure, while explicit targets stay a single exact invocation. Before the fix this file held 10 tests with no nested-dispatch coverage.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Docker run jj-store-validation-5e902fe4-6564-482c-90c1-c51178ec2f9f with the fix in the tree: catalog-interaction passed as four fresh-browser shards (54 files each; 275+284+238+251 = 1048 tests passed, 0 failed); catalog-build, browser-check, browser-unit, browser-interaction, contracts, rustfmt and boundaries all passed. The run stopped at the progress step with five findings because the protected changes (docs/maintenance/coverage.json, scripts/validation.mjs, scripts/validation.test.mjs) require the same-change ledger updates and fresh evidence entries that this record and E-VAULT-BACKUP-023 add.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Docker run jj-store-validation-89091699-11e7-43ef-a41d-d43e753aa23a on the complete slice: every step through storage-harness passed — including catalog-interaction as four fresh-browser shards (1048 tests) and the progress step over the fresh records — and the final workspace Rust suite then failed in the container: crates/jj-store-cli/tests/agent_execution/context_delivery.rs panicked with Os code 18 CrossesDevices on fs::hard_link of CARGO_BIN_EXE_jj-store-jj into the tempdir fixture, because the Docker runtime mounts target/ on a cache volume while tempdir scratch lives on the overlay filesystem. agent_execution target: 21 passed / 1 failed / 2 ignored.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run validation:rust -- test --locked -p jj-store --test agent_execution
```

**Notes**

The first exact Docker rerun never reached cargo: the managed-cargo storage preflight failed closed (run jj-store-validation-3b46aa8d-29a9-47f8-bafa-f4e8de60589f) because the shared target cache volume had grown to 49.2 GiB after spec:check's workspace-test compilation, over its 40 GiB per-target cap with 1780 GiB free. Resolved through runner-owned maintenance without touching the policy: pnpm validate:clean -- --apply removed stopped run containers with reports retained, and docker volume rm reset the rebuildable target volume while keeping the warm pnpm/cargo-registry/uv caches.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run validation:rust -- test --locked -p jj-store --test agent_execution
```

**Notes**

Exact Docker rerun of the complete agent_execution target (run jj-store-validation-64e213bc-1d2e-4860-bc43-dd1b0b835d48): the repaired context_delivery::markdown_deltas_and_bound_task_cli_survive_loaded_and_fresh_sessions PASSED in the same cross-device container layout, proving the hard-link-then-copy fallback (rustfmt --edition 2024 --check clean; no assertion changed). 21 passed / 1 failed / 2 ignored, with the failure now in metadata_tests::recovery::metadata_stale_generation_and_unknown_publication_recover_without_new_inference (metadata_recovery.rs:99 Stale assertion), which had passed in the pre-repair spec:check rust-tests tree.
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
node scripts/validation.mjs run validation:rust -- test --locked -p jj-store --test agent_execution metadata_tests::recovery
```

**Notes**

Isolated Docker repro (run jj-store-validation-20584db0-a971-4dfd-afed-507e6e9001d6): 2 passed / 0 failed in 13.87s. A second complete-binary rerun (jj-store-validation-b6f8baf3-be4b-459f-bd76-4bfa14e04480) again showed 21 passed / 1 failed / 2 ignored at the same assertion: an intermittent load-sensitive pre-existing race in the agent-execution metadata family (E-CHAT-METADATA lineage; fresh local-handle metadata selection against the just-published generation). Fixtures are per-test tempdirs and neither the jj-store-git agent_store nor the jj-store-service selection path is touched by this change, so the flake is owned and reported by that family separately rather than masked here.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Docker rerun on the complete slice, run jj-store-validation-7dea0fdf-c37e-4570-b671-8862c502ed72: every step through storage-harness passed — including catalog-interaction as four fresh-browser shards (54 files each; 275+284+238+251 = 1048 tests, 0 failed), catalog-unit 1208 passed | 1 retained skip, progress over the fresh records, governance-tests and contracts — proving both repairs. The final rust-tests step ran the workspace suite: agent_execution passed completely (22 passed / 0 failed / 2 ignored; the repaired context_delivery and the intermittent metadata_recovery both ok), then executable_native failed 14 passed / 8 failed / 3 ignored with all eight native_git_* failures at the product capability gate NATIVE_GIT_CAPABILITY requires Git 2.55.0 because the validation image ships Git 2.43.0. That family is macOS-qualified per repository docs (M12 GB6 on macOS ARM64 with Git 2.55.0; Linux tests cannot qualify native acceptance), so the gap is pre-existing, environmental and separately owned by the validation image toolchain; no test or gate was weakened.
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

Native host macOS ARM64 (node v24.10.0, pnpm 10.32.1, jj 0.45.1, Rust 1.99.0, uv 0.10.0); remote omarchy Linux x86_64 Docker daemon (16 CPUs/62 GiB), lane resources 8 CPUs/24 GiB/2 GiB shm, ssh-cli transport; full image sha256:486d1bdc34eb0f4799bcd2d09ff726e78d737e2991cabf79212dd7029ed60b2e (jj-store-validation:full-585ee5c4eeb3d3866f6b), ui image sha256:9889f7bb0e260a76592ceabb5410c53fe723a097197e616b7929bf3c3d33b174 (jj-store-validation:ui-5f91ab85f47886aab9df). Source: working-copy revision c61065b9 (integrated 52b061c5 plus the same-change repairs); later lane runs record their own snapshot revisions.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored-format, DTO, report-schema or public API changes. Lane commands, worker JUnit reporting, direct Docker lane sharding and remote run identity are unchanged; only native/nested dispatch gains the documented bounded invocations. The test-harness repair changes no product source or assertion.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Two aggregate-lane repairs for the documented Docker validation runtime. (1) Native and nested worker lane dispatch in scripts/validation.mjs now applies the documented laneInvocations discipline: the unfiltered Storybook suite inside aggregate lanes (spec:check catalog-interaction, test) runs four sequential shards with fresh browser processes, every shard runs even when an earlier one fails, the first failure is rethrown, and explicit targets or caller-selected shards remain exact single invocations. (2) The agent_execution context-delivery fixture stages its bundled binary through a hard-link-then-copy fallback so the workspace Rust suite passes when target/ and the tempdir scratch live on different devices, as in the container cache-volume layout; the quoted-path export and executability assertions are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the aggregate Docker lanes as the integrated-slice gate. Separately owned follow-ups outside this repair: provisioning Git 2.55.0 in the validation images (or platform-scoping the macOS-qualified native_git family) and the intermittent load-sensitive metadata_recovery race in the agent-execution metadata family.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "docs/validation.md and JST-ARCH-005 already require the bounded
  four-shard fresh-browser unfiltered Storybook suite with every shard running
  despite earlier failures; the dispatch repair restores that documented
  contract for nested/native lanes. JST-CLI-004 executable export and
  quoted-path behavior is unchanged: the fixture still stages the real paired
  binary at a spaced bundled path and asserts the exact exported environment and
  executability. Release qualification, publication authority, receipts,
  coverage semantics and hosted CI selection are untouched."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress at its existing release gates; local
  validation-runner maintenance promotes no hosted qualification, publication or
  platform gate.

```
<!-- /evidence-field -->

