# ChatGPT plan provider

JST-COLLAB-034 through JST-COLLAB-036 extend M25 with an explicit `chatgpt-plan`
provider for the existing `local` JJ Agent runner. The initial profile is macOS
LocalTrusted, including a same-machine browser. Multiple account/workspace
registrations have independent opaque references; defaults and per-agent
selections retain conditional configuration writes. Model and effort inheritance
requires the same provider and account. Changes require an explicit worker
restart; renewable credentials do not restart a conversation.

The application owns OAuth authorization, JWKS-validated identity, account
selection, Keychain custody, serialized rotating refresh and revocation. A
stable installation host ID and issued per-registration client ID survive
restart. Access, refresh and ID tokens never enter browser responses, journals,
repository objects, process arguments or diagnostics. Authorization requires an
explicit user action. Passive model discovery never opens a browser. The fixed
system browser helper is an admitted LocalTrusted authentication action. The
short-lived callback listener binds literal 127.0.0.1 and validates fresh state,
nonce and PKCE with the exact redirect URI; it is separate from admitted
application HTTP routes. Pending authorization never replaces a valid account
before identity, account binding and granted scopes have been validated.

The owned worker token source uses a private, account-bound and generation-
fenced Unix-domain broker. Refresh/ID tokens stay with the application. Tokens
and broker capabilities are stripped from tools and registered for exact
in-memory redaction before release, including renewed tokens. Account sign-out
invalidates new requests and dependent workers, cancels pending reauthorization
even for disconnected accounts, clears local tokens and reports unconfirmed
remote revocation. Interrupted work is never automatically replayed.

Subscription requests use the public Responses endpoint, store=false and
stream=true, explicit bounded history and namespaced local function tools.
Unsupported subscription parameters and hosted tools are omitted. Completion,
compaction and summary calls share this dialect. SSE is consumed internally;
only response.completed permits completed answers or complete tool execution.
Failed/incomplete/cancelled/truncated streams remain failures. Opaque reasoning
and replayable output stay in bounded worker memory, separate from canonical
sanitized publication. No automatic billing fallback or replay of uncertain
inference is allowed. A confirmed pre-stream 401 permits one token refresh.

Account-specific models preserve exact slugs, server ordering and visibility.
Empty catalogs remain empty; unadvertised effort choices are omitted. Settings
and agent drafts show the account, plan usage, explicit Continue with ChatGPT,
reconnect/sign-out and usage recovery. Drafts and pending request IDs survive
refresh. Credentials do not pass through the web UI. Linux, remote login,
commercial hosting and live text streaming are deferred. Fixture acceptance is
separate from authenticated provider and signed desktop qualification.
