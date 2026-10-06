---
schema: 1
id: E-M17-015
---

# E-M17-015 details 3

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
cargo test -p jj-store-cli --test local_browser local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads --locked -- --exact > /tmp/jj-store-agent-capture-m172-context-local-browser-test-before.log 2>&1
```

**Notes**

The original local reader acceptance test passes before syntax-only repair.
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
cargo fmt -p jj-store-capture -p jj-store-server -p jj-store-cli
```

**Notes**

Final formatting including unchanged-condition CLI test repair.
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
cargo clippy -p jj-store-cli --test local_browser --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-context-local-browser-after.log 2>&1
```

**Notes**

Exact lint passes after collapsing the existing conditions without changing branches or assertions.
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
cargo test -p jj-store-cli --test local_browser local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads --locked -- --exact > /tmp/jj-store-agent-capture-m172-context-local-browser-test-after.log 2>&1
```

**Notes**

The original disposable real local-JJ reader acceptance test still passes after repair.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-context-final-clippy-after.log 2>&1
```

**Notes**

Final strict all-target lint passes for runtime, server adapter and direct CLI consumer.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-context-boundaries.log 2>&1
```

**Notes**

Owned Rust dependency boundaries pass with no new dependency or upstream JJ import.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-context-format.log 2>&1
```

**Notes**

Rust formatting passes; final repeat includes CLI syntax repair.
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
cargo test -p jj-store-capture --test codex_context --locked > /tmp/jj-store-agent-capture-m172-context-desktop.log 2>&1
```

**Notes**

All six final context tests pass, including independent Desktop 0.159.2 header/context qualification and CLI-header rejection.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-context-doc-write.log 2>&1
```

**Notes**

Canonical documents formatted; partial verification rows stay Planned and M17.2/aggregate stay In progress.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-context-progress.log 2>&1
```

**Notes**

129 tasks validate; 270 evidence entries preserve all 269 parent evidence structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-context-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-context-first.log 2>&1
```

**Notes**

Spec-first passes for 13 protected source/test files and three canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-context-doc-format.log 2>&1
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; independent synthetic Codex CLI 0.156.1 and Desktop 0.159.2 directory-context profiles. Disposable native directory/socket verifier fixtures and real local-JJ code-read acceptance remain distinct from a real harness edit and RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 native/companion objects, source receipts, pending capture, prepared request and per-repository reader journal encodings are unchanged. Memory-only Codex batches add shared immutable directory context and split at context changes. Source-wide progress validates retained journals/pending entries across repository partitions; latest cursor chooses validated generation/sequence without rewriting prior receipts or accepted offsets. Raw paths never become journal metadata.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change yqnuuspx
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One directory-routing behavior affects capture source reading/sanitization, shared native/local spool durability and direct server admission. Exact new context regression precedes complete direct source/discovery/lease/progress/privacy binaries, server admission, CLI spool/publication/recovery and actual native workspace verifier consumers; run owning/direct-consumer all-target checks and strict lint. Prefix context reconstruction reuses bounded verified bytes rather than re-emitting history. Only explicit qualified header/turn cwd changes context; malformed/oversized records withhold context until a valid turn and repeated headers create a sticky generation gap. Unknown bodies cannot enter native sanitization; constant gap conversion retains progress to a separately authorized anchor. Batch admission uses current native/socket/catalog proofs and still grants no write authority. Cross-repository journal history is source-wide, while every existing repository journal/prepared request is retained and repaired before completion. Latest-source lookup and admission include pending sanitized evidence and reject contradictory equal offsets/witnesses/parser identities; no capacity eviction. Shared progress extraction retains public methods and legacy encoding/generation interpretation. One unrelated pre-existing CLI test lint exposed by required direct-consumer all-target lint is reproduced on the exact target, mechanically repaired and qualified with the original real local-JJ reader test; no assertions or product behavior change. This slice does not implement active-only admission/backfill/offline-completion, remaining harness readers, installation capture principal, fair supervised workers or typed controls. Enabled server preparation remains guarded. M17.2 and aggregate M17 remain In progress; M17.3-M17.7 and M17.5 real harness edits remain required. No wire schema, native/WAL publication policy, UI startup/lifecycle, installed service, real RustFS, deployment or root aggregate changed; those lanes are out of scope.
<!-- /evidence-field -->

