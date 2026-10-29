---
schema: 1
id: E-REUSABLE-AGENTS-015
---

# E-REUSABLE-AGENTS-015 details 3

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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- package -p jj-agent --list --allow-dirty --locked --offline
cargo fmt -p jj-agent -p jj-store-git -- --check
```

**Notes**

Offline package inventory includes both new production native_capture modules and the regression test, with the crate README. Both affected Rust packages pass formatting. This is an inventory check, not independent package installation or release qualification. native-store-package.log.
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
pnpm check:progress && pnpm spec:validate && pnpm spec:first
```

**Notes**

After correcting evidence references, progress passes 171 tasks/886 records; seven validators/4,246 chapters/207 requirements and spec-first 14 protected files/four canonical chapters pass. Result was pending until the manual/rebuild/restart gates above finished; those now pass. Original M33 statuses and gates remain unchanged.
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
pnpm progress:history
pnpm check:progress && pnpm spec:validate && pnpm spec:first
pnpm exec prettier --check crates/jj-agent/README.md docs/maintenance/coverage.json docs/maintenance/reusable-agent-workflows-progress.md docs/site/technical/capture.md docs/site/concepts/collaboration.md spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/collaboration/18-agentstore-object-foundation.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/progress/M33/index.md spec/src/current/progress/M33/history/M33/index.md spec/src/current/progress/M33/history/M33.3/index.md spec/src/current/progress/M33/history/M33.4/index.md
```

**Notes**

Final root gates pass: 171 tasks/886 records; seven validators/4,247 chapters/207 requirements; spec-first 14 protected files/four canonical chapters. All 13 scoped Markdown/JSON files pass formatting. Restored 541 unrelated generator projections. Evidence stays within three authored pages, each below 400 lines/32,768 bytes. NOTES-07 remains sole Active; NOTES-07A is the qualified prerequisite and no downstream conversation/restart gate advances.
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

Enclosing Jujutsu change tusoqzyw, based on 9dd09679. Rust 1.99.0, pnpm 10.34.5, macOS ARM64. Concrete Git provider uses gix 0.87.1. Notes source 51bed526; Lapis AI source 8a8f9149. No production dependency version changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds supported owned-controller@jj-native-capture-1/v1 and a bounded decoder for the existing native schema-1 envelope. Retains policy/binding/ordinal/publication/frame bytes as one sanitized observation. The existing AgentObjectV1, Thread records, Git owned-ref CAS and provider receipts retain their canonical formats; controller-v2 behavior is unchanged. No new wire API, repository marker or migration.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

NOTES-07A domain/normalization prerequisite, with a real plain-Git companion provider. One ordinal orders a complete native group; original native cursors and full-width sequences remain evidence. Deferred prefixes release text and do not replay old tool/lifecycle metadata. Error and closure states remain unknown and ended. This is not a Notes native sink, journal/backend selection, native writer drain or actual JJ-vault conversation/restart proof. No UI/story/visual, installed provider, native RustFS or release gate is inferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement the reusable receipt-aware native-capture/store adapter with injected durable intent journal and selected Agent/Thread participants. Connect explicit per-vault Notes backend ownership, then complete NOTES-06C4/06D and NOTES-08/09 application/native restart acceptance. NOTES-07 is the only Active task.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007/JST-COLLAB-033 gain an additive native-capture profile with
  exact envelope/progress validation and existing domain projection. Owning
  reusable-workflow, store and AgentStore chapters document the accepted profile
  and unchanged legacy formats. JST-DOC-001 source review and JST-GOV-002/011
  evidence/milestone rules remain intact.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: NOTES-07A completes only the domain/normalization prerequisite. NOTES-07
  remains sole Active; NOTES-06C/06C4 wait for the actual backend. M32 stays
  Done; M33/M33.1-M33.4 In progress and M33.5-M33.8 Planned. No Notes
  journal/backend, managed JJ conversation, writer drain, cold restart,
  provider, platform, visual or publication gate is completed.

```
<!-- /evidence-field -->

