---
schema: 1
id: E-ORIGINAL-REMOTE-005
---

# E-ORIGINAL-REMOTE-005 details 2

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
node .local/native-remotes-validation.mjs test --locked -p jj-store-service --no-default-features --test working_tree_observer -- --ignored --exact status_accepts_metadata_only_heads_but_not_a_different_working_copy_commit
```

**Notes**

Baseline: Stale after an ignored-working-copy bookmark-only operation. Initial implementation exposed one Unavailable from attempting ancestor access in a selected-only context; the fix opens an independently bounded historical context under the same budget and requires a sole repeated selected head.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-service --no-default-features --test working_tree_observer -- --ignored
```

**Notes**

All seven real CLI observation/status cases pass, including metadata-only descendant status, identical-tree different-commit refusal, unchanged strict mutation witnesses, cancellation, sparse/ignore/race checks and byte-for-byte no-write evidence.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features vault-backup --test working_tree_current --test working_tree_baseline --test vault_original_remotes
```

**Notes**

Validation invocation omitted the existing native feature required by the current/baseline test binaries. No tests ran; the corrected command includes native,vault-backup.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features native,vault-backup --test working_tree_current --test working_tree_baseline --test vault_original_remotes
```

**Notes**

17 passed, zero failed; two pre-existing real-SSH opt-in ignores remain. Exact current witnesses, baseline identity/policy and original mutation admission pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

The new receipt initially had four bare test names instead of file references and omitted six changed-document paths. Corrected in the new uncommitted receipt; no historical evidence changed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm docs:check
```

**Notes**

The implicit native docs-reference build correctly refused the managed 30 GiB disk reserve at 22.4 GiB free. Use the existing qualified native docs_reference helper for the unchanged CLI schema; product Rust validation stays on the isolated remote Docker worker. No reserve or cleanup policy was weakened.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

The two added suite file references also require named test fragments. Corrected before commit.
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
node scripts/spec/validate.mjs
```

**Notes**

Invalid manual invocation: no such script exists. The canonical pnpm spec:check entry point remains the required validation.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-service --no-default-features --test working_tree_projection --test working_tree_inventory --test working_tree_scan --test working_tree_checkout --test working_tree_local
```

**Notes**

23 passed, zero failed, one pre-existing opt-in real-folder ignore; strict checkout guards, full inventory, physical scan and projection remain correct.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
```

**Notes**

New evidence was not yet registered in scoped SUMMARY navigation; added only the new day/record entries.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

The existing protected map also requires owning mutation and documentation chapters for service/docs edits. Added explicit unchanged-mutation-authority and embedding-guidance requirements without changing the mapping.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-service --no-default-features --test working_tree_local -- --ignored
```

**Notes**

The separate cold real-folder comparison passes 1/1. Together with the other affected binaries, 48 native cases pass; the two unchanged original real-SSH opt-ins are not claimed from this source lane.
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
node .local/native-remotes-validation.mjs clippy --locked -p jj-store-jj -p jj-store-service --all-targets -- -D warnings
```

**Notes**

Both affected crates and every target compile with strict default-profile Clippy.
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
node .local/native-remotes-validation.mjs build --locked -p jj-store-jj -p jj-store-service
```

**Notes**

Both affected native library artifacts build successfully. Remote image sha256:7d3aa64f81dce0988978ac830b4cc53ff5713d3acbf19163bd4987ff3e858efe; exact tracked JJ archives and existing workspace-isolated cache; 2 CPUs/8 GiB.
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
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=$PWD/.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

Existing qualified native CLI-reference helper for unchanged command schema. Documentation review, nine dual-theme Mermaid renders, strict site build and full link/anchor closure pass.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
```

**Notes**

All 121 governance cases pass.
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
cargo fmt --all -- --check; JJ_STORE_VALIDATION_RUNTIME=native pnpm check:boundaries; JJ_STORE_VALIDATION_RUNTIME=native pnpm check:architecture
```

**Notes**

Rustfmt, all 27 package boundaries and 2108 production module architecture checks pass.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first; JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Canonical validation, spec-first and progress pass after correcting the new evidence references and registering scoped navigation. The owning targeted policy keeps unrelated UI and whole-workspace product suites out of this Rust-only slice; hosted required CI remains a separate gate.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

