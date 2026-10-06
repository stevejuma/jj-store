# Read capability membership

Application content read pins retain a checked immutable manifest inventory root
and bounded request-local membership lookups. Every requested object requires
selected membership and current permission; opening a read does not audit an
entire repository. The explicit complete-inventory path remains bounded for
mutation preparation and adapter consumers, with their existing Write and source
authorization. This separation does not weaken mutation preconditions, complete
publication closure audits or manifest CAS.

File-row and metadata reads have separate cancellation scopes from mutation
receipt identities. Navigation may cancel obsolete headers, attribution and
content without modifying drafts, prepared writes or unknown outcomes. Scoped
local `file_at` verifies the selected commit/path/file relationship before
opening content; lightweight summaries and row-only requests grant no write or
publication capability. Confirmed file saves continue through the existing
receipt-backed refresh and pinned selection behavior.

Reference picker pagination and exact-name resolution remain reads. They MUST
NOT change mutation preconditions, pending request identities, selected
operation checks or physical working-copy state. Resolving a missing
bookmark/workspace must not silently select another target for a command.

Operation-only read capabilities permit proven operation parents and deny views,
commit access, writes and collection. They do not reconcile heads, publish a
manifest or rewrite a local derived commit index. Mutations retain their
existing complete graph and publication-closure admission.

Local history preparation is a non-publishing service task bound to an explicit
retained operation and current device registration. Start, status, cancellation
and prepared graph access reauthorize that registration; task identifiers confer
no authority. Completed entries retain source pins and derived data without
credentials. Cancellation retains worker and reservation ownership until actual
exit, including an initial authorization wait. Eviction, expiry and restart may
require preparation again but cannot alter native heads, companion state,
working files, source indexes or retention refs.

Local task reads may share an exact ready generation with optional scoped
historical content requests. Clone the handle under the short registry lock,
then release it before source authorization/decoding. Each response retains
current binding, operation seal and ordinary admission; no task lookup grants
publication or repairs source state. Missing generations alone allow the
ordinary content reader. Lifetime, reservation and eviction rules are unchanged.
