---
schema: 1
id: E-INJECTABLE-STORES-006
---

# E-INJECTABLE-STORES-006 details 3

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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:generate
```

**Notes**

Regenerated only stale installation/type manual references from unchanged existing OpenAPI; inspected exact deletion additions. Explicit docs_reference binary was just built from current Rust source by the initial managed check.
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
pnpm docs:review <topic> "specific source review reason"
```

**Notes**

Reviewed publication/local/services/configuration/cli/http/application/contracts and new injectable-stores coverage against actual source and manuals. Exact topic-specific invocations/reasons are in embedding-docs-reviews.log. Existing application review covers previously implemented deletion, tab loading and unique native file ownership, with corresponding manual updates.
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
CARGO_INCREMENTAL=0 JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Strict docs check passed:9Mermaid diagrams in both themes,143files/26626455bytes; all closure links and anchors valid. No extra Rust build or UI lane. embedding-docs-2.log.
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
node --test scripts/docs.test.mjs scripts/docs-assets.test.mjs
```

**Notes**

Complete direct documentation/tooling tests passed14/14, including cold Vite Help replacement tests. embedding-docs-tests.log.
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
node scripts/package-check.mjs
```

**Notes**

All24coordinated packages have crate overviews, registry dependencies and contained compilation inputs. Private embedding fixture remains unpublished.
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
pnpm check:boundaries
```

**Notes**

All23workspace package boundaries passed.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

31tracked Cargo crate roots checked;0missing README files and0broken first-party relative links. Vendor documentation preserved.
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
rustfmt --edition 2024 --check crates/jj-store-jj/tests/fixtures/store-consumer/src/main.rs crates/jj-store-jj/tests/fixtures/store-consumer/src/git.rs crates/jj-store-jj/tests/fixtures/store-consumer/src/native.rs
```

**Notes**

All independent consumer Rust sources pass formatting.
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
pnpm exec prettier --write <changed Markdown/JavaScript/JSON paths excluding generated API references>
```

**Notes**

Formatted actual changed authored documentation, fixture and tooling inputs. Generated API references retain exact generator output.
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
pnpm dev:disk
```

**Notes**

Before retirement:free38.3GiB/root28.9GiB/isolated2.3GiB. Reserve unchanged; dev fixture and staged resources protected.
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
pnpm dev:target:retire -- --target .local/build-targets/store-consumer --apply
```

**Notes**

Isolated target removed successfully; free38.2-&gt;40.5GiB. No source, fixture, retained public-release target or staged runtime removed.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search -- "JST-ARCH-006"
```

**Notes**

Installed CLI rejects literal -- forwarded by pnpm. Retried without separator.
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
pnpm spec:search JST-ARCH-006
```

**Notes**

QMD index refresh unavailable because native modules do not match active Node ABI137. Used explicit supported rg fallback; dependency reinstall is outside this store-library task.
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
rg -n -i --glob '*.md' 'JST-ARCH-006' spec/src/current/contracts/architecture
```

**Notes**

Fallback found owning03-requirements and25-injectable-collaboration-stores; both canonical sources read. QMD helper gap does not replace specification validation or runtime gates.
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
CARGO_INCREMENTAL=0 pnpm dev:backend:build
```

**Notes**

Current server and paired custom JJ binaries built successfully in1m33s. Apple linker emitted its large __eh_frame warning; no build failed. No installed CLI replacement.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/desktop/resources/jj-store-rustfs pnpm dev:ui
```

**Notes**

Restored original Vite mode: UI7460/backend7462, retained .local/dev-demo fixture and ready search. Bundled ACP/control-v3 resources pass normal preflight. No reset/reseed or unrelated process changes.
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
node --input-type=module (retained fixture digest and same-origin development health verification)
```

**Notes**

Fixture SHA256 and repository identity exactly unchanged. Owned server/Vite PIDs live. UI, admitted LocalTrusted bootstrap and retained repository snapshot each return200. dev-restore-verification.log.
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
CARGO_INCREMENTAL=0 JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Final post-format strict documentation rerun passed:143files/26626455bytes, all links and anchors valid. embedding-docs-final.log.
<!-- /run -->
<!-- /evidence-field -->

