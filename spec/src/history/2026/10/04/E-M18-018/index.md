---
schema: 1
id: E-M18-018
---

# E-M18-018

<!-- evidence-field:id markdown -->
## Id

E-M18-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.2
- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wqvwruzt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main workspace backend rebuild and settings review startup
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Integrated settings d269d6de and capture b5942af7; macOS ARM64 development binaries; existing main embedded-RustFS fixture; Vite HMR at7460 with API7462.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository/schema/configuration change. Existing native fixture, saved configuration and Vite process retained. Only main checkout development backend replaced.
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
jj st/log in default and cursor-opencode workspaces
```

**Notes**

Default already clean on b5942af7, matching feature committed parent. No second merge or history rewrite required; parked OpenCode history remains separate.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules pnpm dev:backend:build
```

**Notes**

Main development CLI/backend build passed in3m30s. macOS linker __eh_frame warning retained. /tmp/jj-cursor-main-review-build.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules pnpm dev:backend:restart
```

**Notes**

Main backend18306 replaced by56370; Vite99289 retained. Main URL http://127.0.0.1:7460/; search ready. Main fixture and ui-server.toml bytes unchanged; same repository1bdb89c409c1b891ac7f180c01a265d5. Preview7470/7472 and other services untouched. /tmp/jj-cursor-main-review-restart.log.
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
Production HTTP capture/status/settings reads with original Origin/Fetch Metadata/protocol admission
```

**Notes**

Old running backend returned404 for capture/settings before rebuild; new backend returns200 for both. Desired/effective configuration and resolved defaults available. /tmp/jj-cursor-main-review-api-proof.json.
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
Cold review-page reload; Settings > Agents > Configuration CUA inspection
```

**Notes**

Verified Codex, Cursor Desktop/CLI and OpenCode sections, editable switches/budgets, resolved roots, endpoint, saved/effective state and action layout. Capture remains opt-in Disabled; no configuration save or model prompt sent. User-visible review tab retained. /tmp/jj-cursor-main-settings-review.png.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
GET /v1/repos/1bdb89c409c1b891ac7f180c01a265d5/agent/sessions after backend restart
```

**Notes**

Existing main fixture Agent sessions return503 {version:1,code:unavailable}; repeated read and UI confirm unavailable. Previous binary displayed3 seeded sessions. No reset, migration or deletion performed. This is a retained-fixture upgrade diagnostic, not a new-harness capture acceptance claim.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Investigate existing main fixture Agent-session503 without resetting or
  silently migrating retained history.
- Real Cursor Desktop cold qualification remains pending as recorded in
  E-M18-017.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Authorized local main-workspace review restart only. Prior implementation/qualification evidence unchanged; no milestone promotion, root aggregate lanes, fresh harness prompts, installed CLI replacement, publication or fixture reset.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the retained main settings tab at7460. Diagnose the existing fixture Agent-session read failure separately from passing configuration startup; preserve current-only AgentStore policy and retained user data.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


