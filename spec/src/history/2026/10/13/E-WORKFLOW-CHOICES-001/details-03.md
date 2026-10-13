---
schema: 1
id: E-WORKFLOW-CHOICES-001
---

# E-WORKFLOW-CHOICES-001 details 3

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
pnpm catalog:check
```

**Notes**

Complete repaired lane: 0 errors and 0 warnings; catalog boundaries pass 423 modules.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

153 tasks and 779 evidence records; mapped gate passes 24 protected files and 4 canonical chapters; 7 specification validators pass 3725 chapters and 196 requirements.
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
pnpm exec prettier --check <changed non-history files>; pnpm docs:review
```

**Notes**

Changed files follow formatting; generated immutable history remains excluded by repository policy. Application review refreshed after native actor scope correction; other topic reviews valid.
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
Final original-tab observation and temporary-tab cleanup
```

**Notes**

Only agent-created tab 2 was explicitly reloaded and closed. Vite hot reload also closed the original user dialog during source updates; its unseen unsaved fields cannot be verified. This correction was reported to the user. No workflow command, reset, installed-service change or credential mutation occurred. Final screenshot shows server avatar and Nyra without independent local actor suggestions.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval and existing M28 live provider/Forgejo gates remain
  separate. Agent message event matching is outside this UI slice; agent setup
  IDs cannot be used as authenticated-principal author filters.
- Dev hot reload closed the original user dialog. Unseen unsaved fields are
  unverified; this slice does not qualify browser-reload draft recovery.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11. Installed released Design Core 0.3.1, Svelte 5.57.1 and existing CodeMirror are reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Workflow definition format, native roots, receipts, API schema and authentication unchanged. UI-only source-pinned reads and generic picker opt-in opening/unavailable-entry props.
<!-- /evidence-field -->

