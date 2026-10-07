# Installation agent settings and compute

JST-API-001 extends installation-local LocalTrusted Admin APIs with conditional
agent settings, write-only device-secret input, compute lifecycle and read-only
sent-context projections. Browser requests never select executable commands or
MCP server definitions. Setup options default to non-inheriting for old
journals/requests. OS secret references are scoped to this installation;
external references resolve only at trusted launch. Saved secrets are not
readable through HTTP. Secret custody remains outside ACP, repository metadata
and browser persistence.

Catalog discovery may select an exact saved setup identity. It uses that setup’s
effective provider/environment; unsaved agents use global defaults until saved.
The cache follows effective configuration and compute generations. Probing opens
no coding tools and sends no message.
