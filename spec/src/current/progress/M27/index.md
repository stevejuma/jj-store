# M27 remote server connections and authenticated agents

| ID  | Status      | Depends | Requirements                       | Gate                                                                      | Next                                                                     | Evidence                                                                                                                                                                                   |
| --- | ----------- | ------- | ---------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| M27 | In progress | M24,M26 | JST-SEC-009,JST-API-011,JST-UI-022 | Isolated connections, delegated execution and cold Linux/macOS acceptance | Qualify Linux real-storage, desktop delivery and other provider profiles | E-REMOTE-AGENTS-001,E-REMOTE-AGENTS-002,E-REMOTE-INTEGRATION-001,E-REMOTE-LAND-001,E-SERVER-CONTROLS-001,E-SERVER-CONTROLS-002,E-REMOTE-ADMIN-001,E-APP-RELOAD-001,E-WORKFLOW-SETTINGS-001 |

Implementation was rebased onto the current default workspace, preserving its
directory/avatar normalization and grouped Settings work. Integration and
inherited cleanup are recorded in E-REMOTE-INTEGRATION-001 and
E-LEDGER-CLEANUP-001. E-REMOTE-LAND-001 records landing in default, retiring
remote-agents and restarting main with all four existing repositories retained.
Real storage, installed agents, desktop/browser cold runs and each platform gate
require separate evidence.

The gateway, scoped production owners, authenticated execution/delegation and
cold production-App fixture checks pass. E-REMOTE-AGENTS-002 adds macOS cold
built-App and two-instance real RustFS acceptance with a deterministic ACP
process: live gateway freshness, directory/avatar isolation, native publication,
independent grant revocation, installation fencing and exact restart recovery.
This qualifies that integration slice, not installed providers, Linux real
storage or desktop delivery. The production freshness Origin regression is
recorded with its failing before state and passing correction. The inherited
dialog stories, source-limit violations and malformed historical metadata first
recorded in E-REMOTE-AGENTS-001 are repaired by the integration records.
Original evidence payloads and outcomes remain immutable. No platform or
provider gate is promoted by cleanup or deterministic ACP acceptance.

E-SERVER-CONTROLS-001 qualifies sidebar-matched server buttons, the two-entry
limit and horizontal-kebab overflow, and the shared Settings connection form.
Focused interaction/Axe, owner-history/guard checks and live main responsive
review pass. Switching, authorization, tokens, drafts and development data are
retained. M27 stays In progress; platform/provider and visual gates are
unchanged.

E-SERVER-CONTROLS-002 corrects compressed Add/More action sizing and moves the
connection introduction into the card header. Focused sizing, interaction and
accessibility acceptance passes; the existing M27 gates stay unchanged.

E-REMOTE-ADMIN-001 qualifies independent `manage-agents` configuration/sharing,
identity enrollment and explicit native Write onboarding. The cold built App
checks independent roles, drafts, mobile/Axe, reload, history and secret
non-persistence. Two disposable macOS real-RustFS servers exercise installed
Codex through the gateway: an isolated file edit, session publication, exact
restart recovery without prompt replay and identity revocation. This includes
the authenticated Agents/Threads route-restoration correction and compact
connection-dialog header spacing. Main development data remains retained.

M27 remains In progress. This installed macOS Codex slice does not qualify
Linux, desktop delivery, other providers, hostile-user sandboxing, automatic
home provisioning or Git transfer. Those gates remain separate; no visual
baseline is updated. E-APP-RELOAD-001 qualifies saved operational reload without
server restart, including current protection/sharing, retained capture and sync
owners, original-outcome recovery and cold production UI. The
[runtime reload review](runtime-reload-review.md) keeps M24/M27 In progress and
their independent platform/provider and visual gates unchanged.
E-WORKFLOW-SETTINGS-001 reviews M27 with status unchanged. LocalTrusted Admin
host configuration separates conditional saves from explicit workflows reload.
Shared native/HTTP admission and a CLI-owned optional runtime retain listeners,
journals and original uncertain work. Focused UI, activation and cold acceptance
qualify this configuration slice; provider, Forgejo protocol, additional
platform and visual baseline gates remain separate.
<!-- generated-navigation -->

## Pages

- [Runtime reload review](runtime-reload-review.md)
- [M27 task histories](history/index.md)
