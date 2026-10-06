# Standalone process (continued 2)

The retained authenticated server/CLI fixture opens authenticated SSE, publishes
a native bookmark operation, resumes from the same event ID after server/storage
restart and cache deletion, and checks live stream termination on token
revocation. This qualifies deferred behavior; LocalTrusted cold acceptance uses
token-free SSE and Unix RPC.

The CLI-owned native lifecycle fixtures also exercise every HTTP read route for
generated contract conformance. An opt-in capture path receives only disposable
response bodies and route metadata, never request credentials. This fixture does
not alter CLI presentation or publication semantics.

Server configuration may provide web_root, resolved relative to its TOML file,
to serve a prebuilt browser bundle alongside the protected HTTP APIs. Build it
with pnpm web:build. A malformed or oversized bundle fails server startup
explicitly; no filesystem directory listing or repository content is served as a
browser asset.

CLI-owned native lifecycle fixtures compare internal typed command preparation
with the pinned native repository model, including descendant, bookmark,
workspace and evolution effects. The command preparation library does not add a
new CLI command or advertise HTTP mutation support by itself.

Native lifecycle tests also exercise the typed command service's provisional
preview, strict apply and principal-scoped durable status. Retry evidence checks
native result identity after later publications/restart; it does not advertise a
new CLI extension or imply browser mutation support.

Native lifecycle acceptance also exercises the typed HTTP command transport:
authenticated New/Describe/bookmark commands, strict snapshots, protected
history, durable replay and lost acknowledgements. These routes reuse the
service and do not change ordinary CLI spelling or its capability evidence.
LocalTrusted browser mutations reuse the same transport without bearer
credentials. Authenticated/shared browser finalization remains deferred under
M4.3.

The same lifecycle fixture covers EditFile preparation through the native
command boundary: one child change replaces or renames bounded text, retains
file mode and copy identity, moves the exact bookmark and leaves the selected
manifest unchanged until strict publication. It also rejects ambiguous targets,
invalid paths, oversized text and a replay against a bookmark that has already
moved. This remains application/HTTP command evidence and adds no custom CLI
spelling or physical checkout editor.

Typed reconciliation acceptance compares native descendant rebases and recursive
operation-ancestor merges to pinned upstream behavior, including three current
heads and conflicted bookmarks. The application mutation is explicit and strict;
ordinary CLI head-reconciliation behavior remains unchanged.

Native abandon acceptance exercises both bookmark-retention and
descendant-content options, adjacent targets, merges, replacement workspace
commits, hidden targets, historical/evolution reads, protected policy and typed
HTTP retry behavior. It also preserves binary/mode/copy/symlink/submodule values
through native rewrites. Ordinary CLI commands remain covered by their existing
executable parity lanes.

Native lifecycle acceptance now exercises typed conflict term selection and
deletion through the service and HTTP API, including descendant/bookmark/logical
workspace updates. Ordinary jj resolve remains an upstream client command;
server typed commands never execute client editors or alter physical files.
Uploaded typed replacements have separate scoped-content acceptance below.

Typed upload/resolution acceptance covers the native service and HTTP transport;
ordinary client-side jj resolve continues to use upstream checkout semantics.
The server caps HTTP conflict uploads at the smaller of its configured native
blob limit and the 16 MiB default. Failed streaming staging does not create a
native operation or durable mutation receipt.

Native conflict-resolution acceptance also checks complete bounded reports for
the target and rewritten descendants, explicit empty remaining-path lists, and
legacy receipt decoding. This API contract adds no browser or CLI editor.

Native adapter-source acceptance binds the runner to repository read tokens,
checks committed native framing/identity, excludes staged objects, and verifies
revocation before further projection delivery. This test uses in-process native
stores; it does not claim search, UI or real storage deployment qualification.

RPC deadline/admission acceptance waits for a server-side authorization read to
prove the pending upload owns the permit before issuing the competing request. A
polling competitor must not win admission and accidentally reject the upload
under test. The original transport deadline and failure/release assertions
remain unchanged.

Native adapter acceptance reads complete metadata, trees and files through the
service projection reader, including virtual roots and non-file native entries.
These fixtures exercise pinned JJ codecs without moving upstream imports into
Tantivy, adapters or service contracts.

Native CLI integration fixtures also exercise authorized HTTP search over actual
native objects and local Tantivy generations. These tests require no installed
CLI replacement and make no browser feature changes.

Native integration fixtures verify configured search identity at runner startup
and same-position restart after source exclusions change. Failed rebuilds must
not expose an incompatible prior generation to newly authorized reads.
