# Bundled application documentation

The Help entry uses the primary sidebar's collapsed and expanded spacing. This
presentation change preserves the same overlay, local document routes and
admission policy.

Help reads same-origin presentation attributes to inherit live application
appearance; it adds no API, mutation or credential transport. Generated fonts,
palette CSS and diagram variants remain in the independent documentation asset
manifest and use the existing CSP. Standalone docs do not read app preferences.
The standalone Original palette and initial navigation collapse are local
presentation defaults; they do not change host preferences or asset admission.

Bundled Help is independently allowlisted under `/help/`, with a separate
512-file, 8-MiB-per-file and 32-MiB-total bound. The existing browser bundle
retains its own bounds. Application documents may frame same-origin content, but
only documentation responses permit same-origin ancestors; API responses remain
unframeable. New browser builds include Help; older valid bundles without a help
directory remain loadable.

Nonrecursive path pages include direct file, directory, symlink, submodule and
unresolved structural-conflict entries. They load the selected directory terms
without walking descendant directories. Unfiltered bookmark windows select rows
before target hydration; each returned target retains exact pinned membership.

Path windows accept `include_last_change` (default true for existing clients).
False returns row identities and `not_requested` attribution without walking
parents. Optional attribution finishes in one shared first-parent scan of at
most 64 steps. Available results identify `path_change` or `snapshot`
provenance; older responses without the field are treated as `path_change`.
Snapshot rows show a compact accessible Snapshot icon and link to the selected
commit they describe. The client stops after a successful metadata response;
only busy admission may retry, and navigation cancels obsolete work. Each
metadata result is joined only to matching immutable commit/path/term
identities. Metadata failure cannot discard rows. Directory navigation shows a
skeleton while its file read is loading even when the repository retains a prior
snapshot. Ready file names remain independent of optional row attribution and
pagination retains already loaded rows.

Reference windows support `kind=workspaces` independently of visible-change
history. Each row names one workspace and its exact selected target. Bookmarks
and workspaces support an optional `exact` name; exact lookup uses the captured
view's keyed map, returns no continuation, and rejects a simultaneous text query
or page anchor. Workspace windows seek their own ordered names and hydrate only
the requested window. Continuations bind kind, exact name, query and page size.

Operation log windows without an ID-prefix selector MUST walk bounded operation
metadata directly, without loading commits, trees, views or a commit index.
Native responses report `not_used` for the graph cache. Operation ordering,
parent/child selectors relative to `@`, filtering, pinned membership,
authorization and cancellation retain pinned JJ semantics. ID-prefix selectors
retain their existing complete admission until the prepared operation lookup is
qualified. Local reads must not repair or rewrite a damaged commit index as a
side effect of reading operation metadata.

Native commit-graph startup and explicit preparation count actual verified
commit reads rather than materializing the committed inventory for admission.
Crossing the cold inline threshold returns rebuild-required and discards
staging; every warm or newly built index still observes the hard index-size
limit. Syntax work is independent of total retained repository size. This
preserves query identities, native evaluation, signed continuations, current
authorization and explicit resource errors; broader prepared-lookup and
CPU/memory qualification remain separate gates.

Local history preparation has a distinct process-local service task contract
with local snapshot and statistics, never a fabricated native epoch or cache
report. Its default construction deadline is one hour, completed retention is
five minutes and retained task count is 32. Trusted composition may lower these
limits. Current device binding authorizes every operation; running or usable
ready tasks coalesce by repository, generation, metadata path and operation.
Expired/restarted/evicted task IDs reset; they cannot become valid again on
later completion. A cold prepared-log request reports rebuild-required for its
exact operation. HTTP and browser delivery require their own contract and
acceptance before this service capability is advertised.

LocalTrusted code capabilities advertise `index_tasks`. POST
`/v1/local/repos/{repo}/index/tasks` accepts only an explicit operation and
returns a local task; GET `/index/tasks/{task}` inspects it and bodyless POST
`/index/tasks/{task}/cancel` requests cancellation. All three return HTTP200
with a local snapshot/status; expired or unknown IDs return410 `task_reset`.
Bounds remain trusted server configuration, never request fields. Native task
routes and snapshot types remain unchanged.

Local log, detailed commit and evolution POST queries use the retained prepared
generation. A cold request proves its requested operation without commit
preflight, then returns409 `index_rebuild_required` with `local_snapshot`. Both
first pages and cursor resumes retain the original operation, generation and
complete query. Files, references, headers and operation metadata remain
independently readable. A local error must not fabricate a native
snapshot/epoch. The browser validates the error pin and all start/status/cancel
responses before resuming that query.

Native and local browser index-task adapters forward the owning observation's
abort signal for start, status and bodyless cancel requests. Aborting that HTTP
observation is not a server cancellation result. Clients serialize polling and
explicit cancellation for the captured task ID, retain the original operation,
and continue observing an unconfirmed cancel without automatically replaying the
cancel command. Server-confirmed canceled and ready outcomes are distinct.

Local selected commit-summary, path, file-at, diff, changed-path and conflict
queries use an exact ready local generation when available, retaining signed
cursor and current registration checks. Absence alone selects the existing
bounded scoped reader, keeping directly referenced files usable without
preparation. Generation/authorization/selection errors propagate explicitly;
there is no fallback that admits unrelated or later Git objects. These readers
change no HTTP request, response, schema or capability. Their comprehensive
transport/browser qualification remains in the later bounded-read slice.

Local Changes/All reference queries use their embedded operation's prepared
ordered visibility. When cold they return the existing exact-operation
rebuild-required response, allowing the existing recovery flow. Continuations
preserve lexical order and hydrate only page targets. Nonempty filters
additionally use compact disk postings and exact description checks on pinned
visible targets before page hydration. Filtered Bookmarks share this recovery
and selection path; workspace queries and empty bookmark windows retain
metadata-only cold reads. Different source selections require the
operation-parent proof described below. Reference DTOs, signed cursors and
preparation task contracts are unchanged; a resumed cursor still rejects a
different filter and retains its exact old operation after publication.

Composed native Changes/All reference reads with an empty query MUST reuse the
embedded operation's prepared ordered visible change groups and select their
lexical window before target hydration. Eligible nonempty Changes/All/Bookmarks
filters use operation-bound postings and exact selected description checks
before hydration. Empty bookmark windows and workspace queries retain their
metadata-only readers. Cold large generations return the existing
rebuild-required response with the exact pinned snapshot; retry after
preparation uses the same query and signed-window semantics. No cursor or public
schema changes. Native service reference requests with a different embedded
operation retain the outer read's current authorization and committed object
membership while selecting the source operation's exact prepared generation.
Cold source preparation remains explicit; existing browser pages select the same
operation for their outer snapshot and embedded reference request. Small-cache
profiles preserve their existing bounded filter reader. Paged native RPC is
governed by the additive manifest-pinned InventoryPage contract in
jj-integration.md. Its scan windows admit work, control-record bytes and output
independently of total repository size; sparse windows remain explicitly
continued. Full kind enumeration still scans the hashed catalog under a total
caller budget. Prepared kind-specific lookup remains separate delivery work.

Local operation-prefix graph queries use ready exact-operation generations and
existing rebuild-required recovery when cold. This delivery changes no request,
response, cursor or public schema. Read-only plain operation pages retain their
metadata-only cold path.
