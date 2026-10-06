---
schema: 1
id: E-M17-042
---

# E-M17-042 details 3

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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-foreground-progress.log 2>&1
```

**Notes**

129 tasks validate; prior 296 evidence entries retained. Repeated after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-foreground-doc-check.log 2>&1
```

**Notes**

All four documents pass final formatting after sealing; initial style failure remains recorded.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Native external-S3 lifetime fixture performs no S3 requests. Disposable mock embedded process and actual private OS filesystem locks; no real RustFS or live harness qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Exact File descriptor transfer into retained storage; no descriptor cloning. Only internal ownership/extraction changes; public process entrypoints, ordinary routes, configuration, native domains, cursor/spool/prepared/receipt formats remain unchanged. Current-only AgentStore stays supported.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tssullov
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

Foreground preparation previously owned external native-serving state locks separately from storage, so drop/run cancellation could release the lock while capture retained storage. Native preparation now transfers the original descriptor into storage before any capture lifetime; filesystem-only preparation retains its independent process lock. Late and duplicate attachment fail closed, preserving existing ownership and releasing rejected input. The large process module is extracted into owning preparation/authority/running modules below 300 lines without changing ordinary API/RPC/HTTP/search/replication behavior or the automatic enabled-startup guard. Shared process preparation and runtime/storage ownership affect native, filesystem, search and executable consumers, requiring complete owning library, configuration/embedded inventory, explicit capture runtime and cold CLI executable plus exact filesystem runtime acceptance, affected package checks and strict lint. This is the concrete escalation reason for wider direct lanes; root aggregates, unrelated UI/whole Rust workspace and real storage qualification remain out of scope. The full library lane ran before this entry was written; its escalation reason is shared lifecycle/storage infrastructure, also announced before remaining wider direct commands. Prior 296 entries are preserved. M17/M17.2 remain In progress: actual configured capture worker resource preparation/start/drain integration, broader fault/rotation/root-rebinding/discovery-delivery loss proofs, Cursor/OpenCode, typed authenticated controls/coverage UI, real RustFS/harness/cold full flow and M17.3-M17.7 remain pending. Only existing explicit native capture storage lifetimes are composed here; no configured worker is started, no source roots are scanned, no installed binaries/services/daemons, checkpoints, credentials, deployment or publication changed. Final owning/direct-consumer acceptance and all-target check/strict lint, Rust formatting, boundaries, architecture and canonical gates pass. The initial three-document style failure is retained and fixed. No configured capture worker is connected by this slice. Changed behavioral process modules remain at 119-247 lines. The exact foreground regression, four fresh-process cases, complete library, runtime/configuration and cold executable/filesystem direct consumers qualify retained process-lock ownership only; five real-storage/deployment ignores are not passes. No milestone completion is claimed.
<!-- /evidence-field -->

