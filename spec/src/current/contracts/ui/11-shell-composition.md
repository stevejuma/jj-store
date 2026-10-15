# Shell composition

Responsive tab fitting coalesces resize observations into an animation frame and
cancels pending measurements on teardown. Width fitting must not write layout
during ResizeObserver delivery or produce observer-loop errors.

The released Design Core 0.3.2 shell cleanup removes mobile teleports with their
owner. Catalog units compile its published Svelte exports explicitly, including
on a cold dependency cache. This retains JST-UI-010 shell, focus and draft
contracts; no source checkout substitution is used.

The shell defaults to Branches, a collapsed left sidebar and closed 320px right
sidebar. Per-instance history, selection and controllers never persist across
stories. Repository compact popovers retain focus restoration; mobile navigation
uses the same public shell stages. Community-only room actions are omitted.

E-CI-BROWSER-001 confirms the existing shared Application menu and explicit
`Open file ...` accessible names in cold catalog, served-browser and Storybook
acceptance (JST-UI-010). Standalone repository-action fixtures retain their own
menu label. Current selectors do not change menu ownership, mutation controls,
focus restoration, drafts, source safety or layout contracts.

Use one shared Application menu for native repository actions and supported
saved-settings reload. Desktop places it at the top navigation's right edge;
limited browser viewports hide that navigation and place the menu at the right
of the body toolbar, alongside the existing navigation controls. Workflows uses
the same mobile body-header composition as Projects, including loading and
History/detail views. Settings and other hubs retain access to the same menu.
Viewport changes preserve open dialogs, drafts, original requests and focus
restoration; moving the trigger must not remount its command owners.
