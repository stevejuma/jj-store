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
post-merge, scheduled and explicitly dispatched qualification still run static
specification validation and all impact-selected tests. Generated release PRs
dispatched explicitly because of GitHub token event restrictions retain full
release CI. Bot exemptions never waive package, security or publication gates.

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

CI first plans impact and checks formatting. Independent tooling-test and static
governance jobs then complete before ordinary Rust builds/tests and affected web
checks may start. Their successful or explicitly unselected outcomes admit the
separate reusable heavy workflow. Heavy test shards remain parallel and serial
within each runner. Runtime/native qualification and asset production wait for
heavy tests; storage waits for the verified runtime, and browser acceptance
waits for both storage and assets. Early failures cannot launch the later
stages. Each stage reports its own result, and the stable protected `required`
check validates the complete planned graph, including rejected/skipped gates.
Shared setup omits native headers and build-only tools for early checks.
Fail-fast applies within archived shards and to each matrix: a failing shard
cancels its siblings, and canceled work can never satisfy a qualification gate.
