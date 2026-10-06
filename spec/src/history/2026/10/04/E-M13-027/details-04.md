---
schema: 1
id: E-M13-027
---

# E-M13-027 details 4

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
cargo clippy -p jj-store-jj --test conversion --test local_evidence -p jj-store-service --test conversion --locked -- -D warnings
```

**Notes**

conversion-primitive-test-lint: code/evidence and service graph test targets pass strict lint.
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
cargo test -p jj-store-cli --test native_lifecycle --locked content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

storage-management-regression: 1/1 actual HTTP workspace/storage management fixture passes after mechanical owning conditional consolidation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_management --locked api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free -- --exact (isolated original and diagnostic)
```

**Notes**

contracts-snapshot-regression/regression2: 1 failure per run; original 120s publication deadline expires at Overdue/awaiting_activity with no snapshot. Disposable process stack sample identifies real Kingfisher/vectorscan rule compilation before observation. Diagnostic retains original deadline; ineffective runtime topology experiment is reverted.
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
sample 84180 2 -file .local/repository-conversion/capture-contract-sample.txt
```

**Notes**

capture-contract-sample: sampled only this disposable fixture process. Blocking source worker stack reaches TranscriptSanitizer::new, rules compilation and hs_compile_ext_multi; runtime shutdown waits for that owned compiler task.
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
cargo test -p jj-store-server --test capture_management --locked api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free -- --exact (real scanner setup repair)
```

**Notes**

contracts-snapshot-regression3: 1/1 passes in 189s including actual cold Kingfisher preparation before worker startup. Original 120s publication timeout, single-thread runtime and all receipt/body-free/corruption/authorization assertions retained. No mocks, dropped fixtures or sanitizer policy changes.
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
cargo clippy -p jj-store-server --test capture_management --locked -- -D warnings
```

**Notes**

conversion-capture-test-lint: owning repaired fixture target passes strict lint.
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
sample 94188 1 -file .local/repository-conversion/capture-prewarmed-sample.txt
```

**Notes**

capture-prewarmed-sample: owned disposable fixture is performing real rule compilation in setup, before the publication wait.
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
cargo fmt --all; pnpm exec prettier --write spec/src/cli.md .local/repository-conversion/evidence.json
```

**Notes**

conversion-format9/evidence-format2: fixture/canonical formatting and separately formatted new evidence object; inherited ledger remains untouched.
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
pnpm exec prettier --check changed TS/Svelte/MJS/JSON/canonical chapter files .local/repository-conversion/evidence.json; cargo fmt --all -- --check
```

**Notes**

conversion-prettier-final/rust-format-final: changed source and chapters pass. Historical progress ledger is not rewritten by formatting.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries
```

**Notes**

conversion-progress2/spec-first2/spec-validate2/boundaries2: 136 tasks, 42 protected files/12 mapped chapters, 22 chapters/167 requirements and 17 package boundaries pass. Bookkeeping is not product acceptance.
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
python3 (compare inherited progress evidence and raw ledger bytes with jj file show -r @-)
```

**Notes**

All 415 inherited evidence entries and their original ledger bytes are preserved; only M13.5 task scope and E-M13-027 change.
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
pnpm contracts:check (complete registered capture lane, iteration two)
```

**Notes**

conversion-contracts-check2: all registered owning server and CLI captures pass; 240 actual HTTP/SSE responses conform. Seven contract validation tests and direct generated-client TypeScript check pass. Final scanner-setup fixture separately passes the exact original target after the diagnostic topology experiment is reverted.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write .local/repository-conversion/evidence.json; pnpm exec prettier --check .local/repository-conversion/evidence.json; pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries (final)
```

**Notes**

evidence-format-check-final/spec-first-final/spec-validate-final/boundaries-final pass; conversion-progress-final rejects a provisional pending run result (runs require pass/fail). Correct evidence metadata without changing implementation or gates.
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
pnpm exec prettier --write .local/repository-conversion/evidence.json; pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries (corrected final metadata)
```

**Notes**

evidence-format-final2/conversion-progress-final2/spec-first-final2/spec-validate-final2/boundaries-final2: corrected metadata passes all 136-task bookkeeping, 42-file/12-chapter governance, 22-chapter/167-requirement validation and 17-package boundaries. Final evidence append rechecked with the same progress and specification consumers.
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

<!-- evidence-field:scope markdown -->
## Scope

Independent eligible conversions in the current LocalTrusted installation only. Full actual registered HTTP/SSE capture escalation is required by added public routes/capabilities and generated browser client. All-target checks cover owning libraries and direct CLI consumer; tests remain named affected targets. Cold production browser and isolated real RustFS restart are distinct from MemoryStore fault tests. M13.5 remains In progress because portable code/agent anchored profiles and the prerequisite M13.4 code-command gates remain separate; unsupported profiles fail preflight without changing source. Root aggregate/full Rust workspace/full UI suites, visual baselines, installed services, existing repository migration, remote destinations and release/deployment are out of scope. API snapshot fixture setup warms the real privacy scanner; it preserves the 120-second publication wait, original single-thread runtime and receipt/body-free assertions. This does not qualify cold scanner compilation latency.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain verified bounded conversion; qualify unsupported signed/conflicted/copy code, portable code/agent anchored collaboration and remaining M13 prerequisite/full-flow gates separately.
<!-- /evidence-field -->

