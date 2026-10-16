---
schema: 1
id: E-REPOSITORY-DELETION-001
---

# E-REPOSITORY-DELETION-001 details 2

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
jj --no-pager st; canonical source and parent/repository AGENTS reads; Community ConfirmationExample and Design Core ConfirmDialog inspection
```

**Notes**

Baseline clean. Community remains reference-only; new compositions are independently authored and declared in provenance.
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
pnpm test:storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte -t 'General local removal'
```

**Notes**

Exploratory filter matched no generated names: 11 skipped, not a regression pass.
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
pnpm test:storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte
```

**Notes**

Before implementation: 1 failed, 10 passed; General removal button absent.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/RepositoryRemoval.stories.svelte
```

**Notes**

First implementation run: 5 failed, 13 passed; fixture used a noncanonical ID, and modal outro needed an eventual assertion. No released test contract was weakened.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial new fixture/story typings: missing state return annotation and unsupported Testing Library exact options. Fixed in the new tests/fixture.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/RepositoryRemoval.stories.svelte
```

**Notes**

Follow-up new-story timing/expectations corrected: await modal/toast animation, assert sanitized operational message, and await delayed command completion. Notification portals intentionally attach inside the active modal; assertion now checks absence of the ConfirmDialog inline-error paragraph.
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
pnpm test:storybook catalog/components/store-application/RepositoryRemoval.stories.svelte catalog/components/store-application/RepositoryDeletionRecovery.stories.svelte catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

Fresh browser process: 30 interaction/accessibility cases pass, including permanent server confirmation, cancellation, success, failure toasts, stale revisions, busy guards, reload recovery and Settings teardown during multi-batch erasure.
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
pnpm catalog:test web/tests/vite/local-repository-removal.test.ts web/tests/vite/native-repository-deletion.test.ts
```

**Notes**

6 tests pass: lost acknowledgement, absence observation, replacement-registration CAS protection, original native intent across reload, foreign result rejection and proven preflight-refusal versus uncertainty.
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
pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Latest completed catalog check: 0 errors/warnings. Ownership boundary passes; final rerun follows remaining edits.
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
pnpm dev:disk; pnpm dev:cargo -- check -p jj-store-object -p jj-store-service -p jj-store-server; pnpm dev:cargo -- check --locked -p jj-store-object -p jj-store-http -p jj-store-server
```

**Notes**

Disk preflight used; quick-xml/serde resolved narrowly in Cargo.lock. Initial unused imports removed; locked focused package check passes. No raw Cargo or unrelated daemon changes.
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
pnpm dev:cargo -- test --locked -p jj-store-object
```

**Notes**

Complete object-crate suite: 4 units and 5 contract tests pass. Deterministic socket tests prove canceled PUT retains admission until storage acknowledgement and foreign listed keys cannot reach DELETE; existing signer/stream/CAS/uncertainty/credential/range tests pass.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-server --test repository_deletion -- --ignored --nocapture
```

**Notes**

1 isolated real RustFS test passes. Wrong epoch preserves access; original session and writes are fenced; discovery is removed; a fresh serving client restores the journal fence; exact request resumes and completes; different request/old identity replay is rejected. Independent raw client proves erased manifest/orphan objects while neighbour repository and content remain. No existing review repository was deleted.
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
pnpm contracts:generate; pnpm dev:cargo -- fmt --all
```

**Notes**

Generated strict OpenAPI/TypeScript boundaries. Formatting commands run after Rust edits.
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
pnpm evidence:new -- --id E-REPOSITORY-DELETION-001 --task M8a.9 --task M4.18 --task M13.4 --requirement JST-UI-016 --requirement JST-UI-019 --requirement JST-LOCAL-004 --requirement JST-OBJ-004 --requirement JST-SEC-006 --requirement JST-API-007 --requirement JST-CLI-005
```

**Notes**

Restored 450 generator-only unrelated paths; preserved this record and its date navigation. Wider lanes are not claimed as passed.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm progress:history; pnpm check:progress
```

**Notes**

Documentation bookkeeping initially failed on file budgets, fresh coverage and navigation. Converted five pre-existing M4 prose notes accidentally encoded as table rows back to prose, preserving text and reducing padding below 32 KiB; condensed the source ownership note to retain the 400-line governance limit. Corrected new evidence test references/milestone vocabulary. The first navigation restore hit a transient colocated index lock; after it released, restored only generator-introduced unrelated paths. All product contracts retained.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --locked -p jj-store-http --test repository_deletion
```

**Notes**

New test expected 400 for protocol mismatch; existing admission correctly denies with 403 before invoking the deletion manager. The new assertion now matches the owning contract.
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
pnpm dev:cargo -- test --locked -p jj-store-http --test repository_deletion
```

**Notes**

One boundary test passes all five cases: reader, foreign Origin, protocol mismatch, unknown body fields and admitted LocalTrusted Admin. Denied requests make no manager call.
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
jj workspace list; jj st (forge-stacks); jj log -r "(::forge-stacks@ ~ ::@-) & ~empty()"
```

**Notes**

Only default and forge-stacks registered. forge-stacks working copy is clean and empty; no non-empty workspace change is absent from the reviewed default ancestry. No workspace was removed or rebased.
<!-- /run -->
<!-- /evidence-field -->

