---
schema: 1
id: E-READ-047
---

# E-READ-047 details 3

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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

25.6s command. native-filter-consumer-09-047.log. Focused direct consumers; no full suite.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

2.41s command. native-filter-consumer-10-047.log. Focused direct consumers; no full suite.
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
pnpm check:ui-ownership
```

**Notes**

5.25s command. native-filter-consumer-11-047.log. Focused direct consumers; no full suite.
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

3.46s command. native-filter-consumer-12-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_acceptance:: -- --nocapture
```

**Notes**

6pass/0failed/1.89s tests,17.80s build. native-filter-preparation-consumers-047.log. Correct module filter covers preparation policy, independent work/membership bounds, explicit task/cursor pinning, disposal and cancellation.
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
cargo test -p jj-store-jj --lib --locked native_resident_admission_counts_index_allocations_and_bounded_filter_cache -- --nocapture (before)
```

**Notes**

1fail/0pass/6.25s build. native-filter-resident-before-047.log. Old retention counted a1GiB disk-only database as resident RAM and produced2965377122bytes, rejecting the bounded two-million-row allocation. Arithmetic only; no actual large fixture.
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
cargo test -p jj-store-jj --lib --locked native_resident_admission_counts_index_allocations_and_bounded_filter_cache -- --nocapture (after)
```

**Notes**

1pass/0failed/7.17s build. native-filter-resident-after-047.log. Actual framed allocations plus bounded filter cache leave128MiB headroom in unchanged pool; another512MiB disk-file growth does not increase resident admission. Full file remains counted by cache disk quota; no RSS proof.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate (initial)
```

**Notes**

138tasks;27protected files/4chapters;5validators/23chapters/172requirements pass. native-filter-progress/spec-first/spec-047.log. Initial evidence before final retention adjustment; final checks follow. No milestone promotion.
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
cargo test -p jj-store-jj --lib --locked query::cache -- --nocapture
```

**Notes**

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 0.21s. 1.23s command. native-filter-admission-final-00-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filter -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 22.20s. 95.57s command. native-filter-admission-final-01-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cache_acceptance -- --nocapture
```

**Notes**

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 204 filtered out; finished in 6.02s. 20.74s command. native-filter-admission-final-02-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_ -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 217 filtered out; finished in 1.48s. 17.9s command. native-filter-admission-final-03-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_acceptance:: -- --nocapture
```

**Notes**

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 216 filtered out; finished in 1.81s. 19.97s command. native-filter-admission-final-04-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 0.93s. 15.91s command. native-filter-admission-final-05-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

15.39s command. native-filter-admission-final-06-047.log. Final resident-admission adjustment; focused affected consumers only.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

1.04s command. native-filter-admission-final-07-047.log. Final resident-admission adjustment; focused affected consumers only.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/architecture.md spec/src/cli.md docs/implementation-plan.md; pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Changed canonical/doc/ledger formatting complete. native-filter-admission-doc-format/ledger-admission-format-047.log. Final checks below.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md (complete)
```

**Notes**

Final admission-adjusted evidence:138tasks;27protected files/4canonical chapters;5validators/23chapters/172requirements and changed-doc formatting pass. native-filter-progress-complete/spec-first-complete/spec-complete/format-complete-047.log. No milestone promotion; complete qualification remains pending.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

