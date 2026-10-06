# Paged service reads

A project collection page selects at most 256 projects. A project query returns
that project plus a page of its tasks; a task query returns its project,
selected task and a page of comments. Pages enforce a four MiB body budget as
well as the record count. Opaque random continuations retain the selected root
and bind the exact repository, epoch, filters and page size. The process
registry bounds snapshots, memory, concurrency and five-minute lifetime. Every
resumed page reauthorizes; restart, expiry and resource rejection are explicit.
GET never stages empty roots. Linked-code access uses a fresh target repository
session.

Browser tasks and discussion order loaded parent/reply branches and cap visual
indentation. Missing parents from another page retain their exact identity and a
visible fragment rather than inventing another relationship. The task's main
page shows threaded comments under the initially expanded Activity disclosure
after Subtasks, with the composer below Activity. Collapsing the disclosure
retains composer drafts and reply/edit context; a native comment link reopens
Activity and focuses that comment. Tasks do not use a separate discussion panel
or an Open task discussion action. Projects restoration forwards the exact
project, task and comment selection into the bounded WorkStore reader; an
unavailable comment stays explicit and a route without a comment clears focus.
Visible task comments use the shared reaction picker and count bar, scoped to
the metadata home and exact comment version. Read-only users can inspect counts;
writes require home permission. Reaction feedback retains unknown request
identity and status recovery after comment edits or removal, and reloads
automatically with other task data. Task Open target repository, Edit task and
Add subtask actions use the public secondary Button surface with leading
repository, edit and subtask icons and its semantic hover state so the actions
remain visible at rest across themes. The repository action keeps a small inline
gap before the repository name. Cold RustFS qualification covers separate
source/destination stores, historical WorkStore closures after checkpointing,
retained origin ledger, local readiness and durable service receipts after both
stores restart.
