# Durable command service

The service's typed request includes a version, client-generated request ID,
exact expected snapshot and a validated command. Its principal comes only from
current repository write authorization. Canonical intent includes all original
request fields under the application WAL intent profile; native preparation
results and timestamps do not alter that identity. Publication IDs are derived
from repository, principal and request ID under a distinct application domain.

Preview prepares a bounded native impact report, then applies the same semantic
protection and dependency checks as publication without writing authority. Its
native result IDs are provisional; apply recomputes native execution at the same
snapshot and revalidates authorization and semantics. An already committed
identical request returns its retained receipt. Preview never grants permission
or reserves an operation.

Apply looks up a currently authorized receipt before executing and publishes the
candidate and exact result with one guarded strict manifest CAS. Racing
identical applications return the selected result, including if preparation
discovers the snapshot has moved. Different intent under the same identity fails
explicitly. A selected result retains its versioned original request and native
impact report so status/retry can reconstruct and check the canonical digest
after restart or checkpointing, without an execution cache or separate
intent-binding authority.

Status authenticates current write permission, resolves only the same
principal's request key and validates the selected result's repository,
identity, digest and accepted snapshot. No selected receipt is NotKnown, never
proof of rejection. Transport/storage uncertainty returns an unknown outcome
with the original request ID; semantic rejection remains an explicit error. The
HTTP mutation endpoints consume these service methods; native RPC continues to
use its guarded native publication facade. Browser work remains deferred to
M4.3.

The HTTP boundary consumes the durable command service for preview/apply/status.
Current write permission precedes body decoding and is checked again by the
service. HTTP admission and native command preparation share bounded worker
capacity with reads. Tagged outcomes retain the original request ID across
unknown transport outcomes; status is scoped to the authenticated principal. No
browser mutation controls are introduced by this transport slice.
