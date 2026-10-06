---
schema: 1
id: E-M17-009
---

# E-M17-009 details 3

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
cargo check -p jj-agent -p jj-store-capture -p jj-store-wal -p jj-store-service -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-codex-normalization-check.log 2>&1
```

**Notes**

Affected model/runtime/projection libraries and direct service/server/CLI consumers check.
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
cargo clippy --no-deps -p jj-agent -p jj-store-capture -p jj-store-wal -p jj-store-service -p jj-store-server -p jj-store-cli --lib --test codex_transcript --test transcript_sanitization --test agent_catalog --test agent_hook --test agent_kingfisher --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-codex-normalization-clippy.log 2>&1
```

**Notes**

Strict clippy passes for affected libraries and direct regression binaries.
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
cargo fmt --all
```

**Notes**

Rust formatting applied.
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

Final Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/compatibility.md
```

**Notes**

Owning and compatibility chapters formatted.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm check:boundaries
```

**Notes**

All 17-package boundaries pass.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final evidence ledger formatted.
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
pnpm check:progress
```

**Notes**

129 tasks validate; all 263 parent evidence entries remain unchanged.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

Final changed chapter/ledger formatting passes.
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

JJ 0.45.1; Rust/Cargo 1.98.1; MongoDB Kingfisher 2.7.0 pinned source 63dccba231b6b01990c84319d25f92f49428a431; synthetic CLI 0.156.1 and Desktop 0.159.2 native fixtures on macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New explicit sanitized codex-jsonl/v1 envelope and transcript-paths/v1 sanitizer policy. Additive Desktop normalizer profile uses existing schema 1. Native messages retain IDs; missing IDs use source-message identity and no native alias. Existing hook policy/bytes, repository object kinds/encodings, native/local prepared requests, receipts and wire schemas unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pvklnmrl
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared redaction helper and public normalizer profile/parser change affect models, capture runtime and native/local projection consumers. Run owned model suite plus final native tests, complete direct WAL capture projection binary, runtime source/progress/sanitizer tests, CLI native/local spool/recovery/hook/redaction binaries and affected checks/clippy. Cold native and hook process acceptance is required and executed; stored-policy/receipt semantics and root pinning remain verified. No HTTP/CLI route shape, public schema, UI, real RustFS or root aggregate change; those lanes are out of scope. This slice qualifies sanitization and basic native messages/lifecycle/request projection only. Future M17.2 gates remain result enrichment, verified common aliases, source-context/repository partitioning, approved locations, active-only backfill, other harness readers, leases, supervised worker and controls. Unsupported tool results/rich blocks retain sanitized evidence and diagnostics; no execution/success is invented. M17.3 owns rich conversation/anchor/replay gates and M17.5 owns real harness edit qualification. Enabled server startup deliberately remains unavailable until qualified composition.
<!-- /evidence-field -->

