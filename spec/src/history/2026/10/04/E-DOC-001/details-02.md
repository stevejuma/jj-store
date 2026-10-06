---
schema: 1
id: E-DOC-001
---

# E-DOC-001 details 2

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
jj --no-pager st; jj workspace add ../jj-store-documentation --name documentation -r @- -m "Build documentation site and embedded Help"; inspect JJ docs source/rendered site and owning specifications
```

**Notes**

Isolated workspace parent b11c0486. Original working files preserved. Markdown/Material and generated full-screen Help follow the user-selected plan; no publication.
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
node --test scripts/web-manifest.test.mjs; pnpm docs:check (before)
```

**Notes**

2 existing asset-generator tests passed; documentation command absent before implementation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 24
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --frozen-lockfile; pnpm install --ignore-pnpmfile --frozen-lockfile; pnpm web:check (registry baseline)
```

**Notes**

Existing hook checksum mismatch blocked first install; registry-only install passed. Registry Design Core 0.2.0 lacks APIs already used on parent: 23 Svelte errors in 9 files. No release claim. Logs retained in ignored .scratch.
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
pnpm add -D --save-exact --ignore-pnpmfile mermaid@11.12.0; uv lock; uv sync --locked; pnpm install --no-frozen-lockfile --lockfile=false
```

**Notes**

Locked documentation dependencies; restored existing source dependency validation setup without changing registry package ranges or installing/publishing global tools.
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
cargo run --quiet --locked -p jj-store-cli --example docs_reference (initial/fixed environment); pnpm docs:generate
```

**Notes**

Initial vectorscan build could not find C++ headers. All succeeding Cargo commands use DEVELOPER_DIR=/Library/Developer/CommandLineTools and CXXFLAGS=-isystem/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1. 126 visible command nodes generated without opening a repository; defaults come from Rust. Hidden hooks excluded. Reviewed argument descriptions fill absent parser prose and fail on new undocumented arguments.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check (strict build iterations)
```

**Notes**

Caught Mermaid semicolon parsing, shared-help stale browser link, unprocessed 404 inline scripts, unneeded source maps and oversized highlighted API page. Fixed build hooks and split generated API by domain; original asset limits retained.
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
pnpm docs:check (after repairs)
```

**Notes**

9 network-disabled Mermaid renders; strict MkDocs navigation plus HTML/CSS link closure and anchor validation. 117 assets, approximately 12.7 MB. Generated refs match the parser and checked-in OpenAPI; source fingerprints acknowledge mapped crate/web/desktop sources and reject uncovered new crates.
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
pnpm docs:test; node --test scripts/web-manifest.test.mjs; pnpm exec tsx --test web/tests/help-session.test.ts
```

**Notes**

6 documentation regressions, 1 production configuration-parser test, 2 existing manifest tests and 1 frame-message boundary test pass. Negative fixtures cover stale prose review, new sources, generated reference drift, missing/unsafe/symlink assets, broken anchors, inline scripts, external runtime files and preflight-safe desktop staging.
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
cargo test --locked -p jj-store-http --test assets
```

**Notes**

Complete directly affected HTTP asset suite: 9 passed. Application frame-ancestors remains none; only Help permits same-origin framing; exact preloaded routes enforce independent bounds.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (initial new middleware)
```

**Notes**

10 JS check errors in new Vite middleware were repaired with typed JSDoc and narrowing; final web diagnostics zero.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:test:browser (initial three Chromium runs)
```

**Notes**

Initial 2 test failures used input fill without Material key events and headings without their permalink marker. Next 1 selected a hidden search link after dismissing search. Third 1 exposed actual theme accessibility defects: unnamed search dialog, footer contrast, color-only links. Fixed explicit reader markup/styles and realistic keyboard interactions; original search, dismissal, draft and Axe assertions retained.
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
pnpm test:governance
```

**Notes**

All 49 governance tests passed. Escalated because source mapping and progress evidence policy changed; docs and new build configuration now require same-change evidence.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:test:browser (first Chromium/WebKit run); pnpm exec playwright install webkit
```

**Notes**

3 Chromium passes; sidebar Shift+Tab escaped focus (1 failure), and the newly pinned WebKit runtime was absent (4 launch failures). Installed WebKit 26.6/v2359 and implemented explicit same-origin iframe focus boundaries.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:test:browser (keyboard-boundary run)
```

**Notes**

All 4 Chromium scenarios and 2 WebKit scenarios passed. WebKit mouse clicks do not automatically focus a button: 2 focus-return assertions exposed capture of the previously active element instead of the actual trigger. Fixed by retaining the click currentTarget; kept the assertions.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final source setup: zero web/catalog errors or warnings, 182 catalog modules, 931 architecture modules, valid UI ownership. No visual baselines changed.
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
Vite createServer smoke with generated /help assets on isolated 7482, then server.close()
```

**Notes**

117 admitted assets served at /help/ with scoped same-origin framing. An earlier check raced the concurrently cleaning site build and returned unavailable; sequential rerun after generation passed. No backend or fixture startup.
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
Changed-file Prettier check; targeted rustfmt --check; uv run --locked python -m py_compile scripts/docs/hooks.py scripts/docs/site.py
```

**Notes**

Formatting and Python syntax pass; the historical milestone JSON prefix is not reformatted. Root aggregate check/test/build/spec:check and unrelated Rust/browser/storage lanes remain out of scope.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

23 canonical chapters, 171 requirements; 95 protected files and 9 canonical chapters; 137 tasks. This ledger tracks partial release qualification without promoting other milestones.
<!-- /run -->
<!-- /evidence-field -->

