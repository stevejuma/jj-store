# M26 installation profiles and custom emoji

| ID    | Status  | Depends     | Requirements                                   | Gate                                                                  | Next                                                                      | Evidence                            |
| ----- | ------- | ----------- | ---------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------- | ----------------------------------- |
| M26   | Partial | M3.1,M15.1  | JST-SEC-008,JST-API-010,JST-UI-021,JST-OBJ-006 | Cold local/authenticated object-store profile and emoji flow          | Retain functional acceptance; qualify remaining visual and wider lanes    | E-PROFILES-001,E-PROFILE-AVATAR-002 |
| M26.1 | Partial | M3.1        | JST-SEC-008,JST-OBJ-006                        | Immutable directory, identity isolation, receipts and restart         | Retain storage, revocation and unknown-outcome evidence                   | E-PROFILES-001,E-PROFILE-AVATAR-002 |
| M26.2 | Partial | M26.1       | JST-API-010,JST-UI-021                         | Profile/library APIs and Community Settings functional/axe acceptance | Retain interaction and accessibility acceptance; visual approval separate | E-PROFILES-001,E-PROFILE-AVATAR-002 |
| M26.3 | Partial | M26.1,M15.1 | JST-API-010,JST-OBJ-006                        | Repository-pinned emoji history survives directory changes            | Retain native/local snapshot closures; M15 qualification stays separate   | E-PROFILES-001                      |
| M26.4 | Partial | M26.2,M26.3 | JST-SEC-008,JST-API-010,JST-UI-021,JST-OBJ-006 | Cold real-RustFS local and authenticated browser restart flow         | Retain cold production acceptance; qualify remaining broader server lanes | E-PROFILES-001,E-PROFILE-AVATAR-002 |

Implementation and focused functional gates are delivered in Jujutsu change
qwxwqrxp. This milestone does not promote M15 or claim release, visual-baseline,
replication, other-platform or full-server-suite qualification.

## Image normalization and Settings review — 2026-10-12

E-PROFILE-AVATAR-002 in enclosing change `pklqtuzx` qualifies the shared bounded
avatar/emoji normalizer, stable upload/definition recovery and deferred emoji
uploads, owned Settings presentation and cold local/authenticated real-RustFS
restart flow. M26, M26.1, M26.2 and M26.4 remain Partial. Visual baselines,
wider server/release/platform lanes and inherited audit/evidence diagnostics
remain separate. M26.3 and M15 snapshot/publication qualification are unchanged.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
