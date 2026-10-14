# Public distribution and release orchestration

JST-ARCH-005 governs the coordinated public source, Cargo, native archive and
container distribution. Changesets owns the release version and notes through
the private npm workspace package; npm publication is disabled. The version PR
synchronizes every Cargo package and internal registry dependency, the embedded
RustFS workspace and release metadata. Version tags select immutable qualified
source. Publication never replaces an existing package, tag or digest.

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
