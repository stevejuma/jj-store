---
schema: 1
id: E-EXEC-003
---

# E-EXEC-003

<!-- evidence-field:id markdown -->
## Id

E-EXEC-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-API-001
- JST-SEC-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->



<!-- evidence-field:tests list -->
## Tests

- web/tests/agents-route.test.ts#installation Agents routes round trip without a repository
- web/tests/vite/agent-execution-controller.test.ts#stopping an agent targets its session without changing another chat or replaying a prompt
- crates/jj-store-server/src/agent_execution/commands.rs#presence_requires_live_process_and_completed_handshake
- crates/jj-store-server/src/agent_execution/journal.rs#journal_restart_retains_agent_association_but_never_connection
- crates/jj-store-server/src/agent_execution/avatar.rs#community_avatar_forms_are_bounded_and_inert
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

macOS ARM64; jj 0.45.1; rustc 1.98.1; Node 24.10.0; pnpm 10.32.1; Svelte 5; Vitest 4.1.11; Storybook 10.5.7. Community source 741b8285e2081f1c9af58afe5a9b72f538fad781 (Apache-2.0). Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be inspected as lifecycle inspiration.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional setup_id and last_activity_at_millis, and read-only connected session fields. Legacy journals default missing association/timing and clear connection on load. Description defaults for saved setups remain backward compatible. Existing control version 2, journal version, native JJ identities, capture/publication formats and setup API naming remain. Bounded inert avatars and no credential persistence.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Enclosing Jujutsu change ywvxqomz. Before: repository-bound execution modal, no global management route or live setup association; Community avatars could be rejected, stale saves lacked explicit recovery, and opening details stretched gallery cards. After: owned standalone Community gallery/editor/profile and full-page inbox, fixed 208 × 260px cards and matching skeletons, compact actions and avatar Start/Stop. UI management stays independent of repository selection. No separate Activity page remains: active-session navigation and legacy redirects retain exact session links.

The inbox uses the shared list toggle, a padded header and viewport-bounded 18rem menu, centered Settings-styled draft pickers, an empty-state icon, history skeleton and one composer-owned Send/Queue action. Workspace and runtime is a titled right body sidebar with complete StoreReadOnlyRow copy/link rows, shared Community runtime fields and grouped review/red cleanup rows. Rows show the associated configured agent; unassigned/deleted-agent sessions use retained session values. Disconnected chats keep editable drafts and explain explicit resume. Connected explicit Send applies configured choices only before a confirmed-ready prompt; failed/unknown configuration or navigation retains the draft. Workspace edits open exactly the same StoreAgentSetupEditor/CommunityAgentEditorDialog as agent details. A conditional future-default save is followed by an explicit exact-session configure only for a connected matching harness and selected mode. Saved revision, chat/editor drafts and independent pending identities remain retained on refusal or uncertainty.

Offline agent editing uses its latest retained negotiated harness catalog and labels remembered availability; current negotiation takes precedence even if empty. A missing model catalog preserves unknown saved preferences. Starting revision index recovery is explicit, repository/operation pinned, bounded, cancelable and offered only for the structured matching index-required refusal. Ordinary failures do not create an index or erase the selected revision/draft. Session list timing comes only from retained events; restart preserves it and legacy unknown time stays absent.

Live CUA verification observed Nyra's configured gpt-5.6-luna / medium / agent-full-access values in the new sidebar and the identical full agent editor. Screenshots in .local/agent-community-settings-final.png are observations, never baseline approval. No session command was issued by browser verification. A bounded direct installed Codex ACP session/load diagnostic, with no prompt or tool operations, independently classified the original resume refusal as session_not_found. Safe agent_rejected remains the public ACP boundary; new-session-with-saved-context recovery stays explicit. Resume/command failures now appear in the composer without a duplicate body notice or toast, retaining the draft. Retained history prefers the current attempt's safe ACP error, and a later successful lifecycle event cannot revive an older refusal. The chat reuses Community's composer-row styles and shared LayoutScrollButton. Long history, growing draft and keyboard jump regression passed; live 1280px inspection measured viewport bottom and composer top both 522px. .local/agent-composer-row-final.png and .local/agent-composer-latest-final.png are observations, not baselines. The development process was restored with the existing binaries and preserved fixture after interruption; subsequent backend-only restart restored the explicit ACP/JJ environment while Vite remained running.

Before/after regressions: missing global route tests, stale-save recovery and gallery stretching were repaired in their exact targets. Browser attempts exposed released response bodies, theme-transition timing, a historical operation without newer recorded activity and avatar hit targets; repaired harness checks retain exact history/permission assertions. Two new model stories initially had ambiguous text selectors; scoping the model assertion to the gallery fixed both. A plain Node index test could not import Svelte Query presentation, so the same three transport tests moved to the existing Vite catalog-unit runner and passed. Cold readiness diagnostics proved that the composer returned early while its configure receipt was dispatching and therefore never posted the prompt. It now polls only reads for the exact accepted configuration receipt and negotiated readiness before dispatching the captured explicit message. Unknown/failed outcomes and navigation retain the draft and send no prompt. The final cold run passed with exact prompt-response and Working/Online assertions intact. Conditional-save regression initially returned no acknowledged revision; the controller now returns the saved response revision before a later refresh, preventing silent precondition adoption after a successful save followed by failed configuration. Counted-line failures were resolved through focused helper extraction, without increasing budgets or suppressing diagnostics.

Optional strict Clippy failed on inherited policy.rs:70 and native_inventory cursors.rs:113/cache.rs:62 lints; no suppression or native behavior edits. Focused compile/tests passed. The unrelated filesystem contract refusal was repaired and qualified separately by E-EXEC-005. Root aggregate Rust/UI suites, unrelated packages, installed release qualification and visual baselines remain outside the targeted policy. No reset, publication, installed CLI replacement or unrelated service restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain the qualified macOS LocalTrusted profile; visual baselines, release/package publication and other platforms/harnesses remain separate gates. Live Vite fixture is preserved. Native source edits used disposable workspaces only.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-015, JST-COLLAB-031 through JST-COLLAB-033, API and security owning topics now specify global management, inbox/activity-link routing, live status/association and bounded avatar presentation. Exact prompt, permission, uncertainty, snapshot and manifest publication authority remain intact.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains scoped to the qualified local macOS Codex/Cursor profile. This record extends UI and association evidence without claiming release, visual-baseline, other-platform or other-harness qualification. Final cold evidence is recorded below.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-EXEC-003 files](details-02.md)
- [E-EXEC-003 validation](details-03.md)
- [E-EXEC-003 final repairs and governance](details-04.md)
