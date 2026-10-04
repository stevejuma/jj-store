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
