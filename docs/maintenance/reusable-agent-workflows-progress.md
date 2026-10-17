# Reusable agent workflows: delivery checklist

Last reviewed: 2026-10-17. Overall: **In progress**.

The first usable milestone is **Notes showing pinned JJ history/diffs and one
recoverable AI conversation persisted through AgentStore and ThreadStore**. It
is still pending. Notes' native repository admission and history transport are
verified. The visible history/diff panel is in progress, followed by durable
conversation capture.

This is the delivery checklist for the approved eight-stage plan. The
[M33 milestone ledger](../../spec/src/current/progress/M33/index.md) owns formal
status and gate selections. Notes' consumer gates live in the sibling
`lapis-desktop/spec/src/current/progress/milestones.md` under M13. Checked items
mean that the named slice has evidence; they do not complete its whole stage.

## Current checkpoint

- **Completed foundation:** injectable stores, crate READMEs, redaction/source
  acquisition, conversation primitives and SDKs, history libraries and native
  Notes admission/transport.
- **Active task: NOTES-04A.** Qualify the shared Notes API dependency required
  by the history view, including the remaining wider checks.
- **Waiting: NOTES-04 and NOTES-05.** The history view and review scenarios
  exist in isolated source workspaces; their acceptance and integration depend
  on source qualification and the remaining Design Core contrast repair.
- **Next product milestone:** NOTES-06 through NOTES-09, durable conversation
  ownership, persistence and restart recovery.
- **After that:** addressed agents/discussions, imports, opt-in capture and
  standalone downstream qualification. These remain pending.

## Stage overview

| Stage                                       | Status      | Delivered so far                                              | Remaining gate                                                                    |
| ------------------------------------------- | ----------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| 1. Contracts and compatibility (M33.1)      | In progress | Ownership, preserved formats and injectable boundaries        | Finish shared workflow contract review and compatibility coverage                 |
| 2. Privacy and source acquisition (M33.2)   | In progress | Native redaction, source readers and headless capture         | Remaining source portability/lifecycle qualification                              |
| 3. Conversation engine and client (M33.3)   | In progress | Stream processing, injected prompt coordination and typed SDK | Store/workspace/context adapters, application adoption and shared UI              |
| 4. Local history and Tauri (M33.4)          | In progress | Headless history, lossless client and Notes admission/drain   | Native event ownership and downstream visible history integration                 |
| 5. First usable Notes milestone (M33.5)     | Pending     | Existing Notes AI shell and injection seams                   | Visible history/diff UI plus one durable JJ-backed conversation and restart proof |
| 6. Addressed agents and discussions (M33.6) | Pending     | Existing jj-store behavior available as the source reference  | Shared addressing, queues, anchors and recovery in downstream clients             |
| 7. Imports and opt-in capture (M33.7)       | Pending     | Reusable acquisition prerequisites                            | Explicit deduplicated import and opt-in automatic local capture                   |
| 8. Standalone qualification (M33.8)         | Pending     | Some independent Rust/package consumers pass                  | Complete downstream, packaging, documentation and qualification gates             |

## Completed slices

- [x] **BASE-01** Extract five injectable collaboration stores, named factories,
      shared engines/codecs and Git/native providers; qualify independent
      consumers. [M32](../../spec/src/current/progress/M32/index.md) is Done
      within its source-library scope.
- [x] **BASE-02** Add short crate-root READMEs with context, usage and full-docs
      pointers; retain this requirement for new crates. Evidence:
      E-CRATE-READMES-001.
- [x] **PRIV-01** Delegate native transcript/hook privacy to Agent Redaction
      while preserving policy identities and formats. Evidence:
      E-REUSABLE-AGENTS-001.
- [x] **SRC-01** Delegate source discovery to Lapis AI's reusable capture
      library. Evidence: E-REUSABLE-AGENTS-002.
- [x] **SRC-02** Make headless capture usable without mandatory WAL,
      object-store, RPC or application dependencies. Evidence:
      E-REUSABLE-AGENTS-003.
- [x] **CONV-01** Extract sanitized assistant/tool stream processing and migrate
      jj-store consumers. Evidence: E-REUSABLE-AGENTS-004.
- [x] **CONV-02** Extract prompt coordination with injected admission, context,
      journal and runtime ports; retain unknown outcomes without replay.
      Evidence: E-REUSABLE-AGENTS-005.
- [x] **CONV-03** Build the headless `@lapismd/jj-conversations` SDK and initial
      jj-store mention/delivery adapters. Focused checks pass; broader UI
      qualification remains open in E-REUSABLE-AGENTS-006.
- [x] **HIST-01** Separate query budgets, cancellation and worker ownership,
      then expose a focused `local-history` feature. Evidence:
      E-REUSABLE-AGENTS-007/008.
- [x] **HIST-02** Add the reusable host-admitted history service and optional
      Tauri bridge with pinned reads and drained handles. Evidence:
      E-REUSABLE-AGENTS-009.
- [x] **HIST-03** Share read-only JJ workspace discovery across CLI and
      downstream hosts. Focused checks pass; the broader CLI gap stays open in
      E-REUSABLE-AGENTS-010.
- [x] **HIST-04** Preserve full-width identities/integers through bounded
      versioned JSON text frames. Evidence: E-REUSABLE-AGENTS-011.
- [x] **HIST-05** Add independent `@lapismd/jj-history` ESM/type exports, strict
      decoding and optional Tauri transport. Focused source/archive consumers
      pass; aggregate architecture gaps stay open in E-REUSABLE-AGENTS-012.
- [x] **NOTES-01** Establish Notes persistence injection and shared native vault
      admission prerequisites. Desktop evidence: E-AI-024/025.
- [x] **NOTES-02** Add independent physical JJ repository admission, retained
      witnesses and native generations without enabling AI. Desktop evidence:
      E-REPO-001, committed as `d51d9fcc`.
- [x] **NOTES-03** Connect native repository admit/release commands, exact
      caller/origin grants, retained history handles and drained vault closure.
      Serialize database opening with closure. Desktop evidence: E-REPO-002,
      committed as `9abbdc9a`. Each Rust profile passes 111 tests with two
      existing ignored helpers; strict lint, 47 governance tests and two cold
      hidden native API tests pass. M13.1 is Done; visible Notes UI remains
      required by M13.2.

Root evidence and its original outcomes are linked from the
[M33 evidence history](../../spec/src/current/progress/M33/history/index.md).
Desktop evidence lives under the sibling `lapis-desktop/spec/src/history`.

## Current work and next tasks

Only **Active** identifies the task currently being advanced. **Waiting** means
work has started but awaits a dependency or acceptance step; an unlabeled task
is pending. One agent works through these tasks, with checks sometimes running
in the background. Stage-level **In progress** means the stage has started, not
that every task in it is executing simultaneously.

- [ ] **NOTES-04 — Waiting for NOTES-04A/05/04B:** Add the visible Notes
      history/diff panel using the public history client and Design Core
      exports. Verify the Design Core dependency and retain existing Notes fixes
      through the owning package's public artifacts. Work is isolated in
      `lapis-desktop-history` to preserve concurrent Notes edits. Thirteen
      focused reader/native-adapter cases and all 191 renderer/governance cases
      pass; type checking has no errors or warnings. The implementation is not
      yet committed or integrated.
- [ ] **NOTES-04A — Active:** Qualify the owning Notes API attachment fix
      required by the history view. Exact regression: two pass; complete
      workspace: 131 pass; complete API: 1,160 pass; root check passes. Wider
      unit and Storybook failures remain open below; source commit and public
      artifact provenance must be finalized before integration.
- [ ] **NOTES-04B:** Prove the visible history/diff flow in a cold actual native
      WebView and complete M13.2's original gate. Hidden native API acceptance
      and fixture stories do not substitute for visible native acceptance.
- [ ] **NOTES-05 — Waiting for source qualification:** Add production-component
      review stories for history selection, diff, loading, empty/unavailable,
      failure and recovery states; run interaction and accessibility checks. The
      initial nine-scenario family passes desktop-light and narrow-light.
      Desktop-dark has eight passes and one syntax-token contrast failure;
      narrow-dark has not run. A stronger operation-switch assertion exposed
      retained old diff state; its exact unit regression now passes after the
      fix. A tenth scenario keeps unified diff accessibility under direct test.
      Final both-theme qualification and visual baseline approval remain
      separate.
- [ ] **NOTES-06:** Implement one native raw-event owner. Sanitize and durably
      capture before display/ACK; drain before releasing vault authority.
- [ ] **NOTES-07:** Add receipt-aware AgentStore/ThreadStore conversation
      adapters and explicit per-vault backend selection, retaining legacy file
      conversations.
- [ ] **NOTES-08:** Connect shared conversation store/workspace/context adapters
      to the existing Notes AI plugin and reusable Design Core Chat
      presentation.
- [ ] **NOTES-09:** Prove one real native JJ-backed conversation,
      uncertain-write recovery and cold restart without replaying an unknown
      prompt. This, together with visible history/diffs, qualifies the first
      usable milestone.
- [ ] **AGENT-01:** Extract/adopt participant addressing, persisted agent tags,
      multiple recipients and queued delivery with exact identity/authority
      separation.
- [ ] **DISCUSS-01:** Add exact message/tool/code discussion anchors and
      recovery using the owning stores; qualify unsupported profiles explicitly.
- [ ] **IMPORT-01:** Add explicit imports with deduplication and source
      provenance.
- [ ] **CAPTURE-01:** Add opt-in local automatic capture with source lifecycle,
      progress, acknowledgement and privacy qualification.
- [ ] **QUAL-01:** Complete remaining contract/compatibility and source
      portability gates, independent Rust/Tauri consumers and public package
      archive acceptance.
- [ ] **QUAL-02:** Finish usage documentation/README coverage and required
      native, browser, interaction and accessibility checks across affected
      downstream clients.

## What can be viewed now

These are existing fixture previews, not the new persisted Notes workflow. They
are available while the local preview servers remain running.

- [Notes AI shell and tool activity](http://127.0.0.1:7480/?path=/story/desktop-ai-shell-compositions--chat-with-tool-activity)
- [Design Core continuous transcript](http://127.0.0.1:9210/?path=/story/chat-transcript--continuous-document)
- [Design Core comments and queue](http://127.0.0.1:9210/?path=/story/chat-transcript--inline-comments-and-queue)
- [Design Core split diff](http://127.0.0.1:9210/?path=/story/diff-file-diff--renders-a-split-file-diff)

The native history wiring has hidden WebView acceptance. New production history
view stories are being qualified in the isolated Notes workspace; a preview link
will follow acceptance. There is no persisted-chat story yet. NOTES-04/05 own
the history UI delivery.

## Open qualification gaps

- [ ] **GAP-01:** E-REUSABLE-AGENTS-006 retains the parent-reproduced
      `ClosedDetailsStaysAbsentAfterRepositorySettings` failure and an
      incomplete aggregate cold catalog run. Its affected 216-story family
      passes; a recorded ResizeObserver diagnostic also remains visible for
      wider qualification.
- [ ] **GAP-02:** E-REUSABLE-AGENTS-010 retains the normal-parallel CLI
      `local_browser` result: 61 pass, one busy-index failure, three ignored, on
      both parent and current source. The isolated failing case passes; cause is
      unresolved.
- [ ] **GAP-03:** E-REUSABLE-AGENTS-012 retains unchanged UI size failures in
      `StoreStackWorkspaceApplication.stories.svelte` (401/400),
      `StoreApplicationMain.svelte` (313/300), and `StoreStackWorkspace.svelte`
      (310/300), reproduced with parent/current checkers on unchanged source
      bytes.
- [ ] **GAP-04:** Live provider, complete downstream workflow, additional
      platform and release qualification remain separate. Fixture/native service
      success does not establish these gates. Publication requires a separate
      operator action.
- [ ] **GAP-05:** NOTES-04A's required wider source unit lane stopped at two
      5-second frozen Deno host timeouts (294 pass, two fail, one skipped). Its
      root build did not run. Both exact cases pass at rest (18 others skipped);
      the aggregate rerun passed that host but stopped at a different API
      declaration-test timeout (1,159 pass, one fail). Earlier standalone API
      qualification passed all 1,160 cases. Aggregate qualification and cause
      investigation remain open; no timeout has been relaxed.
- [ ] **GAP-06:** NOTES-04A's first cold source Storybook shard has 75 pass and
      18 fail; three later shards have not run. A late dependency optimization
      reloaded the frame and 16 failures report inaccessible axe imports. Two
      editor diagnostic/marker assertions also failed. Verify each cause and
      rerun without weakening assertions or accessibility checks.

## Keeping this list current

After each verified slice, update the affected stable task IDs, link its fresh
evidence/commit, move the current-work marker and review the canonical milestone
in the same delivery update. Keep unresolved failures and unrun gates explicit.
If work splits, add a task ID rather than checking off an incomplete parent.
When switching tasks, move the single Active marker and state why other started
tasks are Waiting. Do not count a parent and its current child as separate work.
Current uncommitted counts are progress observations, not accepted evidence or
milestone completion. Replace them with final evidence/commit links once their
owning slice passes its required gates.

Do not infer an overall percentage from task counts: the tasks differ in size.
Only mark a stage complete when its canonical acceptance gate is satisfied.
