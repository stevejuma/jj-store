# CI validation plan delivery status

This status accompanies the complete
[CI validation implementation plan](ci-validation-plan-2026-11-02.md) in the
final stacked pull request. The plan is preserved from the 2 November 2026
source attachment; this page records what the stack delivers and what remains.
Canonical specifications and evidence remain authoritative.

## Rollout slices

| Plan slice                                                | Status in this stack     | Delivered and pending work                                                                                                                                                                                                                                                                                                                                           |
| --------------------------------------------------------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0. Stabilize the starting revision and environment        | Partial                  | The stack retains the existing repairs and quarantine identities and adds earlier source and evidence checks. A complete hosted scheduled run, native Git installation/cache qualification, archived-consumer equivalence, and the original Help/clipboard/browser lifecycle failures remain pending.                                                                |
| 1. Make evidence complete                                 | Delivered                | Ordinary lanes upload source-bound JUnit, inventories, commands, and outcomes. Aggregation rejects missing, duplicate, foreign, canceled, incomplete, and empty evidence; ordinary archive compression is removed.                                                                                                                                                   |
| 2. Improve the DAG and setup with unchanged selection     | Partial                  | Independent work starts in parallel, source Storybook no longer requires a built payload, and static catalog and app/Help producers are distinct. Capability-minimal setup and a measured same-source cold/warm latency comparison remain pending.                                                                                                                   |
| 3. Establish scheduled full coverage                      | Partial                  | The heavy coordinator has a canonical-main schedule, independent producers, freshness health, and advisory quarantine repetition. One complete fresh scheduled run on the integrated baseline is still required before coverage can move out of the pull-request path.                                                                                               |
| 4. Introduce domain selection and targeted backend checks | Delivered in shadow mode | Selection schema version 1 records UI, browser, Help, catalog, contract, and backend domains with safe broad fallback. Affected `jj-store`, `jj-store-server`, and `jj-store-capture` packages run as a blocking two-shard targeted cohort. UI domains remain observational until representative hosted samples and scheduled coverage support a separate promotion. |
| 5. Repair the remaining failure mechanisms                | Pending                  | Diagnose and repair Help request lifecycle, clipboard feedback, Storybook resources, history readiness, and the registered geometry and palette/workspace quarantines while retaining their original assertions.                                                                                                                                                     |
| 6. Optimize the remaining measured bottleneck             | Pending                  | Measure 1/2/4 Rust shards, cache behavior, compilation overlap, runner/profile choices, and browser lifetime before selecting an optimization.                                                                                                                                                                                                                       |
| 7. Verify qualification and document the operating model  | Partial                  | The plan, contracts, evidence, and PR operating model are documented. Exact release-candidate qualification, consumer-rerun identity checks on hosted infrastructure, the remaining platform/provider/visual gate inventory, and the representative latency/reliability review remain pending.                                                                       |

## Acceptance status

- [x] Source Storybook runs independently of a built app artifact.
- [x] Ordinary Rust and UI lanes retain source-bound JUnit and execution
      evidence, and required aggregation rejects missing or incomplete results.
- [x] Reverse dependency and source-edge protection remains enabled.
- [x] Affected backend packages receive blocking targeted heavy-profile coverage
      while complete heavy qualification remains separate.
- [x] Shared, generated, unknown, and explicit full-check inputs fail closed to
      broad selection.
- [x] Missing, canceled, failed, duplicated, foreign, and unexpectedly skipped
      selected results fail the stable required aggregate.
- [ ] Demonstrate a complete fresh canonical-main scheduled run on the
      integrated stack.
- [ ] Demonstrate capability-minimal setup and measured cold/warm PR latency on
      hosted runners.
- [ ] Repair the remaining product and harness failure mechanisms and satisfy
      every quarantine re-enable gate.
- [ ] Promote narrower UI selection only through a separate reviewed change
      backed by scheduled coverage and representative shadow observations.
- [ ] Evaluate measured shard/cache/compile/browser bottlenecks without reducing
      coverage or first-attempt reliability.
- [ ] Exercise the exact release candidate's complete qualification plan and
      review representative latency and reliability data.

## Evidence

- `E-CI-LATENCY-001` covers ordinary evidence, parallel execution, scheduled
  validation, freshness, and quarantine diagnostics.
- `E-CI-SELECTION-001` covers shadow domain planning, blocking targeted backend
  checks, safe fallback, and exact execution evidence.
- `E-CI-EVIDENCE-001` binds targeted verification to the heavy-profile JUnit
  path and proves stale ordinary-profile output cannot satisfy the gate.

The second stack's hosted selection and ordinary Rust lanes passed. Its first
targeted backend execution passed but exposed a verifier path mismatch; this
change repairs that gate without changing selection. Existing UI workflow
failures remain part of slices 0 and 5 and do not establish a failure of the new
selection or targeted-backend workflow.
