---
schema: 1
id: E-UI-104
---

# E-UI-104

<!-- evidence-field:id markdown -->
## Id

E-UI-104
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M18.1
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-COLLAB-005
- JST-COLLAB-023
- JST-API-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change trqyosvm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Contained Agent collection notice and fresh current-format development fixture
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The old main fixture selects three unsupported schema-1 transcript pages, producing unavailable AgentStore reads despite intact stored objects. The new AgentActivityUnavailable layout assertion failed before the UI change: the alert differed from its header by 16px (one failure, eight skipped stories). The original stack-workspace-ui workspace was clean and behind main; it was rebased onto committed parent 9e98f99e without including main’s ongoing uncommitted work.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- docs/maintenance/coverage.json
- spec/src/current/contracts/ui/12-repository-presentation.md
- spec/src/current/contracts/collaboration/37-bounded-transcript-tail-format.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M9/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/progress/M18/index.md
- spec/src/current/progress/M18/history/M18.1/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-demo-collaboration.test.mjs#representative agent seed covers
  integrations, tools, transcripts, and lifecycle states
- scripts/dev-demo-collaboration.test.mjs#agent seed installs, publishes, and
  removes each owned integration
- scripts/dev-demo-collaboration.test.mjs#review seed exercises every command
  and each supported sample anchor category
- scripts/dev-demo-collaboration.test.mjs#bounded process input helper forwards
  stdin
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-104 details 2](details-02.md)
- [E-UI-104 details 3](details-03.md)
