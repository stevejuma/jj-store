---
schema: 1
id: E-M21-059
---

# E-M21-059 details 3

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
pnpm spec:validate; pnpm check:progress
```

**Notes**

5 validators/23 chapters/173 requirements and 142 tasks. Integration evidence remains pending until remaining checks finish. /tmp/stacks-merge-spec-validate.log and progress-initial.log.
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
cargo test --locked -p jj-store-http --lib local_browser::
```

**Notes**

Four same-origin, generation and local authority route regressions passed. One explicit physical-checkout test initially ignored and then separately executed. /tmp/stacks-merge-http-tests.log.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj cargo test --locked -p jj-store-http --lib local_browser::working_tree::tests::registered_observation_route_reads_real_checkout_without_publication_or_content_leaks -- --ignored --exact
```

**Notes**

One registered real-checkout observation regression passed against the explicitly built development JJ executable; no publication or content leakage. /tmp/stacks-merge-physical-http.log.
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
cargo test --locked -p jj-store-cli --test local_browser bounded_reads::
```

**Notes**

Seven bounded/read-only/merge-path regressions passed; existing real-large-repository test remains ignored because no existing Linux metadata directory was supplied. Existing linker unwind warning retained. /tmp/stacks-merge-cli-bounded.log.
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
pnpm contracts:generate; pnpm docs:generate
```

**Notes**

Combined OpenAPI/client artifacts are identical to the automatically merged parents. Three generated API reference pages updated to include the existing quick/history last_change_mode. /tmp/stacks-merge-contract-generate.log and docs-generate.log.
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
pnpm exec prettier --check docs/maintenance/coverage.json spec/src/api.md spec/src/ui.md spec/src/spec-governance.md
```

**Notes**

JSON documentation-review command writes an unformatted array; formatted coverage.json with the existing formatter. /tmp/stacks-merge-format-initial.log.
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
pnpm web:check; pnpm catalog:check (after validation dependency refresh)
```

**Notes**

Final refreshed dependency: zero errors/warnings and 244 catalog modules. /tmp/stacks-merge-web-final.log and catalog-final.log.
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
cargo test --locked -p jj-store-cli --test local_browser admission_cache::
```

**Notes**

Three registration inspection/invalidation regressions passed. /tmp/stacks-merge-cli-admission.log.
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
pnpm docs:check (after generation/review)
```

**Notes**

139-file documentation closure and links/anchors valid; nine diagrams rendered in both themes offline. /tmp/stacks-merge-docs-final.log.
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
pnpm exec prettier --check <eight changed canonical/manual files>; scoped ledger prefix and newest entry format; pnpm check:progress
```

**Notes**

Formatting and 142-task evidence bookkeeping pass. All 601 historical objects remain equal to the two parents. /tmp/stacks-merge-format-final.log and progress-final.log.
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
pnpm exec spec-validator first --base <each exact parent SHA> --head <exact final merge SHA>; pnpm spec:validate
```

**Notes**

Final comparisons: Main 473 protected files/12 chapters, Stacks 71 files/10 chapters; five validators/23 chapters/173 requirements. /tmp/stacks-merge-spec-first-main-complete.log, stacks-complete.log and spec-complete.log.
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
pnpm contracts:check
```

**Notes**

295 actual registered HTTP/SSE responses conform to combined generated contracts; seven contract units and generated TypeScript contract checks pass. /tmp/stacks-merge-contracts-check.log.
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

<!-- evidence-field:next markdown -->
## Next

Continue from the integrated Main history; retain the clean Stacks workspace and running source preview. Package publication, release/scale/fault/RustFS and visual-baseline gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Cross-layer merge overlaps justify the affected Rust packages, native Stacks/recovery/registered local-route and admission binaries, directly affected UI/Files stories and controller units, and combined contract/document/specification/progress gates. Root aggregate check/test/build/spec:check lanes, unrelated Rust/UI suites, scale/fault/RustFS/release qualification, package publication and screenshot baselines are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Both completed histories are integrated. Main retains bounded directory/quick-metadata reuse and cached read admission; Stacks retains local TOML, native megamerge/WIP semantics, mutation controls, focused navigation, shared conflict editing, startup skeletons and bounded history chips. Shared bindings retain current registration authorization and cached traversal checks. Original evidence objects are unchanged; generated API manuals now describe the combined route inventory.
<!-- /evidence-field -->

