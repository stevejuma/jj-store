# Browser review edit adaptation

The browser controller exposes the existing `edit_comment` command using a
stable comment ID, the edit-start `expected_revision_id` and a V1
`safe_markdown` replacement body with `review-markdown-v1` sanitization. It does
not add an HTTP schema variant or accept client author/time fields. Reply menus
pass the stable parent through `append_reply`. Unknown command outcomes retain
the request ID and prevent a new command from replacing the pending identity;
receipt processing checks the owning connection before refreshing or navigating.

E-AGENT-REVIEW-DRAFT-001 confirms JST-API-001 unchanged: the stored-session Add
review disclosure and comment-card editor retain existing immutable session
anchors, author/permission checks, draft ownership and pending receipt
identities. Collapsing or reopening the editor issues no review command. No
schema, transport generation or publication behavior changes.

Browser branch/working-copy selection within one pinned repository operation
reuses the loaded reference collection and paging cursor. Only scoped graph and
detail reads run for that selection; their loading state cannot replace the
Branches column. Cold restoration, operation changes and automatic updates
continue to load operation-pinned references through the existing read contract.
Repository cards use full-width content and right-aligned name-row status chips
without changing metadata or permission contracts.
