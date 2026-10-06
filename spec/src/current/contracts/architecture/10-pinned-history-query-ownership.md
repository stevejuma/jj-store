# Pinned history query ownership

History query metadata, pinned standard aliases and bounded operation-filter
parsing belong to `jj-store-jj`. Native and filesystem graph readers consume
these owned interfaces through the existing service admission and read-session
boundaries. Filtering precedes pagination and charges scanned operations without
rewriting their parent identities. Query execution never publishes repository
state. The browser controller owns cancellation, operation/time pinning and
accepted-query routing; Design Core owns the generic custom-language editor.
