# Search HTTP integration

The registered Axum routes expose repository-configured search and status under
/v1/repos/{repo}. The transport shares read admission, deadlines, output budgets
and native content workers. Every request binds a current repository token
before query extraction. Capabilities report configuration; readiness and
freshness remain separate search results.

Five native fixtures exercise actual local Tantivy generations and native
publication/authentication services over memory object storage. They cover
pagination across writes, query/cursor changes, server cursor resets, revoked
and cross-repository access, strict inputs, unconfigured repositories, explicit
head selection, historical queries, missing readers, durable-versus-served index
positions, and degraded responses. GET and HEAD leave native history unchanged.

The generated OpenAPI and TypeScript contracts include search pages, status and
index_lag errors. The contract check validates 47 actual HTTP/SSE responses,
including four new search/capability/freshness responses. Generated client types
are API artifacts; no browser feature or visual work was performed.

E-M6-006 in the [single ledger](../../spec/src/milestones.md) records exact
commands and results, including the initial missing-API compiler failure.
Configured server runners, remote snapshots, deployment and performance evidence
remain open. This fixture does not substitute for RustFS/TLS qualification.

Final validation passed all five native HTTP search fixtures, six HTTP contract
tests, normal checks, build, formatting and all 13 specification lanes. The full
test lane passed 344 Rust tests with zero failures and six external tests
ignored, plus governance, harness, contracts and existing web unit checks.
