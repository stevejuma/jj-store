---
schema: 1
id: E-UI-056
---

# E-UI-056

<!-- evidence-field:id markdown -->
## Id

E-UI-056
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

appearance-workspace-rebase-development-switch
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change klylkzvs
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

User requests reuse of the Appearance workspace, rebase onto committed main and transfer its development service. Appearance is clean at 1c53e7bd above 68a3dbdc. Main's committed parent is nxoksqrt/faf52a70, while its wtqyykvt working change contains unrelated unfinished Thread UI work. Owned server471/storage561 serves the persistent LocalTrusted demo at port7460. Main concurrently rewrote nxoksqrt to ba74e1ea with a bounded-cache correction and rebuilt its executable; Appearance was refreshed to that final committed parent.
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
jj --no-pager st (main and appearance); jj --no-pager log -r '(@ | @- | default@ | default@-) | latest(::default@-, 6)' --limit 12; jj rebase -r @ -o nxoksqrt; jj --no-pager st; jj --no-pager diff --from @ --to default@- --summary
```

**Notes**

Rebased only the clean Appearance working change klylkzvs onto committed main faf52a70; resulting tree equals default@-. No conflicts and no unfinished main changes included. Main tracked files and working change are not rewritten.
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
pnpm web:build
```

**Notes**

Appearance workspace production UI builds successfully; existing chunk-size and plugin-timing advisories only. /tmp/jj-appearance-workspace-build.log.
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
jj --no-pager diff --summary -- crates runtime Cargo.toml Cargo.lock (main); node /tmp/jj-appearance-workspace-start.mjs; lsof -nP -iTCP:7460 -sTCP:LISTEN
```

**Notes**

Main has no uncommitted Rust/runtime changes. Copied its existing qualified executable to ignored Appearance target/debug/jj-store and verified identical SHA256; no Rust rebuild. Owned graceful shutdown and startup switch471-&gt;10251/storage10269 with cwd and executable under Appearance,3121ms readiness. Existing main .local/dev-demo state stays in place; its ignored config now serves Appearance web/dist. Snapshot, Thread directory, fixture hash and installation hash are unchanged; search ready. HTTP200 index equals the new build, SHA25605d5ac0be02b14c7a0e7f92d77fe09a9490676a402390d5e0ba58bcecc7e8180.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cua.getTab(existing in-app browser URL)
```

**Notes**

Optional refresh inspection timed out in CDP Emulation.setFocusEmulationEnabled. No browser reload or preference change performed; service readiness and served bytes are independently verified.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:validate (initial attempts)
```

**Notes**

Both checks encountered the stale workspace after concurrent ancestor rewrite. Prettier passed. Refreshed the workspace before rerunning these exact checks; no validator suppression.
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
jj workspace update-stale; jj --no-pager log -r change_id(klylkzvs); jj --no-pager diff --from faf52a70 --to ba74e1ea --summary; jj file show -r e138ae9d spec/src/milestones.md
```

**Notes**

Automatic ancestor-rewrite resolution left an empty Appearance revision and the evidence edit in a divergent revision. Recovered only E-UI-056 into the fresh ledger, preserving upstream changes. Latest committed parent ba74e1ea changes Rust cache behavior and documentation; no UI changes, so served UI build remains current.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node /tmp/jj-appearance-workspace-start.mjs (latest committed backend refresh)
```

**Notes**

Refreshed native executable from main after its committed cache correction/rebuild, using atomic ignored binary replacement. Appearance service10251-&gt;13961/storage13973,2931ms readiness. Repeated snapshot/directory/fixture/installation equality, built-index equality and search-ready checks pass. No UI source changed in the concurrent rewrite and no Rust build was run in this slice.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Evidence-only ledger formatting and consuming governance checks pass; no product requirements or milestone status change.
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

User-authorized JJ workspace rebase and local development service transfer. UI bundle rebuild and read-only cold readiness/state preservation checks are the complete operational surface. Rust builds/tests, root aggregates, unrelated suites, package publication, installed CLI changes and visual baseline approval are out of scope. No fixture reset/reseed or native application mutation. M16 remains Partial with E-UI-055's separate release, registry-only, inherited suite/composer-CSP and visual gates retained.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; pnpm10.32.1; committed main nxoksqrt/ba74e1ea; existing unchanged development server and embedded RustFS binaries.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing development runtime PID/resource metadata now records Appearance workspaceRoot/webRoot; asset manifest1 rebuilt. Native state, installation identity, fixture format1 and appearance preference jj-store.appearance.v1 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use Appearance workspace and refresh http://127.0.0.1:7460/ for subsequent UI changes. Persistent runtime is still main .local/dev-demo; rebuild Appearance web assets when changing the served UI. Keep unrelated main work separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Private /tmp/jj-appearance-workspace-before.json and ready.json; /tmp/jj-appearance-workspace-{build,start}.log; temporary start helper /tmp/jj-appearance-workspace-start.mjs. New Appearance server remains running as requested; previous main server is stopped.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


