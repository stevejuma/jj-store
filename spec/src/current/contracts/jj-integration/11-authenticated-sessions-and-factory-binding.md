# Authenticated sessions and factory binding

The service provides one repository-bound authority for all three native stores.
Native head-publication IDs are domain-separated hashes of repository,
principal, removed heads and added operation. An identical transition recovers
the same receipt after process restart without a volatile sequence counter.
Different transitions within one CLI invocation receive distinct identities.
Unknown outcomes remain errors under that original identity.

Factories bind local metadata to the repository and native profile using bounded
credential-free marker files. Derived JJ indexes and physical working-copy state
stay in each client's cache and checkout. Attaching uses the selected current
operation and a supplied unique logical workspace name; it never starts an
existing repository's workspace transaction from the virtual root. Explicit read
loading uses load_at, while authorized mutation loading may reconcile heads.

The in-process authority performs real token and semantic validation. Transport
and executable command integration remain M3 gates; library lifecycle acceptance
alone cannot complete them.

Native store futures and returned streams enter a captured caller-owned Tokio
runtime for each poll. JJ's filesystem worker threads do not inherit a reactor;
enter guards must not survive an await or move between threads. The caller keeps
a multithread runtime alive for the session: synchronous upstream reads may
block their calling thread while I/O drivers must keep running. Factories reject
a single-thread runtime. Cache initialization writes only local store bindings
and derived-store metadata; it avoids upstream initialization paths that assume
root reads cannot fail authorization.
