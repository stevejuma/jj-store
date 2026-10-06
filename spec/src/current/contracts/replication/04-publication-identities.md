# Publication identities

`ReplicaId` identifies one durable installation. `WriterEpoch` changes when an
origin stream is deliberately replaced. An `OriginKey` is repository, replica,
writer epoch and monotonically accepted sequence. The canonical origin digest
covers the origin envelope without any destination WAL identity or mutable
manifest reference. A `LocalWalPosition` remains meaningful only with its
issuing replica and log epoch.

Origin envelopes name their predecessor digest, causal frontier, semantic
format, stable request/intent identity, published operation heads, accepted
agent/review facts, complete inventory root and changed domains. A combined
origin remains one atomic visibility unit at every receiver.
