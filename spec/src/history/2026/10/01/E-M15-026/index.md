---
schema: 1
id: E-M15-026
---

# E-M15-026

<!-- evidence-field:id markdown -->
## Id

E-M15-026
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

thread-main-workspace-landing-development-rebuild-readiness
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vysstvos
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main default workspace clean at xnopvwxk / e3a29a58 above oxunmxvt / 0915f828; isolated workspace clean above verified ultsumzo / 59f1cfe8. Both share the same main parent, so landing needs no rebase or conflict resolution. Main service PID 43304 serves the previous assets on 7460; unrelated Appearance work remains in its own workspace.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; inherited Design Core development snapshot 0.1.2 with manifest ^0.2.0; registry-only foundation qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Fast-forward main checkout to verified ultsumzo / 59f1cfe8 using jj new. Production source tree identical to E-M15-025; only this landing evidence is new. Rebuild main web bundle and gracefully restart existing owned development server with qualified binaries, existing config and persistent embedded RustFS volume; no reseeding or native/API format changes.
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
jj --no-pager st; jj --no-pager log -r '@ | @- | ultsumzo'; jj workspace list; scoped owning-specification and development-helper reads
```

**Notes**

Both target workspaces clean; qualified change already descends from main parent. Existing main listener 43304 confirmed owned by development runtime. Appearance and other workspaces untouched.
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
jj new ultsumzo -m "Land Thread presentation and restart main development server"; jj --no-pager st; jj --no-pager diff --from ultsumzo --to @ --name-only
```

**Notes**

Main working change vysstvos based directly on 59f1cfe8; 9 added / 32 modified files landed, no conflicts. Qualified source tree exactly equal before ledger recording. No redundant product suites: 127 direct story interactions/accessibility checks, focused units and cold browser are already qualified in E-M15-025.
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
pnpm web:build
```

**Notes**

Fresh main production bundle and asset manifest pass; existing chunk-size advisory only. /tmp/jj-thread-main-build.log. UI-only landing reuses existing Rust binaries; no Rust builds/tests or root aggregates.
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
node /tmp/jj-thread-main-restart.mjs
```

**Notes**

Uses owned stopPrevious, waitForServer, waitForSearch and processTreeMetrics helpers. Server 43304 -&gt; 18052, storage 18053; readiness 1717 ms. Two distinct native repository snapshots, complete selected Thread directory/detail/timeline, fixture and installation identity unchanged across restart. Content search HTTP 200, Search fresh/nondegraded. Served index exactly matches rebuilt main assets; SHA-256 2a71f1a027283b3d513b00d9a3902ad0648c818c3b20d50710f3d0957d7d7531. Runtime process metadata updated; no fixture writes or application content mutation. /tmp/jj-thread-main-restart.log, /tmp/jj-thread-main-before.json (private read-only capture), /tmp/jj-thread-main-ready.json.
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
cua_repl: bind existing tab 7; reload; open Thread settings and Toggle details; inspect accessibility state and capture screenshot
```

**Notes**

Initial tab binding timed out; documented browser tab inventory recovery succeeded. Reloaded main renders edited-message metadata and rightmost Thread menu. Thread settings and outer Repository details remain open simultaneously; lifecycle rows have icons and destructive Remove colouring. /tmp/jj-thread-main-landed.png is visual-pending, not baseline approval. No messages posted or command actions invoked.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Formatting passes; 119 task records, 5 validators / 21 chapters / 145 requirements pass. No protected implementation changes in landing evidence. Historical evidence unchanged. /tmp/jj-thread-main-ledger-format.log, format-check.log, progress.log, validate.log, first.log. Repeat final document consumers after this result recording.
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

Explicitly user-authorized main landing and dev-server rebuild/restart. Isolated Threads change and other workspaces preserved. Production code is unchanged from its complete E-M15-025 qualification. No package publication, installed CLI replacement, unrelated daemon restart, fixture reset, migration or visual baseline approval. Root aggregates, Rust builds/tests and independent M15 runner/full-flow/registry-only foundation qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the rebuilt main application at http://127.0.0.1:7460/. Keep the isolated workspace and independently pending M15 runner/full-flow and registry-only foundation qualification; visual baseline approval remains separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


