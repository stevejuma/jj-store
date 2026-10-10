# Credentials and access

## LocalTrusted

LocalTrusted is restricted to an explicitly configured local installation. The
browser uses literal loopback HTTP with exact Host/Origin, Fetch Metadata,
protocol-version, method and media-type admission. Native RPC uses an owner-only
Unix socket. Local access still has a stable installation principal and semantic
permission checks; it is not an unauthenticated public-server profile.

Device folder registration is explicit. Knowing a path, repository ID or object
ID is not sufficient authority to read it. Folder dialogs are narrowly admitted
local helpers, not arbitrary shell execution.

## Authenticated repositories

Bearer tokens have public token identities and private random secrets. Policies
retain verifiers and repository-scoped grants with permission/expiry/revocation.
Discovery uses public token identity to find candidates, then validates the
complete credential against each current repository policy. One token can have
independent grants across several repositories.

Read, write and admin permissions are hierarchical within a repository. Creating
a new repository requires a separate operator capability. Linking code into a
project does not grant its readers access to that code repository.

The CLI's login path reads secrets from stdin or a hidden prompt, verifies
access and uses the platform credential facility. `JJ_STORE_TOKEN` overrides
that account for automation. There is no plaintext credential fallback inside
the checkout. LocalTrusted rejects login/logout because it does not use that
bearer credential profile.

## Authorization during long-lived work

Authorization precedes object, history, staging, search, preview and publication
requests and is checked again where publication policy requires it. Old cached
search results or cursors do not bypass revocation. Receipt replay also requires
current authorization; an old request ID is not an access token.

TLS verifies endpoint trust and hostname. Custom CA configuration adds the
explicit deployment trust material. Plain HTTP requires literal loopback and an
explicit setting; do not replace that requirement with broad CORS or disabled
certificate validation.

## Help and browser state

Bundled Help contains application documentation, not repository content. It is
served from an explicit local asset list and can be framed only by its own
origin. The application and API responses remain protected independently. Help
never receives repository tokens. Closing it preserves the underlying view and
in-memory drafts rather than reconnecting the application.

Browser image CSP permits same-origin, HTTPS and data images for Community
avatars. Agent saves accept credential-free HTTPS image URLs, bounded raster
data and the fixed Community emoji SVG template. The server does not fetch
remote avatars. Script, style, connection and frame restrictions remain intact.

## Local models and paired compute

Provider credentials resolve from operator environment references or the desktop
Keychain. Browser drafts remain in memory, and durable configuration contains
only opaque credential references. Resolved values are registered as exact
redactions before transcript capture. Advanced environment settings cannot
override controller-owned launch variables.

Sharing is disabled initially and requires explicit mutual pairing by machine
identity. Peers receive bounded inference access only; files and coding tools
remain on the requesting machine. Revocation closes admitted peer connections
while local inference remains available. Public discovery and automatic cloud
fallback remain disabled.

ChatGPT authorization is an explicit LocalTrusted macOS operation. The temporary
callback binds literal 127.0.0.1, validates state, and exchanges the code with
PKCE and the exact redirect URI. Signed ID tokens must match issuer, issued
client audience, nonce, expiry and the selected account. OAuth bundles stay in
Keychain; browser status and repository data contain no tokens. Model discovery
never opens a browser or Keychain authentication prompt. Sign-out reports
unconfirmed remote revocation separately from local disconnection. Live
subscription and signed desktop qualification remain separate from fixtures.
