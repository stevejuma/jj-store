# Forge-agnostic stack submission

## Requirements

| ID            | Requirement                                                                                                                                                                                                                                                                                                                                 |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-FORGE-001 | Ordinary local Git-backed JJ stacks MUST submit one PR/MR per explicit bookmark segment through shared CLI/application orchestration, with GitHub, GitLab and Forgejo adapters and per-instance capabilities.                                                                                                                               |
| JST-FORGE-002 | Submission MUST pin the local operation, exact branch heads, endpoint identities and expected remote heads, push only required branches, and independently execute creation, retargeting and managed-description steps. Shared dependencies MUST be deduplicated; ambiguous topology, conflicts and private commits MUST block publication. |
| JST-FORGE-003 | External effects MUST retain request identity, exact intent and per-step outcomes in private device state. Unknown outcomes MUST be observed before another attempt and MUST NOT be automatically replayed. These records MUST NOT become code authority.                                                                                   |
| JST-FORGE-004 | PR titles and authored descriptions MUST be editable. Managed navigation MUST be bounded by owned markers and preserve authored text. Status/link maintenance MUST follow observed remote lifecycle changes, retaining drafts and explaining stale observations.                                                                            |
| JST-FORGE-005 | Checks, runs, jobs, logs and permitted rerun/cancel controls MUST be capability-gated and associated with the selected remote revision. Merge MUST respect provider protection, expected-head guards and dependency order; queued/accepted MUST remain distinct from merged.                                                                |
| JST-FORGE-006 | Local rewrites, pushes and app-initiated base changes after landing MUST require explicit Restack. Fork-only parent branches MUST use labelled cumulative upstream PRs with bottom-first merge order. Eligible GitHub native stacks MAY use native membership and asynchronous prefix merge.                                                |
| JST-FORGE-007 | Tokens MUST remain outside checkouts, URLs, logs, receipts and browser persistence. Configured endpoints MUST be explicit approved device-local connections. Production UI MUST use owned components and released Design Core/Mira.                                                                                                         |

## Composition and compatibility

`jj-forge` owns transport-independent contracts, provider adapters,
deterministic planning and external-effect journals. Local JJ
inspection/transport remains in `jj-store-jj`; service/HTTP and standalone CLI
share the adapter. Storage, manifest authority and WAL are independent of forge
runtime dependencies.

The first profile is ordinary local JJ 0.45.1 with SHA-1 Git objects. Native
jj-store stack submission and further forges require separate qualification.
GitHub native stacks are an optional capability; portable PR chains remain the
common model. Fork PRs may have cumulative diffs when the upstream lacks the
intermediate branches. Their managed section explains this mode.

Connections retain API origin, source/target project, remote and target branch
outside checkouts. Secrets are supplied per session or through automation
credentials. GETs and previews never snapshot/reconcile code or publish state.
Existing local Stacks undo/redo cannot undo external forge effects.

## UI and maintenance

Branch and Workspace columns share PR presentation and submission. The app
retains description drafts and exact pending operations during refresh and
navigation. Operational failures use the shared notification service. Logs are
bounded untrusted text and remain separate from native WorkflowStore evidence.
Polling is bounded, deduplicated, paused on teardown and backs off on failures.
Unsupported capabilities remain explicit; a missing check is never a pass.

## Delivery and evidence

M31 owns implementation and qualification. Tests for this work are restricted to
changed components and their contracts, per the user's 2026-10-15 instruction.
Wider test-suite repairs and baseline approval are deferred. Live provider and
platform qualifications require actual evidence and cannot be inferred from
mocked adapter tests.

## Qualified bounds and recovery

The current implementation supports at most 64 segments/navigation entries, 128
submission and 128 Restack receipts, 1,024 PR-action receipts and 8 MiB of
private device state. Known PR observation is bounded to 256 IDs. Capacity
exhaustion fails explicitly; receipts are never silently evicted. A connection
with history cannot be rebound to another project. Journal archival/migration is
future work.

The active repository Stacks controller polls every 15 seconds, backs off to 60
seconds on failure and watches at most 16 visible branches with concurrency
four. Closing its owning repository stops polling. Description maintenance
operates while that controller is active or the CLI explicitly requests status;
there is no always-on backend maintenance daemon. Historical CI selection and
unsaved descriptions remain retained.

Shared-root or cross-combination Restack is unavailable pending qualification.
Conflicted Restack retains the applied native operation and stops publication.
Unknown rerun/cancel outcomes remain unresolved when the provider supplies no
proof; checking never automatically repeats the action. Eligible same-repository
GitHub stacks require exact ordered membership. Merge queue acceptance and its
receipt are durable; only provider observation establishes merged state.

## Comparison and next qualification

The
[comparison and rollout record](../../../history/2026/10/15/E-FORGE-STACKS-001/comparison-and-rollout.md)
records the source revisions, borrowed patterns, capability differences and
remaining acceptance work. Future adapters extend the owned provider boundary;
GitHub-specific stack membership never becomes the common stack model.
