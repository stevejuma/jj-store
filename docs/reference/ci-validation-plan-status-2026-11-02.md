# CI validation plan delivery status

This status accompanies the complete
[CI validation implementation plan](ci-validation-plan-2026-11-02.md) in the
final stacked pull request. The plan is preserved from the 2 November 2026
source attachment; this page records what the stack delivers and what remains.
Canonical specifications and evidence remain authoritative.

## Rollout slices

| Plan slice                                                | Status in this stack | Delivered and pending work                                                                                                                                                                                                                                                                                                                   |
| --------------------------------------------------------- | -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0. Stabilize the starting revision and environment        | Partial              | Exact-source manual complete qualification now passes the integrated stack, including native, storage, UI and cold lanes. A canonical schedule-event run, archived-consumer rerun equivalence and four explicit quarantine re-enable gates remain pending.                                                                                   |
| 1. Make evidence complete                                 | Delivered            | Ordinary lanes upload source-bound JUnit, inventories, commands, and outcomes. Aggregation rejects missing, duplicate, foreign, canceled, incomplete, and empty evidence; ordinary archive compression is removed.                                                                                                                           |
| 2. Improve the DAG and setup with unchanged selection     | Partial              | Independent work starts in parallel, source Storybook no longer requires a built payload, and static catalog and app/Help producers are distinct. Formatting, tooling, governance, archive and Node/browser jobs now request explicit minimal capabilities. A measured same-source cold/warm latency comparison remains pending.             |
| 3. Establish scheduled full coverage                      | Partial              | Manual complete run 41681207161 passes at exact canonical-main source. The coordinator schedule, freshness health and advisory quarantine repetition are present, but one actual schedule-event run is still required before scheduled freshness is established.                                                                             |
| 4. Introduce domain selection and targeted backend checks | Delivered and active | Selection schema version 2 actively selects UI, browser, Help, catalog, contract, and backend domains with safe broad fallback. Affected `jj-store`, `jj-store-server`, and `jj-store-capture` packages run as a blocking two-shard targeted cohort. Explicit `all_checks` and heavy workflow dispatch retain complete manual qualification. |
| 5. Repair the remaining failure mechanisms                | Partial              | Clipboard, Storybook lifetime, history readiness and integrated cold failures pass exact-source complete qualification. Q-STORY-001, Q-STORY-002, Q-BROWSER-001 and Q-RUST-002 retain their independent original assertions and re-enable gates.                                                                                             |
| 6. Optimize the remaining measured bottleneck             | Partial              | The four-partition targeted trial retained two shards and Storybook browser lifetime is bounded. Representative cold/warm category samples, cache/compile comparison and the final latency/reliability review remain pending.                                                                                                                |
| 7. Verify qualification and document the operating model  | Partial              | Exact-source manual complete qualification passes and is documented. Scheduled freshness, consumer-rerun identity on hosted infrastructure, four quarantines, the remaining platform/provider/visual gate inventory and representative latency/reliability review remain pending.                                                            |

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
- [x] Demonstrate capability-minimal setup through executable workflow
      contracts.
- [ ] Measure cold/warm PR latency on hosted runners.
- [ ] Repair the remaining product and harness failure mechanisms and satisfy
      every quarantine re-enable gate.
- [x] Promote narrower UI selection through a separate reviewed change with
      fail-closed fallback and complete manual qualification retained. This is
      explicitly authorized before the outstanding hosted heavy run.
- [ ] Evaluate measured shard/cache/compile/browser bottlenecks without reducing
      coverage or first-attempt reliability.
- [x] Exercise the integrated exact-source candidate through the complete manual
      qualification plan.
- [ ] Review representative latency and first-attempt reliability data.

## Evidence

- `E-CI-LATENCY-001` covers ordinary evidence, parallel execution, scheduled
  validation, freshness, and quarantine diagnostics.
- `E-CI-SELECTION-001` covers shadow domain planning, blocking targeted backend
  checks, safe fallback, and exact execution evidence.
- `E-CI-EVIDENCE-001` binds targeted verification to the heavy-profile JUnit
  path and proves stale ordinary-profile output cannot satisfy the gate.
- `E-CI-ACTIVE-SELECTION-001` promotes domain selection for pull requests while
  retaining complete standard and heavy manual workflows.
- `E-CI-SETUP-001` assigns least-capability setup profiles to the always-running
  policy jobs without changing selected tests.
- `E-CI-TARGETED-SHARDS-001` records the measured four-partition latency trial
  and retains two shards because overall wall time did not improve.
- `E-CI-QUALIFICATION-001` records the passing exact-source manual complete
  qualification and its still-independent scheduled and release gates.

Manual complete qualification run 41681207161 passed on canonical `main` at
`5c2c4c9c8c3dc77ee7ff774a0cf724f23b9d94e3`: 53 jobs succeeded, the complete gate
succeeded and only the independent advisory cache comparison skipped. The manual
dispatch does not establish schedule-event freshness or close the four
quarantines, representative latency/reliability review, publication, provider,
platform or visual gates.
