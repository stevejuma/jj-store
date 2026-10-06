# Search HTTP transport

Configured repositories expose GET /v1/repos/{repo}/search and GET
/v1/repos/{repo}/search/status. Capabilities advertise search per repository;
unconfigured repositories return search_not_configured after authorization.
Search requires q and accepts explicit mode, at_op, scope, page_size,
minimum_position, wait_millis and cursor. Unknown or duplicate fields fail.
Historical mode is explicit and cannot also select at_op. Continuations retain
all original query inputs and their selected native view and index generation.

Search shares HTTP admission, response limits and native content workers with
other read routes. Search pages and status report indexed and repository
positions separately. A bounded freshness wait that cannot reach its target
returns 409 index_lag with currently authorized search_status. Expired or
restarted cursor state returns cursor_reset; multi-head queries require explicit
selection. Current authorization precedes lookup and parsing of search inputs;
revoked access returns no freshness, counts or index metadata. These GETs never
publish history. Generated contracts include responses and errors. M4.15 wires
the browser command overlay to this existing endpoint with selected-operation
queries and opaque in-memory continuations; it introduces no new transport
shape.

Browser native-ID controls derive their compact labels from the complete DTO
identities, preserving the suffix rather than using caller-truncated labels.
Clipboard actions retain the exact underlying identity; the split-control
presentation and interaction contract is owned by JST-UI-003.

Paired native-ID labels default to four leading characters alone. Hover or
keyboard focus reveals the longer prefix, two dots and final two characters. A
single identifier shows that longer abbreviation immediately; configuring the
display never changes clipboard values.
