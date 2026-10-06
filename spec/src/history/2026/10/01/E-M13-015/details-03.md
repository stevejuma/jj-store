---
schema: 1
id: E-M13-015
---

# E-M13-015 details 3

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
pnpm check:progress
```

**Notes**

Temporary partial result is not a ledger enum; corrected to pending during verification.
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
pnpm check:progress && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

111 tasks, 15 package boundaries and 132 requirements validated.
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
cargo fmt --all -- --check
```

**Notes**

Service re-export ordering corrected with cargo fmt.
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

Rust formatting applied.
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
pnpm exec prettier --write spec/src/local-jj.md
```

**Notes**

Mapped canonical chapter formatting checked.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked && CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-server -p jj-store-cli --locked
```

**Notes**

All six local browser fixtures pass; server and CLI package checks pass after final bootstrap gating.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Git SHA-1, macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive format-2 filesystem storage and CLI selection; separate Git-profile schemas and signed process-scoped jlb1 cursors. Native codecs and widths unchanged. Enclosing Jujutsu change mmqntqzp.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose pinned WorkStore/AgentStore and freshness, then profile-aware frontend integration and cold mixed acceptance.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Filesystem-only initialization and HTTP composition, registration/discovery/code query transport. Escalation: optional native host and shared generation/discovery affect native and local consumers, so run all HTTP unit contracts and server configuration/installation binaries. Shared contract capture harness adds filesystem responses, so qualify its complete retained native capture set. UI integration and final real-RustFS mixed/cold browser qualification remain subsequent stages.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- M13.4 remains In progress. Local collaboration and agent capabilities remain
  false until routes are implemented.
- Indexed search, code writes, reviews, synchronization and conversion stay
  disabled.
<!-- /evidence-field -->

