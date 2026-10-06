---
schema: 1
id: E-UI-062
---

# E-UI-062 details 2

<!-- evidence-field:formats markdown -->
## Formats

No native/API/storage/preference formats changed. WorkStore add_comment/reply_to, edit/remove, version guards, bounded pagination, immutable snapshots, retained request identities and memory-only drafts unchanged. Projects registry now forwards existing route.commentId alongside project/task selection; no new URL field or authority. Focus/Split conversation preference no longer creates a task discussion side panel.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue UI iteration in the running Appearance Vite workspace; main landing and visual baseline approval remain separate user actions.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Committed web/tests/real/task-activity-layout.spec.ts; /tmp/jj-task-activity-*.log/png. Isolated browser contexts closed. Existing development fixture and servers retained.
<!-- /evidence-field -->

