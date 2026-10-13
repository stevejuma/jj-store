---
schema: 1
id: E-WORKFLOW-MULTISTEP-001
---

# E-WORKFLOW-MULTISTEP-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st
```

**Notes**

Before clean wrpzywwx fa0cfb67, parent 5162009f. Current native command executor incorrectly creates an already existing attempt directory on the second action; unanchored source branch also lacks initial directory creation.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
Inspect current workflow chapters, running runtime/config, host code and CLI
```

**Notes**

Dev server at 7462 behind Vite 7460 has no workflow_hosts admission; native workflow-host implemented, Forgejo protocol explicitly deferred. forgejo-runner absent; act installed is not substituted as Forgejo qualification. Running server left unchanged.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:disk
```

**Notes**

51.4 GiB managed build storage, target 31.3 GiB, 192.7 GiB free. No cleanup or state removal.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
failures: 1
result: fail
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-cli --test server_executable workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart -- --ignored --exact
```

**Notes**

Before production repair: 1 failed, 20 filtered. Second action fails with File exists (os error 17). Isolated real embedded RustFS fixture and two native hosts; /tmp/jj-store-workflow-multistep-before.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- fmt --package jj-store-cli
```

**Notes**

Format affected CLI crate/test; no unrelated source changes retained.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-cli --test server_executable workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart -- --ignored --exact
```

**Notes**

After: 1 passed, 20 filtered. Three actions share artifact content, source logs equal expected pinned revision, all six Running/Completed step reports ordered, one run selected by competing hosts, typed artifacts present, HTTP/native pages equal, no source operation published and history equal after server restart. /tmp/jj-store-workflow-multistep-after.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check --locked -p jj-store-cli
```

**Notes**

Affected package check passes; /tmp/jj-store-workflow-multistep-check.log. Native test builds emit the existing macOS linker compact-unwind warning in both before and after; not a test failure.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --locked -p jj-store-cli --test server_executable workflows::executor_parent_loss_stops_the_owned_process_group -- --exact
```

**Notes**

1 passes, 20 filtered; owned worker process-group termination remains qualified. /tmp/jj-store-workflow-multistep-supervision.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
Prepare private demo operator configuration and run workflow-host --check
```

**Notes**

Private directory path recorded in /tmp/jj-store-workflow-demo-path.txt, outside checkout and mode 0700. Three /bin/sh named demo commands, no secrets/agents/network. Native CLI principal a9d260b7d6a4e958d668b7978606f983 differs from browser principal; configuration corrected to receipt owner. Existing Persistence test cannot match admitted demo command names/owner. Preflight passes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
custom jj workflow --format json put --request-id REQUEST private/workflow.yaml
```

**Notes**

Live dev definition committed: e483ed58b3ddcf69cc2160f0e62b409e, version f6b70e00d9f080e87e8460cbd32385d3. Name Live multi-step demo; Prepare/Test/Summary; disabled manual trigger and live-demo-only label. Stable request retained in private put-request-id. No run submitted and no host admission changed. A later explicit manual run was committed as 1ed1a78ecfa7695ed2e6a3af53660a3d, still Pending with no admitted host; run request/result retained privately. No command execution or fake reports were submitted.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
custom jj workflow source --repo-id SOURCE --operation PIN --commit PIN private/preflight-workspace; private demo.sh prepare/test/summary
```

**Notes**

Pinned export and all three direct script preflights pass; expected stdout/stderr and demo-result.txt produced. These are private script checks, not live host execution or app log evidence.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
request_user_input_async: temporary workflow-host admission and backend restarts
```

**Notes**

Explicit approval requested because current admission defaults empty and is not live reload. Backend activation and later admission restoration are pending; no elapsed time or default choice is treated as approval.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl: background live workflow tab, Edit prepared definition, hide inspector, Cancel
```

**Notes**

Live server shows the persisted three-action definition and label. No Save, Run, Cancel-run or enable mutation. Screenshot /tmp/jj-store-workflow-demo-prepared.png inspected; user-owned tab unchanged. This is definition presentation, not execution evidence.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write [changed authored Markdown]; pnpm docs:generate
```

**Notes**

Updated source workflow guide and generated jj-workflow help reference; clarified same-attempt workspace sharing.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review cli; pnpm docs:review contracts
```

**Notes**

Reviewed CLI help and unchanged wire/storage/report authority; M28 explicitly unchanged.
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
pnpm dev:cargo -- fmt --package jj-store-cli -- --check
```

**Notes**

Affected CLI format check passes. /tmp/jj-store-workflow-multistep-format-rust.log.
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
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
```

**Notes**

Strict documentation build, 9 local light/dark Mermaid diagrams, links/anchors and 143-file closure pass. /tmp/jj-store-workflow-multistep-docs-check.log.
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
pnpm spec:first
```

**Notes**

Pass: 5 protected files and 3 canonical chapters. /tmp/jj-store-workflow-multistep-first.log.
<!-- /run -->
<!-- /evidence-field -->

