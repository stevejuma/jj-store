# jj-store-model

Storage-independent identities and integrity framing shared by native JJ objects
and collaboration records. Distinct types preserve repository, operation, view,
commit, object and request identities without remapping them.

Use the exported ID types at library boundaries, `portable` for qualified
cross-provider references and `envelope` for bounded object framing and
integrity checks. Domain-specific commands and persistence belong in their
owning crates.

See [object storage and identity](../../docs/site/technical/storage.md) and
[codec coverage](../../docs/codec-coverage.md).
