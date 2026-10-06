---
schema: 1
id: E-M17-030
---

# E-M17-030 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor saturation::saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-saturation-before.log 2>&1
```

**Notes**

Before: the named pressure target was absent; zero tests ran. This documents missing coverage and is not acceptance evidence.
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

Affected Rust formatting for the new pressure fixture and final shutdown status correction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor saturation::saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-saturation-exact.log 2>&1
```

**Notes**

New fixture initially referenced a nondependency IntegrationId. It now decodes the existing canonical codex enum through the direct consumer type; no dependency added.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor saturation::saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-saturation-exact-fixed.log 2>&1
```

**Notes**

Fixture candidate lookup compared logical macOS temporary paths with canonical candidates; fixed the fixture to compare canonical paths.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor saturation::saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-saturation-canonical.log 2>&1
```

**Notes**

Meaningful product regression: immediate shutdown from idle paused owners released the frames but status still reported two resident reservations. Stopped owners now report no occupied/raw reservations; draining owners remain counted.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor saturation::saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-saturation-shutdown-fixed.log 2>&1
```

**Notes**

Original exact pressure target passes: four armed sources, one reader, two retained raw frames, no rejected-batch acknowledgement, A revocation, explicit shutdown/restart with changed capacity, retained source identities and fresh-only admission for the other sessions.
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
cargo check -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-saturation-check.log 2>&1
```

**Notes**

Focused supervisor target compiles before the shutdown counter correction.
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
cargo clippy -p jj-store-server --test capture_supervisor --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-saturation-clippy.log 2>&1
```

**Notes**

Focused supervisor target lint passes before the shutdown counter correction.
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
cargo fmt -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-saturation-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes; modified production module remains below 300 lines.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-saturation-supervisor.log 2>&1
```

**Notes**

All 10 direct connected supervisor cases pass, including the original exact pressure case, paused shutdown/restart and independently stalled draining ownership.
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
cargo check -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-saturation-check-final.log 2>&1
```

**Notes**

Focused affected supervisor package/test target compiles after shutdown status correction.
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
cargo clippy -p jj-store-server --test capture_supervisor --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-saturation-clippy-final.log 2>&1
```

**Notes**

Focused affected supervisor package/test target passes strict lint after shutdown status correction.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-saturation-doc-format.log 2>&1
```

**Notes**

Owning collaboration and verification prose formatted.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-saturation-ledger-format.log 2>&1
```

**Notes**

New ledger evidence formatted; all 284 parent entries preserved.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-saturation-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-saturation-spec-first.log 2>&1
```

**Notes**

Mapped CLI chapter was missing for protected supervisor/test changes; added the same explicit pressure/restart/status scope to CLI canonical prose.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-saturation-progress.log 2>&1
```

**Notes**

All 129 tasks pass bookkeeping with 285 evidence entries; M17/M17.2 remain In progress.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-saturation-doc-check.log 2>&1
```

**Notes**

Initial changed canonical documents pass formatting.
<!-- /run -->
<!-- /evidence-field -->

