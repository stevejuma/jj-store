# Application startup

The production controller completes bootstrap before rendering connection
choices. One ready LocalTrusted catalog entry opens without a token, several
entries render an explicit chooser, and an empty catalog explains repository
creation or verified registration. Authenticated mode asks for one token, then
opens the requested ID-based deep link, auto-opens a sole authorized repository,
or renders every authorized alias. Repository switching reuses that in-memory
credential while closing the previous query/SSE session. Disconnect, token
replacement, revocation and teardown cancel protected requests, event streams
and caches. Local loopback identity is labelled only as `Local`; the UI does not
infer a username from HTTP. Usable backup status is future-release work.

Main page views load on demand while controller and shell owners retain routes,
authentication, selections and drafts. Pending view imports show the existing
accessible, route-specific skeleton. The module-loading fallback and initial
data-loading view preserve the same visible page geometry: Threads and agent
session inboxes retain their list and conversation regions (including stored
widths and mobile behavior), repository files retain their saved tree
visibility, history retains its columns, and Projects use the selected tab's
list, grid, activity or detail composition. Repository Settings retains its
primary navigation, real-subtab strip and selected category’s single-column
cards, divided rows and action footers. Its module and discovery fallbacks use
the same shared presentation as initial section reads, including Local JJ
profile differences. A generic details skeleton never precedes these page
skeletons. Known selection-empty states remain visible while their list loads.
Failed imports use application notifications with Retry; the retry entry URL
comes from the application compiler, including hashed production entries and
development HMR transformations. It is never inferred from an output filename.
Retries never clear drafts, replay mutations or reload the application.
Navigation prevents obsolete pending views from rendering. Loaded optional
overlays stay mounted inside their context owner when closed to retain existing
cleanup and draft behavior. Built-bundle acceptance checks unopened pages remain
outside the initial static import closure and dynamic chunks remain in the
existing browser asset manifest.

Browser interaction fixtures exercise the same bootstrap and repository
discovery requests as the production controller. Authenticated fixtures supply
one memory-only token and an authorized repository page before opening a
repository; LocalTrusted fixtures discover without an Authorization header.

E-UI-VIEW-READINESS-001 confirms the unchanged JST-UI-005/010 loading and
accessibility contract. Interaction fixtures await the mounted view controls;
they retain each owned loading phase, draft and layout assertion. Classic-only
fixtures declare that view locally without changing the Document default.
