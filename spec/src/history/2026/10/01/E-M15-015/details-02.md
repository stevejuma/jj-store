---
schema: 1
id: E-M15-015
---

# E-M15-015 details 2

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
jj rebase -s 'roots(98506452..@)' -o 98506452
```

**Notes**

15 commits rebased; inspect/resolve all conflicted ancestors; parent unfinished work excluded.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool bound_enqueue_ --locked (before)
```

**Notes**

Zero matching tests; exact additional merge regression absent. /tmp/jj-threads-rebase-before.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-http contract_inventory_covers_security_methods_media_and_task_bodies --locked (before)
```

**Notes**

66 registered routes vs stale 58 assertion; preserve eight parent management routes and all 14 Thread/Reaction routes. /tmp/jj-threads-rebase-http-before.log.
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
cargo fmt -p jj-store-cli -p jj-store-http
```

**Notes**

Focused merge regression and route formatting.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool bound_enqueue_ --locked (after)
```

**Notes**

One exact bound lock/admission/identity regression. /tmp/jj-threads-rebase-bound-enqueue.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-http contract_inventory_covers_security_methods_media_and_task_bodies --locked (after)
```

**Notes**

One exact inventory regression now asserts 66 paths and all eight parent management additions. /tmp/jj-threads-rebase-http-inventory.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked (initial)
```

**Notes**

Native capture publication suite: two transient state-lock failures during unrelated parallel subprocess spawns; stopped at first failed binary. /tmp/jj-threads-rebase-capture-consumers.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_publication publication_wait_allows_capture_from_another_process --locked -- --exact
```

**Notes**

One exact real-child enqueue/preparation/publication/cancellation regression. /tmp/jj-threads-rebase-drain-exact.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_publication repository_mismatch_fails_before_any_publication --locked -- --exact
```

**Notes**

One exact repository preflight regression. /tmp/jj-threads-rebase-repository-exact.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked (diagnostic retry)
```

**Notes**

Different native restart/read regression hit the same state-lock failure during parallel subprocess spawn, proving repetition alone does not qualify the merge. /tmp/jj-threads-rebase-capture-consumers-retry.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib capture_lease_releases_with_an_inherited_file_description --locked (before)
```

**Notes**

Deterministic reproduction: duplicate the open file description as fork does, drop the original lease, then reacquire. Closing a File alone leaves the lease held by the duplicate. /tmp/jj-threads-rebase-inherited-before.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib capture_lease_releases_with_an_inherited_file_description --locked (after)
```

**Notes**

One regression covers explicit release of shared compatibility and exclusive mutation/drain lease kinds while an inherited description remains open. Owned lease Drop unlocks before closing; abrupt process-loss recovery remains OS-owned. /tmp/jj-threads-rebase-inherited-after.log.
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
pnpm web:check
```

**Notes**

Zero errors and zero warnings. /tmp/jj-threads-rebase-web-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/http-workspace-links.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-sync-settings.test.ts
```

**Notes**

Five files, 36 direct browser controller/transport tests. No new presentation or cold browser qualification. /tmp/jj-threads-rebase-web-consumers.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --test local_agent --locked (capture-only lease correction)
```

**Notes**

Ten of eleven library tests pass; credential provisioning release hits the same inherited-descriptor issue under parallel folder-picker spawns. Extracted generic CLI StateLease and used it for both capture and credential creation; no keyring operation was run by this fixture. /tmp/jj-threads-rebase-capture-consumers-fixed.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --test local_agent --locked (helper extraction initial)
```

**Notes**

Compilation rejected stale capture-helper imports after extracting StateLease. Corrected imports; no tests executed. /tmp/jj-threads-rebase-capture-consumers-final.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --test local_agent --locked (final)
```

**Notes**

36 passing tests: 11 CLI library, seven native publication, nine spool, three hook, two local agent and four local recovery regressions. Two ignored fixture entry points are invoked by real child-process tests. Normal test parallelism retained; includes cold hook/local Git capture and credential exclusion. /tmp/jj-threads-rebase-capture-consumers-final-fixed.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test local_agent_binding --test native_collab --test collab_pagination --locked
```

**Notes**

20 passing Rust tests; /tmp/jj-threads-rebase-providers.log
<!-- /run -->
<!-- /evidence-field -->

