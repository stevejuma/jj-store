# jj-store-adapters

Infrastructure for rebuildable projections of manifest-selected native history.
It supplies authorized sources, immutable artifact bundles, checkpoints,
cancellation and bounded projection runners.

Implement `Projection`, create a `Source` with an `Authorizer`, and compose them
with `Runner` and explicit limits. Advance the adapter checkpoint only after the
projection has durably committed its own output. Repository state continues to
be selected by the native manifest.

See [reads and search](../../docs/site/technical/reads.md) and
[publication](../../docs/site/technical/publication.md).
