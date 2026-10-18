# Reusable agent workflows: delivery checklist

Last reviewed: 2026-10-18. Overall: **In progress**.

The first usable milestone is **Notes showing pinned JJ history/diffs and one
recoverable AI conversation persisted through AgentStore and ThreadStore**. It
is still pending. Notes' native repository admission and history transport are
verified. The history/diff renderer and cold visible native acceptance are
qualified on macOS ARM64 with physical foreground assistance. Durable
conversation capture remains pending.

This is the delivery checklist for the approved eight-stage plan. The
[M33 milestone ledger](../../spec/src/current/progress/M33/index.md) owns formal
status and gate selections. Notes' consumer gates live in the sibling
`lapis-desktop-history/spec/src/current/progress/milestones.md` under M13.
Checked items mean that the named slice has evidence; they do not complete its
whole stage.

## Current checkpoint

- **Active task: NOTES-07.** Add receipt-aware AgentStore/ThreadStore adapters
  and explicit per-vault backend selection. This supplies the real JJ sink
  required to finish NOTES-06C/D. Work advances one task at a time.
- **Waiting: NOTES-06C/06C4.** Shared capture, complete batches, production
  transcript provenance and the real native privacy prerequisite are qualified.
  Actual backend ownership, publication and drained closure await NOTES-07.
- **Completed: NOTES-06B.** Lapis AI `87958267` qualifies the scoped Tauri
  processor factory: nine focused cases, all 27 host cases and full source
  qualification (1,004 Rust passes, two inherited ignored). Desktop `0535bb1e`
  records six cold default ACP compatibility passes with foreground assistance.
  Notes privacy is qualified separately in NOTES-06C4B; the durable backend
  remains pending.
- **Completed: NOTES-06A.** Lapis AI `6ce65ebb` adds an injected Rust processor
  before startup facts and event publication. Nine focused cases, all 995 Rust
  tests (two inherited ignored), 45 governance and 27 renderer cases, strict
  checks, builds, packages and specification lanes pass. Desktop `242e1fe5`
  records six cold ACP compatibility passes with foreground assistance.
- **Parent gate: NOTES-06.** Waiting for NOTES-06C/D. The shared hook alone does
  not establish Notes sanitation, durable capture or backend selection.
- **Completed: NOTES-04/04B.** Cold actual-window history, exact restored pins,
  source preservation and plain-folder refusal pass in desktop `fb238ae2`.
  Strict native keyboard-window assertions pass. Both launches received physical
  foreground clicks; unattended activation remains GAP-10. Evidence: E-REPO-006.
- **Completed: NOTES-05E.** Pane-relative history layout is committed as
  `3aa56eff` in the isolated desktop workspace. The exact geometry regression,
  all 72 history checks, all 696 catalog checks, types, 191 renderer/governance
  cases and both UI builds pass. Evidence: E-REPO-005.
- **Completed current package/UI slice:** Notes API `a91d5e2e`, Design Core
  `a57a38e2` and desktop history `2209736a` are committed independently in the
  isolated qualified workspaces. They preserve unrelated default-checkout work.
- **Accepted history UI (`3aa56eff`):** 72 history checks and all 696 complete
  catalog checks pass. Types have no errors/warnings; all 191
  renderer/governance cases and renderer/static catalog builds pass.
- **Verified producers:** Notes API exact regressions, all 1,160 API cases,
  complete check/test/build/specification lanes and 261 source stories pass.
  Design Core passes 821 units, 834 stories and 172 browser cases, one existing
  skip. Package provenance and earlier failures remain in owning evidence.
- **Completed: NOTES-06C4B.** Notes selects the real shared privacy policy in
  cold native acceptance and refuses stale admission or missing ordinary backend
  selection. Both Rust profiles pass 112 tests (two inherited ignored each); all
  eight cold ACP cases pass with foreground assistance. The fixture sink remains
  synthetic. Desktop `51bed526`; evidence: E-AI-030/E-REUSABLE-PROGRESS-014.
- **Next acceptance:** Finish NOTES-06C4 and NOTES-06D once NOTES-07 supplies
  the real backend. NOTES-08/09 retain application adoption and one recoverable
  JJ-backed conversation with cold restart proof.
- **After that:** addressed agents/discussions, imports, opt-in capture,
  portable package intake and independent downstream qualification. Visual
  comparison/approval remains separate and pending.

## Stage overview

These are acceptance summaries, not a work queue. **Started; gate open**
corresponds to the canonical ledger's **In progress**: some work is delivered,
but acceptance is incomplete. Only NOTES-07 is currently active. The stages
below are not being worked on simultaneously.

| Stage                                       | Status             | Delivered so far                                                                  | Remaining gate                                                        |
| ------------------------------------------- | ------------------ | --------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| 1. Contracts and compatibility (M33.1)      | Started; gate open | Ownership, preserved formats and injectable boundaries                            | Finish shared workflow contract review and compatibility coverage     |
| 2. Privacy and source acquisition (M33.2)   | Started; gate open | Native redaction, source readers and headless capture                             | Remaining source portability/lifecycle qualification                  |
| 3. Conversation engine and client (M33.3)   | Started; gate open | Stream processing, injected prompt coordination and typed SDK                     | Store/workspace/context adapters, application adoption and shared UI  |
| 4. Local history and Tauri (M33.4)          | Started; gate open | Headless history, lossless client, Notes admission/drain and cold visible history | Native event ownership                                                |
| 5. First usable Notes milestone (M33.5)     | Pending            | Qualified history prerequisites, existing Notes AI shell and injection seams      | One durable JJ-backed conversation and restart proof                  |
| 6. Addressed agents and discussions (M33.6) | Pending            | Existing jj-store behavior available as the source reference                      | Shared addressing, queues, anchors and recovery in downstream clients |
| 7. Imports and opt-in capture (M33.7)       | Pending            | Reusable acquisition prerequisites                                                | Explicit deduplicated import and opt-in automatic local capture       |
| 8. Standalone qualification (M33.8)         | Pending            | Some independent Rust/package consumers pass                                      | Complete downstream, packaging, documentation and qualification gates |

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

- [x] **NOTES-04A** Qualify the Notes API deferred imperative attachment fix on
      the existing merged-conflict producer. Commit `a91d5e2e`; all source gates
      and the complete 692-check current desktop consumer pass.
- [x] **NOTES-05A** Repair dark syntax and isolated consumer-usable Docs. Design
      Core commit `a57a38e2`; exact source/diff consumer cases pass.
- [x] **NOTES-05B** Retain complete tool-disclosure labels and busy-state
      accessibility in the same qualified public artifact.
- [x] **NOTES-05C** Repair dark message error labels in their source component;
      both exact source cases and all 100 affected consumer checks pass.
- [x] **NOTES-05D** Repair desktop-owned conflict error-text contrast; two exact
      regressions and all 128 affected/full 692 catalog checks pass.
- [x] **NOTES-05** Deliver 17 production history review stories (68 checks),
      retained-pin and cleared-diff regressions, complete current-app
      interaction/accessibility and static builds. Desktop commit `2209736a`,
      evidence E-REPO-003. These are fixture/public-package results; visual
      approval and native history/conversation gates remain separate.
- [x] **NOTES-05E** Make history respond to its pane width within a wide shell;
      retain readable revision/file controls and diff content. Add the exact
      bounds/stacking regression and qualify all 18 history cases (72 checks)
      plus the complete 696-check catalog. Desktop commit `3aa56eff`, evidence
      E-REPO-005. Types, 191 renderer/governance tests and both UI builds pass.
- [x] **NOTES-04B** Qualify actual native ribbon/diffs, exact operation pins,
      external-head refresh, dirty source and JJ/Git metadata preservation, cold
      restored pins, changed-operation diff clearing and ordinary-folder
      refusal. Desktop `fb238ae2`, evidence E-REPO-006: one full strict native
      pass and 47 governance passes. Two physical foreground clicks were needed;
      the native focus/visibility assertions are unchanged or strengthened.
- [x] **NOTES-04** Complete the history parent gate with production renderer,
      pane layout and cold visible native evidence. This completes history
      integration in its qualified profile; durable conversations remain below.
- [x] **NOTES-06A** Add the library-owned `SessionProcessor` hook for startup
      activity/facts and actor events before exposure or ACK. Retain exact
      identities/options, bounded output, unknown requests and supervised
      cancellation/drain. Source `6ce65ebb` qualifies all nine focused cases and
      full Lapis AI lanes. Consumer `242e1fe5`, E-AI-026, refreshes the six cold
      default ACP cases. Notes managed capture remains pending.
- [x] **NOTES-06B** Inject the scoped native processor factory with exact
      worker/open-request/target/window/admission identities, current admission
      fences and supervised cleanup. Rejected processing never falls back to raw
      execution. Source `87958267` passes nine focused cases, all 27 host cases,
      1,004 Rust cases (two inherited ignored), 45 governance and 27 renderer
      cases, strict checks/builds/packages and all specification lanes. Desktop
      `0535bb1e`, E-AI-027, passes six cold default ACP cases with foreground
      assistance. Managed Notes capture remains a separate gate.

- [x] **NOTES-06C1** Add shared receipt-fenced native capture with injected
      privacy and durable sink, exact binding/progress and bounded canonical
      batches. Retain deferred prefix frames and scan final projections before
      persistence. Ten focused cases and all 20 crate cases pass, including the
      ordered-JSON feature profile. Evidence: E-REUSABLE-AGENTS-013. This is the
      shared library prerequisite; NOTES-06C stays incomplete until Notes
      connects its actual durable backend. Batch delivery and the real-policy
      prerequisite are qualified in the slices below.

- [x] **NOTES-06C2** Preserve deferred text in atomic acknowledged native
      batches, add the Notes-owned capture adapter and qualify actual Tauri
      delivery. Lapis AI `8a8f9149` passes source qualification (1,013 Rust
      cases, two inherited ignored; 45 governance and 39 renderer). Desktop
      `2c6cacb3`, E-AI-028, qualifies all seven cold ACP cases with foreground
      assistance, both Rust profiles (111 pass, two inherited ignored each),
      strict checks, both native builds and all 191 renderer/governance cases.
      Synthetic privacy and synced private candidates prove transport/receipt
      fencing; production privacy, JJ publication and restart recovery remain
      pending. Delivery evidence: E-REUSABLE-PROGRESS-012.

- [x] **NOTES-06C3** Consume complete native groups in production ACP and
      preserve optional per-frame transcript provenance in both legacy stores.
      Exact replay deduplication retains distinct frames at a shared cursor;
      absent indexes preserve historical record hashes. Desktop `6e8ebd17`,
      E-AI-029: eleven focused/hash checks, 198 renderer/governance, 389 AI
      policy and all 696 current-app interaction/accessibility checks pass. All
      seven cold ACP cases pass together with foreground assistance. Strict
      lint, 111 native service tests (two inherited ignored), renderer/native
      builds and specification checks pass. Actual JJ durability and ordinary
      target admission remain pending; the real-policy profile is qualified in
      NOTES-06C4B. Delivery evidence: E-REUSABLE-PROGRESS-013.

- [x] **NOTES-06C4A** Add the opt-in native privacy bridge to the conversation
      library, selecting the existing Agent Redaction transcript policy. Five
      real-scanner cases cover nested content, UTF-8 chunk boundaries, refusal
      and unknown receipts. The default profile passes 20 tests without the
      scanner; the opt-in ordered-JSON profile passes 25. Governance, CI graph
      and manual checks retain their boundaries. Evidence:
      E-REUSABLE-AGENTS-014. Notes adoption is qualified in NOTES-06C4B; its
      actual durable backend remains pending within NOTES-06C4.

- [x] **NOTES-06C4B** Adopt the real shared privacy policy in the Notes native
      capture adapter, with native-owned paths and a disposable scanner cache.
      Ordinary policy refuses stale admission and missing backend selection.
      Both native Rust profiles pass 112 tests, two inherited ignored each;
      strict lint and both builds pass. All eight cold ACP cases pass with
      foreground assistance, including split paths/credentials before storage
      and renderer delivery. Desktop `51bed526`; evidence:
      E-AI-030/E-REUSABLE-PROGRESS-014. The synthetic sink does not qualify JJ
      publication or restart recovery.

Root evidence and its original outcomes are linked from the
[M33 evidence history](../../spec/src/current/progress/M33/history/index.md).
Desktop evidence lives in the isolated qualified sibling
`lapis-desktop-history/spec/src/history`; default checkouts remain independent.

## Current work and next tasks

Only **Active** identifies the task currently being advanced. **Waiting** means
work has started but awaits a dependency or acceptance step; an unlabeled task
is pending. One agent works through these tasks, with checks sometimes running
in the background. Stage-level **In progress** means the stage has started, not
that every task in it is executing simultaneously.

**Parent acceptance: NOTES-06 remains incomplete.** NOTES-06A supplies the
shared runtime hook and NOTES-06B supplies the qualified Tauri factory.
NOTES-06C/D must connect the Notes capture owner and qualify the complete path.
NOTES-07 is the sole Active implementation task; capture waits for its backend.

- [ ] **NOTES-06C — Waiting for NOTES-07:** Connect the real durable backend
      through the native factory, retaining the qualified privacy/batch owner
      and draining capture before vault release.
- [ ] **NOTES-06C4 — Waiting for NOTES-07:** Its shared native privacy bridge
      and Notes adoption are complete in NOTES-06C4A/B. Exact backend ownership,
      real JJ receipts and drained closure still require the store adapters.
- [ ] **NOTES-06D:** Qualify the native managed path, including withheld or
      failed capture, exact scopes, unknown outcomes, acknowledgement and close.
      Coordinate its concrete backend acceptance with NOTES-07.
- [ ] **NOTES-07 — Active:** Add receipt-aware AgentStore/ThreadStore
      conversation adapters and explicit per-vault backend selection, retaining
      legacy file conversations.
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
      archive acceptance. Wire the optional native-privacy test profile into its
      owning CI qualification lane.
- [ ] **QUAL-02:** Finish usage documentation/README coverage and required
      native, browser, interaction and accessibility checks across affected
      downstream clients.

## What can be viewed now

- [New Notes repository history and diff review](http://127.0.0.1:7490/?path=/story/desktop-repository-history--pinned-history-and-diff)
- [Restored historical operation](http://127.0.0.1:7490/?path=/story/desktop-repository-history--restores-historical-operation)

These are existing fixture previews, not the new persisted Notes workflow. They
are available while the local preview servers remain running.

- [Notes AI shell and tool activity](http://127.0.0.1:7480/?path=/story/desktop-ai-shell-compositions--chat-with-tool-activity)
- [Design Core continuous transcript](http://127.0.0.1:9210/?path=/story/chat-transcript--continuous-document)
- [Design Core comments and queue](http://127.0.0.1:9210/?path=/story/chat-transcript--inline-comments-and-queue)
- [Design Core split diff](http://127.0.0.1:9210/?path=/story/diff-file-diff--renders-a-split-file-diff)

The new isolated Notes catalog on port 7490 renders the qualified production
history components with fixture readers. All 18 history scenarios pass their
interaction/accessibility checks. Actual cold native history also passes in
NOTES-04B with physical foreground assistance. There is no persisted-chat story
yet.

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
- [x] **GAP-05:** The ordinary source unit/check/build lanes pass on the current
      producer and are committed in `a91d5e2e`. Earlier timeout attempts and
      their unproven transient cause remain in the immutable/source records.
- [x] **GAP-06:** The static recovery-icon optimization repairs the cold editor
      family (18 pass) and complete source lane (261 pass). Assertions and axe
      stay unchanged; final source and consumer qualification is committed.
- [ ] **GAP-07:** The passing source browser lane emits two ResizeObserver loop
      diagnostics and three CodeMirror invalid-line RangeErrors in AI panel
      stories. Their cause remains unproven. Keep these separate from expected
      failure-fixture logs and from interaction/accessibility assertion counts.

- [ ] **GAP-08:** Notes currently consumes the new history SDK through the
      private app's built-public-export development link. Move that intake to a
      portable range and verified archive during standalone qualification.
- [ ] **GAP-09:** The optional compare-only dark-diff visual run fails during
      Docker static build (exit 137), then refuses a changed source-record
      artifact. No rendered comparison or baseline update occurred. Source
      functional/consumer gates remain passed; visual review stays pending.
- [ ] **GAP-10:** Unattended macOS foreground activation is unqualified. The
      complete cold native history test passes with two physical foreground
      clicks and strict document/native keyboard-window assertions. The driver
      alone cannot reliably activate a cold app. Preserve this automation gap
      separately from the actual native history result.

## Keeping this list current

After each verified slice, update the affected stable task IDs, link its fresh
evidence/commit, move the current-work marker and review the canonical milestone
in the same delivery update. Keep unresolved failures and unrun gates explicit.
If work splits, add a task ID rather than checking off an incomplete parent.
When switching tasks, move the single Active marker and state why other started
tasks are Waiting. A named next step is pending until it starts. Do not count a
parent and its current child as separate work. Current uncommitted counts are
progress observations, not accepted evidence or milestone completion. Replace
them with final evidence/commit links once their owning slice passes its
required gates.

Do not infer an overall percentage from task counts: the tasks differ in size.
Only mark a stage complete when its canonical acceptance gate is satisfied.
