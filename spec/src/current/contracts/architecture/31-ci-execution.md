# CI execution and scheduled validation

E-CI-LATENCY-001 changes execution ordering and scheduled coverage under
JST-ARCH-005 without reducing ordinary selection. Source Storybook requires
same-run source admission and no payload producer. App/Help and static catalog
require their distinct successful producers and complete source/attempt-bound
receipts. All seven UI jobs remain mandatory exactly once. Ordinary shard
reports and commands MUST prove complete disjoint execution before `required`
passes. Missing, failed, canceled and incomplete evidence MUST fail.

Scheduled production MUST be canonical repository/main in `ci-heavy.yml`, with
source matching the scheduled run SHA. Same-run consumers MAY proceed before the
standard gate; external reuse MUST retain that gate. Complete qualification MUST
retain every standard and heavy lane. Independent freshness health and advisory
three-repetition quarantine diagnostics MUST remain distinct from qualification
and publication. Narrower PR selection remains disabled until full hosted
coverage is established. No latency or reliability improvement is claimed from
local tests alone.

E-CI-SELECTION-001 adds selection schema version 1 in shadow mode. The plan
records source UI, browser, Help, catalog and contract domains, selected package
families, explicit deferred domains and selection reasons. Existing broad Rust
and UI lanes continue to use their prior `rust` and `web` decisions. Unknown,
shared, generated and explicit full-check inputs fail closed to every domain and
all backend packages.

Changes that reach `jj-store`, `jj-store-server` or `jj-store-capture` through
the complete reverse dependency graph MUST run the affected backend packages as
a blocking two-shard `targeted` cohort using the heavy test profile. Its
archive, package inventory, source, digest, partitions, JUnit identities and
successful commands MUST pass the same evidence gate as ordinary Rust. The
complete heavy cohort remains an independent scheduled and release gate. Shadow
UI domains MUST NOT narrow required PR lanes until hosted full coverage and
representative selection observations support a separate promotion.

E-CI-EVIDENCE-001 binds each cohort to the exact JUnit path owned by its
declared Nextest profile. The runner MUST remove that path before execution, and
the evidence gate MUST reject a report from another profile even when its test
identities and counts match. This repair does not change package selection,
partitions, test identities or release boundaries.

E-CI-CLIPBOARD-STABILITY-001 preserves the comparison-parent product contract
while separating two acceptance responsibilities. Source Storybook MUST retain
the transient copied-state and exact-write assertions. Cold application browser
acceptance MUST observe the actual clipboard value across live-refresh remounts
instead of requiring the transient component instance to survive. No retry,
selection, identity or application behavior changes.

E-CI-ACTIVE-SELECTION-001 promotes selection schema version 2 to active
pull-request behavior by explicit user authorization. The stable `required` gate
MUST validate the planned source UI, application browser, Help, catalog and
contract booleans and MUST accept a skipped lane only when its exact domain is
unselected. Catalog-only changes run catalog units and browser acceptance;
Storybook configuration runs source Storybook and catalog acceptance; Help,
application browser and contract checks remain independently selectable.

Shared, generated, unknown and explicit `all_checks` inputs still select every
domain and all eligible packages. `workflow_dispatch` with `all_checks=true` and
the `ci-heavy.yml` coordinator retain every standard UI/Rust lane and the
complete heavy qualification surface for manual or scheduled execution. Focused
PR success is not complete qualification, and activating selection does not
claim a latency or reliability result. The previously required fresh hosted
heavy run is now a subsequent qualification action rather than a prerequisite
for smaller PR checks.

E-CI-STORYBOOK-LIFETIME-001 retains the three required source Storybook jobs
while dividing each job's declared shard into four disjoint twelve-way segments.
Each segment MUST start a fresh browser process, run without retries and emit a
distinct JUnit report. Across the three jobs the twelve segments MUST cover the
source Storybook inventory exactly once; a failed segment MUST fail its owning
job without replaying that segment or starting later work as a retry.
