---
schema: 1
id: E-CHAT-WORKSPACE-004
---

# E-CHAT-WORKSPACE-004 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Binding visibility and non-Send upstream JJ inspection required the owned blocking/current-thread boundary.
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
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Controller, service and additive model compile.
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
cargo test --locked -p jj-store-cli --test agent_execution workspace_comments -- --nocapture
```

**Notes**

Initial isolated local flow passed, then expanded stale-file and exact answer-link flow passed: 1 test per run. Two Unicode comments share one discussion; publication/context, duplicate send and restarted receipt reads retain identities. Stale Send preserves checkout state and publishes nothing.
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
cargo test --locked -p jj-collab --test conversation_submission --lib
```

**Notes**

One model unit test and six existing conversation submission regressions pass.
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
pnpm contracts:generate
```

**Notes**

Regenerated additive optional workspace draft contracts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-session.test.ts
```

**Notes**

6 existing draft lifecycle tests pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workspace-browser.test.ts web/tests/vite/conversation-session.test.ts
```

**Notes**

9 tests pass, including draft source conversion, pending publication and stale asynchronous reads. Initial controller/UI source tests only; historical integration and cold acceptance remain incomplete.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial in-progress UI integration needed public inline generated types, FileListing modes, optional browser access and a Svelte rune variable rename.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

FileListing also supports package mode; use the public type.
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
pnpm web:check
```

**Notes**

Zero errors/warnings after fixes. Full browser interaction remains subsequent work.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native file-draft integration, multi-recipient partial/unknown recovery,
  queued cancellation, durable local turn snapshot retention and empty
  whole-file diff context still need focused qualification.
- Shared repository file reader reuse remains outstanding. The initial workspace
  rail scope leak is handled by the following UI correction slice. Initial UI
  integration is included; historical review projection, exact reload
  navigation, browser interaction/accessibility and cold native/local acceptance
  remain required.
- Initial live browser interaction is verified; complete ResizeObserver/cold
  flows remain pending and visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pnpm 10.32.1; cargo 1.98.1; JJ 0.45.1. Existing macOS linker __eh_frame warning remains. Enclosing Jujutsu change is the revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional omitted-empty workspace_comments; journal maps for deferred prompt, frozen capture and per-draft publication. Existing canonical submission encodings and authority retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Eligible queued capture, exact Unicode sources, stable independent ReviewStore publication and home context dispatch. No fixture reset or main workspace source edits. A backend restart was explicitly requested after the initial verification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete retained history projection and turn evidence, finish shared UI integration, then cold native/local acceptance.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Additive draft carrier requires controller capture before ordinary
  publication; exact snapshots and independent receipts retain authority and
  unknown-outcome semantics.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M13.3 remain In progress. Passing controlled local fixtures do
  not qualify remaining native, multi-agent, browser or visual gates.

```
<!-- /evidence-field -->

