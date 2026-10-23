---
schema: 1
id: E-STORY-DOC-REVIEW-001
---

# E-STORY-DOC-REVIEW-001

<!-- evidence-field:id markdown -->
## Id

E-STORY-DOC-REVIEW-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-24
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Review and repair stale application documentation fingerprints found by the cold Docker browser build. Document already-committed image preparation, transient snapshots, private physical status, Forgejo embedding and pinned history lifetime; record the shared application/catalog architecture.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Documentation maintenance only. No Rust implementation changed. This does not qualify live/native providers or release gates. Compact browser navigation retains its labelled landmark around the relocated toolbar; the application architecture page and source review include this repair.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-DOC-001 remains unchanged. The manuals catch up to the original
  public source contracts and current shared application composition, without
  new product capability, runtime settings or qualification claims.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress with independent hosted, release, provider,
  cross-platform and visual gates.

```
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain independent hosted, release, platform, provider and visual qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/concepts/collaboration.md
- docs/site/reference/configuration.md
- docs/site/technical/capture.md
- docs/site/technical/architecture.md
- docs/site/guides/git.md
- docs/site/technical/local.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/docs.test.mjs#source changes and new files invalidate an acknowledged
  topic
<!-- /evidence-field -->

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
pnpm web:test:browser
```

**Notes**

Docker ecf86141-4dea-43a2-af5b-77aa94a45fa2 built the UI but stopped before browser/Rust fixture execution on a stale collaboration documentation review. Source inspection found seven stale fingerprints; six predate this slice.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --from d824a18f --to @ -- crates/jj-agent/src crates/jj-collab/src crates/jj-review/src crates/jj-work/src crates/jj-workflow/src
jj --no-pager diff --from 8a487398 --to @ -- crates/jj-store-capture/src scripts/ci-redaction-source.mjs scripts/agent-redaction-source.json
jj --no-pager diff --from 2a479c74 --to @ -- crates/jj-store-service/src crates/jj-store-rpc/src
jj --no-pager diff --from 165041c2 --to @ -- crates/jj-store-server/src
jj --no-pager diff --from 168f67b0 --to @ -- web/src
jj --no-pager diff --from 1a5d8cbc --to @ -- crates/jj-forge/src
jj --no-pager diff --from f58b9363 --to @ -- crates/tauri-plugin-jj-store/src crates/tauri-plugin-jj-store/build.rs crates/tauri-plugin-jj-store/permissions/default.toml crates/jj-history-service/src
```

**Notes**

Reviewed collaboration d824a18f (shared bounded raster normalization); capture 8a487398 (immutable 64 MiB transient snapshots and sanitized observations); services 2a479c74 (complete physical status and worker drain); configuration 165041c2 (server image delegation); forge 1a5d8cbc (explicit Forgejo PAT and scoped secrets); history f58b9363 (drained scope retirement and pinned point reads). Application manual reviewed from 168f67b0 shell adoption through current shared-root extraction.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

Docker 6fc016e7-84b4-40e1-8091-6c6f552a3751: strict documentation build passed; nine offline diagrams, 144 files /26731183 bytes, valid links and anchors.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/docs.test.mjs
```

**Notes**

Eight documentation tests passed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:test:browser
```

**Notes**

Docker 50bfd99b-24ef-4c92-ad7e-99e376ebc263: fresh production UI/documentation build and all 20 Help cases passed in Chromium and WebKit. Managed preview preparation finishes before readiness is timed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:test:browser
```

**Notes**

Final Docker 2706ece1-fd34-4944-9041-ea5291637618: fresh complete UI/documentation build and all 20 Chromium/WebKit Help cases passed after the source and documentation repairs.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Existing image/capture/history/forge public versions and runtime configuration remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, storage, credential, capture-progress or publication format changes.
<!-- /evidence-field -->

