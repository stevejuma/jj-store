# Authenticated search sessions

The Tantivy serving layer binds a request's search text, visibility mode,
selected operation, source scope, page size and freshness options. The default
mode uses native commit visibility; historical metadata search is explicit.
Configured source-scope filters select that scope's files before ranking.

Opaque jss1 continuations use a process-local HMAC key. Their claims bind the
repository, epoch, complete query fingerprint, selected-view fingerprint, source
position, index artifact, offset and expiry. Retained pins contain no
credentials or live sessions. Every page authenticates a new native session, and
signatures or changed inputs cannot silently select a newer view/generation.
Expiry and serving restart require explicit reset.

Idle maintenance releases expired pins at an interval no greater than 30 seconds
or the configured TTL. Dropping the pager aborts that task and releases its
registry. Pin count, retained view nodes, request concurrency, query time, page
offset and response bytes are bounded. Failed response encoding releases its new
pin; an offset budget returns explicit truncation without an unusable cursor.

Results distinguish current repository position, pinned indexed position and
artifact, selected native view and indexed scope snapshots. Freshness waits are
bounded and reauthorize while polling. They require a served generation, not
merely a durable commit or advanced adapter cursor. Degraded adapter/reader
state is reported independently from authoritative repository readiness.

Eight native fixtures cover these contracts with actual local Tantivy files,
native objects and production token/publication services over memory storage.
The combined suite initially exposed an artificial one-second full-index-build
assumption in the wait fixture. The service correctly returned index lag. The
fixture now prepares the durable generation separately, tests reader recovery
within the configured five-second budget, and retains explicit zero/short-wait
lag checks. No performance or authorization assertion was waived.

Initial compilation failed on the absent paging API (four diagnostics), followed
by two missing fixture import diagnostics during expansion. Commands, failures
and aggregate results live in E-M6-005 of the
[single ledger](../../spec/src/milestones.md). Local Tantivy schema 2 and
authoritative storage formats remain unchanged. HTTP registration/generated
contracts, server runner configuration, remote snapshots and measured deployment
acceptance remain open; UI work remains deferred.

Final validation passed the original named target, all eight serving fixtures,
all 15 native projection fixtures and ten Tantivy durability fixtures. Full
validation passed check, build, all 13 specification lanes and formatting; the
test lane passed 338 Rust tests with zero failures and six external tests
ignored, plus governance, harness, contract and existing web tests.
