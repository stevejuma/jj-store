---
schema: 1
id: E-CHAT-WORKSPACE-006
---

# E-CHAT-WORKSPACE-006 details 3

<!-- evidence-field:formats markdown -->
## Formats

No API, storage, publication, draft, protocol or shell state format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Inbox-only workspace panel composition. Inspected Community shell and file-tree ownership first, then released Design Core AppShell inline/replace-main behavior. Corrected owned styling rather than altering foundational layout or non-inbox rails. File tree keeps its own row insets; comparison controls, notices and skeletons retain padding.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the separate ResizeObserver owner and complete the outstanding workspace-reader, historical-source and cold native/local acceptance gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/015/018 now specify borderless workspace tabs, a full-width
  file-list divider and square adjoining inspector corners in both inline and
  replace-main layouts. Other rails and publication authority remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. These scoped UI regressions pass but runtime
  resize warnings, broader feature acceptance and visual approval remain open;
  no M13.3 qualification change.

```
<!-- /evidence-field -->

