---
schema: 1
id: E-EXEC-004
---

# E-EXEC-004

<!-- evidence-field:id markdown -->
## Id

E-EXEC-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-008
- JST-ARCH-001
- JST-ARCH-004
- JST-DOC-002
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-server/Cargo.toml
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/avatar.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-cli/tests/agent_execution/setups.rs
- schemas/openapi.json
- docs/ui-provenance.json
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/browsing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/security.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-browser-ui.mjs
- scripts/dev-ui-config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agents-route.test.ts#installation Agents routes round trip without a repository
- web/tests/vite/agent-execution-controller.test.ts#stopping an agent targets its session without changing another chat or replaying a prompt
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
pnpm check:architecture
pnpm check:boundaries
pnpm check:ui-ownership
```

**Notes**

1243 source modules, Rust dependency boundaries and complete owned Community provenance passed.
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
pnpm docs:generate
pnpm docs:review application "Shared complete Community agent editor; workspace runtime rows show associated configured agent with unassigned-session fallback; conditional default save then exact connected-session configure; titled copy/link settings and grouped actions"
pnpm web:build
```

**Notes**

Generated API/manual references and fresh source review; 141-file offline documentation closure staged with valid links/anchors.
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
cargo check -p jj-store-server -p jj-store-http -p jj-store-service --locked
cargo test -p jj-store-http --test assets --locked
```

**Notes**

Affected compile boundary and 9 asset tests passed. No native mutation/publication implementation changed.
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

Same versions and enclosing change ywvxqomz as E-EXEC-003. Community source 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0; released Design Core and Mira remain foundational dependencies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No source-ownership, package-resolution, governance, CLI execution or native storage/publication format changes. Dependency additions support bounded presentation decoding only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Protected maintenance, architecture, service and CLI paths associated with E-EXEC-003. Final 44 production stories, 40 controller/direct-consumer tests, the built cold browser/restart flow, source ownership and source-budget checks passed. The composer uses the existing copied Community row styles and released Design Core scroll control; no foundational source changes or baseline updates. Required UI/API/collaboration/security behavior changes are specified in their owning chapters; this confirmation covers unchanged foundational invariants, manual generation, owned source copies and private test-harness organization.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain owned copies and provenance, generated contract/manual checks, explicit development executables and the targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-GOV-002/JST-GOV-008 still prohibit sibling source/package dependencies and require same-change evidence. JST-ARCH-001/JST-ARCH-004 keep native boundaries, production independence and counted source budgets. JST-DOC-002 still derives contracts and invalidates source reviews. JST-MUT-001 native receipt/publication semantics, JST-CLI-004 explicit binaries and JST-COMPAT-001 upstream license/native identity boundaries are untouched. The new execution/UI behavior is separately governed by E-EXEC-003 and its updated chapters.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 maintenance confirmation adds no new completion gate and does not waive E-EXEC-003 lifecycle qualification. Other milestone statuses are unchanged.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


