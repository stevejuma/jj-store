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

The application adopts the public AppShell topbar with one mounted navigation
and Application-menu subtree. Browser compact mode relocates that subtree into
an explicit target before Main's scrollable content; native chrome retains top
placement and its fixed control geometry. Existing history callbacks, shell
controllers, panel identities, routes, drafts and pending request owners remain
application-owned and unchanged.

Body-adjacent right panels opt into the attached Sidebar variant. Visible
adjacency, joined edges and separators belong to Design Core, while the outer
repository details rail remains independently controlled and uses shared Sidebar
sections. Discussion and conflict pages use SurfaceLayer close/footer parts and
responsive inset tokens; bounded nested editors retain scroll ownership. One
active layer, coverage, suspension and return focus are retained.

E-SHELL-LAYOUT-001 qualifies local adoption against the explicitly packed,
unpublished Design Core shell extension archive. Manifests and the tracked
lockfile retain registry resolution; archive validation is not registry-release
qualification. The archive also carries Design Core's native accessible names
for host-supplied file-row snippets: selection stories address the rendered
filename and metadata while retaining path-identity assertions. A released
version containing these APIs remains an integration gate before registry-only
deployment.

The browser compact toolbar target is a labelled Application navigation
landmark. Its relocated Application controls toolbar remains inside that
landmark on repository pages and Settings; desktop retains its existing
navigation subtree. This preserves landmark coverage without remounting
controls.
