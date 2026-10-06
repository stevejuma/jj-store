---
schema: 1
id: E-M12-002
---

# E-M12-002 details 2

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
cargo test -p jj-store-wal --locked --test work_store
```

**Notes**

Before 6/6; after 6/6. Shared domain/publication infrastructure changes justify complete relevant WAL and JJ suites, plus executable-native direct-consumer acceptance. Unrelated UI and root aggregate lanes remain out of scope.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-wal --locked
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj --offline
```

**Notes**

Pinned APIs require runtime hex parsing and TreeRef::from_bytes; corrected without upgrading dependencies.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj --locked
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --locked
```
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
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

Initial fixture supplied a checkpoint argument unsupported by the owning API; corrected.
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
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

Negative encoding regression caught Gix encoding outside extra_headers; added explicit encoding validation, retaining the assertion.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --locked --test git_bridge lossy_git_metadata_and_incomplete_object_closure_are_rejected -- --exact
```

**Notes**

Expanded mode fixture missed ObjectId trait import; corrected before acceptance.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

Same missing fixture trait before correction.
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
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

Final four scenarios: exact signed parent and child after cache loss/checkpoint/native copy/cold recovery; binary modes/symlink/gitlink/merge order and deterministic export; invalid metadata/closure; WorkStore origin freeze/native signature/byte-budget rejection. Memory native storage, not real RustFS.
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
cargo test -p jj-store-wal -p jj-store-jj --locked
```

**Notes**

108 tests passed across WAL and JJ; 2 real RustFS tests explicitly ignored.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Manual range pattern; corrected to 2..=4.
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
pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Spec validation passed; progress rejected E-M12-002 on M12.3 because this entry belongs to M12.2. Corrected ledger link; later commands did not run.
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
cargo clippy -p jj-store-jj -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Owned comparison lint; compare typed OID instead. Final rerun passed.
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
cargo test -p jj-store-cli --locked --test executable_native
```

**Notes**

14 executable-native tests passed; 3 real OS credential tests ignored. Ordinary Git dispatch and native lifecycle consumers covered.
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
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

Expanded to 5 tests. Immutable disagreement, absent closure, import/export ambiguity, repository binding and unsupported SHA-256 now covered.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all --check
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/git-interop.md spec/src/milestones.md spec/src/publication.md spec/src/identity.md spec/src/architecture.md spec/src/jj-integration.md
```

**Notes**

Ledger table spacing; corrected with Prettier.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```
<!-- /run -->
<!-- /evidence-field -->

