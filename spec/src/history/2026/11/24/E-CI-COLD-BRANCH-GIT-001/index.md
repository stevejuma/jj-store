---
schema: 1
id: E-CI-COLD-BRANCH-GIT-001
---

# E-CI-COLD-BRANCH-GIT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-BRANCH-GIT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
- M12
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GIT-008
- JST-CLI-001
- JST-API-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-24
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/real/native-repository.spec.ts
- web/tests/real/project-polish.spec.ts
- web/tests/real/git-bridge.spec.ts
- crates/jj-store-cli/src/git_bridge/transport.rs
- docs/maintenance/coverage.json
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/src/git_bridge/transport.rs#cache_monitor_accounts_for_entries_after_a_git_lockfile_rename
- crates/jj-store-cli/tests/executable_native.rs#native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 45394305325
```

**Notes**

Five browser failures in shard 2, one in shard 3, one in the RustFS Git-cycle browser and one native Git fetch in targeted shard 2. Two downstream aggregate jobs also failed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 45374255679
```

**Notes**

No complete scheduled success within 48 hours.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --lib cache_monitor
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_SHARD=2/4 pnpm web:test:real (first selector fix)
```

**Notes**

Six passed, one skipped. Divergent history still looked up full identifiers by obsolete copy-button names.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_SHARD=2/4 pnpm web:test:real
```

**Notes**

Fresh fixture, seven passed and one existing skip. Compared full commit and change identifiers using their accessible descriptions.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_SHARD=3/4 pnpm web:test:real
```

**Notes**

Fresh fixture, eleven passed and one existing skip. Both shards used the exact web bundle from run 45394305325.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --lib cache_monitor_accounts_for_entries_after_a_git_lockfile_rename (original error propagation)
```

**Notes**

Deterministic ENOENT when a previously listed lockfile is renamed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm client:build && pnpm exec svelte-check --tsconfig web/tsconfig.json
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
node scripts/docs.mjs check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Linux x86_64, pnpm 10.32.1, checksum-verified Git 2.55.0, main 026f948c.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing cache and repository formats.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cold browser acceptance selects bookmark options by the complete leading bookmark name. Bounded Git cache scans tolerate disappearing descendants during atomic Git updates and continue to enforce root existence, byte, entry and symlink checks.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Focused regressions and both full cold shards passed locally. Verify hosted full qualification and scheduled freshness after integration.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 and JST-GIT-008 retain their release gates and bounded Git
  transport requirements.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains in progress. M12 retains the existing qualified Git profile.

```
<!-- /evidence-field -->

