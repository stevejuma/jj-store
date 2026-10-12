# M27 remote server connections and authenticated agents

| ID  | Status      | Depends | Requirements                       | Gate                                                                      | Next                                                                 | Evidence                                |
| --- | ----------- | ------- | ---------------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------- | --------------------------------------- |
| M27 | In progress | M24,M26 | JST-SEC-009,JST-API-011,JST-UI-022 | Isolated connections, delegated execution and cold Linux/macOS acceptance | Qualify installed providers, Linux real-storage and desktop delivery | E-REMOTE-AGENTS-001,E-REMOTE-AGENTS-002 |

Implementation uses Jujutsu workspace remote-agents. Existing directory/avatar
work in the default workspace remains untouched. Real storage, installed agents,
desktop/browser cold runs and each platform gate require separate evidence.

The gateway, scoped production owners, authenticated execution/delegation and
cold production-App fixture checks pass. E-REMOTE-AGENTS-002 adds macOS cold
built-App and two-instance real RustFS acceptance with a deterministic ACP
process: live gateway freshness, directory/avatar isolation, native publication,
independent grant revocation, installation fencing and exact restart recovery.
This qualifies that integration slice, not installed providers, Linux real
storage or desktop delivery. The production freshness Origin regression is
recorded with its failing before state and passing correction. Inherited dialog
stories, oversized files and historical ledger diagnostics are recorded
separately in E-REMOTE-AGENTS-001; no gate is promoted by those passes.
<!-- generated-navigation -->

## Pages

- [M27 task histories](history/index.md)
