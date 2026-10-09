---
schema: 1
id: E-CHAT-WORKSPACE-003
---

# E-CHAT-WORKSPACE-003 details 2

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
pnpm contracts:check
```

**Notes**

Generated contracts match; 300 actual HTTP/SSE responses conform and seven JavaScript checks pass.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed canonical documents>
```

**Notes**

Four changed canonical chapters needed formatting.
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
pnpm exec prettier --write spec/src/current/contracts/cli/35-interactive-agent-execution.md spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md spec/src/current/contracts/compatibility/13-interactive-agent-execution.md spec/src/current/contracts/publication/10-m15-additive-collaboration-publication.md
```

**Notes**

Formatted the four changed chapters.
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
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Initial additive variant exposed an exhaustive conversion match. Added explicit unsupported-carrier refusal.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Intermediate qualified-reader draft used an unavailable model import and private workspace reader; fixed visibility and the owning model reexport.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-collab --test target_resolution --test comment_reply
```

**Notes**

Initial new test omitted the CodeArtifact box; corrected the fixture before rerunning.
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
cargo test --locked -p jj-store-service --test native_collab --test local_collab --test conversion_agent_discussion
```

**Notes**

26 direct consumer tests pass: 10 native, 15 local and one conversion guard.
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
cargo test --locked -p jj-store-server --lib agent_execution::prompt_context::tests
```

**Notes**

Five prompt-context framing, sanitizer and retained-context tests pass. Existing macOS linker __eh_frame warning remains.
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
pnpm catalog:check
```

**Notes**

Zero type errors/warnings; 349 catalog modules satisfy their ownership boundary.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI validator passes.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Evidence pagination added details-02 after history generation. Regenerated navigation and restored unrelated generated whitespace.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final progress, protected-file coverage and seven specification validators pass after the mapped contract and navigation repairs.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress && pnpm spec:first
```

**Notes**

Progress passed; spec-first required the mapped publication, mutations, CLI and compatibility chapter reviews. Added explicit authority/encoding semantics and unchanged CLI confirmation.
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
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Initial in-progress qualified reader imported a crate not linked by the server. Replaced it with the owned service model reexport.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test local_review_files
```

**Notes**

New test used thread instead of the public result version field. Corrected the fixture.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-wal --test collab_commands
```

**Notes**

New test incorrectly assumed the native fixture has no ReviewStore root. Corrected it to assert the existing root remains unchanged.
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
cargo test --locked -p jj-collab --test target_resolution --test comment_reply
```

**Notes**

14 target-resolution and four protocol tests pass. Repository-qualified replies require exact delivered references; permission loss and omitted references refuse linking. Earlier revisions retain their own links.
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
cargo test --locked -p jj-store-cli --test local_review_files
```

**Notes**

Two isolated flows pass. A separate local conversation home references retained source evidence after cleanup and Git GC; generic publication/context refuse without authority, explicit authority publishes, replay is stable, and no review is copied.
<!-- /run -->
<!-- /evidence-field -->

