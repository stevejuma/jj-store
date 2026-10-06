# Search serving sessions

Search continuations use process-local HMAC signatures and a bounded registry of
credential-free index/view pins. The signature binds repository, epoch, query,
scope, mode, page size, native view, source position and exact index artifact.
Changed inputs or signatures fail; expiry and server restart require an explicit
cursor reset. Expiry maintenance releases idle pins so they cannot indefinitely
hold retired local generations. Retained view nodes and request concurrency have
separate budgets. Every page checks current read permission and epoch.

Results report current repository position, independently pinned indexed
position/generation, scope snapshots, native view, staleness and degradation.
Optional minimum-position requests wait only for a configured bounded duration;
failure to catch up returns INDEX_LAG with authorized freshness metadata. A
continuation never silently upgrades its generation. Source-scope filters select
only that configured scope's files. Historical metadata search is explicit;
visible-commit mode requires operation selection when heads are ambiguous.
Pagination reaching its offset budget reports truncation without an unusable
continuation. Search remains unable to publish repository history.

HTTP composition accepts only explicitly configured per-repository search
handles and runner health receivers. It constructs pagers with the shared native
content worker pool and caps search output to the HTTP response budget. Route
availability does not imply an index is ready; status distinguishes durable and
served positions, and a missing reader returns search_not_ready. Server runner
configuration and remote snapshot restoration are covered by the M6 deployment
acceptance.
