---
schema: 1
id: E-REUSABLE-AGENTS-012
---

# E-REUSABLE-AGENTS-012 details 2

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
jj file list -r @- packages/jj-history
```

**Notes**

Before: no existing typed native history client; prior wire precision proof is retained in E011.
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
node scripts/history-contracts.mjs --write
```

**Notes**

Initial generator rejected root metadata keywords passed as undefined fields; corrected by removing root metadata before structural generation.
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
node scripts/history-contracts.mjs --write
```

**Notes**

Generated types and runtime schema from owned checked Rust producer.
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
pnpm install --offline --ignore-scripts
```

**Notes**

Added only the new workspace importer with released lossless-json 4.3.1 and TypeScript 5.9.3; no existing dependency version changed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm history:check
```

**Notes**

Initial TS schema cast rejected optional fields inferred for schema unions; narrowed owned JSON inventory through unknown before interpreting its explicit Schema type.
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
pnpm history:check
```

**Notes**

Final schema drift and package type check pass.
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
pnpm history:test
```

**Notes**

Initial six runtime cases: five pass / one duplicate-frame rejection fails. Existing parser accepted repeated equal keys. Implemented the bounded integer-only native frame decoder; no dependency patch or field-name workaround.
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
pnpm history:test
```

**Notes**

Final seven runtime cases and public type-negative assertions pass, including full i64/u64 bounds, Unicode, escaped keys, __proto__ preservation, malformed/duplicate/oversized inputs, scope/commands, no retry, close failures and query response matching.
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
node --test scripts/history-client.test.mjs
```

**Notes**

Initial three schema/archive/cold-install cases pass; source then formatted before final normal-governance artifact qualification.
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
pnpm test:governance
```

**Notes**

Initial 87 pass; added complete new package runtime/type suite to the owning archive regression so ordinary CI runs it, rather than only archive samples.
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
pnpm test:governance
```

**Notes**

Final 88 pass. Includes all seven package runtime cases and public type assertions, cold source install and independent archive runtime/type acceptance. Final archive sha256 67f44c66b2f8fee6b1ff1f6d6aca8165e119b4afde10c5b427cc52b959e0a631. No source aliases or copied DTOs.
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
node --test scripts/ci*.test.mjs scripts/release.test.mjs scripts/docs.test.mjs scripts/spec/*.test.mjs
```

**Notes**

89 pass; widened because CI selection, source mapping and generated-file size handling affect shared validation infrastructure. No UI or native application runtime consumed the new client yet.
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
pnpm check:architecture
```

**Notes**

Three original UI size failures; no new authored-source failure. Exact generated type path is checked for source drift separately.
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
node .local/reusable-conversations/history-client-parent-architecture.mjs
```

**Notes**

Parent checker on current tree reports the same three failures plus new generated artifact. All three UI source files byte-equal parent; current checker differs only by exact generated-artifact exclusion.
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
node scripts/check-boundaries.mjs
```

**Notes**

25 package boundaries pass.
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
node scripts/package-check.mjs
```

**Notes**

26 Cargo packages pass contained input and metadata checks; no Rust source change.
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
pnpm docs:review history-client 'Reviewed generated native-schema types and runtime validation, bounded integer-only decoding, arbitrary record keys, immutable vault scope, explicit closure, transport errors and independent archive/type acceptance without Tauri or source aliases.'
```

**Notes**

Owning source review refreshed; coverage includes generator and owned schema.
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
node scripts/docs.mjs render
```

**Notes**

144 files / 26,694,900 bytes, nine offline diagrams, links and anchors valid.
<!-- /run -->
<!-- /evidence-field -->

