---
schema: 1
id: E-M17-061
---

# E-M17-061 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-import-settlement-clippy.log 2>&1
```

**Notes**


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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-settlement-progress-final.log 2>&1
```

**Notes**

Repeated after final evidence metadata sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-settlement-canonical-final.log 2>&1
```

**Notes**

Final canonical/document verification after direct Rust acceptance.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-settlement-spec-first-final.log 2>&1
```

**Notes**

Final canonical/document verification after direct Rust acceptance.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-settlement-doc-check-final.log 2>&1
```

**Notes**

Final canonical/document verification after direct Rust acceptance.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing JJ change myrxntuv
<!-- /evidence-field -->

<!-- evidence-field:versions yaml -->
## Versions

```yaml
rust: 1.98.1
jj: 0.45.1
platform: macOS ARM64
codex-cli: 0.156.1
codex-desktop: 0.159.2

```
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- Existing control journal V1, 1 MiB/4096 receipts; import journal V1, 2
  MiB/4096 receipts unchanged. Exact owned descriptors/stamps are device memory
  only, noncloneable and nonserializable. Current AgentStore only; no
  source/spool/native/public DTO changes.
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

Current AgentStore only. Owned import settlement consumes the retained installation policy guard and pins the exact control file/directory/lock across native admission awaits. Selection additionally retains the exact import receipt/file stamp and gate; checks reuse held descriptors without recursively reacquiring locks. Replay keeps original metadata even when offered binding is invalid, conditional preselection policy/selector rules remain, and postselection policy/runtime/physical loss is unknown with receipt preserved. Filesystem descriptors release before the manager permit. Public library capability only: supplies no source proof/frame, task, native Write or storage ownership and cannot enable or create runtime. Native source enrollment, import scheduling/progress, HTTP/CLI/browser import and full M17.3-M17.7 remain pending. Concrete escalation before wider runs: owned control pinning and import selection are shared by existing policy guard and synchronous journal callbacks, while shared import-policy fixtures now support settlement tests. Run complete server library plus direct management/runtime/control/import journal integrations; server and direct CLI all-target check/strict lint. Existing capture registry/readers/pipelines/supervisor algorithms, public HTTP/CLI contracts/UI and unrelated Rust workspace/root aggregates are unchanged and out of scope. Rust suites serial with original deadlines and within-scenario concurrency. Preserve all 315 prior parsed entries and raw parent JSON bytes. M17.2 In progress; M17.3-M17.7 Planned; full goal active. No installs/publication/migration/restarts or subagents. Existing macOS unwind-table linker warning remains unsuppressed. Final five-test exact settlement target passes after release-order refinement. Seven canonical/boundary/architecture/progress/Rust/document-format consumers pass with original entries preserved. Final selected direct server suites pass 99 top-level tests across five binaries, including the 62-test owning library and management/runtime/both journals. Native authorization/cancellation/current policy/receipt metadata gates have no unresolved failures. No source worker/import scheduling or body ingestion is implemented by this capability. Server/CLI all-target check and strict lint pass. All 315 prior parsed evidence entries remain unchanged, total 316; parent raw JSON bytes retained before formatting. Owned settlement primitive complete; next is actual staged native source enrollment with renewed accepted-prefix checks, common reader/spool progress, bounded worker/frame/storage ownership and executable import scheduling. Public API/CLI/browser import, remaining readers/enabled startup and M17.3-M17.7 remain pending; full goal active.
<!-- /evidence-field -->

