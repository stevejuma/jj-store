---
schema: 1
id: E-M17-075
---

# E-M17-075 details 3

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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Affected canonical documents, governance or formatting only.
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
pnpm spec:validate
```

**Notes**

Affected canonical documents, governance or formatting only.
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
pnpm check:boundaries
```

**Notes**

Affected canonical documents, governance or formatting only.
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
pnpm check:architecture
```

**Notes**

Affected canonical documents, governance or formatting only.
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
cargo fmt --all -- --check
```

**Notes**

Affected canonical documents, governance or formatting only.
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
node /tmp/jj-store-capture-ledger.mjs /tmp/jj-store-e-m17-075.json
```

**Notes**

Final ledger raw-prefix/history protection and changed-document bookkeeping.
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
pnpm check:progress
```

**Notes**

Final ledger raw-prefix/history protection and changed-document bookkeeping.
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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final ledger raw-prefix/history protection and changed-document bookkeeping.
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
jj --no-pager diff --check
```

**Notes**

JJ 0.45.1 has no diff --check option; this unsupported inspection did not validate or mutate files. Targeted diff review, Rust formatting and Prettier supply the final checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; OpenCode 1.18.30 pinned source 3104c1428ec91f809e5ab86631300de41eb6952e; reused pinned Kingfisher cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional typed opencode_snapshot private reader metadata under existing schema 1; None encodings remain byte-compatible. Composite position binds exact witness, message prefix/count and busy/retry/idle class. Only memory indirection changes Codex ready representation. Current AgentStore only; no repository/wire/source-catalog format change.
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

183 selected top-level tests pass (147 capture reader/library and 36 server boundary/API tests), with one fresh-process sanitizer child counted separately. Controlled loopback API, installation-private real files and native MemoryStore qualify observed lifecycle/restart and retained supervision only. Empty sessions, busy/retry/idle, historical statuses and per-message lifecycle, exact original generation recovery above newer allocation floors, rewrite refusal, deduplication, spool pressure and revocation are covered. This shared progress and polling representation has two direct Rust harness consumers; full selected reader/progress/publication/sanitization binaries and direct native consumers are required, plus exact API owned-lane regressions. Root aggregates, full server/workspace libraries, unrelated Rust/UI, public contracts and real harness/RustFS release remain out of scope. M17/M17.2 In progress; directory-aware discovery, management/import/export, common hook/API identities and Cursor still pending, followed by M17.3-M17.7. All 329 historical evidence objects and their raw JSON prefix preserved.
<!-- /evidence-field -->

