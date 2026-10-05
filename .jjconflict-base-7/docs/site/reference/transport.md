# Transport semantics

The application exposes HTTP under `/v1` and native RPC through the configured
connection. [HTTP reference](http-api.md) is generated from the registered
OpenAPI contract. Native RPC serves the custom JJ integration; both transports
consume owned service interfaces and share authorization and semantic policy.

## Read sessions and pagination

Open a read against an explicit authorized selection. Files, graphs and
collaboration catalogs must retain their appropriate code operation and domain
root. GETs never reconcile native heads. Local filesystem selections also bind
the device binding generation and independent companion metadata commit.

Pagination is bounded. Native continuations are opaque and signed for their
query/snapshot/access context. Local CLI continuations can require explicit
metadata selection. Do not interpret cursors as row offsets or carry them to
another repository, credential, query or backend.

Cold prepared-history requests return `409 index_rebuild_required` with the
exact native `snapshot` or local `local_snapshot`. Start/status/cancel
index-task routes remain authorized for that repository and selected operation.
Local tasks also bind the current registration generation. Cancel has no request
body and an interrupted observation is not confirmed cancellation. Keep
observing the captured task until its terminal status; never retarget it to a
newer selection. These tasks build derived indexes and do not publish code.

Local `stack_history` queries retain an exact operation, head and optional base.
Their opaque continuation belongs to that range and page size; raw internal
anchors are refused. The client checks the nested range as well as the envelope
and refuses oversized or non-advancing pages. This additive read variant grants
no stack mutation or physical checkout capability.

## Mutations

Typed commands carry a stable request identity, exact intent and applicable
expected versions. The authenticated actor is derived by the service; caller
labels cannot assign authorship. Preparation/preview and publication are
separate where the command contract requires them.

Responses distinguish committed/replayed results, conflicts or rejection, and
unknown outcomes. Preserve the original request after a timeout. Status is
currently authorized receipt inspection. `not_known` does not establish that
publication failed. Do not build an automatic retry loop that changes the
request ID after each timeout.

## Events and derived data

Committed SSE events are freshness hints. The client reopens authorized reads
rather than treating an event payload or a search hit as repository authority.
The advertised `/v1/events` feed multiplexes authorized repository and
installation domains into one session subscription. It replays a bounded
process-local cursor or sends reset when continuity is lost. Heartbeats arrive
every 15 seconds and streams renew after five minutes; unchanged renewal causes
no data reads. The client marks affected scopes dirty before accepting the
cursor and refreshes only visible consumers. Hidden scopes refresh when
accessed. Serial bounded fallback runs only when the feed is unavailable and
pauses while hidden. Historical code pins, unsaved drafts and unknown command
identities survive ordinary refresh.

Use lossless JSON handling for integers beyond JavaScript's safe number range.
Identity widths and tagged variants are part of the schema. Uploaded bodies,
query work and page output have explicit caps; exceeding one is not a reason to
truncate a typed identity or strip a conflict term.

## Admission and assets

Local requests retain exact loopback origin/protocol rules; shared requests use
current scoped credentials. Static application and Help assets have their own
allowlists. Unknown `/v1` paths never fall back to HTML, and filesystem paths
from a request never select arbitrary static files.
