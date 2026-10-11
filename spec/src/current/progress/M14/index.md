# M14 tasks and gates

| ID    | Status      | Depends                 | Requirements                             | Gate                                                            | Next                                                                     | Evidence                                                                                                                                                                                                                                                         |
| ----- | ----------- | ----------------------- | ---------------------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M14   | In progress | M4.18                   | JST-UI-012,JST-UI-016                    | Local settings, profile-aware device links and safe maintenance | Implement M14.3 agent setup and automatic recovery                       | E-M14-001,E-M14-002,E-M14-003,E-M14-004,E-M14-005,E-M14-006,E-M14-007,E-M14-010,E-M14-011,E-M14-012,E-UI-060,E-M14-013,E-M14-014,E-UI-095,E-UI-098,E-M14-016,E-M14-017,E-M13-028                                                                                 |
| M14.1 | Done        | M4.18                   | JST-UI-012                               | Graph preparation independent of text-search health             | Retain independent graph/search regressions; proceed to M14.2            | E-M14-001,E-M14-011,E-M14-012,E-UI-049,E-SETTINGS-REFRESH-001,E-SETTINGS-SKELETON-001                                                                                                                                                                            |
| M14.2 | Done        | M14.1                   | JST-UI-016,JST-SEC-005,JST-SEC-006       | Typed local management and profile-aware workspace links        | Retain folder-link regressions and proceed to M14.3                      | E-M14-002,E-M14-003,E-M14-005,E-M14-006,E-M14-007,E-M14-010,E-M14-011,E-M14-012,E-UI-063,E-M14-017,E-M13-028,E-M13-029,E-SETTINGS-POLISH-001,E-SETTINGS-SPLIT-001,E-SETTINGS-REFRESH-001,E-SETTINGS-SKELETON-001,E-GLOBAL-SETTINGS-001,E-GLOBAL-SETTINGS-NAV-001 |
| M14.3 | In progress | M14.2                   | JST-UI-016,JST-COLLAB-002,JST-COLLAB-006 | Agent setup, queue inspection and automatic recovery            | Add agent management and worker lifecycle on the shared drain foundation | E-M14-004,E-M14-005                                                                                                                                                                                                                                              |
| M14.4 | Done        | M14.2                   | JST-UI-016,JST-ADAPT-003,JST-ADAPT-004   | Editable Search and isolated worker replacement                 | Add desired/effective configuration and safe text-search rebuild         | E-M14-009,E-M14-010,E-M14-011,E-M14-012,E-M14-016,E-SETTINGS-SPLIT-001,E-SETTINGS-REFRESH-001,E-NUMERIC-SETTINGS-001,E-SETTINGS-SKELETON-001                                                                                                                     |
| M14.5 | Done        | M14.2                   | JST-UI-016,JST-PUB-005,JST-PUB-006       | Storage integrity and non-destructive checkpoints               | Retain bounded maintenance and cold restart coverage                     | E-M14-014,E-SETTINGS-SPLIT-001,E-SETTINGS-REFRESH-001,E-SETTINGS-SKELETON-001                                                                                                                                                                                    |
| M14.6 | In progress | M14.2,M10.7             | JST-UI-016,JST-REPL-012                  | Configured sync status and controls in Settings                 | Retain typed upstream/worker editing; qualify remaining sync integration | E-M14-008,E-M14-010,E-M14-011,E-M14-012,E-M14-013,E-M14-016,E-SETTINGS-SPLIT-001,E-SETTINGS-REFRESH-001,E-NUMERIC-SETTINGS-001,E-SETTINGS-SKELETON-001,E-GLOBAL-SETTINGS-001,E-GLOBAL-SETTINGS-NAV-001                                                           |
| M14.7 | Planned     | M14.3,M14.4,M14.5,M14.6 | JST-UI-016,JST-SEC-006                   | Integrated local settings and cold persistent acceptance        | Qualify completed native flow and preserve unsupported M13 capabilities  | -                                                                                                                                                                                                                                                                |

E-SETTINGS-POLISH-001 reviews M14.2 without changing its status. Focused
Settings form, typography and keyboard/Axe acceptance adds no release/platform
or visual-baseline promotion; inherited diagnostics remain explicit.

E-SETTINGS-SPLIT-001 reviews M14.2, M14.4, M14.5, M14.6 without changing
milestone statuses. Focused route, controller, Storybook interaction/Axe and
cold browser acceptance qualify the scoped Settings separation and stable Sync
observation. Visual review, remaining sync/platform gates and inherited global
diagnostics remain separate. The enclosing Jujutsu change owns this evidence.

E-SETTINGS-REFRESH-001 reviews M14.1, M14.2, M14.4, M14.5, M14.6 without
changing milestone statuses. Repository presentation and settings/form action
guidance are updated; behavioural contracts remain unchanged. User-directed
application CSS retains published dependencies. Inherited diagnostics and visual
approval are recorded separately in the enclosing change.

E-NUMERIC-SETTINGS-001 reviews M14.4 and M14.6 with statuses unchanged. Shared
numeric unit entry and conditional per-field reset preserve Search and Sync
ranges, defaults, Save/Apply or restart behavior and canonical configuration.
Focused interaction and exact conversion evidence does not advance wider
lifecycle or visual approval gates.

E-SETTINGS-SKELETON-001 reviews M14.1, M14.2, M14.4, M14.5, M14.6 with statuses
unchanged. Representative repository Settings placeholders cover module,
discovery and initial data reads; loaded values, drafts and pending reports
remain visible during refresh. Focused interaction and live slow-read evidence
does not promote visual or release gates. The enclosing Jujutsu change is the
evidence revision.

E-GLOBAL-SETTINGS-001 reviews M14.2, M14.6 with statuses unchanged. Shared
global Settings presentation and form guidance retain existing save, permission,
draft and transient navigation semantics. Focused interaction/live acceptance
does not promote visual or release gates; inherited composer and governance
diagnostics remain separate. The enclosing Jujutsu change owns the evidence.

E-GLOBAL-SETTINGS-NAV-001 reviews M14.2, M14.6 with statuses unchanged. Global
Settings now records section/tab URLs and validated root-startup resume;
selected-page skeletons match the refreshed layout. Exact session returns,
owners and pending commands remain retained. Focused cold web acceptance and
inherited diagnostics are separate from native-platform, release and visual
gates. The enclosing Jujutsu change owns this evidence.

<!-- generated-navigation -->

## Pages

- [history](history/index.md)
