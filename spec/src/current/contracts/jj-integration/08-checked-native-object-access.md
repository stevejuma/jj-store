# Checked native object access

Native object access verifies the expected kind/ID, envelope checksum and
decoded identity before returning metadata. Immutable-create collisions load and
compare the complete existing native value. File staging and verification stream
with 64 KiB buffers and configured disk limits. A file reader is exposed only
after full checksum and BLAKE2b verification. Unknown upload outcomes remain
explicit unless a subsequent read proves the identical immutable object is
present.

Dependency enumeration covers all conflict terms, nested remote tags/bookmarks,
per-workspace Git heads, logical working-copy commits and both predecessor
representations. The native object accessor implements the WAL inventory source
using checked reads and explicit virtual-root values, including the empty tree.

The real-storage integration fixture publishes complete native operations
through the guarded coordinator and verifies retained native dependencies after
a fresh process and storage restart. The fixture explicitly permits its
disposable test principal; production transports must provide the shared
authorization/semantic guard and cannot instantiate an implicit permissive
coordinator.
