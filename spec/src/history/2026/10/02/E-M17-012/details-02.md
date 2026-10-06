---
schema: 1
id: E-M17-012
---

# E-M17-012 details 2

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
cargo test -p jj-store-wal --test agent_catalog common_aliases --locked > /tmp/jj-store-agent-capture-m172-aliases-before.log 2>&1
```

**Notes**

Before: three regressions fail. Hook/transcript channels in either order create two sessions; a result-only new channel does not enrich the published call.
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
cargo fmt -p jj-store-wal
```

**Notes**

Formatted initial identity-join modules and regressions.
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
cargo test -p jj-store-wal --test agent_catalog common_aliases --locked > /tmp/jj-store-agent-capture-m172-aliases-after.log 2>&1
```

**Notes**

All three exact regressions pass after selected alias adoption.
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
cargo fmt -p jj-store-wal -p jj-store-service
```

**Notes**

Formatted ambiguity and local cold-reopen acceptance.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-aliases-wal.log 2>&1
```

**Notes**

Initial 18 WAL catalog tests pass, including legacy ambiguity, source isolation and retained roots.
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
cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_paging --locked > /tmp/jj-store-agent-capture-m172-aliases-local.log 2>&1
```

**Notes**

Command selection failure: the target is local_agent_pages, not local_agent_paging. No product tests ran.
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
cargo fmt -p jj-store-wal -p jj-store-service
```

**Notes**

Formatted selected-target availability checks.
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
cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_pages --locked > /tmp/jj-store-agent-capture-m172-aliases-local-final.log 2>&1
```

**Notes**

Corrected direct local command: 19 tests pass including real disposable companion reopen, receipt replay and later source progress.
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
cargo fmt -p jj-store-wal
```

**Notes**

Formatted UUID spelling, message/turn, tool conflict and bounded alias-scan regressions.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-aliases-wal-final.log 2>&1
```

**Notes**

All 20 WAL tests pass, including native message/turn adoption, distinct missing-ID messages and conflicting tool aliases.
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
cargo test -p jj-store-wal --lib publication::agent_capture_aliases --locked > /tmp/jj-store-agent-capture-m172-aliases-budget.log 2>&1
```

**Notes**

Incomplete alias-count or index-page scans yield no partial identity proof; one exact unit test passes.
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
cargo fmt -p jj-store-wal
```

**Notes**

Formatted cross-profile concurrency regression.
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
cargo test -p jj-store-wal --test agent_catalog common_aliases::concurrent_cross_profile_preparations_cannot_publish_duplicate_native_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-aliases-race-before.log 2>&1
```

**Notes**

A second regression reveals independently prepared CLI/Desktop requests can both publish duplicate native sessions through different product projection keys.
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
cargo fmt -p jj-store-wal
```

**Notes**

Formatted shared V1 identity-join projection precondition.
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
cargo test -p jj-store-wal --test agent_catalog common_aliases::concurrent_cross_profile_preparations_cannot_publish_duplicate_native_sessions --locked -- --exact > /tmp/jj-store-agent-capture-m172-aliases-race-after.log 2>&1
```

**Notes**

Exact race regression passes: stale prepared request cannot publish; fresh explicit preparation adopts the selected session.
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
cargo fmt -p jj-store-wal
```

**Notes**

Extracted unchanged alias staging below the 300-line production budget and formatted exact cold native receipt recovery.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-aliases-wal-complete.log 2>&1
```

**Notes**

All 21 direct WAL catalog tests pass, including original regressions, cross-profile race, retained root and fresh Coordinator MemoryStore recovery with exact request/receipt replay.
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
cargo check -p jj-store-wal -p jj-store-service --all-targets --locked > /tmp/jj-store-agent-capture-m172-aliases-check.log 2>&1
```

**Notes**

Affected packages and direct compiled consumers pass all-target checks.
<!-- /run -->
<!-- /evidence-field -->

