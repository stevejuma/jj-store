---
schema: 1
id: E-EVENT-CONNECTIONS-001
---

# E-EVENT-CONNECTIONS-001

<!-- evidence-field:id markdown -->
## Id

E-EVENT-CONNECTIONS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-011
- JST-UI-017
- JST-API-001
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-009
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- scripts/application-events-browser.mjs
- spec/src/current/contracts/api/35-scoped-application-freshness.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/inbox-freshness-review.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/10/E-EVENT-CONNECTIONS-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-EVENT-CONNECTIONS-001/details-02.md
- spec/src/history/2026/10/10/E-EVENT-CONNECTIONS-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/api/application-event-hub.ts
- web/src/lib/api/application-events.ts
- web/src/lib/api/application-events.worker.ts
- web/src/lib/api/shared-application-events.ts
- web/src/lib/controller/application-freshness.ts
- web/tests/fixtures/application-events-browser.html
- web/tests/fixtures/application-events-browser.ts
- web/tests/vite/application-event-hub.test.ts
- web/tests/vite/shared-application-events.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/application-event-hub.test.ts#six app instances occupy one
  event connection and late joiners get current witnesses
- web/tests/vite/application-event-hub.test.ts#credential scopes never share
  authorization or witnesses and release independently
- web/tests/vite/application-event-hub.test.ts#last release and immediate
  reacquisition cannot retain or cancel the new connection
- web/tests/vite/application-event-hub.test.ts#late snapshots invalidate the
  full domain instead of only its last changed identity
- web/tests/vite/shared-application-events.test.ts#browser ownership joins the
  stable worker and abort releases only its port
- web/tests/vite/shared-application-events.test.ts#page exit releases the
  subscription before its document disappears
- web/tests/vite/shared-application-events.test.ts#denial propagates without
  opening a second direct transport
- web/tests/vite/shared-application-events.test.ts#failed worker startup falls
  back, while a silent startup has a bounded deadline
- web/tests/vite/application-event-hub.test.ts#returning owners retain a missed
  repository reset even after later domain changes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-EVENT-CONNECTIONS-001 details 2](details-02.md)
- [E-EVENT-CONNECTIONS-001 details 3](details-03.md)
