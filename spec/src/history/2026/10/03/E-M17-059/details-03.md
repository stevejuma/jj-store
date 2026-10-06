---
schema: 1
id: E-M17-059
---

# E-M17-059 details 3

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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-policy-check-progress.log 2>&1
```

**Notes**

Repeated after metadata sealing.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-policy-format-rust-check.log 2>&1
```

**Notes**

Repeated after metadata sealing.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-policy-doc-format.log 2>&1
```

**Notes**

Repeated after metadata sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-policy-doc-check.log 2>&1
```

**Notes**

Repeated after metadata sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI JSONL 0.156.1, explicit running supervisor on native MemoryStore host and real private device files; no real storage/model qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/CLI/native/source/spool/control/import catalog format change. Import journal remains v1, installation/control bound, 4096 requests within 2 MiB, configured private path state_dir/agent-capture/imports. New non-cloneable library CaptureImportPolicy holds manager gate and original resolved intent; import-only bound control read pins selected file stamp independently of existing control read semantics.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oxowsltv
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

Current AgentStore only. Implements current installation-policy guard for later owned source admission. Actor/command and disabled checks precede capture-state I/O; exact configured journal path/installation/control generation precede receipt resolution. Exact selected receipt bypasses only new revision/temporary-selector checks; both modes require unpaused current control generation and live Running command/Ready-or-Running observation, and new work still requires a retained nonterminal source selector. A control-file replacement with identical bytes fails the import-only retained stamp check. Manager gate and control/import filesystem gates remain held through nested blocking settlement after caller cancellation. Post-application loss is unknown, accepted identity remains. Independent native Write, exact source/prefix, frame/storage ownership and actual enrollment/scheduling remain next; public API/CLI/browser/import/startup gates and full M17 roadmap remain pending. Concrete verification escalation: new retained management coordination can affect existing controls/status/rescan/source operations, and import eligibility reads current runtime command/observations plus control/import journals. Run complete directly affected server library (including cold factory/lifetime proofs), management and runtime integrations and both private journal suites; server plus direct CLI consumer all-target check/strict lint. Rust suites serial with original deadlines and within-scenario concurrency. Existing source worker/supervisor scheduling algorithms, public HTTP DTO/route/runner, CLI transport and UI are unchanged: their independent full suites/contracts/root aggregates, real services and model runs remain out of scope. Preserve all 313 prior parsed entries. M17.2 In progress, M17.3-M17.7 Planned, full goal active. No install/publication/migration/restart or subagents. One initial multi-document patch missed formatter-wrapped collaboration prose and applied no mutations; repaired using current text. The package-filter before command matched no tests and is not misreported as a runtime regression. Source module line limits remain below 300. Final acceptance: seven exact policy regressions plus isolated requested-pause regression pass. Full directly affected suites pass 57 library, ten management, ten runtime, nine control journal and eight import journal tests (94 total), retaining original deadlines and within-scenario concurrency. Server/CLI all-target check and strict lint pass without suppression. Canonical, spec-first, boundary, architecture, progress and Rust/doc formatting gates pass. All 313 prior parsed entries unchanged, total 314. Policy guard complete; native Write/source enrollment/supervised scheduling remain pending. Inspection confirms ordinary SourceRegistry.acquire registers a physical source without a logical-session admission marker; next implementation must add a separately gated explicit durable-consent path, preserving active/admitted aliases and leases.
<!-- /evidence-field -->

