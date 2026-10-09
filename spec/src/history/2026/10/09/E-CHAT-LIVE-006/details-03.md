---
schema: 1
id: E-CHAT-LIVE-006
---

# E-CHAT-LIVE-006 details 3

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
pnpm check:architecture (final)
```

**Notes**

Only pre-existing scripts/dev-ui.acceptance.mjs remains: 311 counted lines exceeds 300. Every changed module is within the cap.
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
pnpm check:ui-ownership
```

**Notes**

Ownership passes.
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
python3 -m py_compile scripts/conversation-acceptance.py
```

**Notes**

Python harness syntax valid.
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
node --check scripts/conversation-browser.mjs
```

**Notes**

Browser harness syntax valid.
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
node --check scripts/conversation-thread-browser.mjs
```

**Notes**

Ordinary-thread harness syntax valid.
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
pnpm exec prettier --check <changed Markdown, JSON, CSS and JavaScript>
```

**Notes**

Changed files formatted before commit.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Final evidence and owning chapters validated before commit.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

E-CHAT-LIVE-005 passes at b07aa3bf after changing its aggregate result to pending to retain its reported gaps. E-CHAT-LIVE-006 also passes the committed ledger check: 146 tasks and 689 evidence records; M24 remains In progress.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec storybook dev --host 127.0.0.1 --port 6048 --no-open --disable-telemetry --ci
```

**Notes**

Restored this workspace Storybook on the original port. The development server later stalled; a restart alone did not fix it. Retained its ignored sb-vite dependency cache under .local, regenerated that cache and restarted only the owned process. Shutdown overlap briefly selected 6049; the final process is confirmed on 6048. A fresh Playwright browser renders the original Delta main-thread story with no page exceptions, and the server returns to idle. Main workspace unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent real-adapter, permission, native IME, platform/desktop and
  visual gates remain; deterministic ACP is not model-compliance evidence.
- The pre-existing scripts/dev-ui.acceptance.mjs 311-line architecture failure
  remains; no cap or baseline relaxed. Bundle/Tauri warnings remain.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Playwright 1.63.0; Axe 4.13; own existing debug CLI/server; isolated real RustFS runtime from main read-only; no Rust rebuild in this UI/acceptance slice.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new durable format or publication authority. Existing ThreadStore selectors, exact immutable protocol links, stable receipts and format-2 native/local authority retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Isolated cold built Chromium/native/local browser acceptance and owned action-visibility fix and local conversation route wiring. No remote deployment, main integration, real-provider dispatch, Rust workspace or root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the local implementation; M24 independent qualification and visual approval remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/015/018 clarify qualified local thread routes and sidebar
  transport; JST-COLLAB-034..036 publication authority retained. The opacity
  removal enforces existing accessible actions; the harness verifies authorized
  publication, immutable links and stable recovery identities.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. Narrow controlled native/local browser gates
  now have fresh evidence; independent provider/desktop/IME and visual gates
  stay open.

```
<!-- /evidence-field -->

