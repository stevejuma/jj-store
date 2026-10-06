---
schema: 1
id: E-EXEC-001
---

# E-EXEC-001 details 5

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
pnpm check:architecture
```

**Notes**

1183 modules; counted-line boundary preserved.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community copies/provenance; no sibling presentation imports.
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
pnpm check:boundaries
```

**Notes**

Storage/WAL/model and JJ import boundaries preserved.
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
pnpm web:build
```

**Notes**

Production Vite bundle, browser asset manifest and strict self-contained manual build; large bundle is an existing nonfatal warning.
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
pnpm desktop:check
```

**Notes**

Tauri host and independently locked dependencies.
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
pnpm desktop:test (initial)
```

**Notes**

Inherited terminal test did not create job.pid under load; no assertion changed.
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
cargo test --manifest-path desktop/Cargo.toml --test terminal_sessions --locked closing_terminal_ends_background_job_in_separate_process_group -- --exact
```

**Notes**

Original terminal target passed unchanged, followed by complete desktop tests.
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
pnpm desktop:test
```

**Notes**

10 units, 3 owned-process, 4 terminal-session and 3 terminal-target tests.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DESKTOP_TEST_RESOURCES=$PWD/desktop/resources cargo test --manifest-path desktop/Cargo.toml --test managed_runtime --locked real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port -- --ignored --exact (before detached rule preparation)
```

**Notes**

Owner-EOF teardown waited for Tokio blocking privacy compilation beyond 40 seconds. Detached preparation owns no process/publication and discards late results.
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
JJ_STORE_DESKTOP_TEST_RESOURCES=$PWD/desktop/resources JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test --manifest-path desktop/Cargo.toml --test managed_runtime --locked real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port -- --ignored --exact
```

**Notes**

Cold managed service with staged ACP/JJ/RustFS resources: restart, stable origin/identity, occupied-port refusal and parent lifetime passed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 25
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj -p jj-store-server -p jj-store-cli -p jj-store-http -p jj-agent -p jj-store-capture --all-targets --locked -- -D warnings (optional wider diagnostic)
```

**Notes**

Inherited jj-store-jj lint findings; all 19 reported source/test files were verified byte-identical to @-. This optional wider diagnostic is not an integration pass. No thresholds or allowlists were changed. Required package checks and named regressions passed under the repository targeted policy.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web web/dist --browser (initial browser harness)
```

**Notes**

Both real native edits/publication passed; browser Instructions locator was ambiguous. Changed to exact textbox role.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web web/dist --browser (save dialog observation)
```

**Notes**

Both real native edits passed; setup dialog remained open. Captured setup response and reproduced the independent revision defect below.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web web/dist --browser (accessibility harness context)
```

**Notes**

Both real native edits, saved setup, browser launch and streamed turn passed; Axe required browser.newContext(). The harness was corrected without disabling any rule.
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
JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-store-cli --test agent_execution --locked saved_setup_revision_is_independent_of_session_activity -- --exact (unqualified module filter)
```

**Notes**

0 tests selected; discovery only, not regression evidence.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-store-cli --test agent_execution --locked setups::saved_setup_revision_is_independent_of_session_activity -- --exact (before)
```

**Notes**

A launch/open advanced the setup revision from 0 to 7 even though no setup changed.
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
JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-store-cli --test agent_execution --locked setups::saved_setup_revision_is_independent_of_session_activity -- --exact
```

**Notes**

Independent setup CAS, duplicate request, stale concurrent edit, restart and legacy private-journal seed. Session activity cannot invalidate setup drafts.
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
cargo check --locked -p jj-store-server -p jj-store-service -p jj-store-cli -p jj-store-http
```

**Notes**

Final directly affected Rust package boundary checks.
<!-- /run -->
<!-- /evidence-field -->

