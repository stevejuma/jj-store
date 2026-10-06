---
schema: 1
id: E-M17-072
---

# E-M17-072 details 2

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
cargo test -p jj-store-capture --test opencode_ingress --locked
```

**Notes**

Before implementation: missing source-binding, registration and snapshot-sanitization interfaces (20 compiler diagnostics). /tmp/jj-store-opencode-ingress-before.log.
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
cargo test -p jj-store-capture --test opencode_ingress --locked
```

**Notes**

Initial implementation: one pass, two failures. Tests incorrectly searched JSON byte-array payload encoding for readable omission reason/redaction tags; changed to inspect payload.bytes(), retaining all privacy assertions. Cold uncached sanitizer setup took 83.37s. /tmp/jj-store-opencode-ingress-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test opencode_ingress --locked missing_context_keeps_only_safe_omission_and_does_not_skip_later_records -- --exact
```

**Notes**

Exact reproduction confirmed the fixture decoding error, rather than source-body disclosure.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test opencode_ingress --locked
```

**Notes**

Three bridge regressions pass; reused pinned sanitizer cache. /tmp/jj-store-opencode-ingress-verified.log.
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
cargo test -p jj-store-server --test opencode_source --locked
```

**Notes**

Before source composition: missing opencode_source and incorrect test import from an undeclared domain crate; use the owned capture interface. /tmp/jj-store-opencode-source-before.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --locked
```

**Notes**

Initial compile incorrectly imported an undeclared WAL dependency. Reexported required source-position contracts through the owning capture module without adding a dependency. /tmp/jj-store-opencode-source-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --locked
```

**Notes**

Initial three integrated source tests pass. /tmp/jj-store-opencode-source-after-2.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --locked a_changed_queued_prefix_starts_reconstruction_without_skipping_to_the_last_record -- --exact
```

**Notes**

Genuine before regression: an early queued-prefix rewrite returned an error instead of restarting generation reconstruction. /tmp/jj-store-opencode-source-prefix-before.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --locked a_changed_queued_prefix_starts_reconstruction_without_skipping_to_the_last_record -- --exact
```

**Notes**

Exact after regression: starts a new monotonic generation and retains old unresolved intent. /tmp/jj-store-opencode-source-prefix-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test opencode_ingress --test opencode_snapshot --test source_registry --test transcript_sanitization --locked
```

**Notes**

16 selected top-level tests plus the launched fresh-process sanitizer child. No behavioral change to existing registry/sanitizer consumers. API/export trust is distinct. /tmp/jj-store-opencode-bridge-capture-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --test opencode_read_api --test capture_source --locked -- --test-threads=1
```

**Notes**

Four existing Codex source and five existing API tests pass. New source tests: three pass, one fixture failure assumed an unbound omission selected session coverage. JST-COLLAB diagnostic-only semantics instead retain source coverage without inventing a session. Corrected to assert exact selected transcript count, recorded privacy/partitioning and unchanged code heads; bridge omission normalization separately proves its gap diagnostic. /tmp/jj-store-opencode-bridge-server-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_source --locked -- --test-threads=1
```

**Notes**

Final five integrated source tests, including current native Write and changed API binding. Native MemoryStore publication and recorded AgentStore reads in two repositories; exact restart/offline tail, source exclusivity, spool pressure, early/terminal rewrites and revocation. /tmp/jj-store-opencode-source-final.log.
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
cargo check -p jj-store-capture -p jj-store-server --all-targets --locked
```

**Notes**

Both changed Rust packages and their typed direct consumers compile. /tmp/jj-store-opencode-bridge-check.log.
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
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

One collapsible_if warning; expressed the identical checkpoint guard as a let-chain. No semantic change. /tmp/jj-store-opencode-bridge-clippy.log.
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

Repeated during implementation and after the final structural lint fix.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Canonical source/driver contract and selected verification narrative formatting.
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
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Final strict two-package lint passes. /tmp/jj-store-opencode-bridge-clippy-final.log.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements.
<!-- /run -->
<!-- /evidence-field -->

