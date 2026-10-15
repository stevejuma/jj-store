# Public distribution and release orchestration

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
is required and administrators cannot bypass the rule. Force pushes and branch
deletion are disabled. Version tags cannot be rewritten or deleted. Automatic
publication remains disabled until the initial release qualification is ready;
CI and Pages qualification continue independently.

Native CI verifies the runner's compiler triple and builds/tests its native
debug profile in one shared target root. This avoids a second optimized build
and duplicate native compilation during PR checks. Release archives still
require the separately qualified optimized build and complete publication gates.

Browser CI consumes the server/client, embedded runtime and browser/Help/catalog
artifacts from the same workflow run and selected source revision. The embedded
runtime builds and passes strict lint on an independent runner; storage and cold
browser acceptance reuse it. An explicit CI-only prebuilt profile validates
regular executable files and complete browser/Help manifests before replacing
any disposable process. Ordinary development retains its build path. Cold
acceptance uses a fresh isolated fixture, checksum-pinned published jj-acp 0.1.0
with control-version-3 prepared-context admission, and the collision-safe
`jj-store-jj` client. No fixture ACP, installed CLI or interactive state may be
substituted. Missing resources fail; existing tests and acceptance assertions
remain required. The aggregate required check includes the runtime producer.

Standard CI runs on pull requests, main pushes, explicit dispatch and reusable
calls. It first plans impact and checks formatting, then runs independent
tooling and governance jobs before standard Rust and affected web checks. The
standard cohort excludes server, CLI and capture packages, including their unit
tests; embedded RustFS, native-platform and browser acceptance belong to
qualification. The stable protected `required` check validates every planned
standard lane and rejects missing, failed, canceled or unexpectedly skipped
prerequisites. Main pushes and `all_checks` select the complete standard cohort.
Superseded PR runs are canceled. No automatic or scheduled standard run starts
heavy qualification.

Manual qualification resolves its source once to an immutable commit, runs all
standard checks, then runs every existing heavy lane. Publication calls this
same complete workflow at the selected release SHA. Heavy archived shards remain
parallel and serial within each runner. Runtime/native qualification and assets
wait for heavy tests; storage waits for the verified runtime, and browser
acceptance waits for storage and assets. All qualification lanes are mandatory.
Failure or cancellation cannot satisfy the final qualification gate. Same-run
artifacts always use the selected source. Automatic publication stays disabled.

Shared setup omits native headers and build-only tools for early and web checks.
The Ubuntu amd64 governance runner installs actionlint from its pinned official
Linux release archive, verifies the tracked SHA-256 before extraction and
exposes only the verified executable directory through the runner's PATH file.
Fail-fast applies within archived shards and matrices. Storybook and Playwright
likewise stop on their first failure; unrun tests never establish acceptance.
Job names identify the tested cohort, browser suite or native platform and
shard.

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
