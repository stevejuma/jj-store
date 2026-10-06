---
schema: 1
id: E-M17-079
---

# E-M17-079 details 2

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
cargo test -p jj-store-wal --test agent_catalog opencode_aliases::opencode_hook_api_and_export_adopt_one_selected_session_message_and_tool --locked -- --exact
```

**Notes**

Initial fixture failed to compile because CaptureSourceStateV1 was unqualified; corrected before the behavioral before observation. /tmp/jj-store-opencode-join-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test agent_catalog opencode_aliases::opencode_hook_api_and_export_adopt_one_selected_session_message_and_tool --locked -- --exact
```

**Notes**

Qualified before observes two selected logical sessions after independent OpenCode hook/API records instead of the required one. /tmp/jj-store-opencode-join-before-qualified.log.
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
cargo test -p jj-store-wal --test agent_catalog opencode_aliases::opencode_hook_api_and_export_adopt_one_selected_session_message_and_tool --locked -- --exact
```

**Notes**

Same regression passes after shared exact-native adoption. /tmp/jj-store-opencode-join-after.log.
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
cargo test -p jj-store-wal --test agent_catalog opencode_aliases:: --locked
```

**Notes**

Four regressions pass: common session/message/tool identities, case/generic labels, selected common precondition, and prior duplicate alias diagnostics/source receipts. /tmp/jj-store-opencode-join-regressions.log.
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
cargo test -p jj-store-service --test local_agent opencode_aliases:: --locked
```

**Notes**

One physical JJ/Git companion reopen/receipt/pinned-root regression passes. /tmp/jj-store-opencode-join-local-regression.log.
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
cargo fmt -p jj-store-wal -p jj-store-service
```

**Notes**

Pinned Rust formatter for changed production/test files.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/compatibility.md
```

**Notes**

Owning canonical requirements formatted before implementation acceptance.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-wal -p jj-store-service -p jj-store-server --test agent_catalog --test bound_capture --test local_agent --test local_agent_binding --test local_agent_pages --test local_collab --test opencode_source --test opencode_publication --locked
```

**Notes**

69 pass across all eight selected native/local/API consumer binaries. Includes all new identity regressions; no full repository claim. /tmp/jj-store-opencode-join-consumers.log.
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
cargo test -p jj-store-wal --lib publication::agent_capture_aliases:: --locked
```

**Notes**

One complete-scan/budget unit passes; incomplete lookup grants no partial identity proof. /tmp/jj-store-opencode-join-lookup-units.log.
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
cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Strict affected packages and all-target consumers pass. /tmp/jj-store-opencode-join-clippy.log.
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
cargo fmt -p jj-store-wal -p jj-store-service -- --check
```

**Notes**

Changed Rust production and tests formatted.
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
pnpm check:boundaries
```

**Notes**

All 17 package boundaries pass.
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
cargo test -p jj-store-service --lib local_repository::agent_descriptor::tests:: --locked
```

**Notes**

Two current-only descriptor tests pass, including obsolete V1 rejection. /tmp/jj-store-opencode-join-current-descriptor.log.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/compatibility.md spec/src/local-jj.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Owning chapters and verification narrative formatted.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/compatibility.md spec/src/local-jj.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Changed canonical formatting passes.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm check:progress
```

**Notes**

129 tasks pass; evidence bookkeeping only.
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
pnpm spec:first
```

**Notes**

Six protected files and six canonical chapters pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; OpenCode 1.18.30 normalization profile.
<!-- /evidence-field -->

