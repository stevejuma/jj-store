---
schema: 1
id: E-CHAT-LIVE-001
---

# E-CHAT-LIVE-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-001
- JST-UI-015
- JST-UI-018
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main integration and live conversation contracts
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oklqtunw
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The chat-body stack diverged from main at 6d3ee2e9. Its prototype did not define production durable comment submission or agent reply-tag validation. Live chats concatenate assistant chunks and expose whole-message discussion targets.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Rebased the 15-change stack onto e26a177a, retaining main provider configuration. Resolved the single governance conflict at the oldest prototype change with both independent paragraphs; all descendant conflicts cleared. The empty prior workspace revision was automatically abandoned during conflict resolution; recreated the working change atop the retained final prototype change. Canonical contracts now distinguish Delta evidence, prototype behavior and required production acceptance. One Document/Classic preference and full comment parity cover agent chats and threads; specialized review/task panels remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/contracts/collaboration/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/index.md
- spec/src/current/contracts/ui/08-device-appearance.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
- spec/src/current/progress/Bootstrap/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-010/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-010/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-014/index.md
- spec/src/history/2026/10/08/E-CHAT-LIVE-001/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-LIVE-001/index.md
- spec/src/history/2026/10/08/E-FLEET-006/index.md
- spec/src/history/2026/10/08/E-FLEET-007/index.md
- spec/src/history/2026/10/08/E-GOV-004/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
jj rebase -s ulzpulnnyvnr -o 'default@-'
```

**Notes**

Rebased 15 commits. One governance paragraph conflict resolved in ulzpulnn; retained provider and prototype text. Recreated working change after JJ abandoned its prior empty revision.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed authored spec files>
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Durable comment contracts, live adapter/renderer parity, protocol publication
  and isolated native/local acceptance require implementation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Node v26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Contract-only extension; existing persisted objects, generated APIs and runtime remain unchanged in this slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Canonical contract and main-workspace integration; no repository aggregates, Rust or visual baseline changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement exact source selectors and receipt-backed submissions with focused model/publication regression coverage.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: The selected UI/collaboration/API requirements now include the accepted
  live integration scope and protocol. This is a required contract, not an
  implementation claim.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Real adapter, cold native/local and visual
  qualification remain open.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


