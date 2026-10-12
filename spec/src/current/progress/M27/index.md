# M27 remote server connections and authenticated agents

| ID  | Status      | Depends | Requirements                       | Gate                                                                      | Next                                                                | Evidence            |
| --- | ----------- | ------- | ---------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------- |
| M27 | In progress | M24,M26 | JST-SEC-009,JST-API-011,JST-UI-022 | Isolated connections, delegated execution and cold Linux/macOS acceptance | Qualify installed-runtime, real-storage and platform delivery gates | E-REMOTE-AGENTS-001 |

Implementation uses Jujutsu workspace remote-agents. Existing directory/avatar
work in the default workspace remains untouched. Real storage, installed agents,
desktop/browser cold runs and each platform gate require separate evidence.

The gateway, scoped production owners, authenticated execution/delegation and
cold production-App fixture checks pass. Source fixtures and MemoryStore
execution do not qualify installed providers or two-instance real RustFS.
Inherited dialog stories, oversized files and historical ledger diagnostics are
recorded separately in E-REMOTE-AGENTS-001; no gate is promoted by those passes.
<!-- generated-navigation -->

## Pages

- [M27 task histories](history/index.md)
