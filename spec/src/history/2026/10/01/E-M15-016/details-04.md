---
schema: 1
id: E-M15-016
---

# E-M15-016 details 4

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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test local_collab_cli --locked
```

**Notes**

6 local CLI tests pass across Git layouts, cold reads, rich values, receipt retry and strict input rejection. /tmp/jj-thread-cli.log.
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
CARGO_INCREMENTAL=0 pnpm contracts:check
```

**Notes**

120 actual isolated HTTP/SSE responses conform; 7 contract tests and generated TypeScript fixture validation pass. /tmp/jj-thread-contracts-check.log.
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
CARGO_INCREMENTAL=0 cargo clippy -p jj-collab -p jj-store-service --lib --locked -- -D warnings
```

**Notes**

Selected domain/service packages and direct dependency closure check without warnings. /tmp/jj-thread-clippy.log.
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
pnpm web:build
```

**Notes**

Production bundle and manifest pass. Existing large-chunk advisory remains. /tmp/jj-thread-web-build.log.
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
pnpm web:check
```

**Notes**

Final check: 0 errors and 0 warnings. /tmp/jj-thread-web-check-last.log.
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
pnpm catalog:check
```

**Notes**

Final check: 0 errors and 0 warnings; 119 catalog modules satisfy boundaries. /tmp/jj-thread-catalog-check-final.log.
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
pnpm check:architecture
```

**Notes**

475 production/catalog modules pass line-budget and owner audit. /tmp/jj-thread-architecture-final.log.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries pass.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters and 141 requirements pass.
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
pnpm spec:first
```

**Notes**

Generated schema changes require compatibility.md in the same change; added legacy-byte and additive-witness compatibility requirements.
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
pnpm spec:first
```

**Notes**

55 protected files mapped to 10 canonical chapters pass.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance --locked
```

**Notes**

2 focused HTTP tests pass: exact targets, assets, signed roots, receipts, permission/input/media bounds and unknown request identity. /tmp/jj-thread-http.log.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting check passes.
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
pnpm check:architecture
```

**Notes**

Final owner and line-budget audit passes for 477 modules.
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
pnpm spec:validate
```

**Notes**

Final 4 validators, 21 chapters and 141 requirements pass.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final 57 protected files map to 10 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final bookkeeping check passes for 119 tasks; remaining qualification stays explicit.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/fixtures/StoreCapturedReactionsFixture.svelte catalog/fixtures/StoreThreadsFixture.svelte catalog/fixtures/thread-transport.ts schemas/openapi.json spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/ui.md web/src/lib/api/client.ts web/src/lib/api/generated.ts web/src/lib/api/reaction-asset.ts web/src/lib/collaboration/thread-presentation.ts web/src/lib/controller/collab-actions.svelte.ts web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-threads-data-source.ts web/src/lib/controller/message-reactions.svelte.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-refresh.ts web/src/lib/controller/production-threads-integration.ts web/src/lib/controller/production-threads.svelte.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/thread-types.ts web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte web/src/lib/ui/store-threads/StoreMessageReactions.svelte web/src/lib/ui/store-threads/StoreThreadCreate.svelte web/src/lib/ui/store-threads/StoreThreadReplies.svelte web/src/lib/ui/store-threads/StoreThreads.svelte web/src/lib/ui/store-threads/store-threads.css web/src/lib/ui/store-threads/thread-navigation.ts web/tests/client.test.ts web/tests/vite/production-threads.test.ts
```

**Notes**

All 48 changed supported files pass focused formatting; Rust is checked separately.
<!-- /run -->
<!-- /evidence-field -->

