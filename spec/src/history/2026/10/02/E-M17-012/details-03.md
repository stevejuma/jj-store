---
schema: 1
id: E-M17-012
---

# E-M17-012 details 3

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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md
```

**Notes**

Formatted canonical common-native, concurrency and provider contracts.
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
cargo test -p jj-store-service --test agent_sessions --test local_agent --test local_agent_binding --test local_agent_pages --locked > /tmp/jj-store-agent-capture-m172-aliases-service-final.log 2>&1
```

**Notes**

Final shared native/local surface: 24 tests pass after shared projection guard and extraction; real companion reopen is separate from native MemoryStore qualification.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-aliases-cli.log 2>&1
```

**Notes**

All 21 direct spool/publication/recovery tests pass. Two subprocess helper entrypoints are ignored in the parent run and explicitly launched by their owning process tests.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-aliases-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass before final ledger/verification traceability additions.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-aliases-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no new runtime/model/storage dependency.
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
cargo clippy -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-aliases-clippy.log 2>&1
```

**Notes**

Strict affected-package lint passes with no suppressions or inherited repairs.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-aliases-format.log 2>&1
```

**Notes**

Pinned Rust formatting passes.
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
rustc --version
```

**Notes**

Verified Rust 1.98.1 (48a229cea 2026-09-01).
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
jj version
```

**Notes**

Verified JJ 0.45.1.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/verification.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

Final canonical and ledger formatting; verification stays Planned with explicit Codex-only evidence and remaining integration gates.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-aliases-progress.log 2>&1
```

**Notes**

129 tasks validate; 267 entries preserve all 266 parent entries and M17.2/aggregate status remains In progress.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-aliases-spec-final.log 2>&1
```

**Notes**

Final five validators, 21 chapters and 160 requirements pass with verification/ledger traceability.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-aliases-first.log 2>&1
```

**Notes**

Spec-first passes for 12 protected files and six owning canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/verification.md spec/src/spec-governance.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-aliases-doc-format.log 2>&1
```

**Notes**

All changed canonical documents pass formatting.
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

Rust 1.98.1; JJ 0.45.1; synthetic Codex CLI 0.156.1/Desktop 0.159.2 sanitized observations on macOS ARM64. Native cold recovery uses MemoryStore; local reopen uses disposable real Git companion metadata. No real harness execution or RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 object, alias, delta, prepared request and receipt shapes remain unchanged. Logical entity targets adopt unambiguous selected aliases; source aliases, event IDs, transcript streams and source receipts remain channel-specific. Codex UUID sessions compare UUID spelling case-insensitively with exact scoped native suffixes. New qualified captures select an additional V1 codex-native-alias-join/v1 projection as a shared cross-profile publication precondition. Legacy prepared deltas and existing IDs remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qnzypmul
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One shared Rust identity/publication behavior affects native and local capture providers. Exact duplicate-channel and cross-profile race failures precede complete direct WAL catalog, native/local service capture/binding/paging and CLI spool/publication/recovery binaries; run affected all-target check and strict lint. Alias scanning is transient, bounded to 4096 alternatives and 8192 index pages, and requires complete proof. Conflicting sessions/tools or unavailable targets retain sanitized evidence, a constant diagnostic and source progress without session/alias writes. No retroactive merge, source registration, directory binding, activity admission, checkpoint or worker authority. Non-UUID labels and other integrations remain source-scoped; messages lacking native IDs remain distinct. Original foreign-source result regression now uses a different UUID session because unambiguous same-native cross-source evidence is intentionally admissible; exact staged/foreign-session isolation remains covered. Common aliases are newly qualified for Codex only; Cursor/OpenCode and larger-history lookup/replay still need acceptance. Source leases/preparation, per-record repository contexts, active backfill, remaining readers, supervised workers/principal and typed controls remain M17.2 gates. Enabled startup stays guarded. M17.5 real harness edits and M17.6/M17.7 remain required. No wire schema, UI lifecycle, real RustFS, installation, daemon restart, publication or root aggregates changed; those lanes are out of scope. Both M17.2 and aggregate M17 remain In progress.
<!-- /evidence-field -->

