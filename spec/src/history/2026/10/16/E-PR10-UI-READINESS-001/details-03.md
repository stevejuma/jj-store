---
schema: 1
id: E-PR10-UI-READINESS-001
---

# E-PR10-UI-READINESS-001 details 3

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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

chat-nav-web-build-95: exit 1, 8.42 seconds.
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
python3 .local/main-qualification-2026-10-16/provider-shard-inventory-95.py
```

**Notes**

provider-shard-inventory-95: exit 0, 0.06 seconds.
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
pnpm web:build
```

**Notes**

chat-nav-web-build-95: exit 0, 53.07 seconds.
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
python3 .local/main-qualification-2026-10-16/provider-shard-inventory-95.py
```

**Notes**

provider-shard-inventory-95: exit 0, 0.05 seconds.
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
python3 .local/main-qualification-2026-10-16/provider-shard-inventory-95.py
```

**Notes**

provider-shard-inventory-95: exit 0, 0.05 seconds.
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
python3 .local/main-qualification-2026-10-16/provider-shard-inventory-95.py
```

**Notes**

provider-shard-inventory-95: exit 0, 0.06 seconds.
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

Local macOS ARM64; Node 24.10.0; pnpm 10.32.1; Vitest browser 4.1.11; hosted Linux x64 Node 22.23.3; registry dependency lockfile unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, API, native identity, storage, credential, transport, preference or command format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed provider interaction-readiness and shared chat-navigation repair slice for PR #10. User explicitly requested fixing the current PR checks. Preserve previous immutable local qualification and repair evidence, user services, unrelated tests and visual baselines. The exact three stories passed locally before editing; that isolated result does not erase the real hosted failure. After the provider repair, the complete same shard passed the original failure but exposed one original chat-layout case: initial Jump to end left its draft 2.86 pixels outside the unchanged one-pixel bound. The two original chat stories passed separately, which does not erase that full-shard failure. A new focus/caret-reveal browser regression failed before and passed after the owned ChatNavigation now focuses the draft before requesting its explicit viewport scroll. Original geometry, keyboard, draft and reduced-motion contracts remain intact. All original 315 hosted cases plus that one new regression pass exactly once in the complete 316-case shard. Web/catalog units, types, ownership, architecture and a fresh web build also pass. The first web-build attempt compiled successfully but correctly stopped at the application documentation review gate. The mapped guides were reviewed, Browsing documents whole-draft navigation, and the review fingerprint was refreshed explicitly before the passing replacement build. Unrelated native source, public contracts and dependency versions remain unchanged. This completed focused repair slice excludes the separately queued full remaining UI shards, served/cold catalog acceptance and fresh hosted checks. Heavy, live-provider, packaged-platform and visual-baseline gates remain in their owning milestones.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit and push the verified provider-menu and chat-navigation repairs, then observe all required hosted PR jobs and fix any new actual failures. Keep the CI-aligned status tracker current; no merge or release is authorized.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-UI-001 retains blocking interaction/accessibility checks, original
  accessible-name/option/draft/geometry assertions, original waitFor and overall
  test deadlines, keyboard Escape interactions, and runner retry settings. The
  private Add-agent scenario now awaits disappearance of all select listboxes
  after each of its two Escape dismissals before moving to a subsequent control.
  Its existing selection helper already awaited closing transitions after
  choosing an option; dismissals now prepare the same settled state. The shared
  owned ChatNavigation changes only focus/scroll ordering and the owning
  chat-body chapter clarifies that existing end-navigation target. JST-API-001
  remains unchanged: this local focus/scroll reordering adds no endpoint,
  request, response, read-session, authorization, command or credential change.
  No shared fixture, dependency CSS, accessibility rule, diagnostic filter,
  mutation authority or timeout was changed. A one-shot fixture editor-focus
  listener models caret reveal competing with the explicit navigation; its
  full-draft visibility assertion fails on the prior implementation and passes
  after the change. This does not alter the original one-pixel geometry bounds
  or substitute a scroll-only threshold. Unsuppressed ResizeObserver
  notifications remain recorded. JST-ARCH-005 and JST-GOV-003/004/011 retain
  standard required-check and fresh-evidence gates; a preceding all-green hosted
  head and the prior complete local corpus do not substitute for the failing
  current head."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Explicit M24 and M30 review retains In progress. Complete local CI-shard
  and direct web qualification establish this repair slice; remaining full-UI
  and hosted acceptance stay explicit. Fresh hosted required checks, heavy
  release qualification, installed adapters, live permissions, other platforms,
  native IME, packaged distribution and visual approval remain independent.

```
<!-- /evidence-field -->

