# Conflict resolution reports

A new resolution preview and selected receipt include a conflict_resolution
report with exact before/after target IDs, sorted selected_paths and a remaining
map keyed by every resulting rewritten or created commit ID. Each map value is
the complete sorted native conflict-path list for that result, including an
empty list when none remain. This covers the target and all affected
descendants; it does not claim to scan untouched history. Tree/file conflicts
remain one native path. Selecting a directory may reveal nested conflicts, which
are reported instead of treating the selection itself as proof of resolution.

Enumeration shares command authorization, metadata, work, output and deadline
budgets. Oversized or unreadable results fail preparation without publication;
there is no truncated success report. Preview IDs remain provisional and the
selected receipt retains the actual report for status and retries. The optional
field is absent from non-resolution reports and defaults to absent when decoding
older receipts, preserving their canonical V1 bytes. Absence means no report was
recorded; it does not mean conflict-free.

A service session can supply an authenticated read-only adapter source. It
checks current repository read permission and authority epoch through the same
token policy as reads and mutations; a saved adapter checkpoint grants no
access. Adapter cursor writes use a dedicated non-authoritative namespace and
cannot change repository heads, receipts, settings or manifest authority.

The service's adapter projection reader reuses the native content engine over
verified snapshot objects. It restricts paths to that inventory, verifies
framing and native identity, and repeats current authorization at storage/read
boundaries. Neither raw object storage nor publication methods are exposed to
extraction.

The M4.10 committed-read service extension adds paged references, recursive
paths, operation-change groups and changed-path summaries. It accepts no command
intent, stages no native object or upload, creates no receipt and cannot invoke
publication. Mutation authorization and guarded apply semantics remain
unchanged.

The M9.4 AgentStore session pager is another read-only service capability. It
reauthorizes every page, resolves summaries only from one audited selected
AgentStore root and retains no principal credential in its process-local cursor.
The pager exposes neither a typed AgentStore delta nor a request/publication ID,
receipt lookup, staging method or publication coordinator. Resuming a cursor
therefore cannot create a collaboration write or dispatch an agent.

Stable-session detail uses the same current-root pin and authorization boundary.
It performs one typed session-catalog lookup and returns only service-owned
summary and immutable-link DTOs. Missing sessions return no object data; a path
ID, selected snapshot ID or returned dependency ID grants no read or publication
capability.

Transcript paging verifies that the selected session owns the requested manifest
and source generation, then converts only already-sanitized chunks into bounded
service DTOs. Its signed continuation retains an immutable root and range
position but no credential, typed delta, request/publication identity or receipt
capability. Reading or resuming a transcript cannot stage or publish a
collaboration object.

Association traversal selects one typed session, tool, operation, revision or
checkpoint subject and returns only immutable checkpoint-version and association
links already selected by the audited AgentStore root. Its cursor retains that
exact subject and root-local page position. The route has no correction command,
typed delta, staging handle, receipt or publication identity, so an empty or
unexpected relation page cannot be repaired through this read surface.

Checkpoint collection and detail are also read-only. Current detail resolves one
stable checkpoint through the selected catalog; immutable detail can only walk
that selection's validated prior-version chain. Neither route accepts a
replacement, correction, expected selection, command identity or publication
request. Reading a prior version therefore cannot reselect it or supersede the
current checkpoint.

Review thread collection and detail are read-only under the same separation.
Their cursor retains an audited ReviewStore snapshot without a credential or
publication handle. Detail resolves only the stable thread's current catalog
selection and its already selected anchor, comment and event objects. The routes
accept no expected version, review command, request/publication identity or
typed delta, so they cannot reply, edit, resolve, re-anchor or publish.

The shared ReviewStore command request, outcome, receipt and status DTOs support
both serialization and deserialization so HTTP and native RPC use one public
contract. Transport callers provide only the versioned request ID and typed
command. The service still derives principal and publication identity, enforces
exact workflow/revision/thread preconditions and decodes only the selected
durable receipt. A committed command emits the existing repository-only wake
hint after publication; collaboration bodies and credentials never enter it.
