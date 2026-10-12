---
schema: 1
id: E-REMOTE-AGENTS-001
---

# E-REMOTE-AGENTS-001 details 3

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
node node_modules/@lapismd/spec-validator/dist/cli/index.js validate --only ui-ownership
```

**Notes**

Incorrect executable path during resumed verification; no source or configuration changed. Corrected to the package bin dist/cli.js.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only ui-ownership
```

**Notes**

Production UI ownership validator passes. New components are declared in provenance and import no Community or catalog runtime source.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/web-architecture.mjs
```

**Notes**

Inherited size diagnostics reproduced in /tmp/jj-remote-architecture-final7.log. All new connection files comply; owner has 298 counted lines. Touched production controller is 314 vs parent 316; StoreApplication is 320 on both. No architectural limit or allowlist was weakened.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes after current source/test edits. The linker separately reports an inherited macOS large __eh_frame warning.
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
node scripts/contracts.mjs --write
```

**Notes**

Generated released public OpenAPI/client artifacts. No package version or local override changed.
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
node --test scripts/contracts.test.mjs
```

**Notes**

Eight contract-generation/validation regressions pass; /tmp/jj-remote-contract-node-tests.log.
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
node node_modules/typescript/bin/tsc -p scripts/tsconfig.contracts.json
```

**Notes**

Contract adapter/client compile check passes.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs build
```

**Notes**

Sequential initial failures: stale generated references, stale topic fingerprints, uv absent from PATH and an out-of-site configuration link. Regenerated/reviewed impacted HTTP/application/contracts pages, used installed uv, and corrected the link to generated CLI documentation.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs build
```

**Notes**

Final strict build and documentation link/anchor closure pass: 141 files, 24,592,989 bytes. /tmp/jj-remote-docs-build9.log. Generated local assets are ignored, not source overrides.
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
cargo check -p jj-store-server -p jj-store-http -p jj-store-rpc -p jj-store-service --locked
```

**Notes**

All four affected crate layers compile. /tmp/jj-remote-crate-check-final.log.
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
node scripts/contracts.mjs --check
```

**Notes**

Generated contracts match and 300 actual baseline HTTP/SSE responses conform. New authenticated execution is separately covered by its exact HTTP/admission/schema regressions. /tmp/jj-remote-contracts-final2.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

Initial final ledger validation: five inherited E-NPM-REGISTRY-001 diagnostics, two own unknown UI requirement diagnostics and four own missing new history-file coverage diagnostics. Moved each new requirement definition into its owning chapter and refreshed evidence file coverage; inherited records remain untouched.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only summary,verification
```

**Notes**

Initial final verification found a UI requirement orphan: mixed-width IDs first affected ledger parsing, then the source parser required an owning Requirements heading. Moved each ID into its owning chapter, added headings and restored complete one-line verification rows. Earlier pre-evidence run also found three links awaiting the new record.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

After repairing all M27 references/coverage, only the five inherited E-NPM-REGISTRY-001 diagnostics remain. /tmp/jj-remote-progress-final2.log. No historical record is edited.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first
```

**Notes**

Automatic change-set gate stops on the same inherited ledger errors. Exact source-to-chapter mapping is validated independently with the complete explicit changed-file set; this does not pass the root ledger trust gate.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only summary,verification
```

**Notes**

New definitions now sit in the owning Requirements sections and verification rows remain complete single-line Markdown records. Two source/document validators pass for 188 requirements. /tmp/jj-remote-spec-docs-final3.log.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only ui-ownership,governance,fileLimits
```

**Notes**

Three validators pass, including source ownership, normative governance and all authored Markdown budgets. /tmp/jj-remote-spec-scope-final.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/spec/book-audit.py
```

**Notes**

No generated specification HTML book exists in this new workspace; source summary/link checks and the application documentation closure pass separately. No book acceptance claim is made.
<!-- /run -->
<!-- /evidence-field -->

