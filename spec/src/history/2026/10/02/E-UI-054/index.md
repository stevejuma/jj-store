---
schema: 1
id: E-UI-054
---

# E-UI-054

<!-- evidence-field:id markdown -->
## Id

E-UI-054
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-appearance-development-rebuild-restart
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xzsyulwq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

User explicitly requests rebuild/restart after local Appearance landing. Main is clean above a9747102; owned development server PID31084/storage31085 serves loopback HTTP200 at port7460 with existing persistent fixture.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
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
pnpm web:build
```

**Notes**

Main fresh UI bundle rebuild passes; inherited large-chunk advisory only. /tmp/jj-appearance-dev-main-build.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-appearance-main node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Reuses inspected, owned stopPrevious/waitForServer/waitForSearch/processTreeMetrics helpers and explicit target/debug/jj-store executable/config. Server31084 -&gt;36884, embedded storage36885, readiness1127ms. Native snapshots, Thread directory, selected Thread/messages, fixture hash and installation hash unchanged. Search ready and content search HTTP200. Served index exactly matches rebuilt main, SHA256 7fd3699ab3bbc57f4bcdd11fdb0e2d708b7c831d0757cae3af0cf4dea59c262d. /tmp/jj-appearance-dev-main-restart.log and /tmp/jj-thread-appearance-main-ready.json.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node .scratch/appearance-live-main.mjs (three preliminary attempts)
```

**Notes**

Ignored ad hoc live harness first used an ambiguous heading locator matching navigation/body. Scoped to the Appearance settings region. The next two attempts, including diagnostic capture, incorrectly classified read-only local WorkStore POST /work/browse queries as mutations. Corrected to the canonical query boundary and added before/after native snapshot equality; no production/test suppression. /tmp/jj-appearance-dev-main-browser-before.log, before2.log and before3.log.
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
node .scratch/appearance-live-main.mjs
```

**Notes**

Fresh headless context against the actual restarted service: Appearance visible, Nord theme and Larger font applied, reload retains preferences, zero repository mutation requests and unchanged native snapshots. No mock APIs. Device preferences live only in disposable browser context. This is not a global mounted-composer/CSP qualification. /tmp/jj-appearance-dev-main-browser.log and browser.json.
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
pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

120 tasks, five validators/21 chapters/146 requirements and changed-ledger formatting pass. /tmp/jj-appearance-dev-main-progress.log, spec-validate.log and format.log. This final evidence-only annotation is consumed again before commit; product lanes are not rerun.
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

pnpm10.32.1; JJ0.45.1; main a9747102/Appearance68a3dbdc; local Design Core yovkqwtu source through existing ignored hook; unchanged qualified development server and embedded RustFS binaries; current installed Playwright1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI assets rebuilt with existing asset-manifest version1. Existing development runtime PID/resource metadata refreshed; fixture, installation identity, repository snapshots and selected Thread/message state preserved. Appearance preference format jj-store.appearance.v1 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-authorized main browser-bundle rebuild and graceful restart of its owned development server/managed storage, with live read-only readiness and isolated device-preference smoke. UI-only source changes reuse qualified native binaries; Rust rebuild/tests, root aggregates and unrelated suite/service lanes are out of scope. No fixture reset/reseed, native application mutations, installed CLI replacement, unrelated daemon restart, package publication or baseline approval. M16 remains Partial; E-UI-052/E-UI-053 own outstanding release, registry-only, inherited suite/composer-CSP and visual gates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use http://127.0.0.1:7460/ for the rebuilt main app. Keep independent M16 qualification gates explicit.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Private mode0600 /tmp/jj-thread-appearance-main-before.json and ready.json; /tmp/jj-appearance-dev-main-*.log; ignored .scratch/appearance-live-main.mjs; persistent state .local/dev-demo. Live main server remains running as requested; temporary browser context closed.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


