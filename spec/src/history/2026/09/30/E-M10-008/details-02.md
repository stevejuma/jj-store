---
schema: 1
id: E-M10-008
---

# E-M10-008 details 2

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The recoverable endpoint switch and its nonempty-workspace/offline-mutation
  acceptance are implemented, but the focused executable acceptance remains
  unexecuted because the pinned Kingfisher Vectorscan build requires missing
  cmake on this host.
- M10.7 production lifecycle composition, honest status/search/SSE reporting,
  resource and disk-pressure qualification, outage proof and cold real RustFS
  acceptance remain outstanding.
- The repository-wide contracts, check, test, build and specification lanes that
  compile Kingfisher Vectorscan require cmake on this host.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; manifest format 2; replication root, origin, seed, bootstrap negotiation and peer checkpoint schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

A seed is the first origin in a newly allocated source stream and covers repository, code, AgentStore and ReviewStore state. Negotiation adds the exact manifest digest, feed lower bound and optional complete bootstrap checkpoint. New service-created repositories allocate independent random ReplicaId and WriterEpoch values. Workspace migration adds a secret-free format-1 client journal and an exact recoverable backup of the prior .jj/repo descriptor; repository formats remain unchanged.
<!-- /evidence-field -->

