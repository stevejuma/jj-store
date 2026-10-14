# Custom data read projection boundary

JST-ADAPT-008 adds service-owned source materialization and secondary lookups.
This is an unchanged-contract confirmation for JST-WORK-005/006/009,
JST-COLLAB-004/014 and JST-WF-006: reads retain exact manifest roots, historical
publications, grants, ordering, cursor meaning and bounded response envelopes.
Source admission remains independent of derived storage. Mutation guards, native
bytes, durable receipts and publication authority are unchanged.

Shared source data contains no actor/session capability or viewer-filtered
answer. Current membership/visibility and epoch checks remain at each request's
disclosure boundary. Source replacement and admission failure cannot be hidden
by a ready derived generation. Large/unsupported text projections retain the
existing bounded scan fallback.
