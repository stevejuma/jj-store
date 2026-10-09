---
schema: 1
id: E-CHAT-WORKSPACE-001
---

# E-CHAT-WORKSPACE-001 details 3

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
pnpm contracts:check
```

**Notes**

Concurrent source edits crossed the service/server build boundary while the contract-capture lane compiled, causing E0560 for incomplete. Final source has the field; rerun after source is stable.
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
pnpm exec prettier --write spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

Canonical formatting.
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
pnpm evidence:new -- --id E-CHAT-WORKSPACE-001 --task M24 --requirement JST-COLLAB-034 --requirement JST-COLLAB-035 --requirement JST-COLLAB-036
```

**Notes**

Created the evidence record; filled actual results here.
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
cargo test --locked -p jj-store-cli --test agent_execution tooling::runtime_receipts_recover_without_execution_or_automatic_install_replay -- --exact
```

**Notes**

Focused rerun passes after correcting the stale runner display label.
<!-- /run -->

<!-- run -->
### Run 5

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

Initial evidence draft used bare test names and omitted affected files; corrected to file references and complete changed-file coverage.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Durable file review publication, cross-repository submissions, UI integration
  and cold native/local acceptance remain required subsequent slices.
- Turn-snapshot evidence retention beyond existing operation lifetime must be
  qualified with file evidence; this slice alone does not close GC/cleanup
  acceptance.
- ResizeObserver recheck and visual-baseline approval are not covered by backend
  checks.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0, pnpm 10.32.1, Rust cargo 1.98.1, JJ 0.45.1. Enclosing Jujutsu change is the revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive workspace inspection DTO v1 and private execution boundary events. Native object/manifest and existing ReviewStore encodings unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Backend inspection and queued dispatch boundaries in jj-store-chat-body. Preserves main workspace, development services and existing live fixture. Bounded typed pinned native/local reads and current physical scans do not snapshot. New start snapshots occur only before explicit prompt dispatch.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify retained review sources, then repository-qualified submissions and the shared workspace inspector UI.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Workspace inspection and pre-dispatch boundary behavior added in owning
  collaboration, API, JJ, mutations, CLI and compatibility contracts. Existing
  manifest/ref-CAS authority, native encodings and no-replay rules preserved.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Read-only inspection does not qualify M13.3 local
  code-review publication or the remaining cold file-comment flows. Visual
  baselines remain unapproved.

```
<!-- /evidence-field -->

