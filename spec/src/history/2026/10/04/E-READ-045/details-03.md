---
schema: 1
id: E-READ-045
---

# E-READ-045 details 3

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
pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/architecture.md spec/src/compatibility.md spec/src/cli.md docs/implementation-plan.md (before final formatting)
```

**Notes**

One jj-integration.md wrap needed formatting after posting-batch requirement update; formatted changed docs and ledger. local-filter-doc-check-045.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial)
```

**Notes**

Ledger used gaps and omitted top-level result; corrected to canonical remaining and pending fields, preserving all gaps. local-filter-progress-045.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture
```

**Notes**

10pass/23.04s tests; 39.09s command. local-filter-complete-00-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo test -p jj-store-cli --test local_browser --locked local_preparation_ -- --nocapture
```

**Notes**

2pass/5.28s tests; 20.29s command. local-filter-complete-01-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo test -p jj-store-cli --test local_browser --locked local_task -- --nocapture
```

**Notes**

4pass/5.55s tests; 21.02s command. local-filter-complete-02-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_and_bookmark_pages_do_not_load_history_or_index -- --nocapture
```

**Notes**

1pass/15.08s tests; 30.2s command. local-filter-complete-03-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_ -- --nocapture
```

**Notes**

2pass/0.62s tests; 19.6s command. local-filter-complete-04-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_preparation_fits_small_cache_and_preserves_historical_scopes -- --nocapture
```

**Notes**

1pass/1.17s tests; 22.75s command. local-filter-complete-05-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --locked
```

**Notes**

15.93s command. local-filter-complete-06-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

1.15s command. local-filter-complete-07-045.log. Focused direct generation/transport/task/native-writer consumers; no full suite.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/architecture.md spec/src/compatibility.md spec/src/cli.md spec/src/milestones.md docs/implementation-plan.md; pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/architecture.md spec/src/compatibility.md spec/src/cli.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

Final changed-doc formatting and check pass. local-filter-doc-format-complete/ledger-format/doc-final-check-045.log.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate (final)
```

**Notes**

138tasks;12protected files/7canonical chapters;5validators/23chapters/172requirements pass. local-filter-progress-final/spec-first-final/spec-final-045.log. This remains pending overall and promotes no milestone.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- 29 unique focused Rust tests and affected JJ/service/HTTP/CLI checks pass.
  Local indexed filtering is delivered with288 differential windows and
  selected-target hydration. Focused evidence does not complete M13 or combined
  bounded-read qualification.
- Native description filtering, differently embedded operation reference lookup,
  indexed operation child neighbors, deep operation/predecessor/copy/search
  continuations, retained oversized operation differences/change grouping, paged
  native RPC contracts and remaining local application search/index delivery
  remain open.
- Extreme false-positive or hidden-target candidate frontiers still consume
  ordinary work/byte/deadline bounds and fail explicitly; retained
  filtered-query continuation remains delivery work. This artifact stores
  normalized descriptions on disk; complete construction time, disk footprint
  and resident pressure remain unqualified.
- Complete combined integration/contracts, Linux/native2million/near-capacity
  fixtures, measured RSS/release latency, full cold production browser,
  cancellation/fault/revocation/restart/publication/real-RustFS,
  registry/release and Help/source-review refresh remain deferred. No milestone
  promoted. UI/schema/contract generation and repository aggregates are out of
  scope for this Rust-only private delivery. Original main/Linux source and dev
  services unchanged; retained workspace remains open. Visual baselines remain
  unapproved. Inherited __eh_frame linker warning and earlier strict-lint gaps
  remain separately deferred.
<!-- /evidence-field -->

