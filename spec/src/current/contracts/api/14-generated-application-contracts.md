# Generated application contracts

E-CI-SUITES-001 preserves JST-API-006: the independently dispatched API suite
still regenerates contracts and validates the same actual HTTP captures. Suite
separation, artifact verification and timing reports change qualification
orchestration only; transport schemas and assertions are unchanged.

The HTTP route inventory also generates OpenAPI 3.1. JSON schemas derive from
the exact owned Rust request and response types, using deserialization rules for
requests and serialization rules for responses. Optional request fields are
distinct from required nullable response fields. Native metadata schema names
are separate from graph presentation names. IDs remain canonical lowercase hex
with their exact widths; tuple arrays and tagged unions retain their wire form.

The generated document includes Bearer security, explicit snapshot selection,
query defaults, index-task bodies, errors, binary ranges, HEAD semantics and SSE
framing. Event data schemas describe JSON inside SSE frames, not the stream as a
JSON response. Heartbeats are comments. TypeScript types derive from this
document and are checked for drift together with it. Generation is offline once
locked build dependencies have been installed and requires no credentials or
server. Generated files are not edited by hand.

Contract shape validation does not establish runtime authorization or native
semantics; the existing transport/lifecycle tests remain required. Rust numeric
widths are retained in JSON schemas. Browser clients must reject unsafe integer
conversion or use lossless parsing when native values exceed JavaScript's exact
integer range; generated TypeScript number types alone do not guarantee this.

Browser presentation preserves empty native descriptions rather than replacing
them with an invented commit message. The working-copy view renders the explicit
`(empty) (no description set)` label while retaining the full native commit and
change identities. Repository landing-document ordering, friendly labels,
responsive overflow, hidden-document selected treatment and selected-document
edit routing, bookmark-filter empty states and no-file diff states are client
presentation rules and do not change the read DTOs or transport format. The
responsive commit-details action and progressive commit-metadata hiding are also
client presentation; the action selects the Commits page before the same
complete commit identity. The content-sized breadcrumb filename field,
line-numbered Mira MDE edit body, focus treatment and filename-selected source
language do not add file-content fields. Static application focus styling
suppresses the native editable-content outline in file editors and Community
comment, reply and mutation composers under the served asset policy. It does not
require admitting injected editor styles or alter edit/submission semantics.
Directory and root navigation clears the file edit presentation and its
workspace guard; an empty selected file path does not initiate editing or
publish a mutation.
