# Local stack topology and focused history

Local Stacks state carries optional bounded topology for the active workspace at
its returned operation: ordered lanes, branch tips/aliases, explicit members and
integrated branches. Incomplete topology is explicit and cannot certify an
independent column. The additive `unapply_branches` command validates the entire
member set under the existing operation/configuration guards and publishes one
native transaction. Single-branch apply/unapply remains compatible.
Reconciliation previews identify the exact obsolete internal merge heads
eligible for cleanup.

Frontend-only `stack_history=live|pinned` is valid on focused Log/Op Log routes.
Absent mode defaults to live; explicit historical navigation emits pinned mode.
The mode does not alter backend operation-pinned reads or authorize mutations.
