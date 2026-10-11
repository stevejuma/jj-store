# Device appearance

JST-UI-018 owns the Buzz Appearance adaptation. The page exposes all 62 theme
styles plus the previous app's Original light/dark pair, System/Light/Dark mode,
paired transitions, ten accent choices, browser glass with 30-90 percent
opacity/reset, prominent navigation, 13/14/15 pixel body typography,
Compact/Comfy/Spacious conversations, local Compact/Rich link cards and
Focus/Split discussions. Preferences are device-local and app-wide; Settings
location remains transient. Defaults are System/Buzz, default font, Comfy
density, Compact links, Split replies, glass off and subtle active tabs.

Appearance uses the shared full-width Theme and Preferences cards with tinted
headers, relevant icons and edge-to-edge dividers, under Theme and Preferences
subtabs. Preferences, Link previews and Thread layout are sibling cards; each
preview stays beside its own control rather than nesting inside Preferences. Tab
selection is session-only and preferences still save immediately. Labels/help
and controls align with other Settings rows and stack on narrow screens. It
inherits the Settings font family and uses the same section-heading size,
weight, foreground and left alignment as other Settings pages. Row labels and
descriptions share their secondary text size; labels retain the same weight as
read-only Settings rows and descriptions use regular muted text. Device font
preferences still scale controls and previews without changing saved settings.

The browser tab uses the accepted blue-jay and gear application artwork as its
favicon, with an explicit SVG icon link in the HTML entry. Its self-contained
64px RGBA export retains transparent surrounds and the pale silhouette edge for
light and dark browser chrome. The favicon loads locally in development and is
included in the manifest-selected built bundle through the existing SVG asset
inventory. This static branding does not change device appearance preferences.
Temporary previews never persist. Startup, cross-tab and OS changes share the
same resolved palette, including portalled controls and emoji pickers. Native
entity links retain chips rather than becoming website cards. Conversation and
reply drafts survive Settings and layout changes within the session. Focus hides
the standard Thread reply pane and opens a nonmodal AppShell structural sheet
covering the body and both right panels, leaving7rem of main content visible on
desktop. Mobile uses the root surface with0.5rem insets. Escape, the scrim and
Close dismiss through the shell and restore opener focus. The right-panel open
states and drafts remain retained. Agent tool comments and composers stay inline
below each call inside either agent response presentation; independent drafts
survive layout changes. Other focused inline discussions retain one
application-owned sheet at a time. Native/local capability and snapshot
authority are unaffected. Browser glass reveals the application background, not
the operating system desktop. The managed macOS desktop instead installs native
Sidebar vibrancy through its narrowly scoped presentation command, then reveals
it under one translucent theme wash; main content and portalled controls stay
opaque. Native readiness, rapid toggle serialization, failure fallback and
disposal are governed by desktop.md. Palette, font and preview assets are local
under the existing strict CSP.

Buzz light and dark draw their gradient once across the full application shell.
The header and left navigation reveal that same image instead of restarting the
gradient within each region. Glass retains its selected opacity and blur above
the shared backdrop; other themes retain their existing chrome surfaces.
Collapsed left-navigation section pickers leave an 8px gap from the rail edge
and retain 8px viewport collision clearance. Both the portalled Popover and its
Command View follow the shared popover surface, keeping the picker opaque even
when browser glass is on. Every Buzz-derived palette binds the semantic popover
background to its app background (white in light Buzz), so search commands,
menus and other portalled popovers inherit the same surface. Foreground,
selected-row, hover, border and focus roles retain their existing tokens. The
Original light/dark pair retains its historical palette snapshot. Search,
keyboard selection, Escape and trigger focus restoration are unchanged.

The workspace search's section action uses a soft muted tint at rest (30% muted
over the app background) and the same stronger tint for pointer hover and
keyboard selection (55% muted). The treatment follows the active palette and
keeps its label and description visible without changing search scope behavior.

Shell header actions, sidebar toggles, navigation history and collapsed sidebar
search use the same foreground tint as sidebar items on hover and keyboard focus
(7% foreground over the shared chrome). Header menu triggers retain this tint
while their menu is open, including after the pointer enters the menu. Disabled
controls retain their disabled treatment, and sidebar toggles remain transparent
at rest in either expansion state. The treatment follows the selected theme
without changing global palette roles or body button styles.

Navigation tooltips use Design Core's standard small-text sizing and restored
inline/block padding, leaving room around the arrow. The foundation retains
public sizing tokens; the app consumes its defaults without a tooltip override.

Menu section labels and popover titles use the menu's sans font at regular
weight, with the smaller secondary text size and muted foreground. These quiet
headings remain distinct from action rows across footer controls, repository
pickers, profile menus and filters. This shared typography also applies to
portalled surfaces and follows the device font-size preference; dialog and page
headings retain their own hierarchy.

Accent swatches keep circular selected rings centered inside their circular
buttons, including after changing accents, on hover and at mobile widths in
either color mode. Keyboard focus remains visible independently of selection.

Prose composers use the resolved app background for their entire input surface
(white in GitHub Light Default), with Buzz's light rounded border, no resting
shell shadow, foreground action icons and circular upward-arrow send control.
Their frameless Mira editor uses transparent public editor/code-editor
background tokens and bridges foreground, placeholder, border, popover, accent
and selection tokens from the owned composer to the resolved app palette. The
bridge captures app roles on the composer ancestor before Mira defines its own
aliases, avoiding cyclic tokens. These tokens stay scoped to prose composers;
standalone code and file editors retain their surfaces.

Normal and confirmation modal surfaces bind the public foundation background and
foreground tokens to the resolved app background and foreground, including
portalled dialogs. WorkStore form values use foreground; field descriptions and
placeholders retain muted foreground. These roles follow every palette without
changing the user's selected accent or device preferences. Original retains the
pre-Appearance Design Core semantic palette and its native accent in light and
dark modes, including System pairing, scoped gallery previews and persisted
restoration. Choosing it retains the saved accent for other themes.

Conversation renderer selection is governed by
[Document conversations](37-document-conversations.md). It is device-wide and
independent of Focus/Split placement; missing preferences select Document.
