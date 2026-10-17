# M32 injectable collaboration libraries

| ID    | Status | Depends     | Requirements                                                     | Gate                                                                          | Next                             | Evidence                |
| ----- | ------ | ----------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------- | -------------------------------- | ----------------------- |
| M32   | Done   | M9.6,M13.1  | JST-ARCH-006,JST-LOCAL-001,JST-LOCAL-003,JST-PUB-004,JST-PUB-005 | Five injectable stores, both providers and independent consumer qualification | None within source-library scope | E-INJECTABLE-STORES-006 |
| M32.1 | Done   | M9.6        | JST-ARCH-006                                                     | Typed store interfaces, named factories and shared engines/codecs             | None within source-library scope | E-INJECTABLE-STORES-006 |
| M32.2 | Done   | M32.1       | JST-ARCH-006,JST-LOCAL-001,JST-LOCAL-003                         | Lightweight Git provider and focused JJ evidence integration                  | None within source-library scope | E-INJECTABLE-STORES-006 |
| M32.3 | Done   | M32.1       | JST-ARCH-006,JST-PUB-004,JST-PUB-005                             | Native provider behind shared interfaces with original authorization          | None within source-library scope | E-INJECTABLE-STORES-006 |
| M32.4 | Done   | M32.2,M32.3 | JST-ARCH-006                                                     | Application migration and external-consumer/provider acceptance               | None within source-library scope | E-INJECTABLE-STORES-006 |

The enclosing Jujutsu change is the revision. M9, M11, M13 and M15 retain their
existing status and qualification gaps. This milestone does not qualify new
storage profiles, publish packages or enable distributed publication.

E-CRATE-READMES-001 records the requested crate overview and usage
documentation. All 31 tracked crate roots now have READMEs; detailed embedding
guidance remains under docs.

E-INJECTABLE-STORES-006 completes the source-library scope with an independent
Cargo consumer, all four feature profiles, active dependency audits, a custom
WorkStore requiring no sibling implementation, Git GC/source preservation and
native preparation/checkpoint/replay. E001-E004 retain core/provider format and
fault evidence; E005 retains application CLI/RPC/HTTP/capture acceptance and the
real cold embedded-RustFS five-store replay proof. The retained development
fixture is restored in Vite mode without reset. WorkflowStore injection,
additional platforms/topologies and release/publication remain separate work.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
