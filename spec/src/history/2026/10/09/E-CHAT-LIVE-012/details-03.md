---
schema: 1
id: E-CHAT-LIVE-012
---

# E-CHAT-LIVE-012 details 3

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

Final governance, summary, file limits and book validation passes.
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

Final evidence ledger and M24 In progress review pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent provider/adapter, permission/cancellation, native IME,
  desktop/platform and visual gates remain open.
- Prior legacy broad-story failures and the development acceptance-script size
  gap remain recorded in E-CHAT-LIVE-011; no unrelated suite was repeated.
- Existing temporary foundational dependency validation setup is retained; no
  manifest or lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, request, receipt, provider capability, captured source, persisted preference or installation format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only Sent-context and runtime-row presentation plus direct story consumers and documentation. Rust, generated public contracts, root aggregate lanes and visual baseline updates are out of scope. Fresh Storybook/Vite test processes and a fresh live browser page were used; no lifecycle or production bundle change requires a cold built-stack restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M24 In progress and continue its separately governed independent qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 specifies resting context background, fixed start alignment
  and trailing runtime menus. JST-API-001 and JST-GOV-008 ownership boundaries
  are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. Focused layout acceptance does not close
  independent qualification or visual approval gates.

```
<!-- /evidence-field -->

