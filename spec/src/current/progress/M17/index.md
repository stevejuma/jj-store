# M17 tasks and gates

| ID    | Status      | Depends | Requirements                                                                  | Gate                                                                            | Next                                                                        | Evidence                                                                                           |
| ----- | ----------- | ------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| M17   | In progress | M8a.4   | JST-DESK-001,JST-DESK-002,JST-DESK-003,JST-DESK-004,JST-DESK-005,JST-DESK-006 | Managed macOS ARM64 desktop application                                         | Qualify native host, controls and package                                   | E-DESK-001,E-DESK-002,E-DESK-003,E-DESK-004,E-DESK-005,E-DESK-006,E-DESK-007,E-DESK-008,E-DESK-010 |
| M17.1 | In progress | M8a.4   | JST-DESK-003                                                                  | Opt-in bounded parent-pipe service supervision                                  | Retain real RustFS and ordinary process evidence                            | E-DESK-001,E-DESK-003                                                                              |
| M17.2 | In progress | M17.1   | JST-DESK-001,JST-DESK-002,JST-DESK-004                                        | Native host and isolated production UI startup                                  | Qualify actual WKWebView and recovery                                       | E-DESK-001,E-DESK-002,E-DESK-003,E-DESK-004,E-DESK-005,E-DESK-006,E-DESK-010,E-BUILD-STORAGE-001   |
| M17.3 | In progress | M17.2   | JST-DESK-004,JST-DESK-005                                                     | Native folder dialogs, chrome, first-render handshake and dirty-edit protection | Qualify remaining native controls and render/dirty-edit gates               | E-DESK-002,E-DESK-003,E-DESK-005,E-DESK-006                                                        |
| M17.4 | Planned     | M17.3   | JST-DESK-005                                                                  | Cold ARM64 app and DMG persistence and relocation                               | Package and qualify local distribution                                      | E-DESK-010                                                                                         |
| M17.5 | Partial     | M17.2   | JST-DESK-006,JST-UI-002,JST-UI-006,JST-SEC-005                                | Desktop repository terminal and retained shell sessions                         | Native terminal qualified; retain M16 registry and M17.2 host prerequisites | E-DESK-007,E-DESK-008,E-DESK-009                                                                   |

E-MAIN-LOCAL-REPAIRS-001 retains M17.2 In progress. Equivalent optional ACP file
validation passes fresh strict desktop Clippy, the complete desktop workspace
tests, native checking and isolated staged service restart acceptance. Signed
app/DMG, WKWebView, relocation and additional platform gates remain independent.

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
