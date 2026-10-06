---
schema: 1
id: E-M17-039
---

# E-M17-039 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_channels:: --locked > /tmp/jj-store-agent-capture-m172-raw-channels-before.log 2>&1
```

**Notes**

Original continued-turn fixture fails active snapshot admission: f.start preceded a transcript append. This is fixture failure, not the intended exclusion regression.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_channels:: --locked > /tmp/jj-store-agent-capture-m172-raw-channels-before-2.log 2>&1
```

**Notes**

Corrected activity snapshot reaches supervisor configuration but fails unequal config/spool capacity. Set the fixture config to its exact physical pending-byte limit.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_channels:: --locked > /tmp/jj-store-agent-capture-m172-raw-channels-before-3.log 2>&1
```

**Notes**

Actual before regression: a paused automatic parked raw frame allows a competing acquisition of its source channel. No assertion weakened.
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
cargo fmt --all
```

**Notes**

Repeated after production and test changes; only this slice changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_channels:: --locked > /tmp/jj-store-agent-capture-m172-raw-channels-after.log 2>&1
```

**Notes**

Original exact channel exclusion and same-limit queue-relief case passes with raw allocation identities preserved and sanitized native B publication.
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
cargo test -p jj-store-capture --test source_parking reservations:: --locked > /tmp/jj-store-agent-capture-m172-reservations.log 2>&1
```

**Notes**

New runtime fixture compilation requires unwrapping the optional activity proof. Corrected without changing production or assertions.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test source_parking reservations:: --locked > /tmp/jj-store-agent-capture-m172-reservations-2.log 2>&1
```

**Notes**

Uninitialized activity baseline conservatively treated the appended start as historical; fixture polling did not terminate. Stopped only its two exact test processes, initialized the baseline before append and bounded fixture polling. No qualifying result from this run.
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
kill -TERM 2649 2418
```

**Notes**

Stopped the hung new runtime fixture and its cargo parent; no other processes changed.
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
cargo test -p jj-store-capture --test source_parking reservations:: --locked > /tmp/jj-store-agent-capture-m172-reservations-3.log 2>&1
```

**Notes**

Five reservation regressions pass after fixture correction.
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
cargo test -p jj-store-capture --test source_parking --locked > /tmp/jj-store-agent-capture-m172-source-parking.log 2>&1
```

**Notes**

All 12 generic/automatic parking cases pass, including seven new raw reservation, partial scan, durable reopen and ephemeral-release cases.
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
cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-reservations-runtime-all.log 2>&1
```

**Notes**

Complete shared capture runtime passes: 102 main-process cases and one visible fresh-process privacy helper. Three process-entry helpers remain ignored in parent runs.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_pipeline --test capture_parking --test capture_reanchor --test capture_resume --test capture_recovery --locked > /tmp/jj-store-agent-capture-m172-reservations-server-all.log 2>&1
```

**Notes**

57 direct native server cases pass with unchanged default concurrency and deadlines, including all 26 then-registered supervisor cases. Later added raw admission fault requires the final supervisor lane.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_channels:: --locked > /tmp/jj-store-agent-capture-m172-raw-channels-matrix.log 2>&1
```

**Notes**

Both exact native raw-channel cases pass: same-limit capacity relief and shutdown/restart without premature cursor acknowledgement.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_handoff:: --locked > /tmp/jj-store-agent-capture-m172-raw-handoff.log 2>&1
```

**Notes**

Raw handoff admission fault passes at both pre-acquisition and final post-acquisition checks. Stalled synchronous verification becomes overdue and drains on pause; current catalog refusal releases physical ownership while retaining the channel. A final explicit resumed quantum is checked separately.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-reservations-check.log 2>&1
```

**Notes**

Owning runtime/server and direct CLI consumer targets compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-reservations-lint.log 2>&1
```

**Notes**

All owning and direct consumer targets pass strict lint; final fault test adds only explicit resumed-step verification.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-reservations-boundaries.log 2>&1
```

**Notes**

Package dependency boundaries pass for 17 packages.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-reservations-architecture.log 2>&1
```

**Notes**

Repository source architecture passes; changed behavioral production modules remain below 300 lines.
<!-- /run -->
<!-- /evidence-field -->

