---
schema: 1
id: E-M17-071
---

# E-M17-071 details 2

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
cargo test -p jj-agent --test opencode_snapshots --locked
```

**Notes**

Initial fixture used nonexistent normalize_observations; corrected to normalize_batch. Logs /tmp/jj-store-opencode-snapshots-before.log and /tmp/jj-store-opencode-snapshots-after.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-agent --test opencode_snapshots --locked
```

**Notes**

Corrected before-state with native dispatch temporarily removed: one pass, two failures. Dispatch restored immediately. /tmp/jj-store-opencode-snapshots-corrected-before.log.
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
cargo test -p jj-agent --test normalizers --test opencode_snapshots --locked
```

**Notes**

Seven existing and four new normalizer tests. /tmp/jj-store-opencode-normalizers-verified.log.
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
cargo test -p jj-store-capture --test opencode_snapshot --locked
```

**Notes**

Three bounded snapshot parser tests. /tmp/jj-store-opencode-acquisition-verified.log.
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
cargo test -p jj-store-server --test opencode_read_api --locked
```

**Notes**

Five real-loopback controlled HTTP acquisition tests. /tmp/jj-store-opencode-api-verified.log.
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
cargo check -p jj-store-server --lib --locked
```

**Notes**

Direct server consumer check. /tmp/jj-store-opencode-api-check.log.
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
cargo clippy -p jj-agent -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Two test filter(...).last() calls rejected; corrected to rfind. /tmp/jj-store-opencode-clippy.log.
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
cargo clippy -p jj-agent -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Final strict three-package lint. /tmp/jj-store-opencode-clippy-final.log.
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
cargo fmt --all
```

**Notes**

Rust formatting completed.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries.
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
pnpm check:architecture
```

**Notes**

732 production modules.
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
pnpm check:progress
```

**Notes**

129 tasks; evidence bookkeeping only.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Three changed documents required formatting; formatted prose/table and only the new evidence, preserving the 325 historical objects and raw JSON prefix.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Owning chapter and verification narrative formatting.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final changed-document check; all 325 historical evidence objects and raw prefix preserved against parent.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; pinned OpenCode 1.18.30 source 3104c1428ec91f809e5ab86631300de41eb6952e.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing repository/wire/spool formats unchanged. Sanitized native envelope opencode-snapshot/v1; memory-only native pages and paths lack Debug/Serialize.
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

Bounded acquisition and normalization only. Final selected set: 19 tests. Explicit busy/retry activity, owner/version/part validation, per-assistant cwd, read-only credential-free loopback GETs, bounded opaque pagination and refusal of redirects, overflow, changed directories and cycles. Startup still accepts only Codex profiles; OpenCode source registration, sanitization/durable progress, supervision, streaming export/history import and live edit acceptance remain pending. Default-instance discovery is not cross-workspace discovery. M17/M17.2 remain In progress; M17.3-M17.7 remain Planned. No public contract regeneration, UI lanes, root aggregates or unrelated Rust suites needed. All 325 previous evidence objects and raw JSON prefix preserved.
<!-- /evidence-field -->

