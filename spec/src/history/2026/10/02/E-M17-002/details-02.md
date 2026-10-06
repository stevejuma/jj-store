---
schema: 1
id: E-M17-002
---

# E-M17-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog diagnostic_only_capture_retains_evidence_and_allows_later_source_progress --locked -- --exact
```

**Notes**

Initial regression fixture used page/entries instead of list_catalog/selections. Five compile errors corrected before behavior reproduction; not product failure evidence.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog diagnostic_only_capture_retains_evidence_and_allows_later_source_progress --locked -- --exact
```

**Notes**

Before production changes, an omission cannot stage the capture; exact behavior failure.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog separately_published_tool_updates_preserve_content_terminal_state_and_conflict_coverage --locked -- --exact
```

**Notes**

Before production changes, separately accepted updates select Succeeded instead of preserving Failed.
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

Rust formatting after edits.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog --locked
```

**Notes**

Initial after-state: all 5 native catalog tests pass, including both regressions. An unused import was then removed and unsupported-profile validation retained; final state rerun required.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent diagnostic_and_partial_tool_captures_survive_local_restart_without_state_loss --locked -- --exact
```

**Notes**

Test compilation: direct jj_agent import unavailable to this package. Use existing jj-store-wal public reexports instead; no production dependency change.
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
pnpm spec:validate
```

**Notes**

Current implementation slice: 5 validators pass.
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
pnpm spec:first
```

**Notes**

9 protected files covered by 5 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

M17 and M17.1 could not resolve E-M17-002; protected-change evidence missing. Corrected duplicate trailing evidence/fences inherited from specification entry editing. All baseline evidence entries verified identical to 2852a3e3.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent diagnostic_and_partial_tool_captures_survive_local_restart_without_state_loss --locked -- --exact
```

**Notes**

Exact local restart regression passes, including original receipt replay, retained omission gap and cumulative failed tool.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog --test agent_provenance --test bound_capture --locked
```

**Notes**

12/12 pass: catalog 6, provenance 1 and exact Thread binding 5. Unsupported-profile rejection leaves selection unchanged; original receipt/byte and authorization regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked
```

**Notes**

24/24 pass across 7 domain test files; no ignored tests.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_pages --test agent_sessions --test agent_checkpoints --test agent_relations --locked
```

**Notes**

21/21 pass across 6 direct service consumer files. Local fixtures reopen actual disposable JJ/Git companion repositories; no existing service restart.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog diagnostic_only_capture_retains_evidence_and_allows_later_source_progress --locked -- --exact
```

**Notes**

Exact native omission/source-progress regression passes after final production edits.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --test agent_catalog separately_published_tool_updates_preserve_content_terminal_state_and_conflict_coverage --locked -- --exact
```

**Notes**

Exact native cross-batch content/terminal-conflict regression passes after final production edits.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md
```

**Notes**

Document formatting after ledger fence repair.
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
pnpm check:progress
```

**Notes**

Ledger repair passes: 129 tasks; bookkeeping only.
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
pnpm spec:validate
```

**Notes**

5 validators, 21 chapters, 160 requirements pass after ledger repair.
<!-- /run -->
<!-- /evidence-field -->

