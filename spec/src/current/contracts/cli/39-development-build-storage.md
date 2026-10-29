# Development build storage

JST-CLI-004 governs device-local build maintenance. The managed Cargo entry
point and repository-owned launchers MUST keep outputs in explicit target
directories belonging to the selected workspace. Independent Cargo roots and
parallel JJ workspaces MUST retain separate output paths. Development and test
profiles retain `debug = 0` and incremental compilation.

`pnpm dev:disk` reports the registered JJ workspace family's Cargo roots and
explicitly registered temporary targets. `--json` returns version-1 local report
data. Artifact categories include dependencies/executables, incremental data,
build-script output and desktop bundles/resources. Protected fixture, archive
and staged-resource paths are reported separately. Allocated directory sizes may
share APFS blocks and MUST NOT be presented as guaranteed recovery.

Managed builds check storage before and after Cargo, including failed builds.
Defaults are 40 GiB per target, 80 GiB across managed targets and 30 GiB minimum
available filesystem space. Positive finite overrides use
`JJ_STORE_BUILD_TARGET_GIB`, `JJ_STORE_BUILD_TOTAL_GIB` and
`JJ_STORE_BUILD_FREE_GIB`. These are maintenance thresholds rather than a hard
quota on a running compiler. Postflight retains the current command's outputs
for staging and launch; unresolved pressure rejects the command with guidance.

Under pressure, completed temporary targets are cleaned first, then oversized
idle targets, then remaining idle targets in oldest-modified order. Cargo's
workspace-package clean retains third-party dependencies; full disposable-cache
cleanup follows only if pressure remains. Ordinary reads and UI-only HMR startup
do not trigger Rust builds.

`pnpm dev:clean -- --dry-run` reports eligible actions without removing
artifacts or changing lifecycle records. `--apply` explicitly performs
maintenance. `--target PATH` limits an explicit cleanup to a known target.
Unknown paths, symlink escapes, invalid Cargo cache markers and protected state
refuse cleanup. No command deletes a repository, credential, persisted fixture,
recovery archive or staged runtime resource, or stops a process to make its
target eligible.

POSIX shared build leases and exclusive cleanup leases live outside targets,
grouped by the shared JJ repository. Registry/preflight changes are serialized;
independent builds retain separate leases. Native Cargo lock contention and
`lsof` observations of executable, mapped-library, cwd or open-file consumers
make a target ineligible. Inspection failure is a refusal. Native full-cache
removal retains Cargo lock files and the cache marker; package selection and
native package-clean locking remain owned by Cargo. Direct Cargo invocations
bypass budget maintenance and MUST NOT run concurrently with explicit cleanup;
repository workflows use `pnpm dev:cargo -- <Cargo arguments>`.

Temporary targets MUST be registered with
`pnpm dev:target:register -- --target .local/build-targets/NAME --reason TEXT`.
Legacy `.local/NAME-target` paths may be explicitly adopted. Active/retained
targets are never automatically removed. `dev:target:complete` marks a target
eligible for maintenance; `dev:target:retire -- --target PATH --apply` completes
and cleans it immediately. Successful qualification retires its temporary
target. Before forgetting a workspace, clean its idle build roots while
preserving code, data and evidence. Whole target-tree copies require an explicit
retained target and reason.

Initial qualification covers macOS ARM64, native locks, live consumers,
dependency retention, repeated cleanup and independent registered JJ builds.
Other platform qualification, application APIs, storage authority and installed
tool custody retain their separate gates under JST-DESK-001 and JST-CLI-004.
