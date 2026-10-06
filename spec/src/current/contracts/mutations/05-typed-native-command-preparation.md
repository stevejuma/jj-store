# Typed native command preparation

The command executor prepares New, Describe and bookmark create/move/delete
through pinned native transactions. It loads one exact current operation, never
implicitly reconciles heads, and leaves the resulting operation unpublished.
Preparation requires current write permission before native reads, staging and
return. It checks the complete expected epoch/position/head snapshot before and
after execution. It is internal execution infrastructure consumed by the durable
preview/apply/status service below; a prepared report grants no authorization.

New preserves ordered explicit parents and uses native tree merging. An optional
existing logical workspace can move to the new commit with native edit behavior;
no physical working copy is created. Describe retains change identity and uses
native reparenting for descendants, keeping their trees intact and updating
bookmarks/workspaces. Existing author identity follows the pinned rewrite
policy: missing fields are filled from the committer, and discardable changes by
that author refresh their timestamp. The committer and new-commit author
identify the authenticated principal using the server's fixed identity policy,
without loading user configuration or signing programs. Root and hidden rewrite
targets are rejected.

Bookmark mutations compare the complete expected ordered native target,
including conflicted and absent terms. Create requires absence, move/delete
require an existing target, and stale expectations fail without a prepared
operation. Reports identify native predecessor rewrites and exact bookmark and
logical workspace changes. They do not constitute durable mutation receipts.

Preparation uses bounded worker admission, metadata/byte/work budgets and
cancellation. Reads are limited to the captured committed inventory plus native
objects staged by that one command. Only conditional immutable native writes are
possible; manifest writes and head publication are denied. Canceled workers
retain their permit until they exit. Later application publication must pass the
existing strict snapshot and semantic protection guard and durable retry flow.
The durable service below supplies that publication flow to each typed command
and the HTTP transport. M4 deployment qualification and browser controls remain
separate gates.

The user deferred further UI work on 27 September 2026. M5 backend command,
preview/apply/status, RPC policy and mutation API gates continue; their browser
work is retained under deferred M4.3 rather than counted as backend completion.
