---
schema: 1
id: E-READ-084
---

# E-READ-084 details 3

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
pnpm web:check
```

**Notes**

Final: zero errors or warnings. Initial run caught the direct review-anchor consumer of readPatch; destructuring its patch preserves its existing contract.
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
pnpm catalog:check
```

**Notes**

Final: zero errors or warnings; 194 catalog module boundaries passed.
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
pnpm check:architecture
```

**Notes**

998 modules passed.
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
pnpm check:ui-ownership
```

**Notes**


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
pnpm exec spec-validator validate --only governance
```

**Notes**


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
pnpm exec spec-validator validate --only summary
```

**Notes**


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
pnpm exec spec-validator validate --only verification
```

**Notes**


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
pnpm exec spec-validator validate --only book
```

**Notes**


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
pnpm exec spec-validator validate --only internal
```

**Notes**

Unsupported validator name; replaced with enabled governance/summary/verification/book validators. Repeating --only flags selected only the last name, so each actual enabled lane was then run separately.
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
jj --no-pager diff --check
```

**Notes**

Unsupported JJ option; formatting and the actual canonical/source gates were used instead.
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
pnpm exec prettier --check catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/fixtures/DiffFixture.svelte spec/src/api.md spec/src/jj-integration.md spec/src/ui.md web/src/lib/controller/production-store-loaders.ts web/src/lib/controller/production-store-reader.svelte.ts web/src/lib/controller/review-anchor-patch.ts web/src/lib/projects/presentation.ts web/src/lib/projects/repository-diff.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte web/tests/production-store-loaders.test.ts web/tests/repository-diff.test.ts web/tests/vite/production-patch-reader.test.ts
```

**Notes**

All changed UI, tests and canonical chapters formatted. Ledger appended separately without rewriting historical entries.
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
pnpm dev:backend:build
```

**Notes**

Built only the development JJ/store binaries. Existing macOS linker unwind-size warning retained.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

First attempt overlapped Cargo replacing the executable and failed with ENOENT. Fixture unchanged. Completed explicit backend build, then repeated successfully; Vite and main services were retained.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

Successful backend-only replacement in the existing preview; persistent RustFS/search fixture retained.
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
CUA/CDP fresh background Linux commit comparison after backend restart; select drivers/edac/altera_edac.c then drivers/edac/versalnet_edac.c
```

**Notes**

Original pinned operation/from/to retained. Altera returns text with 53 structured hunks, 35 additions and 32 deletions (matching the CLI), 1195ms cold request and total charged work 444819/1000000. Both file diffs render. Switching files shows a pending skeleton; no console errors. Screenshot/response summary saved under .local/bounded-diff. Temporary Network/cache changes restored and tab closed.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first
```

**Notes**

First attempt required the mapped cli.md and local-jj.md chapters. Added the narrow selected-diff acceptance and read-only local behavior requirements without widening product scope.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

New evidence arrays needed canonical formatter layout. Historical evidence content and text are retained.
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
pnpm check:progress
```

**Notes**

Final ledger: 139 tasks; historical entries retained; no milestone promotion.
<!-- /run -->
<!-- /evidence-field -->

