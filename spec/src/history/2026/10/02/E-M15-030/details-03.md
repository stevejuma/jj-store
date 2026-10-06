---
schema: 1
id: E-M15-030
---

# E-M15-030 details 3

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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-reaction-row-slots-main node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Final dedicated-slot main rebuild/restart: PID52603-&gt;58258/storage58259,1564ms. Repository snapshots, selected thread/messages, thread directory, fixture and installed binaries unchanged; search ready/content HTTP200. Served index matches rebuilt SHA25695a71cd6b70804f45e57eac9d80bdfcb451d3b985db645039d0d4a04c4b400e9. /tmp/jj-reaction-row-slots-restart.log.
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
CUA main-tab reload and read-only verification; CUA rejected-reaction Storybook screenshot
```

**Notes**

Main tab8 restores selected test thread, unchanged message, timestamp/Edited, reply avatar/count and empty composer after reload. Storybook tab10 visibly shows timestamp, icon-only retry and destructive Failed on one row. No real message/reaction mutation. Storybook6046 and main7460 retained. /tmp/jj-store-reaction-metadata-row-final.png.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/api.md spec/src/ui.md spec/src/milestones.md
```

**Notes**

Canonical UI/API and ledger consumers pass; final pre-ledger verification counted15 protected files/3 chapters,5 validators/21 chapters/146 requirements and120 tasks. All18 changed files also passed targeted formatting. /tmp/jj-reaction-row-{spec-first-final,spec-validate-final,progress-final,format-final}.log. Retained document checks after ledger finalization: /tmp/jj-reaction-row-ledger-{spec-first,spec-validate,progress,format}.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Requested metadata slice is implemented and its3 exact regressions,25
  controller tests,2 cold browser tests, source/build/ownership and live restart
  checks pass. Broader direct-consumer acceptance remains29/31 because the
  existing Native copy-submenu and intermittent Appearance-story failures remain
  unresolved.
- Visual baseline approval remains separate; stories remain visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only owned message row/reaction presentation. Shared CommunityMessage composition justifies complete direct Thread/Agent/Review/WorkStore message families and Appearance discussion; no shared test infrastructure changes. Rust checks/builds/tests, wire generation, root aggregates and unrelated full-suite lanes are out of scope. Native binaries reused without fixture reset/reseed, installed CLI replacement, package publication or unrelated service restart.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Reaction feedback uses a dedicated owned metadataSupplement/messageMetadata slot after timestamp/edited marker and reply avatars/count. General supplements retain their original placement, preserving copy/action presentation. Controls and feedback share controller caches/deduplicated loads. Recovery is icon-only; targeted pending action uses standard Spinner; error uses destructive CircleX/Failed and keyboard-accessible shadcn details. Transient recovery controls use native titles. Request/receipt, native/WAL, snapshot, wire and persistence formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; pnpm10.32.1; Svelte5.57.1; BitsUI2.18.1; installed DesignCore0.1.2 through existing ignored local validation hook, manifest registry range^0.2.0. No dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused metadata acceptance and original status request identity. Independently qualify the existing NativeMessageActions copy submenu and Appearance discussion story synchronization before claiming complete direct-consumer acceptance; retain M15/M15.5 In progress and visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-reaction-row-*.log; private restart proof /tmp/jj-thread-reaction-row{,-slots}-main-{before,ready}.json; /tmp/jj-store-reaction-metadata-row-final.png and /tmp/jj-thread-metadata-{main,reply}.png. Isolated browser_preview process stopped after acceptance; rebuilt main development service and user-facing Storybook error tab remain running.
<!-- /evidence-field -->

