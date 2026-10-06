---
schema: 1
id: E-M4-011
---

# E-M4-011 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Initial fixture compile error: CommittedRead.graph requires an Arc receiver; corrected helper type.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Before implementation: 2 real regressions accept oversized file bytes/lines. The third merge fixture returned Invalid because its path predicate omitted logical workspace context; this result was not accepted as merge-limit evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

After byte/line guards: 2 pass / merge case still Invalid. Switching to files(all()) did not remove native workspace requirements.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_derived_merge_terms_without_limiting_plain_log
```

**Notes**

First temporary preflight-disabled probe still lacked workspace context; Invalid, not valid baseline evidence. Source restored afterward.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Expanded suite before workspace fixture correction: 2 pass / 3 Invalid failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_derived_merge_terms_without_limiting_plain_log
```

**Notes**

Corrected fixture supplies a published logical workspace. With only preflight temporarily disabled, the 34-parent input succeeds instead of Limit. This is the actual merge regression baseline; preflight restored afterward.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

After workspace correction: 4 pass / 1 parity expectation fails. Native 3-by-2 criss-cross merge behavior differs from the fixture assumption. Replaced the assumption with direct pinned native evaluation, preserving exact result comparison.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Oracle fixture had 2 compile diagnostics: direct chrono dependency unavailable and missing expected binding. Used inferred native date type and corrected binding.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Five cases pass, including direct native oracle comparison across 4 histories and 4 predicates.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_recursive_tree_depth_for_graphs_and_content
```

**Notes**

Before path admission, a 70-level tree succeeds through graph traversal instead of Limit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Six cases pass after backend tree-path admission, including graph and content depth rejection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

Boundary fixture: 6 pass / 1 native sorted-tree assertion fails. Zero-padded generated entry names preserve canonical lexical ordering.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

After fixture ordering repair: 6 cases pass, final repeated-file predicate spins under pollster. Live process and one-second sample prove FileReader/Tokio cooperative-budget exhaustion. Owned test process 16674 terminated after diagnosis; no deadline pass claimed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset
```

**Notes**

Bounded manual polling of a budget-consuming returned reader reproduces the runtime gap.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset
```

**Notes**

Exact regression passes after routing each reader poll through RuntimeContext.run; genuine Pending is preserved.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_
```

**Notes**

All 7 cases pass in 2.25 seconds after reader fix: accepted 256 KiB / 901-line input, exact 4 MiB repeated-path reads, over-budget rejections, recursive merges/depth and upstream parity.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --release --locked -p jj-store-cli --test native_lifecycle --no-run --message-format=json-render-diagnostics
```

**Notes**

Release native test binary compiled, then rebuilt after the returned-reader repair and conservative temporary-label reservation.
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
python3 scripts/qualify-query-resources.py --scenario predicates --binary /Users/stevejuma/code/lapismd/jj-store/target/release/deps/native_lifecycle-fa95bbe1873b7c06 --output docs/evidence/m4-native-predicate-budget.json
```

**Notes**

All 7 named acceptance cases pass in final release process: 4.10 seconds elapsed / 0.85 CPU seconds / 19169280 peak RSS bytes. Earlier repaired release process also passed (2.17 elapsed / 0.76 CPU / 19595264 RSS) before final label reservation.
<!-- /run -->
<!-- /evidence-field -->

