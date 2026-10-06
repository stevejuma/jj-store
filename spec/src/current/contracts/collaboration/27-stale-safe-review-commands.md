# Stale-safe review commands

`ReviewCommandV1` admits only create thread, append reply, edit comment, resolve
comment conflict, tombstone comment, moderated hide, resolve thread and reopen
thread. A create stages its validated anchor, safe body, initial revision,
creation event, bounded event pages and first snapshot atomically. No staged
subset is visible until the shared manifest CAS selects the replacement review
root. The generated HTTP contract enumerates all eight commands and requires the
observed alternative revision IDs for conflict resolution.

The authenticated principal is the author and actor for new comments and the
actor for later revisions or workflow actions. The command has no client author
or accepted-time field. Stable thread, comment and action IDs are derived from
the repository plus authenticated request/publication identity before retry, and
server acceptance time is frozen for the command attempt. Receipt results return
those stable IDs and the exact committed event, comment revision and thread
snapshot references.

Concurrency is scoped to the entity being changed. Replies require the observed
workflow version and an open thread, so two replies can reapply to a newer
content version and both survive while workflow state is unchanged. Edits,
tombstones and hides require the exact selected comment revision. Resolve and
reopen require the complete observed thread snapshot ID, including all visible
comment activity. Resolution therefore rejects an unseen reply; a reply that
loses to resolution returns `thread_resolved` until an exact-snapshot reopen
commits.

Each accepted action appends one immutable event. The coordinator reads the
bounded selected event stream and rebuilds pages with at most 256 entries rather
than appending one reverse-linked page per action. Comment revisions retain the
prior revision edge and original author. Tombstone and moderated-hide records
remain auditable selected history rather than erasing prior bytes.
