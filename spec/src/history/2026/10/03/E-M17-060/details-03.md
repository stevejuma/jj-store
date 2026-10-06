---
schema: 1
id: E-M17-060
---

# E-M17-060 details 3

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
jj --version
```

**Notes**

JJ 0.45.1.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_admission --test capture_activity --test capture_authority --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-enrollment-server.log 2>&1
```

**Notes**

Complete selected direct server impact surface passes 133 top-level tests across 13 binaries. Rust binaries serial with unchanged original deadlines and within-scenario concurrency; native A/B admission, revocation, lease/frame preservation, progress, exact receipts, storage/cold/lifecycle and supervisor scheduling proofs pass.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-import-enrollment-check.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-import-enrollment-clippy.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-enrollment-final-format.log 2>&1
```
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-enrollment-progress-final.log 2>&1
```

**Notes**

Repeated after final evidence sealing; all 314 prior parsed entries remain unchanged.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-enrollment-canonical.log 2>&1
```

**Notes**

Final verification after code and test acceptance.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-enrollment-spec-first-final.log 2>&1
```

**Notes**

Final verification after code and test acceptance.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-enrollment-doc-check-final.log 2>&1
```

**Notes**

Final verification after code and test acceptance.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing JJ change oplstxvp
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

Current AgentStore only. This registry primitive requires a completed exact source preparation and caller-retained durable intent/current policy/native Write gates; it does not accept digest metadata or a replay boolean as authority. Rechecks before allocation, before synchronized registry selection and after selection retain current binding. Preselection failure leaves catalog unchanged; postselection failure retains marker for exact intent reconciliation. Import marker is exact file/header/session/profile-bound in existing V1 digest budget; replacement cannot inherit import-only consent, while current active-session semantics and common aliases/leases remain unchanged. Concrete verification escalation before wider runs: acquisition and admitted lookup are shared by active/offline resume, parked raw-channel reacquisition, native ingestion pipelines, supervisor/runtime foreground preparation and CLI automatic publication. Run complete capture crate plus directly affected server library/admission/activity/source/resume/parking/reanchor/recovery/pipeline/pipeline-worker/supervisor/runtime and direct CLI publication consumers; package all-target checks and strict lint. Rust commands/binaries serial, existing scenario deadlines/concurrency preserved. HTTP routes/DTOs, CLI command parsing, UI/contracts, unrelated Rust workspace and root aggregate lanes remain out of scope. Preserve all 314 preceding parsed entries. M17.2 In progress, public startup guarded, full goal active. No installs/publication/migration/restarts/subagents. Review before commit found an unsafe prospective promotion from import marker to ordinary active resume, allowing accepted-prefix revalidation to be skipped before the first cursor. Tightened the final design: import-only markers are inaccessible to generic active lookup/resume; explicit imported lookup, acquisition and reserved physical reacquisition require renewed exact accepted-prefix preparation plus caller-retained receipt/policy/Write checks. No generic replay boolean or digest-only authority. Fresh-process and same-header body-rewrite regressions pass. Table metadata now puts E-M17-055 through E-M17-060 in evidence columns rather than repeating them in next-action prose; no previous evidence entries changed. Final complete capture package passes 122 top-level tests across 15 binaries, with three known fixture entries ignored (fresh-process parent proofs exercised normally). Canonical, spec-first, boundaries, architecture, repaired progress, Rust formatting and changed-document formatting pass. The original macOS unwind-table linker warning in server library linking remains unsuppressed; it is not a test failure. Impact-surface refinement before CLI verification: scoped rg of SourceRegistry/acquire_admitted/acquire_active/acquire_reserved found no CLI implementation or test calls; the CLI consumes the capture crate reexport boundary but not this registry behavior. Do not run the initially considered CLI publication runtime suite. Capture/server direct functional suites plus capture/server/CLI all-target check and strict lint qualify the actual boundary; unrelated CLI command/publication suites remain out of scope. Final directly affected server suites pass 133 top-level tests across 13 binaries. Combined with capture package, 255 top-level tests pass; 12-test exact import target also passes independently. No unresolved functional failures. Final all-target compilation and strict lint pass for capture/server/CLI. Restore the parent raw JSON entry bytes before formatting to avoid unrelated historical formatting churn caused by decoding and serializing the entire ledger. All 314 preceding parsed evidence entries remain unchanged; total 315. Primitive slice complete; next work must propagate explicit import eligibility through native source/pipeline parking and resume, retain journal/policy/native Write plus frame/storage ownership, and connect owned scheduling before typed API/CLI/browser import. M17.2 remains In progress; M17.3-M17.7 Planned; full goal active.
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- Existing registry V1 and admission digest vector unchanged; import-only exact
  physical/header/session/profile markers use a new domain. Current AgentStore
  only. No reader progress, spool, request or native formats changed.
<!-- /evidence-field -->

