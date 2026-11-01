# M3 tasks and gates

| ID   | Status | Depends | Requirements                                    | Gate                                                            | Next                              | Evidence                                                                                                                                                               |
| ---- | ------ | ------- | ----------------------------------------------- | --------------------------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M3   | Done   | M2      | JST-JJ-001,JST-JJ-002,JST-CLI-001,JST-SEC-001   | Native CLI/server vertical slice with authentication            | Implement M4 pinned read sessions | E-M3-001,E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006,E-M3-007,E-M3-008,E-M3-009,E-M3-010,E-M3-011,E-M3-012,E-M3-013,E-M3-014,E-M3-015,E-UI-044,E-LOCAL-TEST-AUDIT-001 |
| M3.1 | Done   | M2      | JST-SEC-001,JST-SEC-002,JST-SEC-003,JST-SEC-004 | Revocable tokens and transport authorization                    | Retain native slice acceptance    | E-M3-001,E-M3-004,E-M3-005,E-M3-006,E-M3-007,E-M3-008,E-M3-009,E-M3-011,E-M3-012,E-M3-014                                                                              |
| M3.2 | Done   | M3.1    | JST-JJ-001,JST-JJ-004,JST-JJ-005                | Create edit describe log restart op-log evolog; multi-workspace | Retain native slice acceptance    | E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006,E-M3-008,E-M3-009,E-M3-010,E-M3-011,E-M3-013,E-M3-014,E-LOCAL-TEST-AUDIT-001                                              |
| M3.3 | Done   | M3.2    | JST-CLI-001,JST-CLI-002,JST-CLI-003,JST-CLI-004 | Custom command presentation and capability parity               | Retain native slice acceptance    | E-M3-006,E-M3-009,E-M3-010,E-M3-011,E-M3-012,E-M3-013,E-M3-014,E-M3-015,E-UI-050,E-BUILD-STORAGE-001                                                                   |

<!-- generated-navigation -->

## Pages

- [history](history/index.md)

E-BUILD-STORAGE-001 reviews managed development build maintenance and cache
custody without changing this milestone's status. Package-clean dependency
retention, native/managed locks, live-consumer refusal, interrupted cleanup,
independent JJ workspace builds and cold UI startup have separate focused
evidence. Cleanup of the completed desktop integration target measures actual
free-space recovery. The running main target remains protected; this development
slice does not qualify additional desktop/platform or release gates.

E-LOCAL-TEST-AUDIT-001 records remote Docker requalification of the pinned
native Git profile and mixed-feature RPC TLS initialization. M3 and M3.2 retain
their existing status; OS credential-provider manual gates and broader release
qualification remain separate.
