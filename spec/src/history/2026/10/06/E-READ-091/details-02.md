---
schema: 1
id: E-READ-091
---

# E-READ-091 details 2

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
jj new e7483f29 -m "Qualify progressive JJ file blame"; jj --no-pager st
```

**Notes**

Clean temporary workspace based on current Main. Retains all committed Stacks work; no unrelated changes.
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
cargo test --locked -p jj-store-jj --lib query::blame_probe:: -- --nocapture
```

**Notes**

Initial batched correctness probe passed (0.63s). SimpleBackend native-width and Git SHA-1 fixtures; ordinary edits, integration merge, repeated lines, empty file, missing final newline; batch sizes 1,2,3,64. Every intermediate proven origin equals the full annotation.
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
cargo test --locked -p jj-store-jj --lib query::blame_probe:: -- --nocapture
```

**Notes**

Initial Linux probe compilation used nonexistent Cancellation::new. Corrected to the existing Default constructor. No product code change.
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
cargo test --locked -p jj-store-jj --lib blame_probe:: -- --nocapture
```

**Notes**

Final focused.log: 1 passed, 2 explicitly ignored opt-in measurements, 83 unrelated tests filtered out. Final fixture run 0.99s. Ignored cases were separately invoked below.
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
JJ_STORE_BLAME_LINUX_METADATA=/Users/stevejuma/code/linux/.jj/repo cargo test --locked -p jj-store-jj --lib local_browser::code_index::blame_probe::linux_first_window_uses_guarded_pinned_reader -- --ignored --exact --nocapture
```

**Notes**

linux-first-window.log: source index copied privately, preparation 4.071s. First/warm first-window seconds COPYING 1.717/1.214, altera_edac.c 1.315/1.358, inode.c 1.283/1.487. This is a direct reader probe, not HTTP and not complete blame.
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
cargo test --locked -p jj-store-jj --lib query::blame_probe::synchronous_comparison_honors_progress_deadline -- --ignored --exact --nocapture
```

**Notes**

synchronous-comparison.log: 240000 bytes and 40000 reverse-ordered unique lines per version, within current 256 KiB preview limit; domain contains one commit. A 1s Tokio timeout returned Ok(Ok(())) after 6.8487925s. Warm progress target &lt;5s failed. FileAnnotator invokes ContentDiff::by_line internally without the service bounded-diff helper or a comparison/cancellation callback. This failure remains intentionally unrepaired and visible behind the opt-in gate; no route or UI was exposed.
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
JJ_STORE_BLAME_LINUX_METADATA=/Users/stevejuma/code/linux/.jj/repo cargo test --locked -p jj-store-jj --lib local_browser::code_index::blame_probe::linux_first_window_uses_guarded_pinned_reader -- --ignored --exact --nocapture
```

**Notes**

linux-source-witness.log: final source metadata names/sizes/modification times unchanged for op heads, source index, Git extra metadata and working-copy state. Preparation 4.114s; first/warm seconds COPYING 2.438/1.828, altera_edac.c 1.502/1.404, inode.c 1.654/1.306. Six reader invocations, each starts fresh annotation and computes one 64-commit window. No retained result cache is implemented. Object bytes/work respectively: 6505114/311675, 7002423/321457, 6776053/312149; 111 metadata records each. Origins found 0/20, 0/2225, 0/3061. OS caches were not cleared; package check ran concurrently. Source witness is not a complete integrity or authorization test.
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
cargo check --locked -p jj-store-jj
```

**Notes**

check.log: affected production package check passed. No unrelated Rust packages or UI suites invoked.
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
pnpm spec:first
```

**Notes**

spec-first.log: mapped local-jj canonical chapter missing for local reader probe registration. Added explicit local feasibility boundary; no transport change.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/jj-integration.md spec/src/local-jj.md spec/src/spec-governance.md
```

**Notes**

format.log: two new prose sections needed wrapping; ran prettier --write on jj-integration.md and spec-governance.md.
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
rustfmt --edition 2024 --check crates/jj-store-jj/src/query/blame_probe.rs crates/jj-store-jj/src/local_browser/code_index/blame_probe.rs
```

**Notes**

Only new Rust probe files formatted with rustfmt; final targeted formatting check passed.
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
pnpm spec:first
```

**Notes**

spec-first-final.log: four protected files/four canonical chapters; local chapter gap resolved.
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
pnpm spec:validate
```

**Notes**

spec-validate.log: five validators, 23 chapters, 173 requirements.
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
pnpm check:progress
```

**Notes**

progress.log: 142 tasks; evidence bookkeeping only; stopped feature gate remains blocked.
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/local-jj.md spec/src/spec-governance.md
```

**Notes**

format-final.log: all changed prose formatted. New ledger entry formatted separately as JSON to preserve historical evidence.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-jj -- --check
```

**Notes**

rust-format.log: two module declarations needed alphabetical ordering; repaired only the new cfg(test) registrations.
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
cargo fmt -p jj-store-jj -- --check
```

**Notes**

rust-format-final.log: affected Rust package formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

blocked
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The explicit bounded-comparison/cancellation gate fails on pinned JJ 0.45.1.
  Smaller history windows do not bound or interrupt one internal synchronous
  comparison. Do not expose this prototype as a production Blame reader.
- Full Linux attribution, first-proven-origin time and retained-memory
  measurements were not pursued after the blocker. The first-window latency
  target passes, but all sampled lines remain unresolved at that window.
- Production native-backend authorization, stale/expired cursors, cross-request
  retention, coalescing, retries, cancellation rollback, API/schema generation,
  UI and cold browser acceptance remain unimplemented/unqualified. The
  native-width correctness fixture is SimpleBackend, not a native service
  authorization proof.
- Existing progress/task statuses remain unchanged. No milestone promotion. Full
  suites, root aggregate checks, release/RustFS qualification, visual baselines
  and service restarts are out of scope for this test-only stopped slice.
<!-- /evidence-field -->

