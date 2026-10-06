---
schema: 1
id: E-M18-001
---

# E-M18-001 details 7

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
LocalTrusted Capture control/API inspection and CUA pause/resume journey
```

**Notes**

Paused revision1 withheld delta; paused backend restart retained identical detail; resumed revision2 published delta exactly once; malformed manually constructed31-character Resume ID was refused before valid original request126f0289e9f94ba4b2917ba375fc0990
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
CUA Import history for source4; inspect original3c9396b517f145568776150354ba5a78
```

**Notes**

Click selected deliberate unlinked trial. Unknown acknowledgement retained; no accepted receipt or body publication. Receipt absence is not definitive rejection. Source-selection/refusal UX remains deferred
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
rustc .local/integration/trial-source-order.rs; .local/integration/trial-source-order
```

**Notes**

Production metadata scanner reproduces known trial order; reads only the four deliberately-created trial headers; source2 is linked history and source4 is unlinked
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
POST /v1/local/capture/import using .local/integration/linked-history-command.json; GET original import status
```

**Notes**

Distinct explicit linked command aeadff33d0fc4d7ca668cc781e23565b accepted200; source2; ordinal1; exact79990-byte/15-source-record boundary; completed with one publication receipt
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
Python LocalTrusted detail/transcript assertions for three real admitted sessions
```

**Notes**

Native sessions41ffb0d2c68320aeb63a8713fe38d6a6,8237dd0f725d7102301bff779b9948d0,a05168b21f11ad2757c9ec6553e3f483; record counts13/5/3; distinct response markers once; unlinked-refusal bodies absent
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:capture:restart (final retention)
```

**Notes**

All three session/snapshot/transcript identities and record counts unchanged after resumed restart; original completed import receipt retained; transient409 busy while cold recovery is recorded
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
sample preview backend1second; CUA selected Agents and scrolled virtualized transcript; saved capture-preview.jpg
```

**Notes**

Read-only diagnostic shows repeated owned import-policy work; UI visibly retains gamma/delta and six sessions; preview tab retained as deliverable
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
pnpm contracts:check (qualified final)
```

**Notes**

Complete registered103-route inventory, actual HTTP/SSE samples, seven contract-tool tests and generated-client TypeScript; shared route generation required full directly affected consumers 221 actual HTTP/SSE responses conform.
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
Python raw evidence comparison against both latest-main integration snapshots
```

**Notes**

392 historical raw objects and payloads preserved;393 total including E-M18-001
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
pnpm check:progress (first close attempt)
```

**Notes**

Pass evidence cannot carry deferred gaps in remaining; moved those unchanged caveats to scope because narrow M18.8 gates are qualified and broader M18 is incomplete.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm spec:validate (sealed handoff documents/ledger)
```

**Notes**

446 protected paths/14 canonical chapters; five validators/22 chapters/166 requirements
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check docs/capture-preview.md docs/implementation-plan.md spec/src/verification.md scripts/dev-capture.mjs spec-validator.config.mjs web/tests/vite/capture-management.test.ts catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

All final edited documents and regression surfaces formatted
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (after keeping deferred scope distinct from narrow preview acceptance)
```

**Notes**

135 tasks; M18.8 Done, broader M18 In progress with Partial/Deferred children
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
GET /v1/local/capture with bounded exact read retries after final backend restart
```

**Notes**

Four attempts observed retained generation3957df423c2f9c4f152bf50d877e70c4/revision2/pausedfalse; transient409 busy remains a documented cold observation
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CUA reload/navigation and disposable preview cleanup
```

**Notes**

Original unlinked unknown request/command remains saved in .local/integration and ledger; no rejection inferred. Cleared trial-only volatile view by reload; main Projects/Agents navigation and captured responses remain usable. Locator failure after collapsing rows was recovered with fresh AX target.
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

JJ 0.45.1; Codex CLI 0.156.1; pnpm 10.32.1; committed main90d119e0; temporary inherited Design Core0.1.2 local source; registry manifests/lock preserved; pinned official Codex source b412ff32c417f855c2b2d1581b77058eed87c84b
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Current AgentStore only; native/capture formats unchanged; combined OpenAPI103 routes(30 local,73 other); historical M17 capture task fields preserved via explicit M18 mapping;392 historical evidence objects including latest main E-UI-094 preserved plus this entry.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

New codex-preview workspace integrated onto latest committed main90d119e0. Original capture stack/main working copy/OpenCode draft preserved. Functional CLI-only native preview, automatic UI freshness, isolated repeatable setup and real native RustFS/edit/history/restart evidence. Root aggregates and full roadmap qualification out of scope. Deferred limitations: Codex Desktop, OpenCode configured/history/export, Cursor, complete conversations/replay, automatic provenance/checkpoints, advanced review/search/archive, broader release/platform/deployment remain Deferred. Cold debug native sanitizer/preparation takes more than two minutes; performance optimization Deferred. Body-free history source ordinals can select an unlinked trial; admission withholds it but unknown pre-acceptance command can remain unresolved. Source-selection and deterministic refusal UX Deferred; retain exact original request. Registry-only Design Core UI release qualification and visual baselines remain open; local preview uses inherited documented temporary validation hook.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Try the running CLI-only preview at http://127.0.0.1:7470/ using docs/capture-preview.md; gather feedback before reopening deferred work.
<!-- /evidence-field -->

