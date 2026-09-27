# Diagnose an existing repository

`jj-store doctor --config PATH --repo-id ID` checks repository authorization and
selected operation heads through the in-process service and configured RustFS.
It uses `JJ_STORE_TOKEN` or the OS credential for the configuration's public
URL. It publishes no operations and does not reconcile multiple heads.

A successful result establishes access to current authoritative metadata.
Adapters report separately; this is not a full retained-history integrity audit.
