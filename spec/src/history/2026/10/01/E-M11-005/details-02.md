---
schema: 1
id: E-M11-005
---

# E-M11-005 details 2

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
jj --no-pager st && jj --no-pager diff --stat
```

**Notes**

Resumed owned projects-hub workspace nuwmlxxk on rkqwtxnl; main application at 7460 preserved. Original regression was absent installation hub and only paged raw WorkStore reads.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial 7 errors and 6 warnings: public chip size and confirmation exports, exact bigint handling, narrowing and initial prop capture. Corrected integrations.
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
pnpm check:architecture
```

**Notes**

Controller initially 361 counted lines; split integration, connection, route and bounded cache helpers. Further 320/313/303/302/301 findings resolved within unchanged 300-line boundary.
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
cargo clippy -p jj-store-service -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

Collapsible if corrected without weakening lint.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

Reset retained optional drafts field; reset now explicitly clears it. Route helper import corrected before final runs.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Initial 6/9 story pass: actual Markdown textbox and named status selectors corrected. Follow-up 5/9 exposed duplicate Close selectors and unnamed legacy status, fixed with dialog title, Done action and named Work command status. Catalog fixture had 2 errors/1 warning; exact selector option types had 2 errors, corrected.
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
pnpm install --offline
```

**Notes**

Independent ignored node_modules using existing local package hook/file snapshots after symlinked dependencies failed Storybook setup. Manifest semver ranges unchanged. Temporary consumer development validation, not registry release qualification.
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
pnpm web:build
```

**Notes**

Broad public Community imports emitted an unused database worker; default IIFE output rejected top-level await. Used public Community tursoBundleBuildPlugin and ES worker configuration.
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
JJ_STORE_DEV_ROOT=.scratch/projects-hub-cold pnpm dev:restart
```

**Notes**

In-checkout state outside explicit .local subtree rejected; retained owned generated failure only until cleanup.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

First cold attempt rejected an unused 15 MB worker by unchanged 8 MiB asset limit; ESRCH in startup cleanup initially masked rejection. Compiler entry traversal now serves reachable imports, dynamic imports, CSS/fonts and agent icons only.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --locked --test native_lifecycle http_work_acceptance::http_workstore_pages_commands_permissions_and_contract_capture -- --exact
```

**Notes**

Shared target cache reused stale service/HTTP artifacts: unknown_route followed by missing browse types on forced HTTP compilation. Replaced target symlink with independent ignored clone cache and rebuilt owning crates; no shared CARGO_TARGET_DIR remains.
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
cargo test -p jj-store-service --locked --test work_store
```

**Notes**

4/4 service cases pass from independent target, including full-root filtering before paging, counts, query binding, permission checks, expiry and selected pins.
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
cargo test -p jj-store-cli --locked --test native_lifecycle http_work_acceptance::http_workstore_pages_commands_permissions_and_contract_capture -- --exact
```

**Notes**

1/1 actual HTTP WorkStore contract capture passes after isolated rebuild.
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
pnpm contracts:check
```

**Notes**

All 11 registered capture fixtures; 80 actual HTTP/SSE responses, 6 schema regressions and TypeScript contract check pass.
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
cargo clippy -p jj-store-service -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

Owning service/HTTP all-target checks pass; no unrelated Rust workspace lane.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

31/31 tests across all five direct controller consumers pass.
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
pnpm web:test
```

**Notes**

Final 60/60 Node tests; includes 9 route codec cases with full native IDs, filters and legacy operation pin.
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
pnpm web:check && pnpm catalog:check
```

**Notes**

0 errors/0 warnings; catalog 84 modules.
<!-- /run -->
<!-- /evidence-field -->

