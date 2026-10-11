---
schema: 1
id: E-GUIDE-STRUCTURE-001
---

# E-GUIDE-STRUCTURE-001 details 2

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
node --test scripts/docs.test.mjs
```

**Notes**

Eight existing generator/reference/review/closure/staging tests pass.
Configuration examples and command semantics were unchanged, so the mixed Rust
docs:test and root build/browser suites are out of scope.
/tmp/jj-guide-structure-generators.log.
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
uv run --locked python: compare original guide paragraphs/headings with sources and built HTML IDs
```

**Notes**

All original source prose and headings survive across 12 guides, allowing
whitespace and paragraph relocation; each legacy heading slug exists in built
HTML; no duplicate headings. Before/after metrics: agent subsections 6 to 52,
browsing 10 to 30, capture 4 to 13, editing 4 to 10, reviews 3 to 8, settings 10
to 17. Longest agent section falls from 164 to 26 lines, browsing 88 to 20 and
editing 47 to 14. /tmp/jj-guide-structure-content.log.
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
CUA fresh Help guide load: inspect headings/TOC, navigate to Browsing, activate Apply and unapply branches
```

**Notes**

Fresh loopback delivery shows final agent headings and paragraph spacing.
Browsing exposes dedicated workspace and agent sections; its TOC link navigates
to #apply-and-unapply-branches and the matching rendered heading. Screenshots
/tmp/jj-store-agent-guide-structured.jpg and
/tmp/jj-store-browsing-guide-structured.jpg. Underlying user tab/view remains
intact; no application/service restart or fixture reset.
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

Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Python 3.14.3; locked uv/MkDocs Material
toolchain from uv.lock; Markdown evidence schema 1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->

## Formats

Authored Markdown H2/H3 sections and paragraphs; unchanged code examples and
generated command/API references; reviewed SHA-256 application source
fingerprint; existing self-contained Help HTML/asset manifest format.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->

## Next

Retain M19 Partial status and registry dependency/packaged WKWebView release
gates. Visual baseline approval and native platform qualification are separate.
Existing E-NPM-REGISTRY-001 evidence bookkeeping remains assigned to its owner.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->

## Contract review

```yaml
status: changed
reason:
  JST-DOC-001 owning documentation chapter now requires task-ordered guide
  sections, descriptive headings and short paragraphs with qualifications beside
  their tasks. JST-DOC-002/003/004 generation, review invalidation, offline
  delivery and bounded asset admission contracts are unchanged; content-only
  rebuild checks their existing closure.
```

<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->

## Milestone review

```yaml
status: unchanged
reason:
  M19 remains Partial. Strict Help builds, eight generator tests, preserved
  source/anchor audits and fresh rendered guide navigation qualify this
  documentation structure slice. They do not close registry dependency, packaged
  WKWebView, signed distribution or visual-baseline gates.
```

<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->

## Excluded scope

No root aggregate, Rust, native configuration examples, application type checks,
full browser/Storybook, installed service/release or baseline lanes: only
authored guide content and review metadata changed. Shared
renderer/theme/navigation code is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:document_checks markdown -->

## Document checks

pnpm exec spec-validator validate --only governance,summary,fileLimits: pass; 3
validators, 3579 chapters, 181 requirements.
/tmp/jj-guide-structure-documents.log.

pnpm exec prettier --check --ignore-path /dev/null with the explicit 14 changed
file paths: pass. /tmp/jj-guide-structure-format.log. Evidence formatting is
checked again after this record is added.

pnpm check:progress: exit 1. Reproduces the five inherited E-NPM-REGISTRY-001
evidence errors: missing .pnpmfile.cjs, prose used as a missing file and invalid
test reference, invalid run result and invalid overall result. No new guide
evidence errors. /tmp/jj-guide-structure-progress.log.

pnpm spec:first: exit 2. SPEC-FIRST-VCS reports the same inherited evidence
errors and cannot determine a trustworthy change set.
/tmp/jj-guide-structure-spec-first.log.

The inherited failures were previously reproduced on the preserved
package-adoption parent at /tmp/jj-unavailable-parent-progress.log and
/tmp/jj-unavailable-parent-spec-first.log. Their evidence remains unchanged; the
guide slice passes its focused checks while those separate governance issues
remain open.
<!-- /evidence-field -->
