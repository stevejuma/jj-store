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
