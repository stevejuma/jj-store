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
