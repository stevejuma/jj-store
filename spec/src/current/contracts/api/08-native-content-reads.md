# Native content reads

Object responses carry an explicit native-profile version, typed key and
complete project-owned V1 metadata. They preserve signatures, ordered conflict
terms, copies, external submodule IDs, remote/local references, workspace
mappings and absent-versus-empty operation predecessor data. Virtual roots
require the same current authorization and captured committed membership as
stored objects.

Tree listing uses pinned native merge semantics without resolving or writing
objects. Paths are canonical repository paths; symlinks and submodules are never
followed. Dot, parent and NUL path components are rejected without
normalization. File/tree conflicts remain explicit terms. Directory pagination
uses the last existing native entry name internally. The service removes raw
continuations from responses and exposes only signed cursors. Graph and
directory requests share bounded snapshot retention and concurrency when
composed with the same cursor registry; changing query kind, commit, path or
page size is rejected. Oversized continuation tokens fail explicitly instead of
returning an unsigned name. Cursor registries retain neither credentials nor
mutable native sessions.

Browser collection routes add bidirectional windows without changing the
complete-response graph, tree, operation-difference or file-difference APIs.
`GET /references` searches real local bookmarks and visible native change IDs;
bookmark conflict terms, every explicit target, target metadata and logical
workspace associations remain intact. The route never assumes a `main` bookmark
and cannot manufacture deleted-bookmark history. Browser state retains the
selected bookmark or change identity separately from the commit resolved from
that reference; resolving one target must not replace the selected reference.
`GET /commits/{commit}/paths` returns direct file and directory terms for a
nonrecursive path window, while a recursive query walks leaf paths only as far
as the requested forward window plus a bounded probe. It reports exact native
terms. Names load without ancestry work. Optional metadata makes one shared
first-parent pass of at most 64 steps through jj-lib commit and tree APIs.
Comparing file values and directory tree IDs accepts integration merges as path
changes. Unresolved rows use the selected commit's genuine metadata with
`attribution: snapshot`; discovered rows use `path_change`. This quick response
is terminal and does not schedule changed-path preparation. History mode
separately reuses the existing bounded ancestry continuation. Core graph
preparation remains available when needed to prove historical membership. Hard
authorization, corruption, cancellation and resource failures remain explicit
errors, never snapshot fallback.

Each browsing response carries signed `previous_cursor` and `next_cursor` values
over the same retained snapshot registry as graph and directory pages. Their
claims bind repository, epoch, operation, publication position, manifest
revision, query/filter/order and page size. A stable bookmark/change/path anchor
and direction can start a new bounded seek; only opaque cursor values resume a
retained window. Cursors never belong in permanent navigation state. Changed
queries, credentials, repository, operation or page size fail closed, and expiry
or process restart requires an explicit snapshot reselection.

File responses verify the complete envelope and native identity into a bounded
private spool before exposing any bytes, including for ranges. The stream checks
current authorization and elapsed budgets before and after every bounded chunk.
Stream lifetime retains its concurrency permit and private spool. File bytes are
binary media, not JSON or trusted HTML; HTTP safe content-disposition and
browser rendering remain transport/presentation requirements. A client composing
multiple file previews MUST keep those reads within native admission rather than
bursting one request per preview; a Busy response must not discard an already
validated directory listing.

Canceling a chunk invalidates that stream handle. Reusing it fails explicitly;
resumption requires opening a fresh verified range. This prevents cancellation
between local I/O and final authorization from silently skipping content.
