---
schema: 1
id: E-M17-065
---

# E-M17-065 details 2

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
cargo test -p jj-store-capture --test reader_publication --locked generations:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-generation-before.log 2>&1
```

**Notes**

Two genuine failures and a fixture ordering error: publishing later within the same repository must respect the native partition head.
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
cargo test -p jj-store-capture --test reader_publication --locked generations:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-generation-before-fixed.log 2>&1
```

**Notes**

Correct cross-repository fixture; all three genuine regressions fail before production change.
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
cargo test -p jj-store-capture --test reader_publication --locked generations:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-generation-after.log 2>&1
```

**Notes**

Same three exact regressions pass.
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
cargo fmt --all
```

**Notes**

Repeated after projection extraction/refinement and missing-channel regression.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-generation-exact-final.log 2>&1
```

**Notes**

16 integration tests pass on final sources including four new generation regressions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-generation-capture-suite.log 2>&1
```

**Notes**

140 top-level tests passed across 16 binaries; one separately invoked child result excluded from the top-level count.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture --all-targets --locked > /tmp/jj-store-agent-capture-m172-generation-check.log 2>&1
```

**Notes**

Affected capture package only.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-generation-lint.log 2>&1
```

**Notes**

Affected capture package only.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generation-doc-format.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-generation-canonical.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-generation-spec-first.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-generation-boundaries.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-generation-architecture.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-generation-progress.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-generation-rust-format.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generation-doc-check.log 2>&1
```
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-generation-progress-final.log 2>&1
```

**Notes**

Repeated after final ledger update.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generation-doc-check-final.log 2>&1
```

**Notes**

Repeated after final ledger update.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic native receipt/device durability evidence, no real harness or RustFS release qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged private version-1 reader publication and reader-progress formats. Read-only aggregation across numeric generations of the exact integration/source/physical-header digest. Current AgentStore only.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change noxowuyz
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact source coverage inspection previously selected only the latest numeric generation and hid earlier physical transcript publication holes. Aggregate all matching physical channels in numeric order, validate every latest range against retained reader progress, retain the prior frontier during pending continuation, and keep missing/unqualified history unavailable. Unrelated physical digests are independent. Three genuine regressions fail three to zero; one fixture first attempted out-of-order publication in the same repository and was corrected to distinct repository partitions before production edits. Fourth regression qualifies loss of an old publication channel. Bounded BTreeMap projection avoids repeated reader-progress scans. No writer, enqueue, receipt format, source lifecycle, HTTP/CLI or UI changes. Call-site inspection found no production consumer of this read API yet; owning capture suite, all-target package check/strict lint and canonical consumers are the complete current impact surface. Server/CLI runtime suites, generated contracts, UI, root aggregate and unrelated Rust workspace are out of scope under targeted policy. Accepted-import association/projection, HTTP/CLI/browser, restart dispatch, remaining readers/enabled startup and M17.3-M17.7 remain pending; M17.2 In progress and full goal active. Preserve all 319 prior evidence entries and raw JSON prefix. Final capture suite passes 140 top-level tests across 16 binaries (child subprocess result separately excluded). Four new regressions pass; exact three-failure before target passes unchanged after fix. Capture all-target check/strict lint and seven canonical/spec-first/boundary/architecture/progress/Rust/document checks pass. No unresolved functional failure. Source coverage prerequisite complete; original-import reader association is next, public progress/API/CLI/browser and full roadmap remain pending.
<!-- /evidence-field -->

