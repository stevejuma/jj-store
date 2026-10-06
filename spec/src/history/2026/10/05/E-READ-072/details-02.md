---
schema: 1
id: E-READ-072
---

# E-READ-072 details 2

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
jj rebase -r @ -o xvyloluq
```

**Notes**

After verified main landing a6c7b0de, rebase empty temporary change onto it; tree comparison empty. Reuse28 main evidence files byte-identically in landing-evidence-reuse-071.json.
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
cargo test -p jj-store-jj --test copy_index --locked copy_family_includes_ancestor_descendants_without_unrelated_parent_branches -- --nocapture
```

**Notes**

Baseline fails compilation with one unresolved copy_index module import; no functional tests ran. copy-index-before-072.log.
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

Rust formatting completed after module and test edits.
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
cargo test -p jj-store-jj --test copy_index --locked copy_family_includes_ancestor_descendants_without_unrelated_parent_branches -- --nocapture
```

**Notes**

Exact new target passes against implemented index; copy-index-after-072.log.
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
cargo test -p jj-store-jj --test copy_index --locked -- --nocapture
```

**Notes**

All6 new acceptance tests pass; copy-index-units-072.log.
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
cargo check -p jj-store-jj --all-targets --locked
```

**Notes**

Initial owning package check passes in14.51s; copy-index-package-072.log.
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
cargo test -p jj-store-jj --test copy_index --locked -- --nocapture
```

**Notes**

7.12s; .local/bounded-reads/performance/copy-index-final-units-072.log
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
cargo check -p jj-store-jj --all-targets --locked
```

**Notes**

3.97s; .local/bounded-reads/performance/copy-index-final-package-072.log
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
cargo fmt --all -- --check
```

**Notes**

5.28s; .local/bounded-reads/performance/copy-index-rust-format-072.log
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
pnpm check:boundaries
```

**Notes**

1.42s; .local/bounded-reads/performance/copy-index-boundaries-072.log
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
pnpm check:architecture
```

**Notes**

1.55s; .local/bounded-reads/performance/copy-index-architecture-072.log
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

2.11s; .local/bounded-reads/performance/copy-index-spec-first-072.log
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

4.39s; .local/bounded-reads/performance/copy-index-spec-validation-072.log
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

5.47s; .local/bounded-reads/performance/copy-index-progress-072.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

4.92s; .local/bounded-reads/performance/copy-index-document-format-072.log
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

Changed canonical and plan documents formatted; copy-index-document-write-072.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Builder consumes descriptor parent sets and creates reverse adjacency only
  after node/edge/work/byte admission. Conservative allocation covers1MiB
  header,1024 bytes per node and512 per edge, including sealing degree/ready
  scratch. Exact duplicate insertion is idempotent; substituted parent relations
  fail. Missing parents and cycles prevent sealing.
- Family lookup selects requested ancestors and all descendants of those
  ancestors. An unrelated parent of a merge descendant is not promoted to an
  ancestor. Result/visited identities and512-byte-per-node traversal scratch are
  admitted before growth; edge/node visits charge query work. Checks precede
  traversal and delivery; CPU loops yield every128 work units for pending
  cancellation.
- Pressure fixture seals10007 copy nodes, then selects the exact five-node
  family under5-node/2560-byte/32-work lookup admission, excluding10000
  unrelated copies and the unrelated parent branch. Reverse-root, leaf and
  multiple-seed cases preserve required descendants. Malformed graphs, failed
  growth without mutation, work/byte/node limits, missing seed, expired
  deadline, preexisting and pending cancellation are covered.
- Index contains only owned IDs and adjacency, retains no
  source/authentication/cancellation capability or native object body, and
  cannot publish. Lookup returns an identity set; deterministic
  children-before-parents body delivery stays in the backend integration. Global
  allocation leases, source/epoch/root scope, current authorization and
  staged-session semantics are not implemented by this primitive.
- All512 prior evidence objects and their original text are preserved verbatim.
  Main remains clean on verified E-READ-071; this follow-up is only in the
  retained temporary workspace. No endpoint behavior, installed executable,
  service, fixture or Linux source changed.
- Final6/6 acceptance tests pass after checked node-growth arithmetic and
  explicit expired-sealing deadline evidence. Owning JJ all-target check, Rust
  formatting, boundaries, architecture, spec-first, spec-validation, progress
  and changed-document formatting pass. No unresolved focused product failure;
  no complete qualification or milestone promotion.
<!-- /evidence-field -->

