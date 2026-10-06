---
schema: 1
id: E-M17-004
---

# E-M17-004 details 3

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
cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_pages --test agent_sessions --test local_collab --test native_collab --test collab_pagination --locked > /tmp/jj-store-agent-capture-m171-service.log 2>&1
```

**Notes**

47 direct service tests passed: capture/root-pinned pagination, native/current authorization and local companion Thread/reaction retention.
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
cargo test -p jj-store-cli --test agent_capture_publication bound_diagnostic_gap_drains_after_restart_without_a_session_edit --locked -- --exact > /tmp/jj-store-agent-capture-m171-spool-exact.log 2>&1
```

**Notes**

Exact private-spool gap regression passes through unknown outcome, restart and exact receipt completion with zero session edits. Fake publisher checks request/spool framing separately from native/local provider acceptance.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --locked > /tmp/jj-store-agent-capture-m171-spool.log 2>&1
```

**Notes**

17 direct capture/spool tests passed; two ignored child-process entry points are invoked explicitly by their parent lease/crash tests.
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
cargo test -p jj-store-rpc --test collab --test agent_publication --locked > /tmp/jj-store-agent-capture-m171-rpc.log 2>&1
```

**Notes**

Five direct Unix RPC tests passed, including old session binding and new diagnostic gap preparation/reconstruction.
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
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --lib --test bound_capture --test local_agent_binding --test collab --test agent_publication --test agent_capture_publication --locked -- -D warnings > /tmp/jj-store-agent-capture-m171-clippy.log 2>&1
```

**Notes**

Transitive untouched jj-store-http source emitted three existing needless_borrow warnings in local_browser.rs. No product changes made there. Restrict clippy to the selected affected packages with --no-deps; HTTP compilation/contracts remain qualified separately.
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
cargo clippy --no-deps -p jj-collab -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --lib --test bound_capture --test local_agent_binding --test collab --test agent_publication --test agent_capture_publication --locked -- -D warnings > /tmp/jj-store-agent-capture-m171-clippy-focused.log 2>&1
```

**Notes**

Strict affected libraries and capture/RPC/test targets pass; transitive untouched HTTP lint baseline remains recorded.
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
cargo test -p jj-store-service --test local_agent_binding bound_gap_with_observed_assignment_preserves_the_real_session_and_rejects_stale_intent --locked -- --exact > /tmp/jj-store-agent-capture-m171-assignment-exact.log 2>&1
```

**Notes**

Initial existing-session gap acceptance passes without replacing the real session or primary assignment.
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
cargo test -p jj-store-service --test local_agent_binding bound_gap_with_observed_assignment_preserves_the_real_session_and_rejects_stale_intent --locked -- --exact > /tmp/jj-store-agent-capture-m171-assignment-exact-final.log 2>&1
```

**Notes**

Strengthened final regression rejects the old assignment after a real explicit reassignment, with a fresh valid source batch and current destination Thread version; neither root selects.
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
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance::http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts --locked -- --exact > /tmp/jj-store-agent-capture-m171-http-exact.log 2>&1
```

**Notes**

Actual HTTP timeline exposes the exact gap binding after native publication. Fixture includes gap timeline in generated-contract samples and retains existing feedback/asset/receipt checks.
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
cargo test -p jj-store-service --test local_agent_binding --locked > /tmp/jj-store-agent-capture-m171-local-binding-final.log 2>&1
```

**Notes**

All five final local binding tests pass, including archived Thread/GC and stale original assignment after actual reassignment.
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
pnpm contracts:check > /tmp/jj-store-agent-capture-m171-contracts.log 2>&1
```

**Notes**

OpenAPI/TypeScript generated output and 139 actual HTTP/SSE native/local response captures including the gap timeline conform; seven contract tests and contract TypeScript check passed.
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
cargo clippy --no-deps -p jj-collab -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --lib --test bound_capture --test local_agent_binding --test collab --test agent_publication --test agent_capture_publication --test native_lifecycle --locked -- -D warnings > /tmp/jj-store-agent-capture-m171-clippy-final.log 2>&1
```

**Notes**

Final strict clippy includes strengthened assignment test and actual HTTP acceptance fixture, with untouched dependency lint baseline excluded under targeted policy.
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

Five validators passed: 21 chapters and 160 requirements.
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
pnpm spec:first
```

**Notes**

Same-change governance passed: 20 protected files and 10 owning chapters.
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

Progress ledger validates all 129 tasks.
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
pnpm check:boundaries
```

**Notes**

Dependency boundary check passed for all 16 packages.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passed.
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
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/ui.md spec/src/verification.md schemas/openapi.json web/src/lib/api/generated.ts web/src/lib/ui/store-threads/thread-context.ts web/tests/vite/thread-context.test.ts
```

**Notes**

All changed canonical, generated and UI files passed formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

