# M30 public release

| Task | Status      | Depends | Requirements                                                             | Acceptance gate                                                           | Next action                                                         | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---- | ----------- | ------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M30  | In progress | M0      | JST-ARCH-005,JST-CLI-002,JST-CLI-004,JST-DOC-001,JST-DOC-002,JST-SEC-004 | Cargo, native bundles, Pages and authenticated GHCR release qualification | Repair qualification failures, then rerun the complete release gate | E-PUBLIC-RELEASE-001,E-AUTOMATION-GOVERNANCE-001,E-CI-BOOTSTRAP-001,E-CI-FORMAT-001,E-CI-TOOL-001,E-CI-ENVIRONMENT-001,E-CI-FIXTURE-001,E-CI-PIPELINE-001,E-CI-BUNDLE-001,E-CI-BROWSER-001,E-CI-GOVERNANCE-001,E-CI-ACTIONLINT-001,E-CI-STANDARD-001,E-CI-AFFECTED-001,E-CI-CACHE-TRIAL-001,E-CI-HEAVY-001,E-CI-SUITES-001,E-CI-SUITES-FIX-001,E-CI-RUNTIME-TOOLS-001,E-CI-SUITES-MEASURE-001,E-UI-CI-001,E-UI-CI-002,E-UI-CI-003,E-LAPIS-AI-001,E-STORY-PARITY-001,E-STORY-DOC-REVIEW-001,E-CI-SUITES-FIX-002,E-CI-SUITES-FIX-003,E-CI-OFFLINE-GOVERNANCE-001,E-PACKAGE-LICENSE-001,E-PACKAGE-LICENSE-002,E-GIT-CRATE-PINS-001,E-MIRA-012-PIN-001,E-PACKAGE-GIT-PINS-001 ,E-CLI-GIT-CRATES-001 |

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
E-UI-CI-002 records the first hosted core UI run: all three Storybook shards,
static catalog, dual-engine Help and the first application shard passed. The
second application shard exposed an unrequested capture receipt observation. The
test now explicitly inspects the original request through the retained refresh
control, preserving identity and the single-import assertion. M30 stays In
progress pending hosted confirmation and the existing full qualification gaps;
the full heavy workflow remains idle. E-UI-CI-003 records the post-merge Search
save-error race in both main standard checks and manual qualification. A status
read could erase the failed save notice while retaining its dirty draft. The
controller now retains that failure until Cancel or a successful explicit Save;
neither observation nor dismissal replays a command. M30 remains In progress:
the failed standard gate prevented heavy suites from starting, and prior
backend/cold failures remain unresolved.

E-MAIN-LOCAL-REPAIRS-001 records the user-authorized 2026-10-16 completed
main-workspace repair validation and wider failure repairs. M30 retains its
existing status and completion gates; local passes do not establish hosted,
additional-platform, live-provider or visual-baseline qualification.
E-MAIN-PICKER-REPAIRS-001 reviews the reference-read cancellation repair with
status unchanged. Exact unit, direct-consumer and fresh native browser
acceptance retain pinned reads and stale-response rejection. Complete local
qualification, hosted checks and the existing release/platform/visual gates
remain separate.

E-MAIN-SYNC-STORIES-001 supplies fresh combined-parent integration evidence for
the unchanged product tree synchronized with remote main. The complete
1,021-case Storybook inventory passes; failed attempts remain recorded. M30
stays In progress. Complete cold application/profile, final governance and
hosted qualification remain pending; no merge or publication is authorized.

E-MAIN-LOCAL-QUALIFICATION-001 records the complete local qualification
inventory, including source-bound repaired-target Rust proof, exact UI
inventories, real storage, cold renewal and isolated profile restart. Earlier
failures and environment prerequisite repairs remain explicit. Final consuming
documentation and stable-tree tooling results accompany the same evidence. M30
stays In progress: this local proof and the separately identified standard CI
head do not qualify heavy hosted release, other platforms or publication.

E-PR10-UI-READINESS-001 records PR #10's single hosted Storybook failure at the
otherwise locally qualified head. The private story waits for dismissed selects
to finish closing. Broader acceptance also exposed editor focus interrupting
chat navigation; focusing before the explicit jump retains the existing
full-draft visibility contract. Qualification gates remain unchanged. M30 stays
In progress. Fresh standard required checks must pass on the repair head; prior
local or hosted passes do not substitute for that result.

E-PR10-AUTOCOMPLETE-001 explicitly reviews M30 with status unchanged. The
remaining hosted workflow failure is traced to Design Core's deferred input
selection. Its source repair and authorized package release retain protected
source/version checks and provenance verification. Fresh required CI on the
updated jj-store head remains mandatory; this dependency repair does not qualify
heavy hosted release, other platforms or jj-store publication.

E-LAPIS-AI-001 records local Lapis AI client/helper adoption and Control V3-only
execution, with exact recovery, bundle and cold compute evidence. This review
retains the current milestone status and its independent qualification gates.
E-STORY-DOC-REVIEW-001 reviews the documentation fingerprints and manuals
against their owning committed source changes. M30 remains In progress; this
maintenance does not promote any provider, release or visual gate.

E-CI-SUITES-FIX-002 restores the documented bounded Storybook sharding for
nested and native validation dispatch, so the unfiltered suite inside aggregate
lanes runs four sequential fresh-browser shards and every shard runs even when
an earlier one fails. Explicit targets and caller-selected shards stay exact.
M30 remains In progress; no hosted-qualification, release or publication gate
changes.

E-CI-SUITES-FIX-003 provisions the protocol compiler in the full local
validation image through its existing apt line, exactly mirroring the hosted CI
embedded-runtime producer, so the aggregate Docker lanes can compile the
Pulsar-dependent embedded runtime. M30 remains In progress; validation-image Git
2.55.0 provisioning for the macOS-qualified native_git family stays a separate
owner decision.

E-CI-OFFLINE-GOVERNANCE-001 mirrors the Docker offline `pnpm store add`
provision onto hosted GitHub Actions setup so the conversation and history
clean-install regressions resolve without network on ubuntu runners. M30 remains
In progress; no publication gate changes.

E-PACKAGE-LICENSE-001 adds the required Apache-2.0 LICENSE.md and NOTICE.md
files for jj-history-service and jj-vault, and refreshes capture, local-compute
and history-client documentation reviews for published sibling pins and the
jj-history publishConfig so Pages and package-check can pass.

E-PACKAGE-LICENSE-002 records the mapped architecture and vault-backup chapter
updates that require LICENSE.md/NOTICE.md on publishable history-service and
vault crates so Spec-first covers E-PACKAGE-LICENSE-001 packaging files.

E-GIT-CRATE-PINS-001 replaces the sibling Cargo paths for `lapis-ai-capture`,
`lapis-ai-runtime`, `lapis-ai-config`, and `lapis-agent-redaction` with the
published git tags. M30 stays In progress. That pin left Mira at `~0.1.1`. The
full release qualification remains open.

E-MIRA-012-PIN-001 raises `@lapismd/mira` from `~0.1.1` to `~0.1.2` after npm
publishes that version. Editor and plugin packages stay on their current ranges.
M30 stays In progress.

E-PACKAGE-GIT-PINS-001 admits those exact published git tags in the release
package check and still rejects every other git dependency. M30 stays In
progress.

E-CLI-GIT-CRATES-001 keeps crates.io publication to the `jj-store` command and
the crates that command links. `jj-history-service`, `jj-vault`,
`tauri-plugin-jj-store`, and `jj-store-embedded-rustfs` stay git-only.
Dependabot's open Cargo updates are combined into one change. M30 stays In
progress.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
