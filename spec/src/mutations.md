# Typed mutations

Previews do not publish or grant authorization. Stale snapshots/conflicts fail
explicitly. Binary, structural, symlink, executable, and multi-sided conflicts
are not flattened into a two-sided text model. Supported term selection and
deletion are explicit; unsupported editors return a typed explanation. Server
rebase/squash/undo are deferred; client capability evidence is separate.

## Requirements

| ID          | Requirement                                                                                                                                                                                  |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-MUT-001 | Typed preview, apply, and status endpoints MUST share canonical intent, exact snapshot preconditions, authorization, and durable receipt semantics.                                          |
| JST-MUT-002 | First release MUST implement New, Describe, bookmark create/move/delete, Abandon, ResolveConflicts, and ReconcileOperations through native transactions.                                     |
| JST-MUT-003 | Abandon MUST enforce protected/root policy, transform descendants, update bookmarks/workspaces, preserve history and evolution, and publish once without deleting objects.                   |
| JST-MUT-004 | Conflict resolution MUST preserve ordered native terms, check fingerprints and authorized uploads, validate all selected paths atomically, rewrite descendants, and retain unresolved paths. |
| JST-MUT-005 | Client-computed RPC publications MUST pass the same semantic protected-history checks as application mutations; no arbitrary shell or CLI argument endpoint MAY be exposed.                  |

The service's publication guard binds the authenticated token principal to the
WAL request actor, validates replacement settings and delegates native
transition validation to an explicit semantic policy. A transport cannot select
a caller's principal independently of its verified credential. No permissive
semantic validator is provided by the service library.

## Native publication policy

Native publications must descend from every removed committed operation. Strict
publications must retain operation ancestry for all prior heads. Newly
introduced operation graphs are bounded and anchored in committed operations;
root-only rollback, unanchored history and parentless non-root objects are
denied. This preserves concurrent siblings while allowing native operation
reconciliation.

The server's trusted protection rules name immutable commit tips and bookmarks.
Their native ancestor closure, always including the virtual root, cannot be
hidden or rewritten. A new object sharing a protected change ID is also a
rewrite and is denied even when a crafted request omits predecessor metadata.
Multiple already-protected divergent incarnations may be retained exactly;
protecting their change ID does not arbitrarily select one of them. Protected
bookmark values cannot be moved or removed. These rules apply equally to direct
native publication and application transitions; they are independent of token
permission. Protected-history checks use native objects rather than trusting
client operation descriptions. Rule configuration is server-owned and never
accepted from a publication request.

A new request cannot resurrect a committed operation that is no longer a head;
identical receipt retries are resolved before this check. Publication also
checks new workspace-name claims against current heads, so concurrent physical
clients cannot silently acquire the same logical workspace name. Reconciliation
can retain mappings inherited from its parent operations.

The native transport facade derives publication actors from repository tokens
and accepts only native removed-head/added-operation transitions. It does not
expose settings mutations or an unguarded coordinator. RPC head updates
therefore use the same protected-history checks and durable retry receipts as
local sessions.

Token administration uses strict snapshot preconditions and a stable request ID.
Grant and revoke reconstruct their exact original intent from the retained
policy at the expected committed snapshot, allowing receipt lookup after later
publications or checkpointing. Different actions under the same identity
conflict, including revoking different already-absent token IDs. Concurrent
administrators cannot overwrite each other's accepted policy changes. This typed
administration boundary does not expose raw settings publication.
