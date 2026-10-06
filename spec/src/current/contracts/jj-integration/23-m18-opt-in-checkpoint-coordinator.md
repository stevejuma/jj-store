# M18 opt-in checkpoint coordinator

The [M18 provenance profile](../collaboration/index.md) keeps working-copy
snapshots in the client JJ boundary. Transcript ingestion alone does not
snapshot or reconcile heads. Explicit per-workspace enablement admits baselines,
before/after/failure scopes and exact operation evidence through existing JJ
transactions and locks. Preexisting dirty work remains a separate baseline;
no-change observations do not manufacture code operations. Immutable provenance
intents avoid hash cycles and late associations never rewrite published
operation identities.
