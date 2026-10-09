---
schema: 1
id: E-CHAT-METADATA-001
---

# E-CHAT-METADATA-001 details 2

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
cargo test -p jj-collab --test conversation_metadata --locked
```

**Notes**

Three passed; the new tombstone test attempted an invalid Created-to-Tombstoned transition. Corrected the fixture to activate its owner first; production lifecycle policy was retained.
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
cargo test -p jj-collab --tests --locked
```

**Notes**

45 tests passed across seven integration binaries. Legacy canonical root bytes, immutable owners, provenance, bounded fields, duplicate source rejection and existing comment/reaction behavior retained. Log: /tmp/metadata-model-family-final.log.
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
cargo test -p jj-store-server --lib metadata_text --locked
```

**Notes**

Three tests passed: visible content excludes reasoning/system/tool bodies; rich OpenCode parts include only declared visible text; UTF-8 truncation retains scalar boundaries. Unknown JSON is not guessed. Log: /tmp/metadata-text-final.log.
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
cargo test -p jj-store-cli --test agent_execution metadata_tests --locked
```

**Notes**

Final four tests passed in 180.80 seconds. Real local companion publication with controlled ACP/helper proves automatic generation, durable ACP suggestions, custom refresh/regeneration, cancellation, no read/history/settings backfill, aliases, duplicate identities, malformed-result retention and restart without inference replay. Log: /tmp/metadata-integration-release.log.
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
cargo test -p jj-store-cli --test agent_execution metadata_stale_generation --locked
```

**Notes**

Final exact source/recovery test passed in 163.45 seconds. Gate-controlled inference retained its result after a concurrent rename. An injected interrupted-publication journal recovered a definitive stale refusal; explicit Publish kept the newer custom title without a second inference. A foreign session/snapshot pair was refused by real local publication without changing selection. Log: /tmp/metadata-source-recovery-final.log.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml --test metadata --locked
```

**Notes**

Four final helper tests passed, including a 120,000-byte transcript whose JSON escapes exceed the old wire limit. No tool request or interactive auth; strict malformed/fenced rejection is retained. Log: /tmp/metadata-helper-escaped-final.log.
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
cargo test --manifest-path runtime/local-agent/vendor/buzz-agent/Cargo.toml metadata_summary_never_retries --locked
```

**Notes**

Final single-attempt metadata transport test passed; coding retries remain unchanged. Log: /tmp/metadata-once-final.log.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked
```

**Notes**

Affected Rust package checks and direct consumers passed. No full Rust workspace or root aggregate lane was run. Log: /tmp/metadata-rust-check-final.log.
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
cargo build -p jj-store-cli --bins --locked
cargo build --manifest-path runtime/local-agent/Cargo.toml --bin jj-store-local-agent --locked
```

**Notes**

Built the isolated custom jj/server and tool-free companion used by acceptance. Existing deployed binaries were not replaced. Linker retains the inherited __eh_frame size warning.
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
cargo fmt --all
cargo fmt --manifest-path runtime/local-agent/Cargo.toml
rustfmt --edition 2024 runtime/local-agent/vendor/buzz-agent/src/llm.rs runtime/local-agent/vendor/buzz-agent/src/lib.rs
```

**Notes**

Formatted owned Rust and maintained vendor changes. No remote package publication or global installation.
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
cargo test -p jj-store-cli --test agent_execution metadata_does_not_backfill --locked
```

**Notes**

Final exact cancellation/restart test passed in 124.63 seconds. Explicit Stop waiting abandons an unknown inference only before publication, retains its original job and starts no model replay. Log: /tmp/metadata-abandon-final.log.
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
cargo test -p jj-store-server --lib metadata_text --locked
```

**Notes**

Four tests passed in the final authored-text projection, including jj-store.sent-context.v1.text with bootstrap and inspector context excluded. Unknown JSON remains unguessed. Log: /tmp/metadata-authored-text-final.log.
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
cargo test -p jj-store-cli --test agent_execution metadata_generates_once --locked
```

**Notes**

Exact final automatic-generation test passed in 171.79 seconds using the actual retained human envelope. Provider input contains authored text and excludes bundled CLI bootstrap, tool bodies and inspector context. Log: /tmp/metadata-authored-integration.log.
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
rustfmt --edition 2024 --check runtime/local-agent/vendor/buzz-agent/src/llm.rs runtime/local-agent/vendor/buzz-agent/src/lib.rs
```

**Notes**

Maintained vendor sources pass explicit formatting checks; the separately owned runtime manifest does not include these sources in its cargo fmt traversal.
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

rustc 1.98.1 (48a229cea); custom jj 0.45.1; Python 3.14.7; Node v26.9.0; pnpm 10.32.1. Base 61df439c; enclosing Jujutsu change opvnwwnn.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

ThreadRoot optional metadata catalog; immutable ConversationMetadataV1; exact AgentStore SessionSnapshot references; version-1 metadata commands/jobs/helper JSON; legacy absent fields retain canonical bytes and request digests.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Backend/model acceptance for repository-backed conversation metadata. ThreadStore remains canonical; native manifest CAS and local companion ref/receipts select metadata. Captures remain in AgentStore. No duplicate human messages, primary Thread reassignment, ACP regeneration method, coding tools, instruction-only turn or inference on GET/restart. Dedicated provider configuration defaults unconfigured. Native/local built-browser evidence is recorded separately in E-CHAT-METADATA-002. JSON wire admission accounts for escape expansion while retaining the 128-KiB decoded bound. Explicit abandonment of unknown inference never discards unknown publication.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Integrated native/local cold flow and discovery qualification are recorded in E-CHAT-METADATA-002. Retain independent real-provider, transfer and platform gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Extend JST-COLLAB-034 through JST-COLLAB-036, JST-API-001 and
  JST-CLI-004 with optional immutable metadata, exact source provenance,
  independent bounded inference and receipt recovery. Owning metadata chapters
  accompany the implementation.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This model/local-publication proof does not
  close live-provider, additional adapter, native IME, distribution, transfer or
  visual approval gates.

```
<!-- /evidence-field -->

