# Complete native stack discovery and boundaries

The
[native stack discovery contract](../local-jj/31-complete-native-stack-discovery-and-boundaries.md)
owns inventory, boundaries and membership semantics.

Stacks Apply/Unapply controls invoke the existing guarded command POST directly,
without a separate advisory preview request. Operation, binding-generation and
configuration-revision checks remain authoritative at execution. This frontend
flow changes no wire schema, grouped-unapply semantics or unknown-outcome retry
policy; an uncertain result retains the exact UI request for state inspection.

Stack row, header and branch-picker selection toggles are presentation state.
Deselecting closes inline details without issuing a command or discarding cached
history, drafts or adjacent diff reads. Explicit programmatic selections retain
their idempotent read behavior. Clipboard path copying stays browser-local.
