# Public distribution and release orchestration

JST-ARCH-006's injectable core is a coordinated library package under the
existing JST-ARCH-005 release policy. Source extraction and package checks do
not publish a release. Existing canonical storage formats remain unchanged.

The standalone Git provider is a coordinated package with contained source
inputs and registry-qualified dependency metadata. Its normal dependency graph
excludes native object/WAL and application transports. The focused JJ companion
feature adds qualified source evidence without those native/application modules.
This source-library qualification does not publish any package or native bundle.

Crate-root READMEs introduce each package's role, actual usage entry points and
dependency context, with links to detailed documentation under `docs`. Separate
maintained helper workspaces have the same overview. Documentation coverage and
package containment do not expand their platform or release qualification.

JST-ARCH-005 governs the coordinated public source, Cargo, native archive and
container distribution. Changesets owns the release version and notes through
the private npm workspace package; npm publication is disabled. The version PR
synchronizes every Cargo package and internal registry dependency, the embedded
RustFS workspace and release metadata. Version tags select immutable qualified
source. Publication never replaces an existing package, tag or digest.

Generated `changeset-release/main` PRs and dependency PRs authored by
`dependabot[bot]` on `dependabot/` branches are exempt from the
JST-GOV-002/003/004/011 same-change topic, evidence and milestone-update gates.
They also need no additional authored Changesets note. Version automation
updates only release metadata and notes; it MUST NOT generate specification or
evidence edits. Ordinary PRs, other Changesets branches and other bots retain
the change-specific gates. Like lapis-notes, those gates run on PR events;
post-merge and explicitly dispatched qualification still run static
specification validation and all impact-selected tests. Generated release PRs
dispatched explicitly because of GitHub token event restrictions retain all
standard checks. Complete qualification is required before publication. Bot
exemptions never waive package, security or publication gates.

Native bundles support GNU/Linux and macOS on ARM64 and x86-64, and contain the
server, collision-safe custom client, embedded RustFS helper, browser and Help.
The authenticated container contains the server and browser/Help, uses external
RustFS, runs without root and requires explicit TLS and installation policy.
LocalTrusted remains literal-loopback only. Desktop, Windows, live provider and
additional storage topology gates retain their separate milestones.

Rust 1.99 and released Kingfisher libraries replace checkout-only dependencies.
Privacy policy and compiled cache identities distinguish the released rule set.
Source admission, immutable identities, manifest CAS and WAL receipts remain
authoritative; distribution and derived caches cannot publish repository state.

CI partitions existing checks by dependency cohort and impact. Ordinary and
heavy tests use distinct archived builds and independent runners. Ignored child
fixtures run only through their acceptance parent. Releases require all
qualification lanes, package-contained inputs, real-storage recovery, native
installation, documentation and container provenance. Required checks validate
planned skips as well as failures. Fork jobs cannot acquire publication secrets
or populate trusted release caches. Interrupted tests never establish a pass.

Canonical `main` changes go through pull requests. GitHub Actions' `required`
check MUST pass against an up-to-date base before merge; conversation resolution
is required and administrators normally cannot bypass the rule. The
user-authorized CI rollout may temporarily disable administrator enforcement
only around each locally verified direct main push; enforcement MUST be restored
and verified immediately after the push attempt. All other protection settings
remain intact. Force pushes and branch deletion are disabled. Version tags
cannot be rewritten or deleted. Automatic publication remains disabled until the
initial release qualification is ready; CI and Pages qualification continue
independently.

Native CI verifies the runner's compiler triple and builds/tests its native
debug profile in one shared target root. This avoids a second optimized build
and duplicate native compilation during qualification. Release archives still
require the separately qualified optimized build and complete publication gates.

Browser CI consumes the server/client, embedded runtime and browser/Help/catalog
artifacts from the selected source revision and verified producer run/attempt.
The embedded runtime builds and passes strict lint on an independent runner;
storage and cold browser acceptance reuse it. An explicit CI-only prebuilt
profile validates regular executable files and complete browser/Help manifests
before replacing any disposable process. Ordinary development retains its build
path. Cold acceptance uses a fresh isolated fixture, Lapis ACP built from the
recorded Lapis AI source revision with Control V3 prepared-context admission,
and the collision-safe `jj-store-jj` client. No fixture ACP, installed CLI or
interactive state may be substituted. Missing resources fail; existing tests and
acceptance assertions remain required. The complete qualification gate includes
the runtime producer.

Standard CI runs on pull requests, main pushes, explicit dispatch and reusable
calls. It first plans impact and checks formatting, then runs independent
tooling and governance jobs before standard Rust and affected web checks. The
standard Rust cohort excludes server, CLI and capture packages, including their
unit tests. Core UI acceptance (Storybook, served application, static catalog
and dual-engine Help) runs with affected web checks and is required for PRs. Its
focused producer compiles only the preview/documentation tools and native HTTP
fixture capture needed by these tests; it does not run backend qualification or
build embedded RustFS. Cold application and native-platform acceptance remain
manual qualification. The stable protected `required` check validates every
planned standard lane and rejects missing, failed, canceled or unexpectedly
skipped prerequisites. Main pushes and `all_checks` select the complete standard
cohort. Superseded PR runs are canceled. No automatic or scheduled standard run
starts heavy qualification.

Manual qualification resolves its source once to an immutable commit and runs
all standard checks before heavy work. Six manual/reusable workflows own backend
Rust, API contracts, storage acceptance, UI acceptance, cold application and
native compatibility. A shared producer definition builds backend tools/tests,
embedded runtime and browser assets. Runtime, API, external storage and native
checks start independently after standard checks; assets need backend tools,
embedded tests need runtime, and browser consumers wait only for their required
artifacts. UI acceptance is already required by standard checks and is not
repeated by the coordinator. Backend failures do not block unrelated consumers.
Every producer, suite and shard remains mandatory for complete qualification and
publication.

Standalone suites require an exact source SHA, producer run and attempt. Reuse
requires a successful source-resolution job, standard gate and relevant artifact
producers in the canonical qualification/publication workflow. Receipts bind
source, repository, run, attempt and payload checksums; missing files, unsafe
paths, symlinks and mismatches fail. Payload names include the attempt. An
unrelated failed test does not invalidate successful producers, but artifacts
never establish a test pass. Publication consumes the same-run browser payload
and still waits for complete qualification. No automatic heavy scheduling or
publication is enabled.

Core UI acceptance also has an independent manual build-and-test entry point at
an exact source SHA. It shares the same seven test jobs with qualification
reruns. Its fresh inputs are admitted only within their own run and attempt,
after the focused UI producer succeeds, with source and payload checksums
verified. They cannot bypass qualification's standard gate or authorize release
artifact reuse. PR UI work follows the preflight gate; a failed, canceled or
missing UI shard fails `required`. Impact selection includes Help sources and
shared contracts. Full qualification remains explicitly invoked only while
individual suites are repaired; focused UI success never establishes release
qualification.

Heavy matrices and test runners collect independent results after failures;
standard CI retains its early failure gates. Existing shard counts, serial Rust
execution within each heavy shard, isolated fixtures and ignored acceptance
parents remain unchanged. Logs, inventories, available JUnit/browser results and
machine-readable timing reports are retained on success and failure. Reports
identify source, run/attempt and runner, distinguish execution from queueing and
dependency waits, and leave unavailable measurements unknown. Runner minutes are
distinct from elapsed time. Missing reports or interrupted execution cannot
establish acceptance. Standalone reruns distinguish execution-only archive reuse
from suites that still compile contracts or native targets.

Shared setup omits native headers and build-only tools for early and web checks.
The Ubuntu amd64 governance runner installs actionlint from its pinned official
Linux release archive, verifies the tracked SHA-256 before extraction and
exposes only the verified executable directory through the runner's PATH file.
Standard CI retains fail-fast; heavy qualification collects all independent
results. Unrun tests never establish acceptance. Job names identify the tested
cohort, browser suite or native platform and shard.

Cold browser acceptance is partitioned into four complete Playwright shards,
each with its own disposable server/storage fixture. Profile restart acceptance
uses a separate fifth fixture because it deliberately terminates its server. The
real-RustFS Git-cycle acceptance runs through its owning ignored Rust parent on
another runner, consuming the existing heavy test archive and current tools and
browser bundle. Its browser child is not an independent generic fixture. The
required aggregate admits no missing shard or fixture lane. The complete local
cold command likewise separates profile restart from the ordinary suite.

Storybook interactions run in three disjoint file shards; served application
acceptance runs in two Playwright shards. Static catalog and dual-engine Help
acceptance each run on their own runner. Every shard retains the original tests
and failure gate; splitting the lanes never turns partial execution into a pass.
Output directories are isolated between served browser, Help and cold shards.
Served shards exclude Help files, which run in both engines in the Help lane.
Browser runners reuse prepared artifacts without native build headers or general
build tools; only the archived Git parent installs the test-archive tooling.
Every browser lane installs Chromium, while only dual-engine Help installs
WebKit. Unused browser engines and build tools are not downloaded per shard.

Standard Rust impact uses Cargo metadata and reverse workspace dependencies,
including build/dev/optional/target-specific edges and shared source fixtures.
One validated package list controls Clippy, doctests and archived tests. Shared
manifests, build/configuration inputs and unknown ownership select the complete
standard cohort; invalid metadata fails planning. Main always runs all standard
checks. Plans disclose selected/excluded packages and reasons. Archive
inventories pin source, packages and test identities; verified empty partitions
are explicit.

Dependency caches may be saved by successful PR jobs within GitHub's isolated PR
scope. Shared-main cache writes require a main workflow context and a selected
source verified on canonical main; arbitrary qualification refs are read-only.
Cache identity includes platform, toolchain, profile and cohort. No cache hit
proves a test pass. Summaries retain selection, cache and job/step timing
evidence.

The optional branch-only cache trial warms isolated caches at one immutable
source, then compares three matched Linux builds per backend. Measured samples
restore the same dependency cohort and cannot write either measured cache.
Strict Clippy, doctests and archive production remain identical; inventory
identity and passing baseline/cached-build tests are prerequisites. Adoption
requires at least 10 percent improvement in median complete build-job duration,
including setup, cache and artifact overhead. A failed or incomplete experiment
cannot justify adoption. Routine PR and release checks do not run the trial.

E-CI-SUITES-FIX-001 confirms JST-ARCH-005 is unchanged by extracting artifact
layout declarations to satisfy the module-size gate. Source admission, checksum
verification, coverage and complete qualification remain mandatory.

The embedded runtime producer provisions and reports the protocol compiler
required by its Pulsar dependency before compilation. Real-storage reports
retain the fixture test inventory, counts and startup/cleanup failures alongside
logs. API, native and embedded-storage standalone runs still compile local
targets; their timings must not be described as execution-only archive reruns.

Timing reports retain Node TAP counts and separate Cargo target preparation from
completed libtest execution. Job creation-to-start measures scheduler waiting,
which may include workflow concurrency waits; runner-only queueing and
dependency waiting remain unknown without readiness timestamps. Completed-run
measurements include post-job cache saves. A failed producer requires a fresh
coordinator run: partial reruns cannot combine receipts or payloads from
different attempts. Standalone test retries may reuse the original successful
producer attempt, even when unrelated tests failed.

E-UI-CI-003 retains JST-ARCH-005: the Search notification repair uses an
ordinary patch Changeset and PR checks. The failed standard browser gate
prevented the new heavy run from starting its heavy suites; those suites remain
unqualified.

E-MAIN-LOCAL-REPAIRS-001 confirms unchanged JST-ARCH-005 release authority for
the main-workspace repair slice and its patch Changeset. A user-requested draft
PR may publish the verified repair snapshot while comprehensive local checks
continue with their pending status explicit. It does not authorize merge or
release, waive required checks, or establish full qualification.

E-PR10-AUTOCOMPLETE-001 retains JST-ARCH-005: shared dependency defects are
repaired and qualified in their source repository, then consumed through a
released registry version. Packed-consumer evidence, released-package acceptance
and hosted required checks remain distinct. Authorization to publish Design Core
does not authorize jj-store publication or waive its qualification gates.

E-REPOSITORY-DELETION-001 records a minor Changeset for confirmed local removal
and managed native object erasure. The note does not publish a release, modify
installed tools or qualify external S3/shared/platform deployment. Existing
source ownership, portable dependency metadata and coordinated release authority
under JST-ARCH-005 remain unchanged.

M32 includes jj-store-native in the coordinated library inventory with a concise
crate README and portable path-plus-version dependencies. Its minor Changeset
adds named native factories without publishing packages or changing existing
manifest/WAL/receipt formats. JST-ARCH-005 release gates remain unchanged.

M33 native privacy source consumption retains immutable sibling-source pins. CI
materializes Agent Redaction separately from Lapis AI, verifies the declared
package version and refuses replacement of an existing source checkout. Only the
exact capture-to-redaction path is admitted. These source-library changes do not
publish either repository or weaken artifact/qualification gates.

M33 acquisition consumption permits only the exact capture-to-`lapis-ai-capture`
versioned sibling path. The immutable Lapis AI source pin includes this registry
crate. Unknown source paths and runtime imports remain refused; JST-ARCH-005
qualification and publication authority are unchanged.

The additive capture feature split preserves default application APIs while
allowing independent headless consumers. Package contents include the short
README and feature contracts; JST-ARCH-005 release qualification remains
separate from feature-specific source and consumer checks.

M33 registers `jj-store-conversations` in the coordinated library inventory with
contained compilation inputs, a short README and public feature/dependency
checks. Its initial stream primitives import no application/runtime/storage
crate. JST-ARCH-005 publication and complete qualification remain separate.

Shared conversation coordination adds async trait ports without an asynchronous
runtime or transport dependency. Borrowed host capabilities retain their scoped
lease; independent crate and native consumer tests qualify the public boundary.
JST-ARCH-005 release and packaged downstream acceptance remain separate.
