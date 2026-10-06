---
schema: 1
id: E-M10-004
---

# E-M10-004 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm build
```

**Notes**

Storybook, web and cached embedded RustFS builds pass; the final Cargo workspace build stops at the unavailable cmake prerequisite for Kingfisher Vectorscan.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M10.4 source acceptance is implemented; its repository-wide cmake-blocked
  lanes remain a dependency before M10.5 can be complete.
- Retain the complete forward, reverse and duplicate delivery matrix for all
  mutable AgentStore catalogs and ReviewStore tombstone/edit and reply/reply
  races.
- The repository-wide contracts, check, test and build lanes that compile
  Kingfisher Vectorscan require cmake on this host.
- M10.6 bootstrap and migration and M10.7 worker, outage and real RustFS
  qualification remain outstanding.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; AgentStore, ReviewStore, origin and apply-origins schema v1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Origin envelopes now retain typed AgentDeltaV1 and ReviewDeltaV1 accepted facts. AgentStore adds an internal catalog-conflict object kind. ReviewStore adds conflict, causal event-graph and merged-thread forms under existing catalog reference kinds; comment revisions omit empty supersession sets so prior canonical bytes remain unchanged. HTTP/OpenAPI read and command contracts expose conflicts and causal parents.
<!-- /evidence-field -->

