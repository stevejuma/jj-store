---
schema: 1
id: E-READ-024
---

# E-READ-024 details 3

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
pnpm web:build (after combined reference/manual review)
```

**Notes**

Complete rebuilt app and bundled Help succeed.9offline light/dark Mermaid diagrams;139Help assets/13,630,132bytes, valid links/anchors. web-build-complete.log. Existing large JS chunk warning retained.
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
pnpm docs:test
```

**Notes**

8generator/drift checks plus1actual configuration parser fixture pass. docs-tests.log.
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
node --test scripts/contracts.test.mjs; pnpm exec tsc -p scripts/tsconfig.contracts.json; pnpm spec:validate; pnpm check:ui-ownership; pnpm check:progress
```

**Notes**

7contract-tool tests, generated direct TypeScript consumer,172requirements/23chapters, repository-owned UI boundary and138task bookkeeping pass. contract-tools.log/contract-types.log/spec-validate-final.log/ui-ownership.log/progress.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained owned fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts local-index-cancel.spec.ts -g 'local_repository_opens_files_history_agents_and_accessible_folder_controls|local preparation recovery and settings cancel the captured task'
```

**Notes**

1cancel pass/1browser failure in18.1s on older reused disposable metadata while contract compilation ran. Files, pinned preparation and Op Log passed; Agents body missed the unchanged5s assertion. Trace shows3.798s session request followed by refresh and another unfinished session request. No deadline or test weakened. cold-browser.log; owned server stopped. Fresh standard fixture run distinguishes integration from retained-data/pressure behavior.
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
pnpm contracts:check
```

**Notes**

Complete registered response inventory:254 actual HTTP/SSE responses conform, generated artifacts match,7contract-tool tests and direct generated TypeScript check pass. Earlier inherited capture timeout did not recur in this combined run; no sanitizer or deadline changed. contracts-check.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_FILESYSTEM_PORT=18779 node scripts/local-browser-acceptance.mjs
```

**Notes**

Fresh standard filesystem-only stack:4pass/1fail in1.5min of browser tests after source hook seeding. Files/preparation/Agents/folder controls pass; first fixture later waits for obsolete lower-case Add menu label. Current main uses Add Local Repository, already required by its sidebar-repository-actions browser tests. Exact string updated; no timeout/assertion loosened. Other project/task/comment, review parity, reconnect/removal and physical Save flows pass. Owned server and new fixture cleaned by the standard harness. cold-browser-fresh.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-filesystem-browser-ewxyzA/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts local-index-cancel.spec.ts -g 'local_repository_opens_files_history_agents_and_accessible_folder_controls|local preparation recovery and settings cancel the captured task'
```

**Notes**

Cold isolated server after exact menu assertion update:2pass, files/history/Agents/folder controls in10.8s and captured-task keyboard/accessibility cancellation. Original5s assertion and60s test deadline unchanged; the earlier session delay did not recur without contract compilation. Source code/metadata preservation assertions pass. All six direct cold consumers qualified across this accepted2-test run and the four fresh-flow passes. Owned18778server stopped afterward. cold-browser-final.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm exec prettier --check spec/src/api.md web/tests/real/local-browser.spec.ts (accepted)
```

**Notes**

Final exact menu-consumer repair preserves0type errors/warnings and focused formatting. web-check-accepted.log/menu-assertion-format.log. No production UI or wire change from this final test repair.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj rebase -r qkqknzqv -o nqrxnoyr (main workspace); jj --no-pager st; verify integration ancestry
```

**Notes**

Main remained clean at1528d2f1 through acceptance. Fast-forwarded its existing empty working change onto the integrated23-slice stack, preserving working-change identity. Main is clean and its parent is the integrated change. Bounded-reads stays registered and open. No other workspace moved; user Linux/dev services are preserved.
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
pnpm spec:first; python3 .local/bounded-reads/integration/spec-first.py; pnpm spec:validate; pnpm check:progress; python3 .local/bounded-reads/integration/audit.py; focused Prettier (accepted integration)
```

**Notes**

13protected/4owning chapters in integration;159protected/12chapters across171stack paths;172requirements,138tasks,458evidence objects and exact source preservation. Final source guidance accurately describes installed validator Git revision flags and supported JJ --file qualification. spec-first-*-accepted.log/spec-accepted.log/progress-accepted.log/preservation-accepted.log/final-format.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "The full bounded-read plan remains incomplete: prepared local change
  references and operation/historical lookups, native visibility/annotations and
  eviction, collaboration/search projections and lightweight visible repository
  cards."
- Latest Linux preparation/RSS/release latency,
  native2m/10krefs/deep-page/near-capacity fixtures and
  restart/revocation/publication/fault/real-RustFS qualification remain
  follow-up work. Earlier42min debug Linux preparation is correctness evidence
  for its old policy only.
- Registry-only Design Core acceptance remains separate from the documented
  source-validation setup. Root aggregates, visual baseline approval and
  unrelated runtime/deployment lanes are outside this integration scope.
  Original development services and Linux checkout are preserved.
- Extend collaboration projection/capacity qualification to retained metadata
  under simultaneous compilation/preparation pressure; the initial reused
  fixture missed Agents timing under that pressure, while the final isolated
  cold test passes without altered deadlines. Small cold hook-fixture seeding
  remains slow in the debug CLI.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue prepared local change references and historical/operation lookup, native visibility/cache reuse, collaboration/card projections and large-scale/fault qualification in the retained bounded-reads workspace. Main contains the completed integrated stack; its running dev backend still needs an explicit rebuild/restart for the landed Rust code.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_details_preserve_hidden_rewrites_without_history_preflight
- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks
- crates/jj-store-server/tests/repository_conversion.rs#conversion_http_derives_admin_authority_and_captures_contracts
- crates/jj-store-server/tests/native_creation.rs#native_creation_http_is_local_admin_only_and_captures_contracts
- web/tests/real/local-browser.spec.ts#local_repository_opens_files_history_agents_and_accessible_folder_controls
- web/tests/real/local-index-cancel.spec.ts#local preparation recovery and
  settings cancel the captured task
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Land the23 already completed bounded-read slices after rebasing onto current committed main; preserve conversion, terminal, capture settings and Help. Follow-up workspace remains open. No new endpoint-plan completion or real-RustFS/release qualification is claimed.
<!-- /evidence-field -->

