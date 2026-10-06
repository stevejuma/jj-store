---
schema: 1
id: E-READ-001
---

# E-READ-001 details 2

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
jj --no-pager st; jj workspace list; jj workspace add --name bounded-reads -r @ ../jj-store-bounded-reads
```

**Notes**

Main was clean at b11c0486 plus empty parent change. New isolated workspace preserves other workspaces, the active dev fixture and Linux checkout.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test native_lifecycle --locked read_session_point_lookup_does_not_materialize_inventory -- --exact
```

**Notes**

Before production changes were compiled: one requested operation failed with Limit because total inventory exceeded max_objects=1. Later builds use the isolated workspace target cloned from the existing cache.
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
cargo test -p jj-store-wal --lib --locked publication::reader::tests -- --nocapture
```

**Notes**

Final two point-lookup regressions pass: a 4096-object catalog needs at most one radix path plus its descriptor; followed dependencies require their own membership; historical roots, wrong descriptor keys and complete audit failures remain explicit. Earlier direct-dependency eager variant also passed before being replaced to avoid one lookup per bookmark.
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
cargo test -p jj-store-cli --test native_lifecycle --locked read_session_ -- --nocapture
```

**Notes**

5 read-session tests pass, including exact new regression, staged exclusion, revocation, historical pins, request object/deadline limits and epoch replacement.
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
cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

Escalation: shared ReadAccess changed across content, graph, mutation and adapter consumers. 185 pass. Existing discovery expectation missed profile/availability; two real-time deadlines failed under unrestricted test contention.
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
/Users/stevejuma/code/lapismd/jj-store/target/debug/deps/native_lifecycle-e6652c34f7ed2d93 --exact content_acceptance::http_acceptance::authenticated_repository_discovery_reauthorizes_one_token_per_repository
```

**Notes**

Saved pre-production-change binary reproduces missing native_v1/ready expected fields. Exact response assertion corrected without changing discovery behavior.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 typed_resolve_rebases_descendants_updates_references_and_preserves_evolution --nocapture; target/debug/deps/native_lifecycle-e6652c34f7ed2d93 http_reads_unpolled_response_expiry_releases_transport_and_native_slots --nocapture
```

**Notes**

Both deadline scenarios pass individually with unchanged production limits (11.73s and 1.43s). Full relevant suite will use bounded test concurrency.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --frozen-lockfile --offline --ignore-scripts
```

**Notes**

Tracked local-source pnpmfile checksum does not match lockfile. No lockfile change was made.
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
pnpm install --frozen-lockfile --offline --ignore-scripts --ignore-pnpmfile
```

**Notes**

Installed registry-resolved lockfile dependencies offline; no local package substitution or lockfile edit.
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
pnpm spec:first
```

**Notes**

Initial draft lacked mutations ownership chapter and used an unmapped standalone plan path. Added mapped chapter and moved plan into existing docs/implementation-plan.md; no governance rule weakened.
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
pnpm spec:first; pnpm check:boundaries; pnpm spec:validate
```

**Notes**

Mapped canonical gate, 17-package boundary check and 167-requirement validation pass.
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
cargo fmt --all -- --check; pnpm exec prettier --check docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/mutations.md spec/src/publication.md
```

**Notes**

Focused Rust/document formatting passes.
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
cargo check -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

All targets of affected native read packages and their direct Rust consumers compile.
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --test-threads=4
```

**Notes**

188 passed, 0 failed in 64.44s. Production timeouts unchanged; bounded test concurrency avoids unrelated test contention.
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
pnpm check:progress
```

**Notes**

136-task progress ledger validates.
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
pnpm exec prettier --check spec/src/milestones.md; jj file show -r @- spec/src/milestones.md | pnpm exec prettier --check --stdin-filepath spec/src/milestones.md
```

**Notes**

Whole-ledger formatting warning reproduces in parent. Historical ledger bytes preserved; new evidence uses formatted JSON. No historical whitespace rewrite.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Local JJ Linux fix, independent files/bookmarks loading, on-demand
  preparation, RPC/query/collaboration projections and large-scale
  cold/real-RustFS acceptance remain later slices.
- UI and root aggregate lanes are out of scope for this native read-capability
  slice; visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement query-specific local JJ access, then remaining bounded-read slices and large-scale qualification.
<!-- /evidence-field -->

