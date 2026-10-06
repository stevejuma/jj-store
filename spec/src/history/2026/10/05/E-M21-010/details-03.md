---
schema: 1
id: E-M21-010
---

# E-M21-010 details 3

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
cargo fmt -p jj-store-cli -p jj-store-http -p jj-store-service
```

**Notes**

Changed Rust source and test fixtures formatted; no historical ledger formatting.
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
cargo fmt --all -- --check
```

**Notes**

working-transport-format-rust.log passes.
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
pnpm docs:generate
```

**Notes**

working-transport-docs-generate.log: registered route and owned types regenerate installation API/type manual references.
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
pnpm docs:review local "Documented registered working-tree observation: exact operation and expected binding generation, server-resolved root, complete current checkout/sparse/ignore proof and changed-path summaries without contents or command tokens. Active-combination and production UI integration remain pending."
```

**Notes**

working-transport-docs-local-review.log: source/manual review acknowledged.
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
pnpm docs:review services "Owned version-1 physical observation request/summary contracts retain exact operation/view/WC and metadata/physical fingerprints, bound complete changes to 4096, and omit private policy, contents and executable selections."
```

**Notes**

working-transport-docs-services-review.log: source/manual review acknowledged.
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
pnpm docs:review http "Registered LocalTrusted administrative working-tree observation resolves the root from current device registration, rechecks the captured binding, and exposes strict path-free inputs and safe bounded response summaries through the endpoint inventory."
```

**Notes**

working-transport-docs-http-review.log: source/manual review acknowledged.
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
pnpm docs:review contracts "Regenerated OpenAPI and client types for registered physical working-tree observation; actual cold filesystem responses now join the existing complete registered-route capture inventory."
```

**Notes**

working-transport-docs-contracts-review.log: source/manual review acknowledged.
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
pnpm docs:review application "Generated client contracts add the registered physical observation route. The production Stacks page still only browses pinned code; the manual distinguishes this read transport from complete Unstaged, named combinations and writable controls."
```

**Notes**

working-transport-docs-application-review.log: source/manual review acknowledged.
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
pnpm docs:check
```

**Notes**

working-transport-docs-check.log: generated client type source required the application review acknowledgement. Browsing manual and explicit review added; no checker bypass.
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
pnpm docs:check
```

**Notes**

working-transport-docs-repaired.log: nine Mermaid diagrams, strict manual build and 139-file link/anchor closure pass.
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
pnpm exec prettier --write (changed docs excluding milestones.md)
```

**Notes**

working-transport-format-docs.log; review coverage, application manual, documentation chapter and final plan formatted in working-transport-format-coverage.log, working-transport-format-app-docs.log and working-transport-format-plan-final.log.
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
pnpm exec prettier --check (changed docs/schema/generated client excluding milestones.md)
```

**Notes**

working-transport-format-docs-check.log: all selected artifacts pass; final later prose changes are rechecked before delivery.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

working-transport-spec-first.log: CLI test and additive API contracts require mapped cli.md, compatibility.md and mutations.md. Added the owning boundary documentation without weakening the change map.
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
pnpm exec prettier --write spec/src/cli.md spec/src/compatibility.md spec/src/mutations.md
```

**Notes**

working-transport-format-mapped-chapters.log: only newly changed mapped chapters formatted.
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
pnpm exec prettier --check spec/src/cli.md spec/src/compatibility.md spec/src/mutations.md
```

**Notes**

working-transport-format-mapped-chapters-check.log passes.
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
pnpm spec:first
```

**Notes**

working-transport-spec-first-repaired.log: 19 protected files and 11 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

working-transport-spec.log: five validators, 23 chapters and 173 requirements pass before final mapped prose additions; repeated before delivery.
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

working-transport-progress.log: 142 tasks pass, evidence bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

