# M33 reusable agent workflows and Notes integration

| ID    | Status      | Depends     | Requirements                | Gate                                                                          | Next                                         | Evidence                                                                                                                            |
| ----- | ----------- | ----------- | --------------------------- | ----------------------------------------------------------------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| M33   | In progress | M32         | JST-ARCH-007,JST-COLLAB-031 | Reusable capture and conversation libraries consumed by jj-store and Notes    | Add source-backed conversation adapters      | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003,E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006 |
| M33.1 | In progress | M32         | JST-ARCH-007                | Public boundaries and preserved-format compatibility fixtures                 | Review shared workflow contracts             | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003                                                                   |
| M33.2 | In progress | M33.1       | JST-ARCH-007,JST-COLLAB-031 | Standalone native redaction and source readers with original capture recovery | Complete remaining source portability gates  | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003                                                                   |
| M33.3 | In progress | M33.2       | JST-ARCH-007                | Injectable conversation engine and typed client consumed by jj-store          | Add store/workspace adapters and shared UI   | E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006                                                                   |
| M33.4 | Planned     | M33.3       | JST-ARCH-007,JST-LOCAL-001  | Focused local-history feature and admitted Tauri bridge                       | Add local history and native event ownership | -                                                                                                                                   |
| M33.5 | Planned     | M33.4       | JST-ARCH-007                | Cold Notes history and one recoverable JJ-backed AI conversation              | Integrate existing AI plugin and Design Core | -                                                                                                                                   |
| M33.6 | Planned     | M33.5       | JST-ARCH-007                | Addressed participants, queues and exact anchored discussion recovery         | Integrate shared agent addressing            | -                                                                                                                                   |
| M33.7 | Planned     | M33.5       | JST-ARCH-007                | Explicit deduplicated imports and opt-in local automatic capture              | Qualify local source lifecycle               | -                                                                                                                                   |
| M33.8 | Planned     | M33.6,M33.7 | JST-ARCH-007                | Independent Rust/Tauri consumers and public package archive acceptance        | Complete downstream qualification            | -                                                                                                                                   |

The first usable milestone is M33.5. Existing M24/M25 and desktop AI gates
retain their original requirements. Code mutations, automatic repository
conversion, metadata synchronization, distribution and publication remain
separate work.

<!-- generated-navigation -->

## Pages

- [Evidence history](history/index.md)
