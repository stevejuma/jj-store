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

Compute status advertises `model_file_picker` only when the host composes a
native GGUF file picker. `POST /v1/local/compute/pick` accepts empty JSON and
returns a version-1 optional absolute path. LocalTrusted Admin and an available
agent manager are required before any OS interaction. The fixed dialog selects
one GGUF file on the service machine; cancellation returns no path. Busy,
timeout, invalid output and unavailable host are distinct failures. Requests
cannot supply scripts, dialog prompts or filters. Selection changes only the
browser draft; the existing explicit Import command owns copying and validation.
