# M16 tasks and gates

| ID  | Status  | Depends | Requirements           | Gate                                                       | Next                                                             | Evidence                                                                                                                                                                                                                                    |
| --- | ------- | ------- | ---------------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M16 | Partial | M15.5   | JST-UI-018,JST-GOV-009 | Complete device Appearance and cold consumer qualification | Complete source release and registry-only consumer qualification | E-UI-052,E-UI-053,E-UI-054,E-UI-055,E-UI-056,E-UI-057,E-UI-058,E-UI-059,E-UI-061,E-UI-062,E-UI-063,E-UI-085,E-UI-086,E-UI-087,E-UI-088,E-UI-089,E-SETTINGS-POLISH-001,E-GLOBAL-SETTINGS-001,E-GLOBAL-SETTINGS-NAV-001,E-UI-INLINE-LINKS-001 |

E-SETTINGS-POLISH-001 reviews M16 without changing its status. Focused Settings
form, typography and keyboard/Axe acceptance adds no release/platform or
visual-baseline promotion; inherited diagnostics remain explicit.

E-GLOBAL-SETTINGS-001 reviews M16 with statuses unchanged. Shared global
Settings presentation and form guidance retain existing save, permission, draft
and transient navigation semantics. Focused interaction/live acceptance does not
promote visual or release gates; inherited composer and governance diagnostics
remain separate. The enclosing Jujutsu change owns the evidence.

E-GLOBAL-SETTINGS-NAV-001 reviews M16 with statuses unchanged. Global Settings
now records section/tab URLs and validated root-startup resume; selected-page
skeletons match the refreshed layout. Exact session returns, owners and pending
commands remain retained. Focused cold web acceptance and inherited diagnostics
are separate from native-platform, release and visual gates. The enclosing
Jujutsu change owns this evidence. E-UI-INLINE-LINKS-001 reviews M16 with status
unchanged. Shared rendered Markdown now distinguishes inline links from eligible
standalone content cards, adds local site icons and preserves saved Compact/Rich
preferences while new preferences default to Inline. Focused rendering,
Settings, contextual PR, keyboard and accessibility acceptance does not promote
registry/platform, release, broader-suite or visual-baseline gates. The
enclosing Jujutsu change owns the evidence.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
