---
schema: 1
id: E-M17-014
---

# E-M17-014 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test source_registry --locked > /tmp/jj-store-agent-capture-m172-registry-before.log 2>&1
```

**Notes**

Before state: two unresolved-import errors prove durable registration and leased source APIs are absent.
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
cargo fmt -p jj-store-capture
```

**Notes**

Repeated during implementation and after final changes; all production modules remain below 300 lines.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test source_registry --locked > /tmp/jj-store-agent-capture-m172-registry-after.log 2>&1
```

**Notes**

First executable regression: three fixture failures. Candidate paths were compared against noncanonical macOS temporary aliases, and one test passed an existing nonprivate temporary root. Corrected fixture canonical comparisons and explicit private child state; production ownership checks remain strict.
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
cargo test -p jj-store-capture --test source_registry --locked > /tmp/jj-store-agent-capture-m172-registry-corrected.log 2>&1
```

**Notes**

Four initial restart/overlap/hardlink, bound-reader and cross-process tests pass.
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
cargo test -p jj-store-capture --test source_registry --test source_registry_policy --locked > /tmp/jj-store-agent-capture-m172-registry-policy.log 2>&1
```

**Notes**

Eight initial policy/lease tests pass.
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
cargo test -p jj-store-capture --test source_registry --test source_registry_policy --test codex_bound_source --test codex_discovery --test codex_transcript --locked > /tmp/jj-store-agent-capture-m172-registry-runtime.log 2>&1
```

**Notes**

Ten registration/lease tests and all 18 existing bound/discovery/transcript reader tests pass.
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
cargo check -p jj-store-capture --all-targets --locked > /tmp/jj-store-agent-capture-m172-registry-check.log 2>&1
```

**Notes**

Initial all-target affected-package check passes.
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
cargo test -p jj-store-capture --test source_registry --test source_registry_policy --test codex_bound_source --test codex_discovery --test codex_transcript --locked > /tmp/jj-store-agent-capture-m172-registry-final-runtime.log 2>&1
```

**Notes**

29 tests pass, including missing catalog and killed-reader recovery; existing readers remain compatible.
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
cargo test -p jj-store-capture --lib source_registry --locked > /tmp/jj-store-agent-capture-m172-registry-unit.log 2>&1
```

**Notes**

Initial exact retained-alias budget test passes.
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
cargo clippy -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-registry-clippy.log 2>&1
```

**Notes**

Initial strict affected-package all-target lint passes.
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
cargo test -p jj-store-capture --lib source_registry --locked > /tmp/jj-store-agent-capture-m172-registry-final-unit.log 2>&1
```

**Notes**

Two unit regressions pass: exact 4096 alias capacity without eviction and explicit lease release with an inherited open file description.
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
cargo check -p jj-store-capture --all-targets --locked > /tmp/jj-store-agent-capture-m172-registry-final-check.log 2>&1
```

**Notes**

Final production all-target check passes.
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
cargo clippy -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-registry-final-clippy.log 2>&1
```

**Notes**

Final production strict lint passes; repeated after adding the last header-rewrite test.
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
cargo test -p jj-store-capture --test source_registry --test source_registry_policy --locked > /tmp/jj-store-agent-capture-m172-registry-final-leases.log 2>&1
```

**Notes**

All 12 final lease/policy tests pass, including new header qualification before readers return batches, missing/corrupt catalogs, independent concurrent channels, changed-header unknown aliases and state rebinding.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-registry-format.log 2>&1
```

**Notes**

Rust formatting passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-registry-boundaries.log 2>&1
```

**Notes**

Owned dependency boundaries pass; no new dependencies or upstream JJ imports.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/architecture.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-registry-doc-write.log 2>&1
```

**Notes**

Changed canonical documents formatted; all 268 parent evidence records preserve their parsed structures.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-registry-progress.log 2>&1
```

**Notes**

129 tasks validate with 269 evidence entries; M17.2 and aggregate M17 remain In progress.
<!-- /run -->
<!-- /evidence-field -->

