# Reusable agent workflows: delivery checklist

Last reviewed: 2026-10-19. Overall: **In progress**.

**Current direction: visible AI delivery.** The user approved a staged
implementation: document/classic Storybook review, live adoption, native JJ
conversation browsing, real lapis-agent chat and cold restart. The user gave
standing implementation approval on 2026-10-18. Focused checks accompany each
slice; broad provider/platform/release and visual baseline qualification remain
deferred.

**Completed local profile: NOTES-07L-FILES and NOTES-09-FILES-A/B/C/D1/D2.**
Notes E-AI-042 qualifies real installed Codex chat, bounded file context,
cancellation, local saving and cold History reopen. E-AI-043 adds pre-dispatch
authored-message and intent durability, conservative unknown recovery and
explicit new-message continuation. Its isolated real Codex test kills the app
after streamed output, reopens the unchanged journal cold and completes one new
message without replay. NOTES-09-FILES-C is complete in Notes `8e3da0be` /
E-AI-044: complete bounded streaming and the refreshed review app are qualified
for local Codex. E-AI-045 completes D1/D2: stable source anchors and
acknowledged local comments survive cold History reopen, including comments-only
Codex turns. The unqualified JJ client draft remains in Notes change `lqqoypxx`
(`567e9e95`), outside the active application. NOTES-07K and JJ capture/restart
stay deferred until requested.

The delivered focused profile is **Notes AI chat with local file transcripts and
cold saved-history reopen**. Notes `e322e36e` / E-AI-047 also qualifies real
Codex application-tool reads, explicitly approved edits, cancellation of an
unapproved edit and cold History without replay. The review app is refreshed;
all 131 existing vault files remain byte-identical. The original M33.5 target of
pinned history/diffs and one conversation in AgentStore/ThreadStore remains
pending and deferred. Notes' native repository admission and history transport
are verified. The history/diff renderer and cold visible native acceptance are
qualified on macOS ARM64 with physical foreground assistance. The real native
factory is qualified; managed conversation capture and restart remain pending.

This is the delivery checklist for the approved eight-stage plan. The
[M33 milestone ledger](../../spec/src/current/progress/M33/index.md) owns formal
status and gate selections. Notes' consumer gates live in the sibling
`lapis-desktop-history/spec/src/current/progress/milestones.md` under M13.
Checked items mean that the named slice has evidence; they do not complete its
whole stage.

## Current checkpoint

- **Completed review implementation: NOTES-08A — Document/classic AI review.**
  Production-owned AI components use the retained controller and catalog-only
  fake clients. Separate Design Core reference stories run inside the same
  AppShell (both docks collapsed) and standalone. Fenced code reuses captured
  code/patch readers with inline comments and line folds. Mira supplies
  immutable tables; table comments split after their source row and retain full
  row context with the selected text highlighted. The user has authorized the
  remaining implementation. Native JJ chat and restart are still pending; source
  component fixes stay in Design Core. E-AI-040 corrects footer height and
  same-row formatting in both Notes and reference; 36 source and 60 consumer
  interaction/accessibility cases pass.
- **Completed: NOTES-08B** adopts the ordinary shared UI and retains the
  per-vault view preference. E-AI-041 records 42 unit tests, 164 interaction/axe
  cases across both widths/themes, clean types/ownership, renderer/native build
  and cold native review; 110 source/Git/JJ files remain unchanged. Design Core
  `6b5654e0` retains readable disabled drafts/avatars after 47 affected source
  stories. **Deferred: NOTES-07K** connects native listing/creation/pinned
  reads; **NOTES-07L** admits bundled lapis-agent using explicit Lapis AI
  configuration and native secret references. The first live scope is chat/file
  context/cancel, followed by **NOTES-09** cold restart and uncertain-outcome
  recovery. Native application tools now have the E-AI-047 local-file profile;
  addressed agents, queues and imports remain later slices.

- **Completed: NOTES-05I7 file history and row actions.** Whole-row pointer
  selection preserves copy/author actions; History and Op Log use the requested
  branch/pull-request icons. The default right sidebar follows editor file
  revision history using native JJ file predicates. Exact native tests match
  stock JJ for merges, conflicts, deletion and multiple index batches; focused
  Notes tests and cold native consumer review pass (E-REPO-017). Delayed
  selection after view replacement is fenced; the ordinary demo remains open.
- **Completed: NOTES-05I8 file blame and sidebar states.** Copied jj-store's
  virtual source/attribution presentation opens from File history at the
  recorded commit and operation. Native stock-JJ attribution, authorization and
  close-drain tests pass; cold Notes review preserves dirty source bytes. The
  sidebar omits ordinary operation/author metadata, aligns its header with the
  main pane and uses shadcn Empty with an icon. Repository loading views use
  representative skeletons with App-owned status progress (E-FILE-BLAME-001;
  Notes E-REPO-018). Native search/evolution, write actions and managed
  conversation/restart remain separate slices within the same NOTES-05I
  umbrella.

- **Waiting task: NOTES-05I.** Close the source-confirmed repository-view gaps:
  highlighted native search, rich Command View parent/revision pickers, stacked
  Op Log detail/diff cards, exact statistics and remaining row/state parity. The
  [parity review](repository-view-parity-review.md) owns the explicit gap matrix
  and ordered slices. NOTES-05I3A qualifies stacked operation/file cards and
  bounded exact parent navigation; NOTES-05I5A opens both logs in workspace tabs
  for review. NOTES-05I2A JJ parent cards and NOTES-05I0A owned-runtime reload
  are qualified; preview maintenance is also complete. Reviewed comparison
  controls are now production-owned; native queries and full view parity stay
  open.
- **Completed: NOTES-05I6 production adoption.** Notes E-REPO-015 promotes
  reviewed presentation and native-read integrations, with 48 repository Node
  tests,66 repository stories at both widths/themes, strict types/ownership and
  renderer build acceptance. Copy/recovery, Markdown/tags, workspace context,
  revision cards and local conflict preview are covered. Broad qualification
  remains deferred. Native query/evolution, write actions, exact comparison
  totals and managed conversations are the next functional slices.
- **Completed: NOTES-05I0B/05I0.** Notes `2a4ed91c` / E-CATALOG-008 qualifies
  managed preview start/status/stop/restart and dry-run/apply cache commands on
  macOS ARM64. Eighteen exact, 242 renderer/governance and all 804 cold catalog
  interaction/axe checks pass, with strict checks, builds and documents. Exact
  process ownership, foreign-listener refusal and cache/data boundaries remain
  explicit. Other Op Log endpoints and native queries are next.
- **Completed: NOTES-05I2A/05I0A.** Notes `564e039d` / E-REPO-011 qualifies the
  JJ parent picker using jj-store's public Command View/Popover with
  author/time/Git/JJ cards, search, tags, bounded pinned missing-parent reads
  and explicit toast recovery. Runtime scripts/fixtures reconstruct the preview
  App; components/styles keep HMR. Sixteen exact and 804 complete catalog
  interaction/axe checks pass, with 224 renderer/governance cases, strict
  types/audits, both builds and document consumers. Actual runtime reload and
  CSS HMR retain their separate live proof. Other comparison selectors and
  lifecycle/cache maintenance are qualified separately above.
- **Completed: NOTES-05I3A/05I5A.** Notes `65404171` / E-REPO-010 ports stacked
  operation/file cards, source author fallback and exact parent navigation
  through the qualified prepared graph profile. Twelve exact layout, 788
  complete existing catalog and four new two-tab review cases pass, along with
  eight controller/215 renderer-governance cases, strict checks and builds.
  Design Core `b9477996` owns the accessibility/readiness prerequisite. This
  qualifies composition and a review preview, not whole-view parity.
- **Waiting: NOTES-07.** After the first AI review, resume renderer projection,
  executable/credential admission, ordinary runtime dispatch and per-vault
  persistence after this requested UI alignment. Native
  preparation/factory/read/continuation slices remain qualified; managed
  chat/restart remains open.
- **Completed: NOTES-07J.** Notes `d00a69b3` / E-AI-038 qualifies retained
  native transcript continuation with opaque read/stream handles, original
  immutable selection, exact issued sequence positions and fresh receipt checks.
  Eleven focused reads, both complete 174-case native profiles (two inherited
  ignored each), strict lint and four cold hidden API cases pass. Idle reads
  retain no counted writer calls. Renderer projection and managed dispatch stay
  open.
- **Completed: NOTES-05H.** Notes `792cbfa8` / E-REPO-009 aligns History and Op
  Log with jj-store: Column Canvas begins directly with compact pin/refresh
  controls, without duplicate body headings/pin blocks. Eight exact header, 152
  repository and 776 full catalog interaction/axe checks pass, with strict
  checks, both builds and 212 renderer/governance cases. Visual approval stays
  pending.
- **Completed: NOTES-07I.** Notes `d1c27e17` / E-AI-037 returns confirmed native
  Agent/Thread data as bounded lossless JSON text with explicit incomplete
  transcript ranges. Eight focused reads, both complete 171-case native profiles
  (two inherited ignored each), strict lint and four cold hidden API cases pass,
  including exact read after actual restart. Renderer projection, continuation
  transport and managed execution remain separate work.
- **Completed: NOTES-07H.** Notes `cd5b5a1e` / E-AI-036 constructs the ordinary
  native processor from a confirmed conversation with real privacy, private
  journal and Agent/Thread participants. Thirty backend cases, both 163-case
  native profiles (two inherited ignored each), strict lint, 389 AI policy and
  212 renderer/governance cases pass. Four cold hidden API cases qualify exact
  capture preparation and original-conversation recovery after restart. Actual
  managed dispatch, renderer adoption and chat/restart remain open.
- **Completed: NOTES-05G.** Notes `c28fc92a` / E-REPO-008 corrects the primary
  history preview: its disposable sidebars close to show log/diff together, and
  both repository views retain public AppShell surface tokens. All 144 affected
  and 768 full catalog checks pass; visual approval stays pending.
- **Completed: NOTES-05F.** Notes `fb80f410` / E-REPO-007 ports both repository
  views into public AppShell/Column Canvas, with split IDs, expanded file modes
  and matching diffs. All 768 full catalog cases, 212 renderer/governance cases,
  strict checks/builds and three cold native read cases pass. Source producer
  `8118da89` and Design Core `aae7dc35` are independently qualified. Visual
  approval and actual managed conversation/restart remain separate.
- **Waiting: NOTES-06C/06C4.** Real privacy and selected-store factory callbacks
  are qualified in NOTES-07H. Supervised ordinary worker dispatch,
  acknowledgement and full managed publication/drained closure still await
  executable admission and renderer adoption in NOTES-07.
- **NOTES-07 checkpoint:** 07A qualifies batch normalization; 07B/C supply the
  store adapter and private journal; 07D/E own native backend profiles and IPC;
  07F1/F2 retain exact processing authority; 07G prepares and recovers the
  original conversation; 07H constructs and tests the real selected-store
  processor; 07I reads confirmed native conversation data. Native
  executable/credential admission, ordinary runtime dispatch, renderer
  persistence and managed conversation/restart remain pending.
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
but acceptance is incomplete. NOTES-08A awaits visual confirmation; no task is
Active. The stages below are not being worked on simultaneously.

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

- [x] **NOTES-07H — Ordinary native capture construction.** Notes `cd5b5a1e`
      (E-AI-036) prepares bounded opaque targets for exact confirmed
      conversations and constructs native privacy, selected Agent/Thread stores
      and the private journal. Content-level proof checks split path redaction,
      detected secrets, persisted transcript content, primary thread assignment
      and original receipt recovery without publication. Nine guard/lifetime
      cases and all 30 backend tests pass; both complete native profiles pass
      163 cases with two inherited ignored helpers each, strict Clippy, 389 AI
      policy and 212 renderer/governance cases pass. Four cold command cases
      include exact capture preparation/recovery with preserved source state.
      Targets remain empty; managed execution, renderer writer and chat/restart
      retain their separate gates.

- [x] **NOTES-05G — Review preview and shell styling.** Notes `c28fc92a`
      (E-REPO-008) closes only the primary story's disposable Files/Outline
      sidebars and retains public AppShell inset, border, radius and shadow.
      Dedicated compact-pane coverage remains. The exact regression, 144
      affected and 768 full catalog interaction/axe checks, 212
      renderer/governance cases, strict types/ownership audits and both UI
      builds pass. Native capture and visual approval retain their separate
      gates.

- [x] **NOTES-07G — Ordinary native conversation preparation.** Notes `4e628fb2`
      (E-AI-035) prepares a ThreadStore conversation through the current
      explicitly selected JJ owner. Native identities, private actor and exact
      intent precede publication; retries recover original Git receipts without
      executing again. Twenty backend cases, both 153-case native profiles (two
      inherited ignored each), strict Clippy, 389 AI policy and 212
      renderer/governance cases pass. Four cold hidden API cases include exact
      receipt recovery after app restart and unchanged dirty notes/JJ operation.
      Actual managed capture, agent admission, renderer persistence and
      chat/restart remain pending.

- [x] **NOTES-05F — Repository history and Op Log column canvases.** Notes
      `fb80f410` (E-REPO-007) reuses public AppShell, Column Canvas, FileListing
      and Diff with attributed jj-store graph/identifier ports. Rows retain file
      choices in compact panes; metadata opens explicitly. Full native IDs,
      comparison choices, unavailable states, clipboard toasts, retained pins
      and reader drain remain qualified. 144 affected and 768 full catalog
      interaction/axe checks, 212 renderer/governance cases, strict
      checks/builds and three cold hidden native API cases pass. Root `8118da89`
      (E-REUSABLE-AGENTS-017) supplies exact prepared operation reads/status
      tags; Design Core `aae7dc35` supplies accessible file-count labels with
      independent package qualification. Visual review stays pending; this does
      not complete durable capture or the first usable milestone.

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

- [x] **NOTES-07A** Add the versioned native capture envelope and normalization
      profile without rewriting its binding, ordinal or native cursors. Five
      model cases and a real Git-provider case qualify exact evidence, one final
      answer, atomic Agent/Thread roots, pinned reads, fresh opening and
      original receipts. All 70 affected crate tests and strict lint pass.
      Evidence: E-REUSABLE-AGENTS-015. The Notes receipt journal, backend
      adapter and native conversation/restart acceptance remain pending within
      NOTES-07/09.

- [x] **NOTES-07B** Add optional selected Agent/Thread native capture with an
      injected durable journal, exact provider preparation and original-receipt
      recovery. Real Git publication and fsynced test-host journal reopen prove
      output fencing, lost replies, cancelled waits, stale/foreign Threads and
      recovery without another execute. Native manifest/WAL preparation and
      receipt recovery pass with memory object storage. Default dependencies
      remain isolated. This completes the reusable adapter prerequisite only;
      the Notes journal is qualified separately in NOTES-07C. Backend selection,
      native drain and cold app conversation/restart remain pending. Evidence:
      E-REUSABLE-AGENTS-016.

- [x] **NOTES-07C** Implement Notes' private native journal with exclusive OS
      leases, bounded atomic synced records, monotonic transitions and current
      admission checks. Seven journal cases include a cold-process lease/reopen
      proof. Notes' real privacy/capture adapter and the production
      store/journal compose against an actual Git-backed JJ vault, preserving
      dirty note bytes and the JJ operation while recovering original receipts
      without another publication. Both native profiles pass 120 cases with two
      inherited ignored each; strict checks/builds and 198 renderer/governance
      cases pass. This service composition does not complete ordinary backend
      selection, native callback ownership/drain or packaged-app chat/restart.
      Notes `e88f1487`; evidence: E-AI-031/E-REUSABLE-PROGRESS-015.

- [x] **NOTES-07D** Add native backend profiles with explicit `vault-files`/`jj`
      choices, retained original receipts, synced preparation/selection records
      and an exclusive owner lease. Immediate close fences even an unpolled or
      abandoned wait; outstanding calls retain the lease. Seven backend cases
      include cold-process ownership, uncertain-write recovery and read-only
      real JJ witness checks. Both native profiles pass 127 cases with two
      inherited ignored each; strict lint, both native builds, types and 198
      renderer/governance cases pass. This service is not connected to ordinary
      app IPC, backend selection UI or managed capture yet. Notes `a48fce96`;
      evidence: E-AI-032/E-REUSABLE-PROGRESS-016.

- [x] **NOTES-07E** Consume those profiles in the ordinary native host with a
      bounded versioned text command and exact local main-window grant.
      Admission, backend operations and closure share native serialization;
      cancelled IO retains it, reload reattaches with a fresh handle and stale
      closes cannot revoke the new owner. Window destruction fences delayed
      admission. All nine backend/16 AI-host checks, both 136-case native
      profiles (two inherited ignored each), strict lint/builds, types and 198
      renderer/governance cases pass. Three cold hidden-WebView cases prove
      actual IPC, real JJ source preservation, saved-choice/receipt reopening
      and existing admission/history compatibility. This is not renderer
      persistence or managed chat/capture/restart qualification. Notes
      `81765a34`; evidence: E-AI-033/E-REUSABLE-PROGRESS-017.

- [x] **NOTES-07F1** Add the shared native processing authorization hook for
      exact retained worker bindings during renderer suspension. Renderer
      commands, factory construction and startup completion still require
      current admission. Five focused cases, all 34 host cases, 1,018 Rust
      passes (two inherited ignored), 39 renderer and 45 governance cases,
      strict checks, builds, package inventories and all ten specification lanes
      pass. Lapis AI `e12d303c`; evidence: E-REUSABLE-PROGRESS-018. Notes
      retained owner adoption and actual store/processor construction remain
      pending.

- [x] **NOTES-07F2** Retain exact native capture authority through renderer
      reload and old-token closure with physical/JJ witnesses, counted backend
      permits and fenced drain. Logical backend/journal lease release is
      independent of incidental duplicated kernel descriptors. Both native
      profiles pass 147 cases (two inherited ignored each), strict lint/builds,
      198 renderer/governance cases, types, all four specification lanes and
      three cold hidden native API compatibility cases pass. Notes change
      `pquxnosp`; evidence: E-AI-034/E-REUSABLE-PROGRESS-019. This qualifies a
      lifetime prerequisite; actual managed capture/chat/restart remain open.

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
NOTES-07L-FILES and NOTES-09-FILES-A are complete under the revised local-file
priority. NOTES-09-FILES-B completes durable local recovery; NOTES-09-FILES-C
completes bounded native Codex streaming. D1/D2 complete stable local anchors
and acknowledged durable comment context. TOOLS-01A completes the reusable MCP
transport prerequisite. TOOLS-01B is complete for authenticated native IPC,
owned coordinator/launch lifetime and durable turn gates. TOOLS-01C1 qualifies
the real Codex local-file workflow; wider effect-fault qualification waits.
AGENT-01A completes recipient selection and saved tags. Independent logical
participants, multiple-recipient routing and queued delivery remain next; no
implementation child is Active at this verified checkpoint. NOTES-05F/05G/05H
complete the initial canvas/header port, not full source-view parity. The shared
AI UI and local-file chat are working; JJ capture awaits its deferred managed
backend integration.

- [ ] **NOTES-06C — Waiting for NOTES-07:** Connect the real durable backend
      through the native factory, retaining the qualified privacy/batch owner
      and draining capture before vault release.
- [ ] **NOTES-06C4 — Waiting for NOTES-07:** Its shared native privacy bridge
      and Notes adoption are complete in NOTES-06C4A/B. Native callback
      callbacks and real receipts are qualified in NOTES-07H. Supervised managed
      dispatch, acknowledgement and drained closure still require ordinary
      execution/renderer integration.
- [ ] **NOTES-06D:** Qualify the native managed path, including withheld or
      failed capture, exact scopes, unknown outcomes, acknowledgement and close.
      Coordinate its concrete backend acceptance with NOTES-07.
- [x] **NOTES-05H — Complete:** Notes `792cbfa8` / E-REPO-009 aligns History and
      Op Log headers with jj-store; both widths/themes and existing pin/recovery
      flows pass all 776 full catalog checks.
- [x] **NOTES-07J — Complete:** Notes `d00a69b3` / E-AI-038 qualifies retained
      native transcript continuation, bounded opaque handles and close/reload
      ownership. Renderer projection and managed chat remain open.
- [x] **NOTES-07I — Complete:** Notes `d1c27e17` / E-AI-037 qualifies bounded
      confirmed native conversation reads and cold exact-read restart. This
      supplies no managed-chat or complete continuation acceptance.
- [x] **NOTES-05I3A — Complete:** E-REPO-010 / Notes `65404171` qualifies
      stacked operation/file cards and bounded pinned parent lookup.
- [x] **NOTES-05I5A — Complete:** The two-tab comparison story uses both
      production log views and retains file/diff selections. It is open for
      review; whole-view acceptance and visual approval remain separate.
- [x] **NOTES-05I2A — Complete:** Notes `564e039d` / E-REPO-011 qualifies JJ
      parent Command View cards, exact pinned metadata, search/keyboard, native
      tags and toast recovery. Other Op Log endpoints remain pending.
- [x] **NOTES-05I0A — Complete:** Owned-runtime preview reconstruction fixes
      stale controller state after source/fixture edits; component/style HMR
      remains qualified. Lifecycle/cache commands complete NOTES-05I0 below.
- [x] **NOTES-05I0B — Complete:** Notes `2a4ed91c` / E-CATALOG-008 qualifies
      checkout/port-scoped lifecycle and stopped-cache commands, exact
      process/readiness checks and foreign-listener/data safeguards.
- [x] **NOTES-05I0 — Complete:** Runtime/style HMR and managed preview
      maintenance are qualified on macOS ARM64; other platforms remain
      unqualified.
- [x] **NOTES-05I2B/05I2C — Complete:** Operation parent cards and header
      operation-snapshot Command View pickers; exact pin semantics retained;
      focused production acceptance is in E-REPO-015.
- [x] **NOTES-05I3B — Complete:** Split diff fills its available column/card
      height using the public shared layout contract, including narrow panes;
      E-REPO-015 qualifies the affected repository family.
- [x] **NOTES-05I6 — Complete:** Reviewed UI lives in production; workspace,
      revision and conflict inspection use retained native reads. Storybook
      imports the same components. E-REPO-015 records focused functional/axe,
      source/build checks and precise native capability limits.
- [ ] **NOTES-05I — Waiting:** Complete the source/browser-confirmed parity gaps
      in [the review](repository-view-parity-review.md). Native query/evolution,
      write capabilities, exact statistics and remaining state/whole-view
      acceptance run as ordered slices; remaining qualification is deferred.
- [ ] **NOTES-07 — Waiting for NOTES-07K/L:** Add receipt-aware
      AgentStore/ThreadStore conversation adapters and explicit per-vault
      backend selection, retaining legacy file conversations. The reusable
      adapter is complete in NOTES-07B; NOTES-07C supplies Notes' journal and
      NOTES-07D its native backend profile and owner service; NOTES-07E connects
      native host/IPC and closure. NOTES-07G completes native conversation
      preparation and original-receipt recovery. NOTES-07H completes real
      selected-store processor construction. NOTES-07I/J qualify bounded
      confirmed reads and retained continuation. Next add renderer projection,
      executable/credential admission, ordinary runtime dispatch and per-vault
      renderer persistence. NOTES-07F1/F2 complete shared and Notes
      retained-authority prerequisites; no managed conversation gate is closed.
- [x] **NOTES-08A — Complete (review implementation):** Document/classic shared
      transcript and focused checks; baseline approval remains separate.
- [x] **NOTES-08B — Complete:** Notes E-AI-041 adopts the ordinary shared UI,
      retains per-vault preference and chat actions, and qualifies all 164
      affected UI checks plus cold rebuilt-native review.
- [x] **NOTES-07L-FILES — Complete:** Notes E-AI-042 qualifies actual installed
      Codex chat, bounded file context, cancellation and local transcript
      saving. Lapis AI `25f488e8` supplies the private Codex compatibility
      profile. No JJ conversation persistence is enabled.
- [x] **NOTES-09-FILES-A — Complete:** Cold History reopen retains the exact
      authored prompt/answer, all ten transcript/binding journals and all 110
      source/Git/JJ files. No prompt replay or repository publication occurs;
      E-AI-042 records the native review.
- [x] **NOTES-09-FILES-B — Complete:** E-AI-043 saves the exact authored prompt
      and intent before provider dispatch; read-only cold recovery shows an
      unknown outcome without Retry. Real Codex crash/reopen preserves the
      journal and a new explicit message completes without replaying the old
      one. Focused service/controller/browser/native checks pass.
- [x] **NOTES-09-FILES — Complete for the local Codex profile:** E-AI-042/043
      qualify normal saving, cancellation and cold saved/unknown recovery. Wider
      provider/platform/distribution gates remain separate.
- [x] **NOTES-09-FILES-C — Complete:** Notes E-AI-044 and Lapis AI `d3a1785b`
      preserve complete output across bounded helper, native, renderer and UI
      delivery. Real Codex saves all 500 requested lines exactly; crash/cold
      recovery still retains the unknown prompt without replay. The refreshed
      ordinary preview preserves existing notes and conversations. Broader
      provider, JJ, application-tool and distribution gates remain separate.
- [x] **NOTES-09-FILES-D1 — Complete:** Notes E-AI-045 gives live and saved
      message/tool blocks identical identities, preserving source provenance and
      avoiding duplicate assistant entries after close.
- [x] **NOTES-09-FILES-D2 — Complete:** Notes E-AI-045 persists submitted
      comment captures before dispatch and restores only exact acknowledged
      feedback. Code/table stories retain original quotes and row context, Sent
      status, both presentations and continuation handoff. Real installed Codex
      accepts a comments-only turn; cold History restores visible inline
      feedback with unchanged transcript and no replay. File snapshots stay
      transient; JJ persistence remains deferred. One native pre-dispatch
      startup timeout is retained; the unchanged isolated500-line check passes.
- [ ] **NOTES-07K — Deferred by user:** Native JJ listing/creation/pinned reads;
      preserve the client draft for later, without enabling it now.
- [ ] **NOTES-07L — Deferred JJ capture:** Run bundled lapis-agent with native
      capture and explicit configured provider/model; first scope is chat, file
      context and cancel.
- [ ] **NOTES-08:** Connect shared conversation store/workspace/context adapters
      to the existing Notes AI plugin and reusable Design Core Chat
      presentation.
- [ ] **NOTES-09:** Prove one real native JJ-backed conversation,
      uncertain-write recovery and cold restart without replaying an unknown
      prompt. This, together with visible history/diffs, qualifies the first
      usable milestone.
- [x] **TOOLS-01A — Complete:** Lapis AI `e3bfffde` supplies the independent
      `lapis-ai-app-tools` crate, with an injected immutable tool list and async
      handler. All 15 byte-stream, identity, bounds, cancellation/deadline,
      unknown-effect and EOF/drop cases pass, including blocked output. Its
      verified Cargo archive, crate README and full embedding guide are
      available. The engine grants no native tool access by itself; Notes owns
      runtime admission and application policy separately.
- [x] **TOOLS-01B — Complete:** Authenticated native IPC and owned bridge
      lifetime reuse the existing AppToolHost, registry, scope and approvals.
- [x] **TOOLS-01B1 — Complete:** Notes `34ae452a` / E-AI-046 supplies a
      native-only authenticated socket and bundled stdio role, strict opaque IPC
      and exact original-owner cleanup. Cold helper tests pass without Tauri
      startup. Cancellation retains actual callback ownership; errors after
      dispatch remain unknown. Current policy is rechecked after
      approval/capture waits. Lapis AI `96ced413` validates snapshots before
      allocation and redacts MCP environment diagnostics. The runtime capability
      remained disabled until the B2/C1 acceptance below.
- [x] **TOOLS-01B2 — Complete:** Notes `e322e36e` / E-AI-047 adapts the existing
      coordinator, approvals, local event saving and App status API. Native
      Codex aliases admit exactly one worker under the original vault, window
      and generation. Tools arm after durable turn preparation; completion waits
      for callback/publication drain. Lost acknowledgements reconcile the
      original reply without executing again. Owner decisions retain separate
      journal identities and error notices retain their parent.
- [ ] **TOOLS-01C — Waiting for wider qualification:** Qualify actual reads,
      guarded edits, cancellation, unknown outcomes and cold cleanup while
      retaining local transcript files. Original M10.3 gates remain separate.
- [x] **TOOLS-01C1 — Complete for installed Codex/local files:** E-AI-047 passes
      real read → approved edit → cancel → cold History. The note stays
      unchanged before explicit App approval; cancellation prevents the second
      unapproved edit and retains an honest unknown outcome. Cold reopening
      neither replays callbacks nor changes journal/note bytes or creates `.jj`.
      All 462 AI policy tests, 29 adapter cases, 14 native service/helper cases,
      the exact native launch case, normal/e2e strict Rust checks and both
      application builds pass. The ordinary review app preserves all 131 files.
- [ ] **TOOLS-01C2 — Deferred qualification:** Inject mounted post-dispatch
      failures, delayed actual effects, lost replies and process teardown across
      the remaining provider/platform profiles. Source fixtures and one real
      pending-edit cancellation do not complete this original qualification.
- [ ] **AGENT-01 — In progress:** Extract/adopt participant addressing,
      persisted agent tags, multiple recipients and queued delivery with exact
      identity/authority separation.
- [x] **AGENT-01A — Complete:** Notes `6933af75` / E-AI-048 reuses the shared
      agent-token codec and adapts removable recipient pills. Exact metadata
      saves separately from authored text before dispatch and restores in
      History. Failed or unknown Send retains recipients and attachments. All
      469 AI tests, 120 affected story interaction/axe cases, strict types,
      source audits and fresh real Codex/native read/edit/cancel/cold History
      pass. Ordinary native pointer selection works; all 131 review-vault files
      remain byte-identical. Multiple recipients still refuse explicitly.
- [ ] **AGENT-01B:** Add independent logical participants and explicit
      multiple-recipient delivery, retaining per-recipient outcomes.
- [ ] **AGENT-01C:** Add persisted bounded queues with explicit cancellation and
      unknown-outcome recovery; no implicit replay on restart.
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

- [Addressed Codex message ready to send](http://127.0.0.1:7490/iframe.html?id=desktop-ai-addressed-messages--ready-to-send&viewMode=story)

- [AI document conversation](http://127.0.0.1:7490/iframe.html?id=desktop-ai-transcript-review--document-conversation&viewMode=story)
- [AI comments as context](http://127.0.0.1:7490/iframe.html?id=desktop-ai-transcript-review--comments-as-context&viewMode=story)
- [Restored local code/table comments](http://127.0.0.1:7490/iframe.html?id=desktop-ai-comment-history--restored-code-and-table-feedback&viewMode=story)

The rebuilt native review app also runs real Codex with local files; its
file-context conversation is open in the main panel after cold History reopen.
It uses the documented device configuration in Notes `docs/local-ai.md`.

These opt-in stories mount production AI components with a disposable memory
session host. Document/classic share the controller, drafts and anchored
comments. Source-owned fixes center avatars, render tables and fenced code, and
remove embedded Markdown whitespace. They do not qualify native JJ chat.

- [JJ Log and Op Log in separate workspace tabs](http://127.0.0.1:7490/iframe.html?id=desktop-repository-comparison--jj-log-and-op-log-tabs&viewMode=story)
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
