---
schema: 1
id: E-M17-003
---

# E-M17-003 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible --locked -- --exact
```

**Notes**

Initial regression fixture lacked AgentRootV1/AgentObjectKind imports. Corrected before behavior reproduction; not product failure evidence.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible --locked -- --exact
```

**Notes**

Fixture exceeded the existing 256-observation batch limit. Split 400 messages across two accepted batches; no limit weakened.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible --locked -- --exact
```

**Notes**

Before production changes: a single append reads 2 pages and 400 prior chunks.
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

Rust formatting after additive tail format and shared append/read changes.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible --locked -- --exact
```

**Notes**

Exact regression passes: shared append reads at most one selected tail page and one prior chunk; older manifest remains independently readable.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog --test agent_provenance --test bound_capture --test native_copy --locked
```

**Notes**

Final witness state: 16 tests passed, including mixed legacy/V2 closure cold copy and misleading source-cursor rejection. MemoryStore evidence; not real RustFS.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent transcript_tail_rollover_preserves_pinned_history_after_local_restart --locked -- --exact
```

**Notes**

Final local provider ordering witnesses: exact 256/257 rollover, reopened companion, pinned old manifest and stable receipt replay passed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent --all-targets -p jj-store-wal --lib --test agent_catalog --test agent_provenance --test bound_capture --test native_copy -p jj-store-service --lib --test local_agent --test local_bound_capture --test local_agent_pages --test agent_sessions --locked -- -D warnings
```

**Notes**

Invalid command: repeated --lib rejected before compilation. Replaced with one shared --lib selector and the existing local_agent_binding target.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service --lib --test activity --test agent_catalog --test agent_provenance --test bound_capture --test native_copy --test local_agent --test local_agent_binding --test local_agent_pages --test agent_sessions --locked -- -D warnings
```

**Notes**

Affected libraries and direct changed/consumer targets pass strict clippy.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent -p jj-store-service --test activity --test indexes --test model --test normalize --test portable --test provenance --test local_agent --test local_agent_binding --test local_agent_pages --test agent_sessions --locked
```

**Notes**

Invalid target normalize; actual target is normalizers. No tests ran. Corrected by running the complete small model package and explicit direct service targets.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked
```

**Notes**

25 model tests passed; canonical legacy page bytes, V2 bounds/roundtrip and strong prefix edges retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_pages --test agent_sessions --locked
```

**Notes**

20 direct service tests passed: native session detail/ranges, local capture/restart/rollover, exact Thread binding and captured-root pagination.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Changed canonical chapters and evidence formatted.
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
pnpm spec:validate
```

**Notes**

5 validators, 21 canonical chapters, 160 requirements.
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
pnpm spec:first
```

**Notes**

17 protected files and five owning chapters passed.
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

129 tasks; evidence bookkeeping passed.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries passed.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passed.
<!-- /run -->
<!-- /evidence-field -->

