---
schema: 1
id: E-UI-064
---

# E-UI-064 details 5

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
pnpm exec prettier --write web/tests/real/task-reactions.spec.ts
```

**Notes**

Native hover helper waits for visible toolbar state; per-action timeout15s, total test120s. Lost committed responses now cover both external edit and tombstone before status recovery.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_DEV_PORT=7484 JJ_STORE_REAL_TEST=task-reactions.spec.ts JJ_STORE_REAL_KEEP=1 pnpm web:test:real
```

**Notes**

Fresh isolated fixture: 1035 ms startup, 870 ms restart, 178368 KiB resident, 423992 ms preparation. Reproduced real hover bug after removing focused reaction chip: focusout hid toolbar while pointer remained over the message. Fixed owned shared hover helper; fixture preserved for exact retries.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

After shared hover fix: both 0 errors/warnings; catalog 129 modules. Logs jj-task-reactions-hover-*-check.log.
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
pnpm test:storybook
```

**Notes**

Shared helper consumer escalation. 37/53 files and 158 tests passed, then StoreConnection browser WebSocket/birpc connection closed. No assertion failure; remaining families did not execute. Exact StoreConnection retry and sequential full lane follow.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_DEV_PORT=7484 pnpm dev:restart
```

**Notes**

Rebuilt final UI/development binaries and restarted isolated server with preserved fixture: 4013 ms restart, 165360 KiB RSS. User 7460/7462 stack untouched.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7484 JJ_STORE_REAL_REPOSITORY=24a50486d238649e99b7de7d4411eae3 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-appearance pnpm exec playwright test --config web/playwright.real.config.ts task-reactions.spec.ts
```

**Notes**

cold-final.log: second-client count missed the 5 second expectation under simultaneous full Storybook load. Exact retry without production/assertion changes follows; suites thereafter sequential.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7484 JJ_STORE_REAL_REPOSITORY=24a50486d238649e99b7de7d4411eae3 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-appearance pnpm exec playwright test --config web/playwright.real.config.ts task-reactions.spec.ts
```

**Notes**

cold-final2.log: status locator ambiguous between clipboard sr-only status and reaction status. Scoped status/check controls to Message reactions group; no product change.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7484 JJ_STORE_REAL_REPOSITORY=24a50486d238649e99b7de7d4411eae3 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-appearance pnpm exec playwright test --config web/playwright.real.config.ts task-reactions.spec.ts
```

**Notes**

cold-final3.log: lost-response fixture raced external comment edit before intercepted write committed; status lookup correctly retained unknown request. Network trace proved intercepted write was 404 and receipt unknown. Added explicit successful committed response assertion before abort/edit/removal, and wait for pending status to disappear on recovery. This strengthens lost-committed-response proof.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7484 JJ_STORE_REAL_REPOSITORY=24a50486d238649e99b7de7d4411eae3 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-appearance pnpm exec playwright test --config web/playwright.real.config.ts task-reactions.spec.ts
```

**Notes**

cold-final4.log: committed lost-response recovery after external edit passed. Second picker selection was silently suppressed when automatic WorkStore refresh set loading between opening and selecting. Decoupled reactions from background read disable state while retaining writable, own busy/pending and exact server validation guards. Added deterministic Task reactions during background reads story.
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
pnpm exec prettier --write web/src/lib/ui/store-collaboration/StoreMessageActions.svelte web/src/lib/ui/store-work/StoreWorkComment.svelte catalog/fixtures/StoreWorkFixture.svelte catalog/components/store-work/StoreWork.stories.svelte spec/src/ui.md spec/src/milestones.md
```

**Notes**

Focused formatting after background-read fix.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_DEV_PORT=7484 pnpm dev:restart
```

**Notes**

Final background-read fix in built UI; preserved isolated fixture and explicit binaries; 4768 ms restart, 162592 KiB RSS. User HMR stack unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte
```

**Notes**

Exact previously interrupted connection family plus final task family: 14/14 in 2 files, including background-read picker acceptance.
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
pnpm web:check; pnpm catalog:check; pnpm spec:first; pnpm check:architecture
```

**Notes**

After final production edits: both type checks 0 errors/warnings, catalog 129 modules; 35 protected files/12 canonical chapters; architecture 737 modules.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7484 JJ_STORE_REAL_REPOSITORY=24a50486d238649e99b7de7d4411eae3 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-appearance pnpm exec playwright test --config web/playwright.real.config.ts task-reactions.spec.ts
```

**Notes**

cold-final5.log: final built-stack regression 1/1 in 15.2 seconds. Native persisted emoji picker/counts, reload, second-client automatic set/clear, exact Work root query, actual committed response loss, original receipt status after external edit/tombstone, no tombstone picker, leading Copy alignment, both task buttons rest/real hover, zero targeted Axe violations and zero page errors. Final built server cold-restarted with preserved isolated fixture before retry.
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
pnpm check:ui-ownership; cargo fmt --all -- --check
```

**Notes**

Final production ownership validator and affected Rust formatting pass.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-hZnlr0 JJ_STORE_DEV_PORT=7484 pnpm dev:stop; python3 disposable-fixture cleanup; curl --max-time 10 http://127.0.0.1:7460/
```

**Notes**

Stopped only isolated acceptance stack, verified port closed and exact fixture identity, removed temporary root. User Vite returned 200; persistent development data and 7460/7462 processes preserved.
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
pnpm test:storybook
```

**Notes**

Final unsharded run: 40/53 files and 172 tests passed; NavigationPersistence browser WebSocket/birpc connection closed at 171.34 seconds. No assertion failures. Exact interrupted-family retry, then same full suite split into 3 sequential fresh-session shards; no tests skipped or changed for this infrastructure interruption.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check (all changed supported paths from jj diff --name-only)
```

**Notes**

All matched changed source, generated contracts, story and canonical document files formatted.
<!-- /run -->
<!-- /evidence-field -->

