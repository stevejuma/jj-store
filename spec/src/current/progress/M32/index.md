# M32 injectable collaboration libraries

| ID    | Status      | Depends     | Requirements                                                     | Gate                                                                          | Next                                         | Evidence                |
| ----- | ----------- | ----------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------- | -------------------------------------------- | ----------------------- |
| M32   | In progress | M9.6,M13.1  | JST-ARCH-006,JST-LOCAL-001,JST-LOCAL-003,JST-PUB-004,JST-PUB-005 | Five injectable stores, both providers and independent consumer qualification | Qualify independent consumers                | E-INJECTABLE-STORES-005 |
| M32.1 | In progress | M9.6        | JST-ARCH-006                                                     | Typed store interfaces, named factories and shared engines/codecs             | Qualify both provider compositions           | E-INJECTABLE-STORES-004 |
| M32.2 | In progress | M32.1       | JST-ARCH-006,JST-LOCAL-001,JST-LOCAL-003                         | Lightweight Git provider and focused JJ evidence integration                  | Qualify external Git consumer                | E-INJECTABLE-STORES-004 |
| M32.3 | In progress | M32.1       | JST-ARCH-006,JST-PUB-004,JST-PUB-005                             | Native provider behind shared interfaces with original authorization          | Complete native consumer acceptance          | E-INJECTABLE-STORES-005 |
| M32.4 | In progress | M32.2,M32.3 | JST-ARCH-006                                                     | Application migration and external-consumer/provider acceptance               | Qualify standalone consumer and cold storage | E-INJECTABLE-STORES-005 |

The enclosing Jujutsu change is the revision. M9, M11, M13 and M15 retain their
existing status and qualification gaps. This milestone does not qualify new
storage profiles, publish packages or enable distributed publication.

E-CRATE-READMES-001 records the requested crate overview and usage
documentation. M32 remains In progress for provider composition, native adapters
and independent consumer qualification.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
