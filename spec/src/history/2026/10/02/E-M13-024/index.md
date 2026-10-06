---
schema: 1
id: E-M13-024
---

# E-M13-024

<!-- evidence-field:id markdown -->
## Id

E-M13-024
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

wmwwmqsp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Local-completion on vtqozuvm/bb24714d with physical Save yyknxszk awaiting inherited Storybook acceptance. V2 portable schema has code-only targets and no retained-agent read validation. New exact portable_agent regression fails compilation with18 missing APIs.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust1.98.1; JJ0.45.1; Git SHA-1; existing locked dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive V2 AgentSession, AgentToolArtifact and AgentToolText variants bind actual retained session/tool/content records, without invented attempts or message/block IDs. V1 native types, bytes, stored commands and codecs unchanged. Structural reader must be backed by qualified canonical selected records; no publication capability enabled. Portable code evidence binds anchor repository and both diff sides.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-review/src/lib.rs
- crates/jj-review/src/portable.rs
- crates/jj-review/src/portable_agent.rs
- crates/jj-review/tests/portable.rs
- crates/jj-review/tests/portable_agent.rs
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/portable_agent.rs#portable_tool_anchor_uses_retained_invocation_without_invented_attempts
- crates/jj-review/tests/portable_agent.rs#portable_tool_requires_exact_membership_and_both_stable_ids
- crates/jj-review/tests/portable_agent.rs#portable_tool_text_validates_part_exact_content_and_utf8_digest
- crates/jj-review/tests/portable_agent.rs#legacy_native_anchor_bytes_and_identity_remain_readable
- crates/jj-review/tests/portable.rs#portable_code_evidence_binds_the_anchor_repository_and_complete_diff_pair
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-review --test portable_agent
```

**Notes**

Before implementation: new retained-agent regression cannot compile; missing additive types/variants/read validation. /tmp/local-completion-portable-agent-before.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-review --test portable_agent --test portable
```

**Notes**

Initial nine cases pass; unused fixture variable warning found and subsequently repaired. /tmp/local-completion-portable-agent-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-review (first full run)
```

**Notes**

An over-broad warning cleanup removed a used fixture binding; two compile diagnostics. Binding restored and actual unused binding removed; no diagnostics suppressed. /tmp/local-completion-portable-agent-suite.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-review (final full run)
```

**Notes**

26 model tests pass including unchanged V1 anchors/objects, delta/resolver regressions, exact tool membership/content, no attempts, unavailable/wrong records, canonical decoding and cross-repository evidence. No warnings. /tmp/local-completion-portable-agent-suite2.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-review
```

**Notes**

Owning crate and existing dependencies pass. /tmp/local-completion-portable-agent-check.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy --locked --no-deps -p jj-review --all-targets -- -D warnings
```

**Notes**

Owning model crate, tests and targets pass with no diagnostics. /tmp/local-completion-portable-agent-clippy.log.
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
cargo fmt --all; pnpm exec prettier --write spec/src/collaboration.md spec/src/local-jj.md
```

**Notes**

Pinned Rust formatter and canonical prose formatter pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-review --test portable_agent legacy_native_anchor_bytes_and_identity_remain_readable
```

**Notes**

Fixed original V1 canonical bytes and original anchor SHA-256 identity remain readable. /tmp/local-completion-portable-agent-golden.log.
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

120 tasks; evidence bookkeeping only. /tmp/local-completion-portable-agent-progress.log.
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
pnpm spec:first
```

**Notes**

Five protected model files, three owning canonical chapters. /tmp/local-completion-portable-agent-spec-first.log.
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
pnpm spec:validate
```

**Notes**

Five validators,21 chapters,146 requirements. /tmp/local-completion-portable-agent-validate.log.
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
cargo fmt -p jj-review -- --check
```

**Notes**

Owning Rust format check passes. /tmp/local-completion-portable-agent-fmt-check.log.
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

<!-- evidence-field:scope markdown -->
## Scope

Portable anchor model and pure retained-agent structural validation only. Existing portable V2 has no production consumers yet (source inventory inspected); full jj-review suite covers additive schema/direct model consumers. HTTP/UI/RustFS/root aggregates outside this model-only surface. Browser publication remains gated; the physical Save pending qualification is retained separately. Visual baselines unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Wire additive portable anchors through stored ReviewStore commands/audits, complete selected Git/native evidence retention and historical context before enabling browser actions; continue pending search, Snapshot/recoverable code commands, conversion and M13.6. Local sync remains excluded.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


