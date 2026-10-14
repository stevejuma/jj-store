# Workflows

WorkflowStore format 6 adds an independent typed root. Historical catalogs and
report chunks are audited by recovery and native copy. Only manifest CAS selects
the root; stable requests retain receipts. See JST-WF-001 through JST-WF-007.

E-CUSTOM-INDEX-001 adds an owned bounded read facade for complete WorkflowStore
source admission. It checks caller object and aggregate payload budgets before
state accumulation; the unqualified source is never cached as ready. JST-WF-001
and native manifest/root/object/catalog bytes, staging, validation and receipt
semantics remain unchanged. Mutation admission keeps its existing proof.
