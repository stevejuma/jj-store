# M30 public release

| Task | Status      | Depends | Requirements                                                             | Acceptance gate                                                           | Next action                                                                       | Evidence                                                                            |
| ---- | ----------- | ------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| M30  | In progress | M0      | JST-ARCH-005,JST-CLI-002,JST-CLI-004,JST-DOC-001,JST-DOC-002,JST-SEC-004 | Cargo, native bundles, Pages and authenticated GHCR release qualification | Complete release infrastructure qualification; bot governance exemptions verified | E-PUBLIC-RELEASE-001,E-AUTOMATION-GOVERNANCE-001,E-CI-BOOTSTRAP-001,E-CI-FORMAT-001 |

E-AUTOMATION-GOVERNANCE-001 qualifies the generated release/dependency PR
exception. Static spec checks, all planned tests and publication gates remain;
M30 stays In progress until the full release qualification completes.

M29's paused qualification is a release gate. Publication does not promote other
platform, provider, desktop or replication milestones.
<!-- generated-navigation -->

## Pages

E-CI-BOOTSTRAP-001 records the initial hosted failures, focused repairs and
PR/check/tag protection. Required hosted qualification still precedes merge.
