---
schema: 1
id: E-EXEC-005
---

# E-EXEC-005

<!-- evidence-field:id markdown -->
## Id

E-EXEC-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-SEC-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/local_browser/binding.rs
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M13/history/M13.4/index.md
- docs/maintenance/coverage.json
- docs/site/reference/transport.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/local_browser/binding.rs#watcher_race_rechecks_before_admitting_the_binding
- crates/jj-store-http/src/local_browser/binding.rs#recheck_refuses_changed_identity_and_continuous_invalidation
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
<!-- /evidence-field -->

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
cargo test --quiet --locked -p jj-store-http --lib local_browser::binding::tests
```

**Notes**

2/2 deterministic tests: one watcher invalidation admits only after a fresh physical check; changed generation and continuous invalidation refuse without a cached stale proof. Log: .local/agent-alignment-binding-tests.log.
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
pnpm exec cargo test --quiet --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

1/1 in the pnpm contract environment. Before the fix, this exact target reproduced the Stacks 403 refusal. Log: .local/agent-alignment-contract-binding-fixed.log.
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
pnpm contracts:check
```

**Notes**

Generated artifacts match; 300 actual HTTP/SSE responses conform, 7 contract tests and TypeScript fixtures pass. Log: .local/agent-alignment-contracts-fixed.log. Agent timestamp additions are checked separately in E-EXEC-003.
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
pnpm docs:review http "A watcher invalidation permits one fresh physical admission recheck before any command dispatch; changed identity/generation and continued invalidation remain refused; bounded agent avatar/CSP admission remains"
```

**Notes**

Fresh HTTP source/manual review; no request replay or authorization widening.
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

macOS ARM64; jj 0.45.1; rustc 1.98.1; Node 24.10.0; pnpm 10.32.1. Enclosing Jujutsu change ywvxqomz.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No identity, operation pin, registration generation, cursor, journal, native publication or admission-cache format changes. Existing physical proofs and limits remain authoritative.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Contract validation initially failed three times in the unrelated filesystem Stacks fixture with 403/access_denied. An isolated plain cargo run could pass while the pnpm environment reproduced it. Temporary diagnostics identified management::Error::Stale from watcher invalidation during physical admission inspection; diagnostics were removed. The HTTP binding now permits exactly one forced fresh physical check after that race. The command has not been dispatched, so this retries only inspection. Changed identity/generation and repeated invalidation still fail closed. Service admission implementation is unchanged. Exact tests and the full contract lane passed after the fix. Root aggregate tests and unrelated Rust/UI suites are out of scope under the targeted policy; final governance/format checks are recorded with E-EXEC-003.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain bounded pre-dispatch admission and qualify remaining M13.4 indexed search/recoverable-code-command gates separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-004 owning bounded-local-code-reads chapter now permits one physical admission recheck before dispatch. JST-SEC-001 current authority, exact identity/generation validation and fail-closed refusal remain unchanged; no stale proof or mutation replay is admitted.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M13.4 remains In progress. This admission-race regression preserves the qualified browsing stage and does not complete indexed search or recoverable code commands.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

