---
schema: 1
id: E-PUBLIC-SURFACES-001
---

# E-PUBLIC-SURFACES-001

<!-- evidence-field:id markdown -->
## Id

E-PUBLIC-SURFACES-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/publish-spec-pages.yml
- .github/workflows/publish-main-image.yml
- .storybook/main.ts
- README.md
- docs/releases.md
- scripts/ci-workflows.test.mjs
- scripts/image-tags.mjs
- scripts/release-archive.mjs
- scripts/release-finalize.mjs
- scripts/release-metadata.mjs
- scripts/release.test.mjs
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/ui/42-application-story-parity.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/index.md
- spec/src/history/2026/11/03/E-PUBLIC-SURFACES-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#every tracked workflow has an explicit current
  owner
- scripts/ci-workflows.test.mjs#Pages validates the spec, manual and source
  Storybook together
- scripts/release.test.mjs#native releases expose a bundle and standalone server
  for every target
- scripts/release.test.mjs#main images use bounded source and architecture tags
<!-- /evidence-field -->

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
node --test scripts/release.test.mjs scripts/ci-workflows.test.mjs
```

**Notes**

Ten focused tests passed. The release inventory test could not invoke Cargo
because the ambient mise shim had no selected Rust version. No product or
repository state changed; the retry used the workspace-owned Rust 1.99.0
toolchain explicitly.
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
/tmp/tmp.mxwzjBmpYE/actionlint .github/workflows/publish-main-image.yml .github/workflows/publish-release.yml && npx --yes pnpm@10.32.1 release:check
```

**Notes**

Actionlint 1.7.12 passed both changed publication workflows. All 88 release
checks passed, including workflow ownership, bounded tags and native assets.
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
MDBOOK_OUTPUT__HTML__SITE_URL=/jj-store/ STORYBOOK_BASE_PATH=/jj-store/storybook/ npx --yes pnpm@10.32.1 spec:build
```

**Notes**

The first exact Pages attempt reached the ambient mise mdBook shim without a
selected version. The retry used the repository-pinned mdBook 0.5.4 executable.
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
PATH="/home/steve/.local/share/mise/installs/mdbook/0.5.4:$PATH" MDBOOK_OUTPUT__HTML__SITE_URL=/jj-store/ STORYBOOK_BASE_PATH=/jj-store/storybook/ npx --yes pnpm@10.32.1 spec:build && npx --yes pnpm@10.32.1 spec:links && node scripts/docs.mjs render
```

**Notes**

The specification built and all 127,080 local references passed, but the manual
renderer reached an unselected ambient uv shim. The retry selected the pinned uv
0.9.28 installation explicitly.
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
PATH="/home/steve/.local/share/mise/installs/uv/0.9.28/.mise-bins:$PATH" STORYBOOK_BASE_PATH=/jj-store/storybook/ node scripts/docs.mjs render && mkdir -p spec/book/manual && cp -R docs/build/. spec/book/manual/ && npx --yes pnpm@10.32.1 build-storybook --output-dir spec/book/storybook && test -f spec/book/index.html && test -f spec/book/current/progress/M30/index.html && test -f spec/book/manual/index.html && test -f spec/book/storybook/index.html && test -f spec/book/storybook/iframe.html && rg -q 'Application manual' spec/book/index.html && rg -q 'Storybook component catalog' spec/book/index.html && rg -q '/jj-store/storybook/' spec/book/storybook/index.html && rg -q '/jj-store/storybook/' spec/book/storybook/iframe.html
```

**Notes**

The strict 144-file manual closure passed, Storybook built 6,823 modules at the
Pages subpath, the required entry files existed, and the root index linked the
manual and Storybook. Existing Lightning CSS and chunk-size warnings remained
non-fatal; this build does not claim interaction, accessibility or visual
approval.
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
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

The first progress check correctly rejected the still-pending evidence result,
missing runs and file-only test references. The record now names discoverable
Node test anchors and the actual validation attempts.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

Two bookkeeping retries rejected unsupported evidence results: `pass` cannot
retain hosted gaps, and `partial` is not a current schema value. The record now
uses `pending`, which preserves successful local runs without claiming hosted
deployment or publication.
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
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

Progress validation passed for 186 tasks and 1,029 evidence records with the M30
status and remaining gaps unchanged.
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
npx --yes pnpm@10.32.1 docs:check
```

**Notes**

The isolated strict documentation checker passed the changed README, release
guide, specification landing page, navigation and manual sources.
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
npx --yes pnpm@10.32.1 test:governance
```

**Notes**

An initial invocation correctly refused to overlap the still-finishing owned
documentation container. After that successful run released its lease, all 121
governance tests passed.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 spec:first
```

**Notes**

Spec-first required the exact mapped public-distribution, Storybook UI and
governance owners in addition to the focused CI chapter. Those canonical pages
now record the same bounded contract.
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
npx --yes pnpm@10.32.1 spec:first
```

**Notes**

Spec-first passed 13 protected files against four canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted Pages, first main image and first qualified tagged release remain pending.
- Storybook interaction, accessibility and visual approval remain separate gates.
- Scheduled freshness, four quarantines, latency/reliability, provider and additional platform gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 22; pnpm 10.32.1; Storybook 10.5.7; actionlint 1.7.12; Pages artifact v4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub Actions YAML, static Pages HTML/assets, OCI image tags, native executable
and tar.gz assets, SHA-256 manifests, Markdown documentation and evidence.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The audit classifies every tracked workflow and confirms that none is obsolete:
direct, reusable, experimental and gated publication entry points all retain a
current caller or documented owner. Pages now builds the specification, manual
and source Storybook from one revision, and its root index links all three
published surfaces. Qualified release finalization attaches both the complete
native bundle and standalone server executable for each of four supported
targets. The README is shortened into a project-oriented entry point and no
longer claims that heavy qualification is unscheduled. Successful canonical
main checks now publish scanned `sha-<12-hex>` and rolling `main` image tags;
qualified versions retain their exact Git tag and independently gated image.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Pass protected PR checks, merge, then observe Pages and the main-image publish.
The first version publication must prove the immutable release-asset contract.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now requires combined Pages, bounded source-image tags and both bundle and standalone native-server release assets with immutable checksums; JST-DOC-001/002 retain accurate entry-point and fail-closed build requirements.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress because local build evidence cannot establish hosted Pages publication, actual version publication, scheduled freshness, quarantine closure, latency/reliability, provider, platform or visual gates.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages
