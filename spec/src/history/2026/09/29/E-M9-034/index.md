---
schema: 1
id: E-M9-034
---

# E-M9-034

<!-- evidence-field:id markdown -->
## Id

E-M9-034
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

representative-collaboration-demo-fixture
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/capture.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- scripts/dev-demo-agent-reviews.mjs
- scripts/dev-demo-agents.mjs
- scripts/dev-demo-collaboration.test.mjs
- scripts/dev-demo-repository.mjs
- scripts/dev-demo-reviews.mjs
- scripts/dev-demo-search.mjs
- scripts/dev-demo-services.mjs
- scripts/dev-demo.mjs
- README.md
- docs/browser.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/agent_catalog.rs#normalized_capture_stages_one_root_pinned_publication_delta
- scripts/dev-demo-collaboration.test.mjs#representative agent seed covers
  integrations, tools, transcripts, and lifecycle states
- scripts/dev-demo-collaboration.test.mjs#agent seed installs, publishes, and
  removes each owned integration
- scripts/dev-demo-collaboration.test.mjs#review seed exercises every command
  and each supported sample anchor category
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test agent_catalog normalized_capture_stages_one_root_pinned_publication_delta --locked -- --exact
```

**Notes**

The exact regression proves two accepted batches replace the selected transcript manifest with one cumulative immutable closure, preserve ordered normalized message sequences 2 and 10, and keep the earlier root independently readable.
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
node --test scripts/dev-demo-state.test.mjs scripts/dev-demo-collaboration.test.mjs
```

**Notes**

All development-state and collaboration seed regressions pass, including all three supported integrations, transcript and tool activity, every ReviewStore command, and all seven typed review targets.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

The first aggregate check rejected the expanded scripts/dev-demo.mjs because it exceeded the 300 counted-line script budget. The fixture orchestration, collaboration seeds, search readiness and service helpers were split into narrow owned modules rather than weakening the architecture limit.
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
pnpm check
```

**Notes**

Specification validation, package boundaries, the repository architecture scan, progress validation, Rust formatting, embedded RustFS and generated contracts, web/catalog diagnostics and locked workspace Clippy all pass after the structural split.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test
```

**Notes**

The first aggregate test exposed that serde_json workspace feature unification could preserve insertion order and make SanitizedPayloadV1 JSON bytes depend on the dependency graph. The sanitizer boundary now recursively sorts object keys before encoding.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

The pre-existing captured browser lane still drives the removed Repository ID field in web/tests/browser/native-browser-fixture.ts, navigation.spec.ts and reads.spec.ts; the current manual-token discovery application exposes Access token. This failure is outside the collaboration fixture paths and remains explicit.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=.local/dev-demo-collab-test JJ_STORE_DEV_PORT=7461 DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The first collaboration reset exposed a stale repositoryId variable in initializeFixture; the initialized repository identity is now passed explicitly.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=.local/dev-demo-collab-test JJ_STORE_DEV_PORT=7461 DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The first review seed sent the display change ID to a native revision anchor; the fixture now requests change_id.normal_hex() so the exact 16-byte identity validates.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=.local/dev-demo-collab-test JJ_STORE_DEV_PORT=7461 DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The collaboration-heavy cold bootstrap remained search-degraded with the original 30-second worker deadline; a reordered code-only bootstrap reproduced the same limit. Temporary diagnostics identified the bounded adapter read context as Error::Limit.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=.local/dev-demo-collab-test JJ_STORE_DEV_PORT=7461 DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

A clean isolated reset with the 120-second search worker deadline and five-minute readiness window publishes through the production agent hook and ReviewCommand paths, then reports ready at repository, durable and indexed position 72 without stale or degraded search.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
isolated HTTP and production-browser collaboration inspection on http://127.0.0.1:7461
```

**Notes**

The root-pinned APIs and built application render Codex ended, Cursor idle and OpenCode active with one transcript and three tools each; Cursor reconstructs sequences 2 and 10. Reviews render seven target variants, open and resolved states, visible, tombstoned and hidden comments, and the exact agent-text selection.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test
```

**Notes**

All governance, storage, embedded and managed RustFS, LocalTrusted, contract, web, catalog, Storybook, cold catalog-browser and complete locked Rust workspace tests pass with the canonical JSON repair and representative fixture regressions.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Storybook, the production browser bundle, pinned embedded RustFS and the complete locked Rust workspace build successfully. The build retains the existing Svelte global-selector and large-bundle warnings.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The owned interactive sample on port 7460 was replaced with repository ab1ae3ec9357ca35709d5bfc527317f5 and left running. Repository, durable and indexed positions are all 72; search is current, healthy and not degraded.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
root-pinned HTTP and production-browser collaboration inspection on http://127.0.0.1:7460
```

**Notes**

The real sample renders Codex ended, Cursor idle and OpenCode active with one transcript and three tools each; Cursor renders cumulative messages at sequences 2 and 10. Seven review threads cover every supported target, open and resolved workflow states, edit, tombstone, moderation hide and reopen history, and the agent-text discussion displays its exact sanitized selection.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-034 details 2](details-02.md)
