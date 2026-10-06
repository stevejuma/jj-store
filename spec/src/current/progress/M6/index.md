# M6 tasks and gates

| ID   | Status | Depends | Requirements                                            | Gate                                                         | Next                                               | Evidence                                                                                     |
| ---- | ------ | ------- | ------------------------------------------------------- | ------------------------------------------------------------ | -------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| M6   | Done   | M5      | JST-ADAPT-001,JST-ADAPT-002,JST-ADAPT-003,JST-ADAPT-004 | Replay-equivalent searchable projection with freshness       | Retain snapshot, search and deployment conformance | E-M6-001,E-M6-002,E-M6-003,E-M6-004,E-M6-005,E-M6-006,E-M6-007,E-M6-008,E-M6-009,E-M6-010    |
| M6.1 | Done   | M5      | JST-ADAPT-001,JST-ADAPT-002                             | Ordered committed reader and durable replay-safe checkpoints | Implement Tantivy projection and search            | E-M6-001                                                                                     |
| M6.2 | Done   | M6.1    | JST-ADAPT-003,JST-ADAPT-004                             | Tantivy crash/rebuild/generation and revocation tests        | Retain snapshot, search and deployment conformance | E-SCOPE-001,E-M6-002,E-M6-003,E-M6-004,E-M6-005,E-M6-006,E-M6-007,E-M6-008,E-M6-009,E-M6-010 |
<!-- generated-navigation -->
## Pages

- [history](history/index.md)
