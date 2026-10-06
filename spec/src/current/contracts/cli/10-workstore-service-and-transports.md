# WorkStore service and transports

Native jj project and jj task commands read and mutate WorkStore through the
local repository connection. Task/project commands require exact request IDs and
entity/project versions; they are independent of optional upstream connectivity.
For an ordinary Git-backed JJ repository, M13 dispatches the same project/task
commands directly to the LocalRepository service without a server connection.
Reads do not snapshot the checkout, reconcile operation heads or initialize
metadata. Local writes use stable request-derived entity IDs and shared domain
validation; their version-2 receipts/pages identify the Git metadata root.
Native commands retain their existing version-1 transport and output contracts.

Disposable development/browser qualification may explicitly reuse a separately
built pinned RustFS binary through `JJ_STORE_DEV_RUSTFS_RUNTIME`. The server
still checks runtime identity and owns isolated state and process shutdown. This
option does not install or replace a binary or alter a running deployment.

Development reset prepares the full representative fixture and search projection
on an owned temporary instance, then stops that instance. Restart timing
measures a fresh server and storage process opening the persisted fixture, and
stops when the server and existing search index are ready. Initial search
construction and representative sanitizer/review work are recorded as
`fixturePreparationMillis`. The cold runner retains the 30-second startup and
512-MiB resident-memory gates and reports actual measurements on rejection.
Search cache-loss rebuild timing remains a separate storage/search
qualification.
