# Complete supported configuration settings

JST-UI-016 includes typed editors in Local service (resource limits), Local
service / Protection (installation-wide protected bookmarks/commits), Sync
(repository upstream, credential environment reference, CA path and every
bounded worker limit), and Search (polling, timeout and snapshot interval
alongside existing scopes/budget). Local service / Launch and Storage
configuration show the remaining deployment, authority and backup configuration
in divided read-only rows with explicit launch, migration or deferred-feature
reasons. No supported operational configuration is omitted.
Service/protection/sync saves require the next server startup; desktop uses
Quit/reopen. Search retains explicit Apply. Each section retains its draft and
base file revision across refresh, tab changes and repository navigation; stale
observations cannot silently rebase a draft and unknown saves cannot be blindly
replayed. Operational feedback uses JST-UI-019 notifications under a stable
installation scope; repository navigation cannot replay an unchanged dismissed
condition. Editors are installation-owned and unavailable on
authenticated/non-Admin connections. The presentation convention below governs
new and refreshed Settings and forms across the app. This slice adopts it in
repository Settings; existing global Settings keeps its presentation until its
separate refresh.

## Settings and form presentation

Use shared presentation components composed from released Design Core primitives
and application theme tokens. Keep full-width cards in a single column with
compact spacing, subtle borders, moderately rounded corners, tinted headers and
restrained, relevant icons. Group read-only information, editable configuration
and maintenance into separate logical sections; avoid nested card borders.

Read-only properties use consistently aligned key-value rows with full-width
separators. Copyable identifiers stay on one line with an ellipsis while hover
and Copy retain the exact full value. Descriptive paths and capability lists
wrap. Icons, labels, values and compact copy/Open controls align vertically at
the centre. Editable rows put labels and help on the left and controls on the
right; numeric inputs use a content-appropriate width rather than stretching
across the column.

Commands belong in a dedicated footer at the bottom of their relevant section,
never in ordinary key-value rows or centred beneath content. Footer text is
optional and sits on the left; related buttons align right with the primary
action last and secondary actions immediately before it. Maintenance commands
remain separate from Save/Cancel. A form spanning several sections uses one
shared save footer. On narrow screens, label/control columns stack and footer
buttons wrap or stack without overflow. Existing inline copy/Open controls and
input Browse addons remain attached to the value they act on.

Search Configuration contains Index status (saved, currently active, operation),
Search options (enablement and budget with Save/Cancel), and Index maintenance
(Apply, Retry, Rebuild). File scopes and Advanced reuse these groupings for
their existing fields. Sync Configuration separates current status, upstream
fields and bounded limits and retains one form-level Save/Cancel footer. No
mockup-only sync settings or commands are introduced. Storage and Advanced
follow the same status/maintenance separation and footer convention.

Primary navigation retains pills; real secondary tabs use compact icons,
left-aligned labels and a thick active underline in the theme accent colour.
Active labels are bold. The secondary strip uses the body horizontal padding; no
title-only strip is shown. Primary and secondary navigation stay outside the
bounded Design Core Scroll Area used by repository settings. Card headers meet
the first row without extra spacing. Keyboard selection and overflow remain
unchanged.

Presentation changes MUST retain validation, capability and permission filters,
disabled states, loading feedback, confirmation flows and all original request
identities. Section grouping cannot create additional saves, alter action
availability or replace controller owners. Drafts, history, polling, exact Back
restoration and authorization contracts remain unchanged.

Shared read-only form rows place navigation to the left of Copy. Copy stays at
the far right, and rows without navigation reserve no empty action slot. Exact
copy values and independent navigation hit targets remain unchanged.
