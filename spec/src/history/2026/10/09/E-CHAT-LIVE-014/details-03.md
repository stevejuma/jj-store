---
schema: 1
id: E-CHAT-LIVE-014
---

# E-CHAT-LIVE-014 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,book
```

**Notes**

Four document validators pass across 3338 chapters (.local/inbox-header-spec-check.log).
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
pnpm check:progress
```

**Notes**

Ledger passes for 146 tasks and 697 evidence records (.local/inbox-header-progress-check.log).
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent provider/adapter, permissions/cancellation, native IME,
  desktop/platform and visual approval gates remain open.
- Prior unrelated broad legacy-story and development acceptance-script size gaps
  remain recorded in E-CHAT-LIVE-011; these unrelated lanes were not repeated.
- The existing temporary foundational dependency validation setup is retained;
  no manifest or lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, publication, request/receipt, source mapping, execution, binding, preference or installation format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only repository sidebar composition and direct story/fixture consumers. Rust, generated public contracts, root aggregate lanes and visual baselines are out of scope. Fresh Vitest/Vite browser processes and a fresh live page qualify the focused flow; no built-bundle or backend lifecycle change requires a cold stack restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M24 In progress and continue its independent qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 specifies independent header links, shared background, inset
  controls, final chevron and grouped initial skeleton. JST-API-001 and
  JST-GOV-002 publication/authorization and ownership boundaries are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress; focused sidebar verification does not close
  independent provider, platform, native IME or visual approval gates.

```
<!-- /evidence-field -->

