# Pinned history query ownership

History query metadata, pinned standard aliases and bounded operation-filter
parsing belong to `jj-store-jj`. Native and filesystem graph readers consume
these owned interfaces through the existing service admission and read-session
boundaries. Filtering precedes pagination and charges scanned operations without
rewriting their parent identities. Query execution never publishes repository
state. The browser controller owns cancellation, operation/time pinning and
accepted-query routing; Design Core owns the generic custom-language editor.

M33 separates shared read budgets, diagnostics, statistics, cancellation and
worker admission into storage-independent internal modules. Existing public
native/local query paths retain type identity through re-exports. Preparation
and observation preserve their original non-queuing slots and shared serving
instance. The move grants no read pin, source admission or publication method.
