# Rich history query delivery

`GET /v1/repos/{repo}/query-info?at_op=...` MUST return versioned `QueryInfo`
inside the existing authorized operation-pinned envelope, including JJ version,
callable signatures/insertion text, standard aliases, operators, operation
fields and profile restrictions. `GET /operations` accepts optional metadata
`query` text through the existing signed pagination contract. Malformed draft
queries return HTTP 400 `invalid_query` with a bounded optional `diagnostic`
carrying kind and message; existing non-query error codes and HTTP envelopes
remain unchanged. There is no separate validation request or query publication.

E-UI-CI-001 confirms JST-API-001 is unchanged: retaining an active completion
during graph refresh stabilizes the browser's existing editor vocabulary. It
does not alter query requests, pinned snapshots, diagnostics or generated
contracts. M30 remains In progress pending complete release qualification.
