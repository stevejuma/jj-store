---
schema: 1
id: E-M21-066
---

# E-M21-066 details 2

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
jj --no-pager st/log/diff; target/debug/jj-store --no-pager stack --help
```

**Notes**

Started clean above 1238d234. before.log: stack is unrecognized, exit 2.
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
cargo check -p jj-store-jj --locked
```

**Notes**

jj-check-initial.log: initial reader passed Arc&lt;ReadonlyRepo&gt; to a Repo trait object. Fixed by borrowing its underlying repository.
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
cargo check -p jj-store-cli --locked
```

**Notes**

cli-check-initial.log: new command family and reader compile.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test stack_cli --locked
```

**Notes**

executable-initial.log: 4 passed/3 failed. Fixed subdirectory discovery with native find_workspace_dir. Corrected test JSON option placement before -- and stable graph fixture selectors.
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
cargo test -p jj-store-cli --test stack_cli --locked
```

**Notes**

executable-second.log: 8 passed/1 failed. The inserted commit advanced its bookmark, making the fixture source and destination identical. Changed graph coverage to valid distinct native commits without loosening assertions.
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
cargo test -p jj-store-cli --test stack_cli --locked
```

**Notes**

executable-third.log and executable-final.log: 11 passed. Includes all local mutation mappings, exact hunks, mixed-source selection, native conflicts/Undo/Redo, shared unapply, linked workspaces and nonmutating concurrent-head refusal.
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
cargo test -p jj-store-cli --test stack_cli --locked
```

**Notes**

executable-complete.log: final 12 passed after adding before-setup history and bounded diff/history continuation, integrated/empty branch and reconciliation cases.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

native-stacks.log: all 33 directly affected native scenarios pass, including guards, WIP identities, conflicts, operation metadata and applied-settings failure.
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
cargo test -p jj-store-cli --test server_executable standalone_help_configuration_completions_and_build_info_are_consistent --locked
```

**Notes**

standalone.log: one existing presentation/help/completion/build-information test passes.
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
cargo test -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked
```

**Notes**

http-interop.log: one registered-folder host test passes, including the existing actual Stacks HTTP command/contract qualification. Isolated filesystem service; no RustFS or live sample mutation.
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
cargo check -p jj-store-cli -p jj-store-jj --locked
```

**Notes**

check-final.log: affected package checks pass.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 22
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-cli -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

clippy.log: 21 inherited JJ diagnostics plus a new reader &amp;PathBuf diagnostic. Fixed the reader and newly exposed preview pointer diagnostic. Unrelated diagnostic files remain untouched.
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
cargo clippy -p jj-store-cli -p jj-store-jj --lib --locked
```

**Notes**

clippy-advisory.log and clippy-final.log complete with inherited warnings retained. clippy-final-provenance.log proves all 21 diagnostic source files byte-identical to parent; no diagnostic in Stacks CLI/read/preview changes. This is not a strict-lint pass.
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
cargo fmt --all; cargo fmt --all -- --check; pnpm exec prettier --write changed canonical/manual files
```

**Notes**

rustfmt-final.log and format-final.log pass. Only task files formatted; generated command reference uses generator conventions. Linker compact-unwind warning is inherited and does not fail builds.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:generate
```

**Notes**

docs-generate.log: missing argument documentation for stack show bookmark. Added authored help for all stack arguments.
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
pnpm docs:generate; pnpm docs:review cli/local/contracts/application with topic-specific explanations
```

**Notes**

docs-generate-final.log: parser reference regenerated. CLI/local/contracts reviews recorded. Baseline application review was stale despite no UI edits; inspected current Stacks controller/card semantics and documented CLI interoperability, current selection toggles and per-row graphs before renewing that review.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

docs-check.log: inherited stale application source-review fingerprint; corrected through actual source/document review, not by disabling checks.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

spec-first.log: missing mapped JJ integration and documentation chapters. Added their owning delivery/verification requirements.
<!-- /run -->
<!-- /evidence-field -->

