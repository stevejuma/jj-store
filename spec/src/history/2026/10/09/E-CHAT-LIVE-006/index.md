---
schema: 1
id: E-CHAT-LIVE-006
---

# E-CHAT-LIVE-006

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-UI-018
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Cold native/local document-conversation publication and recovery acceptance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change slvvrxkr
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Production UI and focused contracts passed; isolated browser publication, restart and receipt-loss flows were not yet qualified. Seven earlier prototype ResizeObserver warnings had not been rechecked.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

A disposable built-UI fixture qualifies native and local publication: three selected comments at two anchors receive two exact linked answers in one assistant turn, including a shared same-anchor answer, Unicode and streaming tag boundaries. Duplicate commands retain the same feedback. Local lost command acknowledgement and temporarily unavailable receipt lookup retain the submission until an explicit receipt check, without another prompt. Ordinary human threads publish comments-only batches with no dummy message. Fresh browser and server restart preserve exact sources and destinations. Native unsupported replication is explicitly refused, then a disposable supported WorkStore home is used. The cold local ordinary-thread route exposed missing repository transport wiring. Local conversations and sidebar navigation now use the existing companion ThreadStore, retain independent pins, refuse unavailable owners, and reject changed pagination revisions. Initial absent-thread probes return 404 as expected; concurrent local publications reject stale metadata reads with 409 and the final navigation/reload assertions recover on fresh pins. No real provider or desktop deployment is implied. Cold Axe caught intermediate hover-action opacity contrast; removing the short fade fixes the owned UI. All four agent runs report zero ResizeObserver warnings; that does not retroactively erase the older fixture warnings.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- scripts/conversation-acceptance.py
- scripts/conversation-browser.mjs
- scripts/conversation-thread-browser.mjs
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M24/index.md
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/local-inbox-threads.ts
- web/src/lib/controller/production-threads-integration.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
- web/tests/vite/local-inbox-threads.test.ts
- web/tests/vite/repository-thread-navigation.test.ts
- web/tests/vite/thread-navigation-integration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-006 details 2](details-02.md)
- [E-CHAT-LIVE-006 details 3](details-03.md)
