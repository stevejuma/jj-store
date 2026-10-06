---
schema: 1
id: E-M17-043
---

# E-M17-043 details 2

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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-before.log 2>&1
```

**Notes**

Initial fixture has an unresolved self-crate import and guessed profile variant in addition to missing private capture factory. Corrected only test plumbing using a cfg(test,unix) self alias and actual profile variant.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-before-corrected.log 2>&1
```

**Notes**

Corrected before target fails compilation only because the capture factory module is absent; no behavioral before failure is claimed.
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
cargo fmt --all
```

**Notes**

Pinned Rust formatting applied after each production/test edit.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-after.log 2>&1
```

**Notes**

Initial factory target passes one parent behavior plus helper registration, dispatching seven fresh-process disabled/preparation/degradation/cancellation cases.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-expanded.log 2>&1
```

**Notes**

Expanded fixture fails compilation because json! does not accept array repetition syntax; changed assertion to serde_json::to_value of the exact byte array. No production failure claimed.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-composed.log 2>&1
```

**Notes**

Expanded native progress assertion guessed a direct next_offset field; corrected to the existing cursor position contract.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-corrected.log 2>&1
```

**Notes**

The first cursor assertion correction used a transitive jj_agent crate not in server dependencies. Used the existing serialized position contract; no dependency or format changed.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-regression.log 2>&1
```

**Notes**

Actual active-case total observation budget expires at 30 seconds during fresh Kingfisher cache compilation; child finishes in 85.64 seconds. Canonical synchronous work may outlive one step deadline. Corrected only the cold total-flow observation budget to 180 seconds and added assertions that observed overdue/draining work retains storage. The configured worker deadline, shutdown bound and all publication/progress/private-state assertions remain unchanged; no cache priming or substituted pass is used.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-factory-check.log 2>&1
```

**Notes**

Owning server and direct CLI all-target checks pass; final changed assertion compile/lint rerun still required.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-factory-clippy.log 2>&1
```

**Notes**

Owning server and direct CLI strict all-target lint passes; final changed assertion rerun still required.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-cold.log 2>&1
```

**Notes**

Cold active pipeline passes with unchanged worker deadline and storage ownership, then the symlink fixture assertion finds the preexisting fixture spool via a link to the fixture root. Corrected test target to a dedicated foreign directory with sentinel; production already refuses the symlink.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-factory-doc-format.log 2>&1
```

**Notes**

Changed canonical chapters and ledger formatted; repeated after final evidence.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-factory-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass; final sealing rerun required.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-factory-spec-first.log 2>&1
```

**Notes**

Nine protected paths and three canonical chapters pass; final sealing rerun required.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-factory-progress.log 2>&1
```

**Notes**

Initial new evidence omitted mandatory date and kind fields. Added both from the actual ledger contract; all previous entries remain untouched.
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
JJ_STORE_CAPTURE_FACTORY_TEST_MODE=symlink JJ_STORE_S3_ACCESS_KEY=capture-factory-fixture-access JJ_STORE_S3_SECRET_KEY=capture-factory-fixture-secret cargo test -p jj-store-server --lib --locked process::tests::capture_factory::capture_factory_helper -- --exact --nocapture > /tmp/jj-store-agent-capture-m172-factory-symlink.log 2>&1
```

**Notes**

Exact isolated symlink helper passes with dedicated foreign state; sentinel preserved and no spool creation.
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked > /tmp/jj-store-agent-capture-m172-factory-exact-final.log 2>&1
```

**Notes**

Complete exact factory target passes: one parent behavior and no-op helper registration dispatch 16 fresh-process cases. Cold active flow uses a fresh cache and verifies overdue retention, exact source offset, an empty published queue and the actual message in a current AgentStore session. Configured step deadlines and five-second stop bounds are unchanged. Cold main target completes in 96.74 seconds; no cache priming/substitute fixture.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-factory-rustfmt.log 2>&1
```

**Notes**

Main and embedded manifests pass pinned Rust formatting.
<!-- /run -->
<!-- /evidence-field -->

