# jj-store-http

HTTP/SSE application API and browser asset delivery. It exposes shared service
contracts, operation-pinned reads, typed mutations, live events and bounded Help
assets through the application's request-admission policy.

Compose `HttpApi` with its `Config`, access mode and admitted service resources,
then mount it in the HTTP host. Browser Origin, authentication and repository
permissions remain part of that composition. The contract module owns the
OpenAPI description used by generated clients.

See the [API reference](../../docs/site/reference/http-api.md),
[transport semantics](../../docs/site/reference/transport.md) and
[access policy](../../docs/site/reference/security.md).
