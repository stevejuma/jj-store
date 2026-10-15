# M30 public release

| Task | Status      | Depends | Requirements                                                             | Acceptance gate                                                           | Next action                                                                       | Evidence                                                                                                                                                                                                                                                                                                   |
| ---- | ----------- | ------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M30  | In progress | M0      | JST-ARCH-005,JST-CLI-002,JST-CLI-004,JST-DOC-001,JST-DOC-002,JST-SEC-004 | Cargo, native bundles, Pages and authenticated GHCR release qualification | Complete release infrastructure qualification; bot governance exemptions verified | E-PUBLIC-RELEASE-001,E-AUTOMATION-GOVERNANCE-001,E-CI-BOOTSTRAP-001,E-CI-FORMAT-001,E-CI-TOOL-001,E-CI-ENVIRONMENT-001,E-CI-FIXTURE-001,E-CI-PIPELINE-001,E-CI-BUNDLE-001,E-CI-BROWSER-001,E-CI-GOVERNANCE-001,E-CI-ACTIONLINT-001,E-CI-STANDARD-001,E-CI-AFFECTED-001,E-CI-CACHE-TRIAL-001,E-CI-HEAVY-001 |

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
complete qualification or permit publication.
<!-- generated-navigation -->

## Pages
