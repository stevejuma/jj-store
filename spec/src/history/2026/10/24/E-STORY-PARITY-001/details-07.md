---
schema: 1
id: E-STORY-PARITY-001
---

# E-STORY-PARITY-001 details 7

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
pnpm docs:test:browser
```

**Notes**

Final Docker 2706ece1-fd34-4944-9041-ea5291637618: all 20 Help cases passed across Chromium and WebKit.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Docker 6f1b8199-a4b1-4a67-92d7-55ea3aec5180: four type errors in private story queries. Testing Library role queries do not expose Playwright's exact option; string role names already match exactly. Removed the unsupported ignored options without changing query semantics.
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
pnpm catalog:check
```

**Notes**

Docker 4d7d26bd-a7f0-4b93-ab67-a635c5020276: zero errors/warnings; 489 catalog modules; storybook-parity validator passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte
```

**Notes**

All ten directly affected cases passed after the type-only query correction.
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

Docker a5b71173-47e4-473d-9822-2b2183882cec: zero errors and warnings.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:architecture
```

**Notes**

Final architecture audit passed all 2079 modules.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
```

**Notes**

Final static validation passed eight validators and 208 requirements, including ownership and story parity.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Final working-tree bookkeeping passed 180 tasks /960 records. This includes the pending unrelated M33 checkpoint; it does not qualify that external Notes work.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 4d660d4c --head a6c550664b0f103acc12de6e36bb01e83982b8fe
```

**Notes**

The isolated verified slice had two missing mapped canonical areas: API and governance. Added explicit client-read and evidence/owner-coverage notes in their existing chapters; no diagnostic, mapping, requirement or qualification gate was weakened.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 4d660d4c --head 765b4a154924ca13c0365efdb2449dac53195428
```

**Notes**

The isolated committed slice passes spec-first: 369 protected files and seven canonical chapters. The original four M33 edits, their pending external checkpoint and its navigation remain uncommitted; the normal working-tree guard continues to reject that unqualified checkpoint.
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

Node 22.22.0; pnpm 10.32.1; Playwright 1.63.0; JJ 0.45.1; Rust 1.99.0. Local upstream archives are explicit validation inputs. Upstream Design Core verified change ournyxwx /8a195a231c15; archive product bytes are unchanged by its acceptance-only follow-up. Native bounded Node checks use Node 26.9.0 and pnpm 10.32.1; Docker acceptance uses the pinned Node 22.22.0 toolchain.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Story jjStoreStory parameters; internal ServerConnectionsView; local validation configuration and versioned source/run manifests. Public storage and API formats unchanged.
<!-- /evidence-field -->

