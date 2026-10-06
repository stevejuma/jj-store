---
schema: 1
id: E-M17-005
---

# E-M17-005 details 2

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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test agent_kingfisher --test local_agent_publication --locked
```

**Notes**

Selection error: local_agent_publication is a source module; corrected to existing local_agent_recovery test binary before baseline.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test agent_kingfisher --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-runtime-before.log 2>&1
```

**Notes**

Before extraction: 24 direct tests pass; two ignored child-process entry points are explicitly invoked by parent crash/lease tests. Cold Kingfisher rule compilation took 102 seconds.
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
node --test --test-name-pattern=capture_runtime_consumes_owned_interfaces_without_reverse_dependencies scripts/governance.test.mjs > /tmp/jj-store-agent-capture-m172-boundary-before.log 2>&1
```

**Notes**

Before dependency registration: server rejects the proposed owned runtime; new runtime reverse edges lack a dedicated allowlist.
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
cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-runtime.log 2>&1
```

**Notes**

New local package needs Cargo.lock registration; no dependency version changed.
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
cargo metadata --no-deps --format-version 1 > /tmp/jj-store-agent-capture-m172-metadata.json
```

**Notes**

Workspace metadata discovers the new capture package; no lock resolution occurs under --no-deps.
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
cargo test -p jj-store-capture --offline > /tmp/jj-store-agent-capture-m172-runtime.log 2>&1
```

**Notes**

Registers the new local package in Cargo.lock without network or version changes; moved inherited-file-description lease regression passes.
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
cargo fmt --all
```

**Notes**

Rust source formatted.
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
node --test --test-name-pattern=capture_runtime_consumes_owned_interfaces_without_reverse_dependencies scripts/governance.test.mjs
```

**Notes**

One exact boundary regression passes; owned interfaces accepted and all eight runtime reverse/upstream leaks rejected.
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
pnpm check:boundaries
```

**Notes**

17 owned packages satisfy the dependency audit.
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
pnpm spec:validate
```

**Notes**

Five validators pass for 21 chapters and 160 requirements.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

New capture crate mapping was added to canonical table but executable validator config had not yet been registered. Added same explicit chapter mapping to config.
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
pnpm check:progress
```

**Notes**

Temporary entry result partial is not a valid ledger enum. Use fail while post-extraction gates are pending, then record pass after verification.
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
node --test scripts/governance.test.mjs > /tmp/jj-store-agent-capture-m172-governance.log 2>&1
```

**Notes**

25 directly affected governance tests pass.
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
cargo test -p jj-store-server --test configuration --test local_installation --locked > /tmp/jj-store-agent-capture-m172-server.log 2>&1
```

**Notes**

19 tests pass: strict configuration, private path/lock policy, installation initialization and catalog restart binding.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-check.log 2>&1
```

**Notes**

All three affected packages check successfully.
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
cargo clippy --no-deps -p jj-store-capture -p jj-store-server -p jj-store-cli --lib --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-clippy.log 2>&1
```

**Notes**

Strict affected libraries pass; untouched transitive HTTP lint baseline remains excluded under targeted policy.
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
cargo test -p jj-store-cli --lib credentials::tests::competing_operator_provision_cannot_replace_the_first_credential --locked -- --exact > /tmp/jj-store-agent-capture-m172-credentials.log 2>&1
```

**Notes**

Moved shared lease retains credential provisioning exclusivity; one exact test passes.
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
python3 (compare extracted modules to parent logic and Cargo.lock package identities)
```

**Notes**

All seven modules preserve logic after explicit import/visibility adjustments and rustfmt ordering. Cargo.lock adds one local package with no external package/source/version changes.
<!-- /run -->
<!-- /evidence-field -->

