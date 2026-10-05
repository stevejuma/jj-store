# Diagnose an existing repository

`jj-store doctor --config PATH --repo-id ID` checks repository authorization and
selected operation heads through the in-process service and configured RustFS.
It uses `JJ_STORE_TOKEN` or the OS credential for the configuration's public
URL. It publishes no operations and does not reconcile multiple heads.

A successful result establishes access to current authoritative metadata.
Adapters report separately; this is not a full retained-history integrity audit.

For configured search, doctor also queries the running server's authenticated
search status route with the same repository token. It reports ready, stale,
degraded, not-ready, access-denied or unavailable search independently of
`storage_ready`. The structured result includes live durable/served positions
when available. Requests have a five-second cap, do not follow redirects and
bound response bytes. A stopped listener therefore leaves search unavailable;
doctor does not open the index or start an indexer. TLS uses normal verification
plus the configured server certificate as a trust root.
