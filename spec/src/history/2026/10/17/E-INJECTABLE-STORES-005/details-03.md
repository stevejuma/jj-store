---
schema: 1
id: E-INJECTABLE-STORES-005
---

# E-INJECTABLE-STORES-005 details 3

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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test native_lifecycle http_collaboration --locked --message-format=short
```

**Notes**

3Agent/review/checkpoint HTTP regressions passed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test native_lifecycle http_work --locked --message-format=short
```

**Notes**

1WorkStore HTTP regression passed, including command/page/permission/status contract samples.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test native_lifecycle http_threads_reactions --locked --message-format=short
```

**Notes**

2Thread/Reaction HTTP regressions passed: exact targets, assets, signed roots, receipts, refusal and unknown identity.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test agent_execution repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart --locked --message-format=short
```

**Notes**

1personal-home/conversation lifecycle regression passed. The server default local Work/thread factories retain isolated homes and no prompt replay after reconstruction.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-git -p jj-store-native -p jj-store-service -p jj-store-rpc -p jj-store-capture -p jj-store-http -p jj-store-server -p jj-store --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

All directly affected package and test targets passed strict lint. The later private facade conversion refinement is separately checked below.
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
CARGO_INCREMENTAL=0 node scripts/contracts.mjs --write; jj --no-pager diff --name-only -- schemas/openapi.json web/src/lib/api/generated.ts
```

**Notes**

Generated OpenAPI and TypeScript are exactly equal to the tracked originals:0changed generated paths. No public wire/schema drift.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test local_work factory_facade_replays_sync_receipts_and_keeps_semantic_errors --locked -- --exact
```

**Notes**

Final exact Work facade regression passed1 after preserving the old low-level Git read conversions and adapting semantic command errors only at the service facade.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test local_agent --test local_review --test local_collab --locked --message-format=short
```

**Notes**

Final45direct local consumer cases passed (Agent21/Review8/Collab16) after the private facade conversion refinement. Low-level Git immutable-read error behavior remains unchanged.
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
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

Progress162tasks/855records,31protected files/7owning chapters and7validators/4117chapters/206requirements passed after updating the actual governance owner.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/governance.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

38passed/1failed because this validation pins the working copy and the concurrent fresh-evidence writer changed it. The guard correctly rejected a moving validation source; reran the exact39tests with tracked files frozen.
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
node --test scripts/governance.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

Exact rerun39passed with tracked files frozen; migration archives and revision-pinned validation remained intact.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Final private service facade conversion and its direct source dependencies passed strict lint after preserving lower-level Git read behavior.
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
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

Final migration ledger162tasks/855records,29protected files/7owning chapters,7validators/4118chapters/206requirements passed. Changed Rust formatting and Markdown/JavaScript/JSON formatting checked; generated contracts unchanged.
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

macOS ARM64; pinned Rust1.99/JJ0.45.1/Git2.55.0; workspace libraries0.1.0. Cold qualification uses the staged embedded runtime identified by SHA256; its source manifest declares RustFS1.0.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API/CLI/schema or authoritative disk format changes. Existing native AgentDeltaRequest, opaque results, hashed publication IDs, canonical original receipts and Git companion ref/record formats are retained exactly.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Application native/local command and status facades now consume independently named store factories. Fresh session principal/epoch policy remains service-owned; LocalTrusted folder admission and filesystem actor provenance remain owned. Native Agent preparation retains original session-clock precision; other defaults retain system millis. Synchronous compatibility and exact historical read capabilities remain. Typed Invalid/NotFound/review-state errors pass through; unavailable selected native receipt records retain uncertainty rather than becoming target errors. Real RustFS cold replay and the affected CLI/RPC/HTTP/capture consumers pass. Low-level Git read error maps are unchanged; private service facade adapters retain semantic command errors.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified application migration. Complete the independent standalone Cargo consumer, dependency profile audit and detailed SDK guide, then restore the retained Vite development stack. M32 overall remains In progress until that final qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-006 gains application composition and precise
  unavailable-receipt adaptation through new public library handles, with owning
  architecture/mutation/local/CLI/API/compatibility updates. Existing
  publication identities/authority, durable receipt bytes, domain semantics,
  transport contracts and LocalTrusted boundaries remain unchanged.
  Direct-consumer regressions and original fault/replay checks qualify the
  refactor without inventing native identities.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M32.4 is In progress for this now-verified application migration. M32
  and provider stages retain their pending final independent embedding
  qualification; their statuses are unchanged in this slice. Real embedded
  RustFS cold evidence is recorded separately from MemoryStore fault and library
  interface checks. No new platform or release qualification is inferred.

```
<!-- /evidence-field -->

