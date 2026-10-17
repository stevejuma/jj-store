# jj-store-embedded-rustfs

Paired managed RustFS process used by local native installations. Its separate
Cargo workspace isolates RustFS dependencies and allocator requirements from the
application and JJ libraries.

Select this helper through the server's embedded-storage configuration, or use a
complete native release bundle. The managed host supplies private storage
configuration and owns the helper's lifetime. Its executable is named
`jj-store-rustfs`.

See [storage configuration](../../docs/site/reference/configuration.md),
[process lifecycle](../../docs/site/technical/lifecycle.md) and
[embedded runtime qualification](../../docs/embedded-rustfs-qualification.md).
