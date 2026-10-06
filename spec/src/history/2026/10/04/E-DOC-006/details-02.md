---
schema: 1
id: E-DOC-006
---

# E-DOC-006 details 2

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
jj --no-pager st in main and documentation; jj workspace list; jj log/diff; jj new pkrqsmov soumsxws -m <merge description>; inspect both parent ledgers and stale topic fingerprints
```

**Notes**

Both workspaces clean before merge. Resolved seven additive canonical/ledger conflicts and one StoreApplication import conflict, retaining the terminal composition plus Help. Preserved every original evidence object: 427 main plus five documentation-only entries. Updated 11 stale topic reviews after reading current code/contracts and correcting capture, conversion, creation and terminal prose; added terminal-engine source coverage.
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
pnpm docs:generate; reuse pinned registry Mermaid 11.12.0 installation closure from documentation node_modules; pnpm web:build (initial)
```

**Notes**

References regenerated successfully and production UI built; strict MkDocs found the new shared agent help link to repository-only capture-preview.md. Reused 123 package directories for the newly merged pinned Mermaid dependency without changing existing foundational validation links. Existing CSS :global and bundle-size warnings remain.
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
node --test --test-name-pattern="shared agent long help" scripts/docs.test.mjs (before)
```

**Notes**

New regression reproduced missing ../capture-preview.md in the generated manual. The generator now maps this shared-source link to the published capture guide; executable long help remains unchanged.
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
node --test --test-name-pattern="shared agent long help" scripts/docs.test.mjs (after); node --test scripts/docs.test.mjs; pnpm docs:generate; node scripts/docs.mjs stage
```

**Notes**

Exact regression 1/1 and complete docs tooling 8/8 pass, including desktop Help resource staging. Strict final build validates links/anchors and 139 admitted files, 13486725 bytes, nine local light/dark Mermaid pairs. Generated CLI/API references match merged sources and every topic review is current.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture; pnpm test:governance
```

**Notes**

Zero web/catalog diagnostics; 190 catalog modules, 960 architecture modules, valid ownership and 49/49 governance tests. Shared generator/change-map and browser/desktop asset delivery justify their complete direct tooling lanes; no root aggregate invoked.
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
cargo test --locked -p jj-store-http --test assets; cargo test --locked -p jj-store-server --test configuration documentation_configuration_examples_use_production_validation -- --exact; pnpm exec tsx --test web/tests/help-session.test.ts
```

**Notes**

9/9 HTTP asset tests, 1/1 production configuration-loader example and 1/1 exact same-origin frame protocol test pass. Admission, bounds, symlinks, CSP and framing remain covered; no unrelated Rust suite.
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
cargo build --locked -p jj-store-http --example browser_preview; pnpm exec playwright test --config web/playwright.docs.config.ts
```

**Notes**

12/12 cold merged Chromium/WebKit Help cases pass on fresh native asset-router process and current production web/docs assets: live theme, Original palette, collapsed/mobile navigation, search, Axe, network denial, draft retention, missing-doc recovery and focus/dismissal.
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
pnpm spec:first (default merge base)
```

**Notes**

Existing validator cannot use @- for a two-parent merge. Retried using explicit full commit hashes for each parent and the merged head; no product or spec rule changed.
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
pnpm spec:first --base <8b94ae12 full hash> --head <merged full hash>; pnpm spec:first --base <fecf1a25 full hash> --head <merged full hash>; pnpm spec:validate; parseLedger comparison against both parents; changed-file pnpm exec prettier --write
```

**Notes**

Both explicit spec-first comparisons pass (100 protected/9 canonical and 257 protected/16 canonical); specification: five validators, 23 chapters, 172 requirements. Both parent evidence sets and all main milestone states preserved. Formatting excludes historical ledger prefix and generated references.
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
pnpm check:progress; pnpm exec prettier --check <all merged documentation-supported files except historical ledger>; jj version
```

**Notes**

Final progress checker passes 138 tasks; all matched changed files formatted. Jujutsu 0.45.1. Original evidence records remain unchanged and M19 stays Partial.
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

<!-- evidence-field:scope markdown -->
## Scope

Merge acceptance only: docs runtime/assets/tooling and directly affected UI consumers, with canonical source review for features added since the branch. M19 remains Partial; foundational registry-only and packaged WKWebView/DMG release gates from E-DOC-001 remain unchanged. No package publication, deployment, fixture reset, repository conversion, broad storage/contract rerun or visual baseline update.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the validated merge, preserve documentation workspace local evidence, stop only its owned previews and retire that clean workspace.
<!-- /evidence-field -->

