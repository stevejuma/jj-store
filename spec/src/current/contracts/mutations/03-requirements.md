# Requirements

| ID          | Requirement                                                                                                                                                                                                                                                        |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-MUT-001 | Typed preview, apply, and status endpoints MUST share canonical intent, exact snapshot preconditions, authorization, and durable receipt semantics.                                                                                                                |
| JST-MUT-002 | First release MUST implement New, Describe, bookmark create/move/delete, Abandon, ResolveConflicts, and ReconcileOperations through native transactions.                                                                                                           |
| JST-MUT-003 | Abandon MUST enforce protected/root policy, transform descendants, update bookmarks/workspaces, preserve history and evolution, and publish once without deleting objects.                                                                                         |
| JST-MUT-004 | Conflict resolution MUST preserve ordered native terms, check fingerprints and authorized uploads, validate all selected paths atomically, rewrite descendants, and retain unresolved paths.                                                                       |
| JST-MUT-005 | Client-computed RPC publications MUST pass the same semantic protected-history checks as application mutations; no arbitrary shell or CLI argument endpoint MAY be exposed.                                                                                        |
| JST-MUT-006 | EditFile MUST create one child change from the exact single-target bookmark, replace or rename one existing bounded UTF-8 file, preserve native file mode/copy identity, move that bookmark atomically, and retain durable preview/apply/status receipt semantics. |

Native collaboration command requests have no actor or publication-ID input. The
service derives both from current Write admission, reauthorizes receipt
responses, preserves accepted/unknown status, and emits freshness only for a
selected publication. Read pins retain immutable source roots without sessions
or credentials and reject a different repository, viewer or epoch on reuse.
Actor visibility is an injected trusted repository policy evaluated before
reaction aggregates and identity pages; default repository-scoped admission has
no private actor subresource grants. The native collaboration pager
independently bounds pin capacity, retained graph bytes, objects, expiry and
query continuations. Its signed viewer/query binding cannot confer read or write
permission: every resumed page rechecks current admission, and token revocation
denies even a previously issued cursor. Paging never publishes metadata, emits
mutation freshness or retries a command.

Local collaboration command/result aliases consume the shared jj-collab domain
contract. Provider requests and receipts retain their existing local encodings;
this extraction does not transfer actor admission or publication authority to
the model crate.

The service's publication guard binds the authenticated token principal to the
WAL request actor, validates replacement settings and delegates native
transition validation to an explicit semantic policy. A transport cannot select
a caller's principal independently of its verified credential. No permissive
semantic validator is provided by the service library.

The same guard accepts a principal only through the shared access-policy
boundary. LocalTrusted derives that principal from the installation and verified
local peer, with one explicitly configured permission; authenticated access
continues to derive it from the repository token. Existing WAL requests and
receipts store only the resulting `PrincipalId`, so the access-mode addition
does not rewrite historical records or weaken protection, snapshot, idempotency,
semantic or resource checks.

AgentStore domain publication uses the same guard with current write
authorization. The guard compares the domain request actor with the principal
derived from the current access policy before receipt lookup, staging or CAS.
The prepare service rechecks that principal after staging immutable objects; a
later publish repeats authorization, and a peer cannot replay another
principal's prepared request. Domain writes do not run native history semantics
because they preserve code heads and code-state version through their separate
manifest root.

ReviewStore commands use a separate typed service over that same guarded domain
publication. The HTTP request carries only version, request ID and validated
semantic command. The service derives principal and repository-bound publication
ID, validates the committed result bytes and exact receipt identity,
reauthorizes after publication or status lookup, and emits only a freshness hint
after commitment. Apply returns `outcome_unknown` when commitment cannot be
classified; status returning `not_known` is never treated as rejection.
