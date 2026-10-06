---
schema: 1
id: E-M15-018
---

# E-M15-018

<!-- evidence-field:id markdown -->
## Id

E-M15-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-development-rebuild-readiness
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tpkpymol
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Main source tree 1f6f222d including committed channel UI cea45f3a; existing local package snapshots as qualified in E-M15-016. Cargo incremental compilation disabled. Unchanged explicit embedded RustFS development binary reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, authority, schema or format edits. Existing LocalTrusted fixture, installation identity and RustFS volume preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Owned helper rebuilt web assets and jj-store CLI bins, restarted main, and confirmed Search ready. Server PID changed from 66628 to 27703. Repository 1bdb89c409c1b891ac7f180c01a265d5 is unchanged. Restart readiness took 4745 ms. /tmp/jj-thread-main-dev-restart.log.
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
node --input-type=module (initial served-index and Thread HTTP probe)
```

**Notes**

Index matched the built file, but the bare Node API request omitted LocalTrusted Origin/fetch metadata and correctly received 403. No policy or product changes were made.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module (served-index and Thread HTTP probe with the existing development helper headers)
```

**Notes**

Root and authorized Thread page return 200; 0 Threads loaded. Served index SHA-256 c3eb9bf5b8090ea5b9f4e846d97c42fdeab4911ba37fd7a02809c8fb88aa387c matches web/dist/index.html exactly.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl: reload existing browser tab, open navigation sidebar, select Browse threads, wait for loading to finish and capture screenshot
```

**Notes**

Live rebuilt UI exposes Threads sidebar section and its production empty channel directory with Refresh threads and New thread. Existing tab is left on Threads. Local screenshot /tmp/jj-store-threads-live.jpg remains visual-pending.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Evidence formatting and bookkeeping pass for 119 tasks; specification validates 4 validators, 21 chapters and 141 requirements. No protected production files changed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 (compare parent and current evidence); ps -p 27703 -o pid=,comm=; jj --no-pager st
```

**Notes**

All 229 previous evidence entries are identical. Owned main server remains running; only the evidence ledger changed.
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

<!-- evidence-field:scope markdown -->
## Scope

Authorized main dev rebuild/restart and read-only browser readiness only. No fixture reset, new Thread/message/reaction publication, dependency changes, unrelated daemon restart or visual baseline approval. This is not full backend/browser conversation or real-RustFS qualification; broader gaps remain in E-M15-016. No repeated product test suites are required for unchanged source.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue remaining M15 full-flow and released-package qualification. M15 and M15.5 remain In progress.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


