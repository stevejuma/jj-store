# Local stack workspace frontend boundary

The JST-UI-020 catalog owns TypeScript workspace snapshots, capability gates,
file/hunk selections, exact loaded directory selections, per-stack inline drafts
with exact staged selection lists, multi-file transfers to mutable commits or
back to Unstaged, preview requests, command outcomes and request-status
contracts. An injected adapter supplies them to the production controller. They
are frontend contracts, not new HTTP/RPC endpoints or generated Rust schemas.
The fixture adapter performs isolated in-memory transitions only. Column
resizing preserves these selections and natural commit content heights without
changing snapshot, request or draft state. Backend implementation was approved
on 5 October 2026. Production eligibility, publication, physical checkout and
durability MUST be qualified independently of the catalog. Versioned wire
contracts MUST bind an authorized device binding, explicit code operation/view,
named combination generation, exact content selection and original request
identity. Numeric frontend revisions remain presentation state and MUST NOT
substitute for those preconditions.

The snapshot also identifies the workspace target branch and its exact head,
with author/message metadata for its bookmark card. A separate injected history
read accepts repository, snapshot revision, branch name, head, bounded limit and
opaque continuation cursor, and returns newest-first commits plus the next
cursor. The controller permits one page request at a time, deduplicates commit
identities, stops at exhaustion and rejects non-advancing cursors. Refreshing to
a different revision/head invalidates prior pages and late responses; teardown
also discards late responses. History and its file diff remain read-only and
separate from applied stacks, draft state and command inputs. The isolated mock
and production operation-pinned adapter implement this frontend read contract;
the contract itself grants no mutation or persistence authority.

Production stack browsing reuses the existing LocalTrusted registry and code
query endpoints. Optional branch continuations and stack histories remain
bounded and bind repository, revision, opaque read key and exact head/base. The
presentation revision is not a code authority. All production source reads
validate the captured operation and binding generation; source IDs not issued
within that scope cannot broaden a comparison. Preview/apply/status remain
unavailable until a qualified durable command provider exists.

The production App explicitly injects its repository/binding-scoped Stacks
owner. Reference continuations retain exact operation context and unavailable
bookmark targets. Loaded source ranges survive refresh only after validation of
their exact head/base; new operations receive new cursors and annotations.
Asynchronous reads check scope before issuing work and before caching results.
Only a completely read candidate scope replaces the visible owner. Discovery or
binding mismatch cannot grant access through a route or presentation identity.

The frontend command union also carries optional exact selections to stage after
confirmed branch creation, an optional parent stack for dependent creation,
explicit above/below placement for empty commits and new branch heads,
branch-specific rename/delete/squash, commit deletion and draft-based commit
editing with exact additional selections. Immutable history cannot be rewritten;
copying identifiers/messages and placing branch heads are read-only or bookmark
operations. These contracts are UI adapter inputs only and introduce no service,
RPC, CLI or persistence endpoints.

Branch aggregate selections retain both the stack identity and the exact branch
name. Their file list and diff include only that branch segment's commits;
refresh preserves the same source while it exists. Hunk transfers resolve the
aggregate to one mutable owning commit before staging or previewing a command.
Ambiguous ownership remains read-only. Selecting a branch never replaces another
column's open diff.

Commit movement may carry an exact destination branch name with its stack
identity. A header drop inserts the commit at that segment's head. Row sorting
carries an exact target commit and above/below placement; existing directional
reorder commands remain supported. Header feedback is scoped by stack and
branch. Feedback and preview validation reject missing targets, self or
redundant positions and moves that rewrite immutable history. These remain
frontend mock inputs with no transport or persistence change.

Compact column panel toggles and independent accessible column titles are
presentation-only under JST-UI-020. The source-controlled icon and its
hover/focus preview add no request, command, persistence or transport fields.

Unstaged pinning uses Design Core's public `fixed` presentation property. Its
expanded/collapsed position and temporary capacity fallback introduce no new
workspace adapter, command, transport or persisted layout fields; file/hunk drag
semantics and adjacent diff origins retain the existing frontend contract.

Production branch file summaries may expose an adapter-owned `selectionSource`
only after proving a complete loaded linear segment, one changing commit for
that path and exact equality of ordered native before/after terms, modes and
provenance with the net comparison. Resolution is bounded to 40 commits and 50
file-summary pages, loads no patches, and caches only within the exact read key.
Opening a resolved diff requests native hunk tokens for that concrete commit.
The controller projects those exact files into the owning commit for existing
drop validation; drag and keyboard assignment carry only concrete commit/path
and issued tokens. Missing ancestry, multiple owners, immutability, mismatched
terms or exhausted bounds remain read-only. This adds no HTTP/RPC schema or
command; exact operation, configuration, binding, preview/apply and unknown
outcome guards remain authoritative under JST-API-001/002.
