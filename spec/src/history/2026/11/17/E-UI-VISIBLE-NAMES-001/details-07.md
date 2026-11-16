---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 7

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentChatBody.stories.svelte --testNamePattern '^MessageReactionsAndXMLContext$'
```

**Notes**

First selector adjustment incorrectly treated the former appended count as an actor label. The exact test still fails; retained /tmp/jj-store-core0135-chat-reaction-exact-after.log. Inspecting the production fixture and installed Core label construction establishes the exact name1 reaction with👍:👍1. No production change was made.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentChatBody.stories.svelte --testNamePattern '^MessageReactionsAndXMLContext$'
```

**Notes**

Correct source-derived exact name passes1 case/22 unselected in a fresh worker; count, pressed state, hide/show persistence, removal and XML-context assertions remain. /tmp/jj-store-core0135-chat-reaction-exact-after2.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

All23 complete story-family cases pass in a separate fresh worker8bde0627. Full cold rerun follows current type/unit/static/browser checks; no full-suite pass is inferred. /tmp/jj-store-core0135-chat-reaction-family-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Current source:0 Svelte errors/warnings,490-module ownership/boundaries and4700-chapter parity pass. ab8d7f1d. Logs /tmp/jj-store-core0135-current-* retained.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Current source:0 Svelte errors/warnings.0931d18d. Logs /tmp/jj-store-core0135-current-* retained.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

Complete193 unit files:1221pass/one existing skip,0fail.4461c173. Logs /tmp/jj-store-core0135-current-* retained.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

All180 current web units pass,0fail.fabfe6e3. Logs /tmp/jj-store-core0135-current-* retained.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

Cold build succeeds, exact cold_catalog_contract fails at legacy exact option name main; visible status/author now participates in its accessible name. Keep just-now author assertion and all network/geometry/copy checks. /tmp/jj-store-core0135-current-catalog-test-browser.log, workerd14a43af.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 26
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser
```

**Notes**

All115 application browser cases execute:88pass/26fail/one existing skip. JUnit splits failures into21 assertion failures and5 timeout errors. Most stale branch main exact names, plus source-derived Working copy laptop, Configuration:Codex and contextual repository filter text. Repair queries/expected labels only; routes, mutations, freshness, persistence, no-replay and deadlines stay unchanged. c804cdd6 output collection initially Premature close; same terminal worker collection retried separately. /tmp/jj-store-core0135-current-web-test-browser.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Complete fresh1049-case source acceptance:1047pass/0fail/two existing skips. Four disjoint shards273/0/2,284/0/0,238/0/0,252/0/0. Original source failures and corrected chat selector all pass with unchanged assertions/deadlines. /tmp/jj-store-core0135-full-storybook-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

After branch query repair, cold contract reaches stale full-ID accessible-name expectation in actual pointer/touch helper. /tmp/jj-store-core0135-catalog-browser-after.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

Stable Copy GIT/JJ prefixes initially require a trailing space, but native hover collapses the sibling to Copy JJ with no suffix. Full identity descriptions already pass; repair matches space or end, retaining every native geometry, clipboard and timing assertion. /tmp/jj-store-core0135-catalog-browser-after2.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

Complete cold static catalog contract passes after source-derived queries; full native identities now separately asserted in descriptions and copied bytes. Desktop/touch geometry, clipping, responsive behavior and network isolation retained. Workeree8e64aa; /tmp/jj-store-core0135-catalog-browser-after3.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser -- --grep (four original branch/capture/filter/working-copy targets)
```

**Notes**

Three original cases pass; grouped capture reaches one further exact text mismatch: expected2 statuses versus actual contextual Filter by source status:2 statuses. /tmp/jj-store-core0135-browser-exact-after.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser -- --grep 'cold grouped Codex configuration and harness-filtered source table retain identities'
```

**Notes**

Exact remaining grouped capture case passes after preserving full contextual status/count label; all filters, state and accessibility assertions retained. bb19f6f2; /tmp/jj-store-core0135-browser-exact-after2.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser
```

**Notes**

Complete115-case cold built-application suite:114pass/0fail/one existing skip. Actual worker0fadc641 exits0; attached local log stopped after71 cases during interruption, so authoritative terminal container log/JUnit and complete reports were collected from that same worker rather than rerunning it. /tmp/jj-store-core0135-browser-final-terminal.log and browser-final-collection.log. Earlier115-case88pass/26fail/1skip remains retained.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Current supported tooling candidate: Storybook10.6.7, Vitest4.1.12, Vite8.3.8, Playwright1.63.0, axe-core4.14.0, Design Core0.13.4 and validator0.2.1. No package/runtime/storage/API version changes in this naming repair.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Accessible names now derive from visible content; full native IDs remain exact aria-describedby descriptions and clipboard payloads. Visual layout, native IDs, routes and storage formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned cards, profile/search controls and native identifiers feed production application and catalog surfaces. Exact workflow names remain alongside visible summaries. Full IDs stay described and copied. Complete UI/catalog and full interaction/accessibility/cold lanes are required by the shared tool-cohort change; no Rust product or native-platform qualification is inferred. Stack/forge/search exact regression is repaired; broader affected family, type and ownership gates remain required. The exact CSF Story names are retained in validation commands and notes; they are not fn/def/test declarations accepted by the named-test metadata field. No product test is deleted or weakened. Account/status and Apply/Unapply names retain visible content. Branch option assertions use complete accessible names in order. Inbox descriptions preserve full identity/status without changing selection, storage or lifecycle. Mobile search exposed stale loading-skeleton elements; loaded-title readiness repairs its exact regression without product or dependency changes. Wider acceptance stays separate. Member/avatar/profile, capture disclosure and repository directory names now compose their full visible information with action context. No image text, live presence, full identity, clipboard callback, permission or persistence behavior is removed. Complete cold UI rerun is required now because the accumulated source/fixture fixes affect independent application, graph, settings, workflow and conversation surfaces under the shared tool-cohort qualification. It will diagnose remaining owned failures separately from known released-Core reaction/contrast holds. Rust lanes remain outside this UI-only scope. Local source UI acceptance is complete; this record does not qualify hosted version-runtime guard adoption or the entire M30/public release.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run final source specification/evidence gates, commit and obtain hosted source acceptance; retain whole-version guard adoption and original native/public release/visual gates separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001 now explicitly retains visible card/identifier labels in
  accessible names and full IDs in descriptions and clipboard payloads; owning
  canonical topic and verification are updated.

```
<!-- /evidence-field -->

