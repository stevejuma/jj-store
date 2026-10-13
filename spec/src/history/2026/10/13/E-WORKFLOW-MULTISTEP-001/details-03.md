---
schema: 1
id: E-WORKFLOW-MULTISTEP-001
---

# E-WORKFLOW-MULTISTEP-001 details 3

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
pnpm check:progress
```

**Notes**

Pass: 153 tasks and 784 evidence records; M28 unchanged. /tmp/jj-store-workflow-multistep-progress.log.
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
pnpm spec:validate
```

**Notes**

Pass: 7 validators, 3745 chapters, 196 requirements; final evidence regeneration also validated before commit. /tmp/jj-store-workflow-multistep-spec.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-cli --test server_executable workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart -- --ignored --exact
```

**Notes**

Final shared-output guards use set -eu so missing or reset intermediate artifacts fail rather than allowing a later printf to mask the failure. 1 test passes with the same ordered reports, one-owner, source/history assertions. /tmp/jj-store-workflow-multistep-final.log.
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
cua_repl: exact pending live demo run URL
```

**Notes**

Run 1ed1a78ecfa7695ed2e6a3af53660a3d renders as Pending, Waiting for an eligible host, with exact operation/commit/bookmark controls. It has not executed and no synthetic step/log evidence was published. Host admission/restart approval is pending.
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
pnpm exec prettier --check [changed authored non-Rust files]
```

**Notes**

Pass; generated CLI references and evidence are formatter-ignored.
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
pnpm spec:validate
```

**Notes**

Final 7 validators pass, 3746 chapters/196 requirements. /tmp/jj-store-workflow-multistep-spec-final.log.
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
pnpm check:progress
```

**Notes**

Final ledger passes: 153 tasks/784 evidence records. /tmp/jj-store-workflow-multistep-progress-final.log.
<!-- /run -->
<!-- /evidence-field -->

