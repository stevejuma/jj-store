---
schema: 1
id: E-M17-076
---

# E-M17-076 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --locked
```

**Notes**

Exact before missed both native workspaces with empty default status; no project-directory reads. /tmp/jj-store-opencode-directory-before.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --locked
```

**Notes**

Original two discovery/admission regressions pass after directory composition. /tmp/jj-store-opencode-directory-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --test opencode_supervisor --test opencode_source --test opencode_read_api --test opencode_lifecycle --locked
```

**Notes**

Linker could not write opencode_supervisor: errno 28, No space left on device. No test conclusion from this interrupted compile. /tmp/jj-store-opencode-directory-regressions.log.
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
df -h /Users/stevejuma/code/lapismd/jj-store-agent-capture; du -sh target/debug/incremental target/debug/deps
```

**Notes**

Disk only 574 MiB available; own ignored incremental cache 30 GiB, dependencies 16 GiB.
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
python3 - <<PY (shutil.rmtree of /Users/stevejuma/code/lapismd/jj-store-agent-capture/target/debug/incremental only)
PY
```

**Notes**

Removed only disposable build intermediates after cargo jobs settled; reclaimed approximately 30 GiB, 29 GiB available. No source, installed binary, service state or other workspace changed.
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
python3 pinned-source retrieval from raw.githubusercontent.com/anomalyco/opencode/3104c1428ec91f809e5ab86631300de41eb6952e
```

**Notes**

Initial core project/index.ts path returned HTTP 404. The pinned tree supplied project.ts and project/directories.ts; subsequent exact-version retrieval succeeded. This is source-contract inspection, separate from product verification.
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
python3 pinned-source retrieval of project.ts, project/directories.ts and project HTTP groups/handlers at OpenCode 1.18.30
```

**Notes**

Pinned source proves global/non-Git Project.saveProjectDirectory intentionally returns early. Corrected the candidate channel to bounded legacy session metadata under directory=/, scope=project and limit=4096 before final qualification. No source file copied into runtime.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --test opencode_supervisor --test opencode_source --test opencode_read_api --test opencode_lifecycle --locked
```

**Notes**

25 selected tests pass after cache recovery and global metadata correction: four discovery, seven lifecycle, five read-API, six source, three supervisor. /tmp/jj-store-opencode-directory-regressions-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests -- --test-threads=1
```

**Notes**

Both exact-proof/blocked-publication owned-lane tests pass; final includes global metadata composition. Existing macOS unwind-size linker warning remains distinct. /tmp/jj-store-opencode-directory-owned.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests -- --test-threads=1
```

**Notes**

Both exact-proof/blocked-publication owned-lane tests pass; final includes global metadata composition. Existing macOS unwind-size linker warning remains distinct. /tmp/jj-store-opencode-directory-owned-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --locked
```

**Notes**

Fixture assumed one five-second shutdown call proves completed drain under parallel load. Documented runtime timeout retains the exact owner for retry. Fixture now retries only that explicit transient result, with bounded 15-second completion and a Stopped assertion; no production timeout or failure semantics changed. /tmp/jj-store-opencode-directory-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --locked
```

**Notes**

Four discovery tests pass with non-global project endpoint and exact-limit incomplete-coverage checks; /tmp/jj-store-opencode-directory-complete.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_discovery --locked -- --test-threads=1
```

**Notes**

Final four-test exact target after explicit retained-drain completion fixture. /tmp/jj-store-opencode-directory-complete-final.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Affected package/all targets pass; /tmp/jj-store-opencode-directory-clippy.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Affected package/all targets pass; /tmp/jj-store-opencode-directory-clippy-final.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Affected package/all targets pass; /tmp/jj-store-opencode-directory-clippy-complete.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Affected package/all targets pass; /tmp/jj-store-opencode-directory-clippy-qualified.log.
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
cargo fmt --all
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->
<!-- /evidence-field -->

