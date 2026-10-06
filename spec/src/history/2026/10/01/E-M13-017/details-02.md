---
schema: 1
id: E-M13-017
---

# E-M13-017 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-http --locked
```

**Notes**

Portable type imports were not owned service reexports; inline inferred fields avoid an invalid jj-agent dependency. A delimiter error was corrected during the edit.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-http --locked
```

**Notes**

HTTP Error cannot be embedded directly in an io stream error; changed to bounded neutral freshness-unavailable text.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-http --locked
```

**Notes**

AgentStore routes and local freshness compile.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --locked -- browser_agent_presentation_pins_metadata_and_transcript_membership --exact
```

**Notes**

Test passed a model GitSha1Id instead of the companion GitOid; corrected.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --locked -- browser_agent_presentation_pins_metadata_and_transcript_membership --exact
```

**Notes**

Zero page limit surfaced a lower-layer error; added explicit API bound validation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_agent_provenance --locked -- browser_associations_preserve_source_identity_and_pinned_pages --exact
```

**Notes**

Source repository, evidence capsules, 40-character Git commits, filtered pages and independent old pin pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --test local_agent_pages --test agent_sessions --test agent_relations --test agent_checkpoints --locked
```

**Notes**

10 local and 7 native tests pass, including pinned sessions/transcripts, staged-selection rejection and timing/activity presentation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

Fixture request reused a WorkStore request ID; domain conflict correctly rejected it. Assigned a distinct capture request.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

Real filesystem HTTP AgentStore sessions/activity/transcript/relations and SSE after external JJ code change pass; old metadata pin stays empty.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Generated additive local AgentStore and SSE schemas.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

13 HTTP unit and contract inventory tests pass.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm spec:first
```

**Notes**

Mutation chapter mapped to CLI and query transport updated.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first && pnpm check:progress && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

Spec-first passes; ledger used invalid in-progress result. Corrected to pending until qualification finishes.
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
cargo fmt --all
```

**Notes**

Rust format updated.
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
pnpm exec prettier --write spec/src/{local-jj,collaboration,api,compatibility,cli,milestones}.md
```

**Notes**

Canonical files formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

114 actual HTTP/SSE responses conform; 6 contract tests and contract TypeScript pass.
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
pnpm spec:first && pnpm check:progress && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

22 protected files, 7 chapters, 111 tasks and 132 requirements pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

