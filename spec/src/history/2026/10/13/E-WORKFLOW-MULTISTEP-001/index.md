---
schema: 1
id: E-WORKFLOW-MULTISTEP-001
---

# E-WORKFLOW-MULTISTEP-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-MULTISTEP-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-003
- JST-WF-004
- JST-WF-005
- JST-WF-006
- JST-WF-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/workflow_host/command.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- docs/cli/workflows.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-workflow.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/cli/33-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair native command host shared attempt workspace creation/reuse. Qualify three ordered actions through competing hosts, exact source export, intermediate files, ordered step/log/artifact reports and history recovery using an isolated real embedded-RustFS server. Live dev workflow definition prepared; live execution and host admission approval remain distinct. Enclosing change wrpzywwx is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Obtain explicit temporary native-host admission and backend restart approval, execute the prepared dev-server workflow, inspect app steps/logs and restore admission. Keep Forgejo/provider/visual gates separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-WF-005/007 now explicitly specify one pinned private workspace per
  command attempt, shared across its ordered actions. Correct misplaced creation
  else branch: existing workspaces are reused, and unanchored sources create an
  empty workspace on first use. JST-WF-003/004/006 ownership, admission,
  CLI/wire syntax, durable reporting, cancellation and unknown-attempt
  non-replay remain unchanged. No storage or public API format change."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. Isolated native command-host real-storage
  evidence is distinct from a running-dev-server demonstration and from live
  Forgejo wrapper/provider/ConnectRPC or approved visual baselines. Code-pinned
  multi-step execution is qualified here; no additional live conversation or
  agent execution gate is promoted.

```
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; pinned embedded RustFS executable in runtime/embedded-rustfs/target/debug. Existing paired native workflow v1 CLI/server. rustc 1.98.1 (48a229cea 2026-09-01).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

WorkflowStore/manifest format 6, native identities, v1 workflow requests/reports/journal, WAL and receipts unchanged. Rust-only production slice; UI code/types/Storybook and root aggregate lanes out of scope. Managed Cargo controls affected CLI check and exact tests; documentation generation/check consumes native help generation. No installed service/package change or development restart/reset.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live dev execution awaits explicit temporary workflow-host admission and
  backend restart approval; definition is saved disabled.
- Forgejo Runner binary is absent; local Forgejo wrapper and unchanged remote
  registration/polling/ConnectRPC remain unqualified/deferred.
- Approved visual baselines and M28 agent/provider qualification remain separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-MULTISTEP-001 details 2](details-02.md)
- [E-WORKFLOW-MULTISTEP-001 details 3](details-03.md)
