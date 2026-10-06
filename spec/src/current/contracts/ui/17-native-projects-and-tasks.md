# Native projects and tasks

Projects is a repository-wide destination backed by WorkStore, independent of
the selected code operation. Project/task IDs use validated `project` and `task`
hash fields. A task route requires its project. Projects exposes linked
repositories, retired membership, versioned tasks with state and parent, and
threaded comments including edits and tombstones. Home permission governs the
forms. A linked repository opens through ordinary independently authorized
repository connection; absent target access remains visible as a metadata link.
Community task presentation informs the composition; Design Core provides the
form controls. Production imports no catalog data.

Each paged list retains its own immutable WorkStore snapshot and validates
continuation identity. Confirmed saves immediately reload the selected
project/task. Live events and periodic checks reload active lists automatically,
preserving drafts and pending request bytes; unknown outcomes never authorize a
second command or imply publication. Status `not_known` does not prove
rejection; retry uses the same request ID and intent. Session replacement
discards late reads/receipts. Narrow screens stack the project list, task list
and discussion; controls remain keyboard reachable. Story interactions and
accessibility are separate from visual baseline approval.

Switching between Projects and JJ Log clears the destination-specific task or
evolution selection while retaining the full code operation pin. Browser Back,
Forward and reload restore each destination's own identities independently.

Navigation stories assert all seven destinations, including Projects in the
right rail. Conflict file-browser assertions await virtual rows after restoring
the sidebar; header actions, native file identities and retained drafts remain
required.

The shared cold native browser scenarios use the current repository tabs and the
Markdown editor textbox in mutation forms. External-publication assertions
capture the complete route after the initial seeded review association loads;
the stable entity selections must survive automatic refresh; the current-head
operation pin advances without a manual action.

Cold atomic-conflict acceptance targets the current named merge-sheet region and
full native conflict path, selecting the left term through the merge editor's
accept action. It still requires identical preview/apply requests, one complete
path selection with native term zero and the full native fingerprint.
