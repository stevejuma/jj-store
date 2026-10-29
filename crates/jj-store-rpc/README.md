# jj-store-rpc

Native repository RPC client, server and wire adapters. It carries object,
operation, collaboration and administration requests through the shared
application services, including owner-only local socket access.

Use `client::NativeClient` and its connection options for a native service, or
compose the `server` module around an authorized service. Keep authentication,
repository binding and original mutation identities with the request; wire
adapters preserve the owned service contracts.

See [transport semantics](../../docs/site/reference/transport.md) and
[native repository setup](../../docs/site/getting-started/native.md).
