---
schema: 1
id: E-M13-008
---

# E-M13-008 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance_cli
```

**Notes**

Before state: no executable provenance command acceptance target.
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
cargo fmt -p jj-store-cli -p jj-store-service
```

**Notes**

Repeated after command types extraction and new executable cases.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-cli --locked
```

**Notes**

CLI and provider page compile; one unused AgentObjectKind import warned and removed before final lint.
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
pnpm exec prettier --write docs/cli/agent.md spec/src/cli.md spec/src/mutations.md spec/src/local-jj.md
```

**Notes**

Authored CLI help and canonical behavior formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance_cli
```

**Notes**

Initial three cold executable cases pass, including private retry without input document or source operations.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance_cli --test local_agent_provenance --test local_agent_recovery
```

**Notes**

4 cold provenance commands, 7 provider and 2 capture-recovery tests pass; final journal intent-corruption assertion is pending rerun.
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
cargo fmt -p jj-store-cli
```

**Notes**

Final journal intent checksum and binding checks formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance_cli
```

**Notes**

Final four cold executable cases pass, including canonical intent-corruption rejection before retry and original request replay after source operation-store loss.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent local_agent_reads_and_initialization_accept_all_git_layouts_and_workspaces -- --exact
```

**Notes**

1 layout/read/initialization fixture passes for colocated, internal, external Git and sibling workspaces. Updated capability flag is qualified by real provenance commands; unrelated sanitizer cold fixture not rerun.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test executable_native custom_help_completion_build_info_and_ordinary_git_commands -- --exact
```

**Notes**

1 ordinary JJ/custom help/completion/build-info regression passes; 24 unrelated tests filtered.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_agent_pages --test local_agent
```

**Notes**

2 selected session cursor and 7 capture/provider regressions pass.
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
cargo fmt -p jj-store-cli -p jj-store-service -- --check
```

**Notes**

Affected Rust packages pass formatting; every new production module below 300 lines and executable test below 500.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Evidence ledger formatted.
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
pnpm spec:first
```

**Notes**

11 protected files and 4 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

21 chapters and 129 requirements pass.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-cli --test local_agent_provenance_cli --test local_agent --test local_agent_recovery -p jj-store-service --test local_agent_pages --test local_agent --locked -- -D warnings
```

**Notes**

Affected libraries and exact consumers pass without warnings; repeated after final page-kind guard.
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
cargo fmt -p jj-store-service
```

**Notes**

Final pager semantic-kind guard formatted.
<!-- /run -->
<!-- /evidence-field -->

