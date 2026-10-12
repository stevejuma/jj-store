# M27 remote server connections and authenticated agents

| ID  | Status      | Depends | Requirements                       | Gate                                                                      | Next                                                                 | Evidence                                                         |
| --- | ----------- | ------- | ---------------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------- |
| M27 | In progress | M24,M26 | JST-SEC-009,JST-API-011,JST-UI-022 | Isolated connections, delegated execution and cold Linux/macOS acceptance | Qualify installed providers, Linux real-storage and desktop delivery | E-REMOTE-AGENTS-001,E-REMOTE-AGENTS-002,E-REMOTE-INTEGRATION-001 |

Implementation was rebased onto the current default workspace, preserving its
directory/avatar normalization and grouped Settings work. Integration and
inherited cleanup are recorded in E-REMOTE-INTEGRATION-001 and
E-LEDGER-CLEANUP-001. Real storage, installed agents, desktop/browser cold runs
and each platform gate require separate evidence.

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
<!-- generated-navigation -->

## Pages

- [M27 task histories](history/index.md)
