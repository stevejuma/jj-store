# jj-store embedding example

An independent Cargo consumer that composes the collaboration libraries with
stock `jj-lib`, replaces one WorkStore with its own implementation and exercises
Git/native command and original-receipt behavior. It has its own workspace and
lockfile and depends on no application service, UI, search or runner package.

The default `stock-jj` feature creates a real stock JJ Git repository and uses
the focused companion integration. Disable defaults for plain Git; add `native`
for the manifest/WAL example. The native example uses deterministic memory
storage; the repository's cold embedded-RustFS test supplies real-storage
qualification separately. All example data and admitted identities are
synthetic.

From the repository root, use the managed Cargo wrapper with this manifest:

```sh
pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked
pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --no-default-features
pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --features native
pnpm dev:cargo -- run --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --features native
```

See the [embedding guide](../../../../../docs/injectable-stores.md) for the full
API, host-admission, publication and recovery context.
