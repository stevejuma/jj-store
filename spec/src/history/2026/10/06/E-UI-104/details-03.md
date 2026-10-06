---
schema: 1
id: E-UI-104
---

# E-UI-104 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
tests: 10
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node .local/verify-current-agent-fixture.mjs
```

**Notes**

Repeated the same 10 admitted reads after the owned Vite/backend switch; data and transcript identities persist.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CUA: new background tab → Agents → Cursor; reload after cold restart and Vite startup
```

**Notes**

Three named session rows render; the retained Cursor selection reloads with “Update the parser”, “Updated the parser.” and three captured tool calls. Loading uses the existing skeleton; no Agent activity unavailable banner remains. Screenshot: ignored .local/agents-fresh.jpg. No screenshot comparison or baseline update.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
tests: 7
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Seven specification validators pass, 3,054 chapters and 178 requirements.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
tests: 3
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first
```

**Notes**

Both lanes rejected three evidence references: Storybook ID is not a supported named test reference, and two JavaScript files had no #test name. Corrected metadata to four actual Node test definitions; Storybook and executable fixture acceptance remain described in their actual run records.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
tests: 1
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

One attempt overlapped progress:history generation and inspected temporary history/navigation rewrites outside this slice. Completed the generator and reran sequentially; final working-copy diff contains only the intended M9.5/M18.1 histories.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Sequential final run passes: 144 tasks, 623 evidence records. Evidence bookkeeping only; no milestone promotion.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Protected-change gate passes: three protected source files, three canonical chapters; explicit unchanged API contract confirmation qualified.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Changed Markdown line/byte audit
```

**Notes**

Every changed authored/generated Markdown page is within 400 physical lines and 32,768 bytes. No unrelated whole-repository architecture lane.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tooling_failures list -->
## Tooling failures

- evidence:new initially failed because the older ignored dependency
  installation lacked yaml; frozen installation restored it and the command then
  created this record.
- An early ignored API probe ran before runtime.json existed (ENOENT); it was
  repeated after startup and passed.
- pnpm spec:validate -- --only governance --only fileLimits --only verification
  was rejected for the extra -- separator. Final validation uses the normal pnpm
  spec:validate command.
- Native development executables reject --version; recorded JJ compatibility is
  the pinned dependency profile rather than that unsupported invocation.
- Initial progress/spec-first runs rejected unsupported story/unnamed
  test-reference syntax in this new evidence entry. Corrected evidence
  references; no test or product assertion was weakened.
- A progress check overlapped task-history generation and saw temporary
  unrelated history/navigation edits. Once generation finished, the normal check
  passed sequentially. No unrelated progress files were included in the final
  diff.
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

macOS ARM64; Node 26.9.0, pnpm 10.32.1, Rust 1.98.1, Git 2.55.0; pinned JJ 0.45.1 dependency profile; Svelte 5.57.1, Vitest 4.1.11. Synthetic fixture profiles: Codex 0.156.1, Cursor 3.22.7, OpenCode 1.18.30. Actual installed-harness execution is not qualified by this fixture.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Current transcript_tail_page schema 2 under the existing TranscriptPage kind. AgentRootV1, chunks, manifest and API DTO encodings remain unchanged; no migration/compatibility reader. Fixture/runtime metadata formats 1/2 are unchanged. AgentStore transcript DTO version 1 is distinct from stored tail schema 2.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only contained notice: 16px side inset matching the header, 8px vertical margin, 0.45rem radius and 1px complete border. Shared column alerts and notification/error semantics are unchanged. Fresh data lives only in the ignored session workspace root .local/current-format-demo; native-local repo d945d53727f0a0f3c92f583e0a4271f7. Old main fixture and main uncommitted source work are preserved. Native compilation was required solely to generate current-format fixture data, not because Rust source changed. Root aggregate lanes, unrelated Rust tests/architecture, live harness capture, TLS deployment qualification, package publication, screenshot baselines and main integration are outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the separate session fixture at http://127.0.0.1:7464/ for further review. Future commands for it must retain the explicit JJ_STORE_DEV_ROOT and port. Keep existing main data separate; any future integration remains a new request.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-UI-005 and JST-COLLAB-005 already require reviewed owned components
  and accessible failure/loading presentation; this refinement clarifies
  contained banner geometry without changing state or interaction contracts.
  JST-COLLAB-023 still supports only schema-2 tails and rejects schema-1: new
  generation exercises the existing encoder, not migration. JST-API-005
  transport/schema, snapshot/error values and ownership remain unchanged.
  JST-DOC-002 mapped source acknowledgement was explicitly refreshed. The owning
  presentation/tail chapters clarify these existing obligations."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M9.5 remains Done and M18.1 remains Done; their existing gates are
  retained with focused banner and fresh current-tail fixture evidence. M18
  remains In progress and its installed Desktop/live-harness and deferred
  release gates are not promoted. UI/browser verification does not approve
  visual baselines.

```
<!-- /evidence-field -->

