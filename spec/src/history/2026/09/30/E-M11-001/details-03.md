---
schema: 1
id: E-M11-001
---

# E-M11-001 details 3

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
pnpm spec:validate
```

**Notes**

19 chapters and 115 requirements validate.
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
pnpm check:progress
```

**Notes**

77 tasks and same-change native evidence validate.
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
cargo clippy -p jj-work -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Intermediate collapsible reply validation; fixed without suppressing lint.
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
cargo clippy -p jj-work -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Intermediate manual range pattern; used the accepted 1..=3 manifest format range.
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
cargo test -p jj-store-wal --locked
```

**Notes**

Final affected authority suite passes 58 tests, zero failures, including six WorkStore regressions and the strengthened checkpoint domain check.
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
cargo clippy -p jj-work -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Affected native packages and targets pass with warnings denied.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native model and fake-store publication/recovery/copy only; M11.2/M11.3 and real RustFS WorkStore acceptance remain separate. Root aggregate and unrelated UI lanes are out of scope.
<!-- /evidence-field -->

