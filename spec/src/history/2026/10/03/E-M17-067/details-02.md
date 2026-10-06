---
schema: 1
id: E-M17-067
---

# E-M17-067 details 2

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
cargo test -p jj-store-capture --test reader_publication inspection:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-before.log 2>&1
```

**Notes**

Genuine before regressions: existing projection loses a completed accepted boundary after unknown tail history and opens pending bodies during inspection.
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
cargo check -p jj-store-server -p jj-store-capture -p jj-store-http -p jj-store-service --all-targets --locked > /tmp/jj-store-agent-capture-m172-public-import-check.log 2>&1
```

**Notes**

Initial typed service/server/HTTP compilation before CLI addition.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-capture.log 2>&1
```

**Notes**

Initial 18 publication tests including both before regressions pass; additional inspection cases follow.
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
cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-http.log 2>&1
```

**Notes**

Initial owned-worker transition returned before body work on every parked renewal; corrected to settle only initial admission. Second cold uncached sanitizer attempt was terminated at exact disposable PIDs 48906 and 48807 after its deadline while teardown waited on rule database compilation. No deadline or assertion changes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-http.log 2>&1
```

**Notes**

Initial cached runtime completion deadline failed; diagnostic exact rerun retained its ten-second bound and passed. Preserve this result; no weaker completion assertion.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-http-diagnostic.log 2>&1
```

**Notes**

Actual HTTP runtime test passes (8.36 seconds), including completion, transcript-free paused replay, stopped tail, revocation/disable. Initial diagnostic attempt failed to compile a child module path; explicit path corrected.
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
cargo fmt --all
```

**Notes**

Repeated after implementation/test edits. One intermediate invocation failed on the same new child-module path; fixed without changing runtime.
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
sample 48906 1 -file /tmp/jj-store-agent-capture-m172-public-import-hang.txt
```

**Notes**

Disposable test stack located uncached vectorscan rule database compilation; no user process sampled or terminated.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication --test reader_progress --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-inspection-final.log 2>&1
```

**Notes**

15.04 seconds; 37 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication inspection::inspection_keeps_earlier_holes_and_unknown_history_unqualified --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-inspection-fixture.log 2>&1
```

**Notes**

4.47 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication --test reader_progress --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-inspection-final.log 2>&1
```

**Notes**

11.26 seconds; 38 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-http-final.log 2>&1
```

**Notes**

15.93 seconds; 13 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_import_cli --test capture_cli --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-cli-exact.log 2>&1
```

**Notes**

52.48 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_import_cli --test capture_cli --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-cli-exact.log 2>&1
```

**Notes**

51.82 seconds; 12 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_import_cli --test capture_cli --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-cli-exact.log 2>&1
```

**Notes**

50.02 seconds; 13 top-level tests
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-management-library.log 2>&1
```

**Notes**

238.21 seconds; 41 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib fresh_import_cannot_reuse_published_reader_history_from_a_rewritten_prefix --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-prefix-after.log 2>&1
```

**Notes**

19.91 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib old_reader_link_needs_explicit_prefix_proof_and_upgrade_retains_original_generation --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-proof-upgrade.log 2>&1
```

**Notes**

1.1 seconds; 1 top-level tests
<!-- /run -->
<!-- /evidence-field -->

