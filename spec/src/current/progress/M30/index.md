# M30 public release

| Task | Status      | Depends | Requirements                                                             | Acceptance gate                                                           | Next action                                                         | Evidence                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---- | ----------- | ------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M30  | In progress | M0      | JST-ARCH-005,JST-CLI-002,JST-CLI-004,JST-DOC-001,JST-DOC-002,JST-SEC-004 | Cargo, native bundles, Pages and authenticated GHCR release qualification | Repair qualification failures, then rerun the complete release gate | E-PUBLIC-RELEASE-001,E-AUTOMATION-GOVERNANCE-001,E-CI-BOOTSTRAP-001,E-CI-FORMAT-001,E-CI-TOOL-001,E-CI-ENVIRONMENT-001,E-CI-FIXTURE-001,E-CI-PIPELINE-001,E-CI-BUNDLE-001,E-CI-BROWSER-001,E-CI-GOVERNANCE-001,E-CI-ACTIONLINT-001,E-CI-STANDARD-001,E-CI-AFFECTED-001,E-CI-CACHE-TRIAL-001,E-CI-HEAVY-001,E-CI-SUITES-001,E-CI-SUITES-FIX-001,E-CI-RUNTIME-TOOLS-001,E-CI-SUITES-MEASURE-001,E-UI-CI-001 |

E-AUTOMATION-GOVERNANCE-001 qualifies the generated release/dependency PR
exception. Static spec checks, all planned tests and publication gates remain;
M30 stays In progress until the full release qualification completes.

M29's paused qualification is a release gate. Publication does not promote other
platform, provider, desktop or replication milestones.

E-CI-BOOTSTRAP-001 records the initial hosted failures, focused repairs and
PR/check/tag protection. Required hosted qualification still precedes merge.

E-CI-TOOL-001 repairs GNU/BSD archive member selection with real verified
upstream archives. Complete hosted qualification remains a merge gate.

E-CI-ENVIRONMENT-001 qualifies CI revision/color fixture isolation and the
remaining Linux mode-cast repair. Fresh complete hosted CI remains a merge gate.

E-CI-FIXTURE-001 qualifies PATH-portable stock fixtures and exact public
migration reference. E-CI-PIPELINE-001 records staged/fail-fast qualification
and resource reuse; E-CI-BUNDLE-001 retains current cold/served-browser and
hosted gaps. E-CI-BROWSER-001 records isolated browser shards, cold lifecycle
repairs and local regression qualification. M30 remains In progress; no merge or
publication gate is promoted. E-CI-GOVERNANCE-001 qualifies the Git/JJ
evidence-baseline repair with local CI-context checks and
missing-evidence/merge-parent regressions. M30 remains In progress; hosted
qualification and the remaining release gates are separate.

E-CI-ACTIONLINT-001 qualifies the pinned Linux workflow-linter installation from
its official release archive. The hosted progress and spec-first checks passed
before exposing this independent installer failure. M30 remains In progress with
the same release and merge gates.

E-CI-STANDARD-001 separates the protected standard merge check from full manual
and release qualification. Earlier references to complete hosted CI as a merge
gate now mean the standard `required` check. M30 remains In progress; the known
heavy shard failure and unrun qualification lanes remain release gaps.

E-CI-AFFECTED-001 qualifies Cargo reverse-dependency selection, conservative
shared-input fallback, exact archived inventories and scoped dependency-cache
writes. Local tooling and archive smoke evidence does not replace hosted cache
reuse or full manual qualification. M30 remains In progress; the known heavy
failure remains unresolved and publication remains disabled.

E-CI-CACHE-TRIAL-001 adds the bounded Linux compilation-cache comparison. M30
remains In progress. A hosted experiment and recorded threshold decision are
required before any normal workflow adopts sccache. Heavy qualification and
publication gaps remain unchanged.

E-CI-HEAVY-001 records the protected merge of PR #1, verified Rust cache reuse
and the first complete manual qualification attempt on merged main. The cache
experiment is deferred at the user's request; normal checks retain rust-cache
alone. The prior heavy failure exposes an obsolete version-1 AgentStore test,
which must enforce the current-only format decision from E-M17-034. M30 remains
In progress; the focused repair and pending heavy lanes do not establish
complete qualification or permit publication. E-CI-SUITES-001 separates six
qualification suites and validates reusable artifact provenance while retaining
complete release gating. It records the user-authorized brief administrator
exception for each verified main push. Independent heavy suites now retain
complete diagnostics and timing evidence. M30 remains In progress: the
Kingfisher identity regression exposed by run 38716420649 and unqualified
downstream lanes are not waived. New hosted timing and standalone-suite results
remain required before claiming measured coverage.

E-CI-SUITES-FIX-001 records the first hosted governance failure and the
behavior-preserving artifact-layout extraction. M30 remains In progress;
complete hosted and standalone measurements are still required.
E-CI-RUNTIME-TOOLS-001 records the missing protocol compiler in the first
parallel runtime build and the scoped CI repair. Structured RustFS counts are
retained. M30 stays In progress pending a fresh complete qualification;
unrelated suites continue after producer failure and their results remain
independent.

E-CI-SUITES-MEASURE-001 records completed parallel coordinator 38730859426, its
successful artifact producers and precise timing/count reporting. All
independent jobs finished; backend/UI failures and cold fixture setup failure
remain qualification gaps. Storage, native platforms, API contracts, profile
restart and Git round-trip passed. Six standalone reruns verify separate
histories and artifact reuse; their recorded outcomes never substitute for the
complete release gate. M30 remains In progress with no publication authorized.
E-UI-CI-001 promotes the unchanged seven-job core UI inventory into affected
standard checks and adds an independent fresh-input UI entry point. Repairs
retain history completions during graph refresh and prompt disclosure access,
and defer automatic refresh while explicit reference pagination completes;
platform-aware geometry and settled focus/layout checks preserve the existing
assertions. M30 remains In progress. Full qualification stays idle during this
repair; prior backend and cold-application failures remain release blockers.
Focused UI and tooling results do not qualify publication or visual baselines.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
