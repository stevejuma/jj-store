---
schema: 1
id: E-M17-005
---

# E-M17-005 details 3

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
pnpm spec:first
```

**Notes**

26 protected files and five owning canonical chapters pass after registering executable mapping.
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

129 tasks validate; moved helper file and test paths are explicitly retired without changing prior evidence entries.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes.
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
pnpm exec prettier --check scripts/check-boundaries.mjs scripts/governance.test.mjs spec-validator.config.mjs spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

Changed governance and canonical documents pass formatting.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test agent_hook --test agent_kingfisher --test agent_setup --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-cli.log 2>&1
```

**Notes**

All 33 direct tests pass after extraction, including cold hook sequences/redaction, setup ownership, native/local retry/unknown outcomes, restart and concurrency. Two ignored publication child entry points and one Kingfisher scan child are explicitly invoked by their parent tests.
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
cargo test -p jj-store-server --test configuration automatic_capture_configuration_is_explicit_local_writable_and_bounded --locked -- --exact > /tmp/jj-store-agent-capture-m172-config-before.log 2>&1
```

**Notes**

Before configuration implementation: strict Config rejects the unknown agent_capture table.
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
cargo test -p jj-store-server --test configuration automatic_capture_configuration_is_explicit_local_writable_and_bounded --locked -- --exact > /tmp/jj-store-agent-capture-m172-config.log 2>&1
```

**Notes**

Exact configured LocalTrusted capture admission and invalid limit/profile/version/field cases pass without state creation.
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
cargo test -p jj-store-server --test configuration --test local_installation --locked > /tmp/jj-store-agent-capture-m172-server-final.log 2>&1
```

**Notes**

All 22 final direct server tests pass: old serialization, approved-root resolution, credential-free loopback API restrictions, native/write admission and startup fail-closed gate included.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-check-final.log 2>&1
```

**Notes**

All affected packages check after new configuration fields and startup guard.
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
cargo clippy --no-deps -p jj-store-capture -p jj-store-server -p jj-store-cli --lib --test configuration --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-clippy-final.log 2>&1
```

**Notes**

Strict affected libraries and complete new config test target pass.
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
cargo test -p jj-store-cli --test server_executable cold_capture_config_check_and_startup_gate_preserve_private_state --locked -- --exact > /tmp/jj-store-agent-capture-m172-executable-exact.log 2>&1
```

**Notes**

Cold executable accepts Config check without creating state and rejects enabled startup before state/socket/listener creation while workers are pending.
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
cargo test -p jj-store-cli --test server_executable --locked > /tmp/jj-store-agent-capture-m172-executable.log 2>&1
```

**Notes**

All 15 ordinary cold server executable tests pass, including default startup, TLS, foreground shutdown and lock recovery. Three ignored real RustFS deployment gates remain separate and were not run.
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
cargo clippy --no-deps -p jj-store-cli --test server_executable --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-executable-clippy.log 2>&1
```

**Notes**

Strict cold executable test target passes.
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

Final canonical state validates: five validators, 21 chapters and 160 requirements.
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

36 protected files and six canonical chapters have same-change evidence.
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

129 tasks validate with explicit remaining M17.2 gates.
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

Final 17-package dependency boundaries pass.
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

Final Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

