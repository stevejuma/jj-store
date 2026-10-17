# jj-store

Command-line entry points for the jj-store application. The `jj-store` binary
owns installation and server administration; `jj-store-jj` is the collision-safe
custom JJ client with application command families. The Rust library target is
named `jj_store_cli`.

Start with `jj-store --help` or `jj-store-jj --help`. Commands choose native
service or local Git companion behavior according to the repository context.
Complete native bundles also include browser Help and the embedded storage
helper; an existing upstream `jj` installation is preserved.

See [installation](../../docs/site/getting-started/install.md),
[repository contexts](../../docs/site/getting-started/contexts.md) and the
[CLI reference](../../docs/site/cli/index.md).
