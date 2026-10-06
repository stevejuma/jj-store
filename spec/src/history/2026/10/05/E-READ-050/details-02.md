---
schema: 1
id: E-READ-050
---

# E-READ-050 details 2

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
jj --no-pager st; jj rebase -r opnpqprl -o woktsopu; jj describe -m "Reuse native prepared generations for embedded reference operations"
```

**Notes**

Verified main merge woktsopu/69f37e5b is landed and clean; retained bounded-reads workspace now has that parent, with ignored evidence preserved. No other workspace or development service changed.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance::reference_embedded::native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority -- --exact --nocapture (before production change)
```

**Notes**

Exact new regression fails with Limit instead of IndexRebuildRequired: a differently embedded historical request bypasses prepared dispatch and walks history despite explicit source selection.0 passed,1 failed,223 filtered out. Log embedded-before-050.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance::reference_embedded::native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority -- --exact --nocapture (first implementation compile)
```

**Notes**

Two Rust visibility diagnostics (E0364/E0603) for a private helper re-export; corrected the helper visibility to the owning query module, keeping it non-public. No test executed. Log embedded-after-050.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance::reference_embedded::native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority -- --exact --nocapture (first corrected implementation)
```

**Notes**

1 passed,223 filtered out;19.20s test run. Log embedded-after-fixed-050.log. A later test-only reordering verifies the frozen older capability before extensive differential windows, retaining ordinary request deadlines; final exact target and direct family pass again.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance::reference_embedded::native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 17.85s; 35.98s command. embedded-final-050.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance::reference -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 49.92s; 64.26s command. embedded-reference-consumers-050.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked content_acceptance::content_read_rechecks_revocation_and_limits_and_rejects_staged_objects -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 0.22s; 13.64s command. embedded-current-access-050.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked read_session_pins_history_and_reports_divergent_heads_without_publication -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 0.20s; 13.74s command. embedded-pinned-access-050.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

    Finished `dev` profile [unoptimized] target(s) in 29.46s; 29.55s command. embedded-package-check-050.log.
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

Exit 0; 0.92s command. embedded-rust-format-050.log.
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages) embedded-boundaries-050.log.
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

Spec-first gate passed: 7 protected file(s), 5 canonical chapter(s). embedded-spec-first-050.log.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/milestones.md
```

**Notes**

docs/implementation-plan.md 115ms (unchanged); spec/src/milestones.md 1443ms embedded-doc-format-050.log.
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

progress ok (139 tasks; evidence bookkeeping only) embedded-progress-050.log.
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
pnpm spec:validate
```

**Notes**

Specification validated: 5 validators, 23 chapters, 172 requirements. embedded-spec-050.log.
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
pnpm exec prettier --check docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/milestones.md
```

**Notes**

All matched files use Prettier code style! embedded-format-check-050.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native differently embedded reference delivery passes 7 distinct focused Rust
  tests:5 reference/filter/window/operation cases plus current-access and
  frozen-read security. New exact regression compares 48 windows
  (query/kind/direction/anchor), preserves outer snapshot and source page
  operation, records at most 3 native object reads per page, recovers through
  explicit source preparation and rejects newer/unpublished operations absent
  from the outer capability despite warm caches. Point-only access refuses full
  inventory enumeration. Package/format/boundary/spec-first checks pass; final
  ledger/spec and documentation formatting checks also pass. These synthetic
  correctness bounds are not measured RSS or large-fixture qualification.
- Local differently embedded source recovery, indexed child neighbors, deeper
  operation/predecessor/copy/search continuations, retained oversized
  comparisons/change groups and extreme candidate continuation, paged native RPC
  and remaining local application search/index remain delivery work.
- Complete large-scale, measured RSS/latency, fault/lifecycle/cold
  browser/RustFS and release qualification remain deferred; no milestone
  promotion. Main remains the verified E-READ-049 landing; follow-up changes
  stay in bounded-reads.
<!-- /evidence-field -->

