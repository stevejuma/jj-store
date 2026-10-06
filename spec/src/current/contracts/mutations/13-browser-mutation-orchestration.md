# Browser mutation orchestration

The authenticated browser constructs only the existing typed V1 command union.
Preview, apply and durable status use one stable request ID and exact captured
epoch, position and operation heads. A preview is provisional, an unknown
outcome retains the request for retry or status lookup, and only a committed
receipt can refresh the selected snapshot. Disconnect, token replacement and
revocation cancel outstanding work and prevent late responses from restoring
protected state.

Semantic protection is distinct from authentication failure at the HTTP
boundary. A protected bookmark, commit or root transition returns the typed 409
`protected_transition` code and keeps the authenticated session active. Invalid
or revoked credentials remain 401/403 `access_denied`; the browser disconnects
and clears protected cache state for those responses. Both cases publish
nothing.

Strict code mutations compare epoch, code-state version and exact native heads.
The global WAL position remains receipt and ordering evidence but is not itself
a code-staleness predicate, because AgentStore and ReviewStore publications may
advance it without changing native code. Conflict uploads carry the same
code-state token as their captured head, while zero-valued legacy receipt and
upload snapshots retain their original canonical bytes.

Collaboration context discovery remains a read-only service path. Session timing
is derived only from normalized events and their exact selected observation
evidence, with at most 4,096 candidate events inspected; it cannot stage an
object or invoke manifest publication. Browser review replies, resolve and
reopen continue to use the existing receipt-backed ReviewStore command path and
retain outcome-unknown request identities.
