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

Typed commands use `POST /v1/repos/{repo}/mutations/preview`,
`POST /v1/repos/{repo}/mutations`, and
`GET /v1/repos/{repo}/mutations/{request_id}`. All require current write
permission. The request is `{ version: 1, request_id, expected, command }`;
`expected` contains the exact epoch, position and heads. Use full IDs, a fresh
request ID for a new intention, and the same complete request for every retry.
Capability discovery lists the currently supported command tags.

Previews return provisional results without publication. Apply returns
`committed` with its durable receipt or `outcome_unknown` with the request ID.
Status returns `committed`, `outcome_unknown` or `not_known`; the latter is not
proof that an in-flight request cannot commit. These tagged outcomes use
HTTP 200. Stale snapshots and changed intent are distinct HTTP 409 errors.
Always retain the original request ID across network, deadline and delivery
failures. Browser controls for these commands remain deferred.
