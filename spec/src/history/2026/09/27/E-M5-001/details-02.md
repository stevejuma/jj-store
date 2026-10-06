---
schema: 1
id: E-M5-001
---

# E-M5-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact
```

**Notes**

Baseline compile: missing service commands module and prepare_command method; two test-only uses of MergedTree::id corrected to tree_ids.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact
```

**Notes**

Native ref/workspace name types do not implement Display; use as_str().to_owned().
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact
```

**Notes**

E0283 on ambiguous BTreeMap lookup; use typed native RefName.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact
```

**Notes**

Executor incorrectly reused graph cache path validation; extracted pure limit validation while retaining graph absolute-path checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact
```

**Notes**

Author fixture assumed native blank fields remained blank; inspected pinned commit-builder policy, used explicit original identity/timestamp, and asserted native preservation. Describe uses pinned reparenting semantics.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked
```

**Notes**

Expanded fixture inherited auth Result alias; qualify the command result type.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked
```

**Notes**

Initial five native command fixtures passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_new_merges_ordered_parents_and_moves_only_logical_workspace --locked -- --exact
```

**Notes**

Strengthened assertion exposed New being reported as a rewrite because native predecessor vectors were empty.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_new_merges_ordered_parents_and_moves_only_logical_workspace --locked -- --exact
```

**Notes**

Empty predecessor vectors now contribute only to created reports; native operation data is unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked
```

**Notes**

Eight fixtures pass including shared worker cancellation, mid-flight snapshot changes and token revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_commands_distinguish_hidden_noop_and_ambiguous_heads --locked -- --exact
```

**Notes**

Hidden Describe is rejected; committed historical parent New works; no-op reports no effects; multiple heads are rejected without reconciliation.
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
pnpm format:rust:check
```

**Notes**

Pinned rustfmt detects unformatted newly authored sources.
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
pnpm format:rust:check
```

**Notes**

cargo fmt --all -- --check passes after pnpm format:rust.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Before recording E-M5-001: missing same-change ledger update and evidence entry.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked
```

**Notes**

All nine command preparation acceptance tests pass after formatting; zero failures.
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
pnpm check:progress
```

**Notes**

31 task references, same-change evidence and named tests pass. Bookkeeping does not complete M5.
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
pnpm spec:first
```

**Notes**

12 protected implementation/report files and five owning chapters pass mapped spec-first.
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
pnpm check:boundaries
```

**Notes**

All nine Rust package boundaries pass.
<!-- /run -->
<!-- /evidence-field -->

