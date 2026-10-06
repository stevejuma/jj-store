---
schema: 1
id: E-M17-024
---

# E-M17-024 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline revoked_pending_repository_does_not_block_later_independent_ingress_and_publication --locked -- --exact > /tmp/jj-store-agent-capture-m172-pipeline-before.log 2>&1
```

**Notes**

Original exact regression fails on the absent into_pipeline API; the serial driver withholds later ingress behind unresolved publication.
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
cargo fmt -p jj-store-server
```

**Notes**

Affected formatting repeated after extraction and expanded fixtures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline revoked_pending_repository_does_not_block_later_independent_ingress_and_publication --locked -- --exact > /tmp/jj-store-agent-capture-m172-pipeline-after.log 2>&1
```

**Notes**

Original revoked-A/later-B regression passes. Initial fixture dead-code warnings disappear after expanded cases use the shared secret and authority-revocation helpers.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline --locked > /tmp/jj-store-agent-capture-m172-pipeline-expanded.log 2>&1
```

**Notes**

Expanded fixtures initially fail to compile: unsupported direct WAL import and moved cursor position. Corrected to the owning service transport facade and a borrowed position without adding dependencies.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-pipeline-check.log 2>&1
```

**Notes**

Concurrent check observes those same two fixture compilation errors before correction.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-pipeline-clippy.log 2>&1
```

**Notes**

Concurrent strict lint observes those same two fixture compilation errors before correction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline --locked > /tmp/jj-store-agent-capture-m172-pipeline-fixed.log 2>&1
```

**Notes**

All four pipeline cases pass: revoked queued A with independent B; staged admitted offline resume past exact unknown A; A/B/A same-partition order; exact backpressure/current Write/spool binding.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-pipeline-server-direct.log 2>&1
```

**Notes**

All 23 direct native pipeline/source/resume/recovery/admission/activity/authority cases pass after shared ingress/recovery extraction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-pipeline-cli-direct.log 2>&1
```

**Notes**

All 31 primary direct publication/shared spool/local recovery cases pass; the two process fixtures execute from their parent tests.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-pipeline-check-fixed.log 2>&1
```

**Notes**

Affected server and direct CLI consumers compile after fixture corrections.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-pipeline-clippy-fixed.log 2>&1
```

**Notes**

Affected/direct-consumer strict lint passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-pipeline-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; server consumes existing owned transport types.
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
cargo fmt -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-pipeline-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline revoked_pending_repository_does_not_block_later_independent_ingress_and_publication --locked -- --exact > /tmp/jj-store-agent-capture-m172-pipeline-exact-final.log 2>&1
```

**Notes**

Original exact revoked-A/later-B regression rerun passes after all final expanded fixtures.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-pipeline-doc-write.log 2>&1
```

**Notes**

Final canonical chapters, verification and evidence formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-pipeline-progress.log 2>&1
```

**Notes**

129 tasks validate; 279 evidence entries preserve all 278 parent structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-pipeline-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-pipeline-first.log 2>&1
```

**Notes**

Final protected changes include the owning canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

