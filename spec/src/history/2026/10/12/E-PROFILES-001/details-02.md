---
schema: 1
id: E-PROFILES-001
---

# E-PROFILES-001 details 2

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
cargo test -p jj-collab -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc --locked
```

**Notes**

Initial broad runs stopped at the stale HTTP inventory count/admission assertion, then at local_collab agent-discussion pagination. Inventory now recognizes all 177 paths and directory admission. The pagination fixture now creates actual user threads: the unchanged catalog excludes agent discussions, which remain direct targets. Exact corrected regressions pass.
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
cargo test -p jj-store-service --test local_collab agent_discussion::saved_agent_replies_and_reactions_survive_restart_and_keep_exact_targets --locked -- --exact
```

**Notes**

Exact corrected pagination/restart/replies/reactions regression passed. The first fixture edit omitted baseline and failed compilation; baseline: None repaired it without changing production directory membership.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --lib --locked
```

**Notes**

Interrupted after 20 observed provider/capture/process failures and prolonged unfinished tests. No full-server pass is claimed. Provider focused serial tests pass separately. The foreground audit failure was isolated and repaired by separating corruption from transient transport unavailability. Remaining broad capture tests are unqualified.
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
cargo test -p jj-store-server --lib agent_execution::chatgpt::tests --locked -- --test-threads=1
```

**Notes**

5 tests passed in 177.68 seconds, including both cases that failed in the heavily concurrent broad run.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service -p jj-store-wal --locked
```

**Notes**

One attempt exhausted disk while linking; another found a non-executable WorkStore artifact left by that failure. Removed only stale generated incremental caches (636 directories older than 24 hours) and the two explicitly identified incomplete generated executables. Recovered 37 GiB; a clean rerun follows.
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
pnpm contracts:check
```

**Notes**

Generated contracts match; 300 actual HTTP/SSE responses conform; 8 contract tests and contract TypeScript check passed.
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
pnpm test:contracts
```

**Notes**

8 schema/media/identity/command tests and contract TypeScript check passed on the generated directory contracts.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Both Svelte checks report 0 errors and 0 warnings; catalog boundary check passed (402 modules).
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
pnpm web:test
```

**Notes**

174 tests passed. Existing malformed managed-agent fixture now includes required repository_bindings: []; no production authentication behavior was weakened.
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
pnpm catalog:test
```

**Notes**

164 files passed; 975 tests passed and 1 existing test skipped. Later focused Settings login coverage adds the zero-repository direct-link regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory.test.ts web/tests/vite/global-settings-history.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

12 tests passed across 3 files. Covers retained profile/emoji drafts, stale revisions, stable unknown-outcome receipts, stale media isolation and direct Settings authentication with no repository grants.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreProfileSettings.stories.svelte
```

**Notes**

4 interaction/axe stories passed for profile and custom emoji. Independent visual approval remains pending; no baselines changed.
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
pnpm check:ui-ownership
pnpm check:boundaries
```

**Notes**

Community source ownership and 17 package boundaries pass. Community copies use revision a5f64a4c7dea2f29b4058aa7666a98c4c0dee656 and retained Apache-2.0 attribution.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

19 inherited source-budget findings remain, each verified unchanged or smaller against parent 6063b9bd. Production-store-controller remains at its inherited 316 counted lines. Profile author projection was extracted after formatting exposed chat-body growth; that body is now below 300, with 23 interaction/axe stories passing. Production-threads shrinks from 311 to 295 and CommunityMessage is below 300. No baseline was weakened.
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

7 validators, 3629 chapters and 185 requirements pass after canonical requirement definitions, mapped verification rows and supported Implemented statuses were added.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The final record will be checked after coverage is populated. Historical E-NPM-REGISTRY-001 has five inherited diagnostics: missing .pnpmfile.cjs, missing/invalid prose test reference, invalid run result and invalid result.
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
env PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/profile-acceptance.mjs
```

**Notes**

Cold real embedded RustFS and production UI: local profile/private email and emoji image upload/save; actual image decode; axe; graceful restart; manage-identities operator provisioning and identity CLI enrollment; secret-free journal; authenticated zero-repo discovery, browser profile editing, public email omission and restart persistence. Final rerun after the transient-audit correction follows. Earlier attempts found missing uv PATH, stale docs reviews/generated references, incomplete Node Fetch Metadata, and direct Settings login handling; each is repaired. All disposable server children are explicitly stopped.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
env PATH="/Users/stevejuma/.local/bin:$PATH" JJ_STORE_REAL_TEST=profiles-emoji.spec.ts JJ_STORE_DEV_PORT=7486 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm web:test:real
```

**Notes**

General harness spent time in unrelated demo agent seeding; interrupted it and stopped its owned temporary service. The empty-installation harness supplies the required real-store cold acceptance independently.
<!-- /run -->
<!-- /evidence-field -->

