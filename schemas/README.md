# Application contracts

`openapi.json` is generated from the HTTP route inventory and the Rust types
actually serialized or deserialized by those routes. It uses OpenAPI 3.1 and
JSON Schema 2020-12. `web/src/lib/api/generated.ts` is generated from that
document with the locked, released `openapi-typescript` package.

- Run `pnpm contracts:generate` after changing a route or DTO.
- Run `pnpm contracts:check` to detect drift and validate actual authenticated
  HTTP/SSE fixture responses. This runs without Docker or user credentials.
- Run `pnpm test:contracts` for schema and TypeScript consumer acceptance.

Both normal and specification checks include contract drift and acceptance. Do
not edit generated files by hand. Persisted native metadata remains covered by
the separate frozen envelope and identity tests.

The event endpoint returns `text/event-stream`. Its `x-sse-data-schema`
describes JSON inside data frames; it does not describe the whole stream.
Heartbeats are comments. Clients use authenticated fetch streaming, retain
tokens only in memory, and resume the last fully parsed ID after disconnection.
Binary file responses are attachments and expose their snapshot in headers.

TypeScript represents JSON numbers as `number`. Rust integer bounds appear in
the schema, but a browser must also handle JavaScript integer precision. Native
IDs and SSE IDs are strings and must never be shortened for API requests.

The source-of-truth route prefix is `/v1/repos/{repo}`. There is no public
repository enumeration route: repository selection uses IDs supplied by the user
or deployment. Read routes do not publish or reconcile operations.
