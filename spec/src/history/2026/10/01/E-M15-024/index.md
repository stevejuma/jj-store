---
schema: 1
id: E-M15-024
---

# E-M15-024

<!-- evidence-field:id markdown -->
## Id

E-M15-024
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
- M15.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

landed-main-development-rebuild-readiness
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oxunmxvt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main is clean at empty oxunmxvt / 77cb2f47 above landing fee3a172, with qualified implementation msszrzpn / 02f94c13. Development server PID 23243 serves the preceding UI build on port 7460. Installed dependency links still include Community and omit direct Mira/bits-ui dependencies from the landed manifest.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vite 8.3.1; Svelte 5.57.1; registry Mira 0.0.2 and bits-ui 2.18.1. Inherited Design Core 0.1.2 local development substitution retained, not registry-only foundation qualification. CARGO_INCREMENTAL=0; unchanged explicit embedded RustFS development executable reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, generated contract or storage format edits. Rebuilt main web bundle and development CLI/server binaries. Existing native snapshot, immutable Thread records, fixture and installation identity preserved; additive Thread directory metadata now served. Source tree qualified in E-M15-022/E-M15-023 unchanged.
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
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build (main preflight)
```

**Notes**

Build stops resolving @lapismd/mira/styles.css because main dependency links predate the landed manifest; /tmp/jj-main-alignment-prebuild.log. No product or lockfile change to work around it.
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
pnpm install --offline --lockfile=false --ignore-scripts
```

**Notes**

Refreshes local dependency links from cache, removes Community, links registry Mira 0.0.2 and bits-ui 2.18.1. Tracked lockfile and package manifest unchanged; /tmp/jj-main-alignment-install.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
curl --silent --fail http://127.0.0.1:7460/v1/bootstrap (initial probe)
```

**Notes**

Bare probe omits LocalTrusted Origin/fetch metadata and returns 403 as required. Corrected metadata-bearing Node probes pass; no policy change.
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
node --input-type=module (metadata-bearing pre-restart snapshot, Threads, bootstrap and fixture probe)
```

**Notes**

Captures native snapshot, immutable Thread page, fixture and installation hash for before/after comparison; private local evidence /tmp/jj-main-restart-before.json.
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
CARGO_INCREMENTAL=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Owned helper rebuilds assets and CLI/server bins, gracefully restarts development service from PID 23243 to 43304, reuses embedded RustFS with PID 43321, and reports Search ready. 4437 ms restart readiness; no fixture reset. /tmp/jj-main-alignment-dev-restart.log.
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
pnpm web:check; pnpm check:ui-ownership
```

**Notes**

0 errors/warnings; ownership validator passes after dependency refresh. /tmp/jj-main-alignment-web-check.log and /tmp/jj-main-alignment-ownership.log.
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
node --input-type=module (initial post-restart readiness probe)
```

**Notes**

Exact old HTTP Thread response comparison flags the planned additive directory metadata. Immutable snapshot/records remain equal; corrected probe separately requires new metadata and compares complete prior record/response shape.
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
node /tmp/jj-main-restart-readiness.mjs
```

**Notes**

Native snapshot, immutable Thread page, fixture and installation hash match before state; directory metadata appears and content search returns 200. Search is fresh/nondegraded. Served index exactly matches rebuilt web/dist/index.html, SHA-256 42d5316b99388a61911ec033b613e3f3ef2b1d8fefb8cc4af1e1dc3eb9211e1d. /tmp/jj-main-restart-ready.json.
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
cua_repl: select existing tab 5, reload and read accessibility state; save rebuilt UI screenshot
```

**Notes**

Existing browser tab reloads the built UI and renders Threads toolbar/listing and navigation without repository tabs. User-selected server-only repository has no threads. Screenshot /tmp/jj-store-main-restarted.jpg is visual-pending; no data publication or baseline approval.
<!-- /run -->

<!-- run -->
### Run 10

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

Documentation-only evidence formatting and governance checks; previous product qualification remains applicable to unchanged source.
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

Explicit user-authorized main development rebuild/restart and read-only readiness. No fixture reset, application content mutation, installed CLI replacement, package publication, unrelated daemon restart, repository migration or visual baseline approval. Real-RustFS restart readiness is distinct from remaining M15.7 full-flow/runner qualification. Root aggregates, unrelated product suites and full Rust workspace remain out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the rebuilt application at http://127.0.0.1:7460/. M15/M15.5 retain In progress; keep independent visual and remaining full-flow qualification gates explicit.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


