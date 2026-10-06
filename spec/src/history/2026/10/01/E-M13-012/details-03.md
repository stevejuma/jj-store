---
schema: 1
id: E-M13-012
---

# E-M13-012 details 3

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
rustc --version; jj version; git --version; jj --no-pager st (local workspace)
```

**Notes**

Rust 1.98.1, JJ 0.45.1 and Git 2.55.0 verified. Isolated workspace remains clean at wpqnwurs/fbb792d5 with qtxvrqqm/0823662b parent.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Evidence entry normalized; repeated once to retain complete formatter result.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

Generated public contracts match. Eleven exact native HTTP/SSE fixture captures execute; all 80 responses conform. Six contract validation regressions and generated TypeScript consumer check pass. This required contract lane covers the retained native browsing/collaboration/work boundaries without unrelated browser or root suites.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test executable_native custom_help_completion_build_info_and_ordinary_git_commands -- --exact
```

**Notes**

1 ordinary-JJ/custom help/completion/build-info executable regression passes; 24 unrelated cases filtered. CLI remains a stock Git-backed JJ superset.
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
pnpm exec prettier --check AGENTS.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md
```

**Notes**

Final changed-document formatting passes; repeated after final evidence normalization.
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
pnpm exec spec-validator first --base 6350738b6dafd45045c405cfcf9b2c1b73056e3b --head 8e0883ca5b396c3af19cfe95aefee9fa3b0d6e62
```

**Notes**

Final integration snapshot passes for 255 protected paths and 15 owning chapters; same explicit merge gate is repeated after evidence-only normalization.
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
ps -p 60331,60596 -o pid,ppid,etime,command
```

**Notes**

Existing main development service and embedded RustFS retain their original PIDs; integration performs no restart or deployment.
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

<!-- evidence-field:deferred list -->
## Deferred

- M13.3 portable code/agent review anchors, backend-aware review
  CLI/detail/pagination/retry, automatic hook code associations and native
  capsule carrier remain pending. Repository-only review persistence does not
  enable reviews capability.
- M13.4 filesystem-only LocalTrusted server/UI, device folder bindings,
  versioned APIs/code search and recoverable code mutation journal remain
  planned.
- M13.5 bounded durable independent local/native conversion, complete
  collaboration history/evidence archives, mapping reports, fresh destination
  operation log, source-safe preflight and manifest-last/new-folder publication
  remain planned.
- M13.6 cold completed-flow qualification and retained native regressions remain
  planned.
- No metadata network synchronization, installed CLI replacement, publication or
  service restart. Root aggregates, full Rust workspace, unrelated UI/browser
  and real-RustFS lanes remain out of scope; previously qualified main UI/demo
  source and evidence are retained.
- Existing M11.4 shared-package release/visual gates and the unrelated
  unfiltered real-browser Git-fixture collector mismatch remain recorded in
  retained parent evidence; this integration neither completes nor changes those
  gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1 (48a229cea); JJ 0.45.1; Git 2.55.0; Git SHA-1 companion profile.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing refs/lapis/state/v1 immutable companion commit/receipt profile, LocalWorkStore V1, canonical AgentStore/ReviewStore records and typed portable capsules. Native manifest/WAL/identity formats unchanged. Merge retains original parent evidence and canonical requirement identities. Integration introduces no new persistence format or public contract.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the landed native/local regression gates; resume the explicit pending M13.3 portable review/capture capabilities in a subsequent implementation slice.
<!-- /evidence-field -->

