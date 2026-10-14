# M30 public release

| Task | Status      | Depends | Requirements                                                             | Acceptance gate                                                           | Next action                                                                       | Evidence                                                                                                                                                                                   |
| ---- | ----------- | ------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| M30  | In progress | M0      | JST-ARCH-005,JST-CLI-002,JST-CLI-004,JST-DOC-001,JST-DOC-002,JST-SEC-004 | Cargo, native bundles, Pages and authenticated GHCR release qualification | Complete release infrastructure qualification; bot governance exemptions verified | E-PUBLIC-RELEASE-001,E-AUTOMATION-GOVERNANCE-001,E-CI-BOOTSTRAP-001,E-CI-FORMAT-001,E-CI-TOOL-001,E-CI-ENVIRONMENT-001,E-CI-FIXTURE-001,E-CI-PIPELINE-001,E-CI-BUNDLE-001,E-CI-BROWSER-001 |

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
publication gate is promoted.
<!-- generated-navigation -->

## Pages
