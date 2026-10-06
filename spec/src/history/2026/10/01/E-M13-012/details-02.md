---
schema: 1
id: E-M13-012
---

# E-M13-012 details 2

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
jj --no-pager st (main and local workspace); jj new vqqlmmol qtxvrqqm -m 'Land local Git-backed collaboration and record pending M13 gates'
```

**Notes**

Clean parents integrated in main; four conflicts recorded before resolution. Preserve isolated workspace and both parent histories.
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
Structured merge of milestone/verification rows and JSON evidence by ID against psszmplx; explicit paging and mutation-prose conflict resolution
```

**Notes**

No divergent double edits or evidence collisions. All 191 parent evidence entries retained exactly; both native browse and local paging use pub(crate) take_page. Specification prose and status boundaries updated.
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
common_ancestors() revset diagnostics (two attempts)
```

**Notes**

Installed JJ has no such revset function. Read-only diagnostics corrected to heads(::vqqlmmol &amp; ::qtxvrqqm), selecting psszmplx/6350738b; no repository state loss.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
apply_patch execution-scope and governance update (initial attempt)
```

**Notes**

Governance context mismatch rejected the patch. Corrected exact context; no partial semantic change.
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
pnpm exec prettier --write AGENTS.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md
```

**Notes**

Resolved canonical docs and tracked guidance formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test work_store --test local_work --test local_review --test local_agent --test local_agent_pages
```

**Notes**

26 pass: 4 native browse/paging/authorization and 22 local work/agent/review receipt, cursor and domain-preservation regressions. No functional before/after failure; initial merge source has unresolved visibility syntax until resolution.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_work --test local_agent --test local_agent_provenance --test local_agent_provenance_cli --test local_agent_recovery --test agent_capture_spool --test review_commands
```

**Notes**

22 pass: local WorkStore/workspaces, cold sanitized hook/layouts, portable provenance and private unknown/retry recovery, shared spool and retained review command contracts. Cold hook takes 98.44 seconds while compiling the secret scanner; it passes without code or sanitizer changes.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test local_metadata --test local_evidence --test native_codec -p jj-store-wal --test review_projector --test agent_provenance -p jj-agent --test portable --test provenance -p jj-review --test portable --test anchors
```

**Notes**

42 pass across companion CAS/crash/GC, exact capsules, frozen native codecs, portable/native anchor models and shared native agent/review projectors. No test assertions or source semantics changed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-service --lib --test work_store --test local_work --test local_agent --test local_agent_pages --test local_review -p jj-store-cli --test local_work --test local_agent --test local_agent_provenance --test local_agent_provenance_cli --test local_agent_recovery --test agent_capture_spool --test review_commands -p jj-store-jj --test local_metadata --test local_evidence --test native_codec -p jj-store-wal --test review_projector --test agent_provenance -p jj-agent --test portable --test provenance -p jj-review --test portable --test anchors --locked -- -D warnings
```

**Notes**

Affected libraries and exact integration targets lint clean. Direct CLI/service consumers are checked; full Rust workspace remains out of scope.
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
cargo fmt -p jj-store-service -p jj-store-cli -p jj-store-jj -p jj-store-model -p jj-store-wal -p jj-agent -p jj-review -- --check
```

**Notes**

Affected Rust package formatting passes.
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
pnpm spec:validate
```

**Notes**

Initial and final document gates pass: 4 validators, 21 chapters and 131 requirements.
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
pnpm check:boundaries
```

**Notes**

15 owned package boundaries pass.
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
pnpm check:progress
```

**Notes**

Initial and final merge-aware gates pass for 103 tasks. Every direct parent is inspected; both parent evidence ledgers remain exactly preserved. This is bookkeeping rather than product qualification.
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
node --test scripts/governance.test.mjs
```

**Notes**

24 pass including preservation of every merge-parent evidence variant and rejection of reused parent evidence as new integration proof. Repeated once because the initial short tool yield did not retain its completion handle.
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
pnpm exec spec-validator first --base 6350738b6dafd45045c405cfcf9b2c1b73056e3b --head ca36ff2211003051d3949fe635de10724989c68e
```

**Notes**

Explicit common-ancestor gate passes for 255 protected paths and 15 canonical chapters; default single-parent @- mode is intentionally not used.
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
pnpm check:architecture
```

**Notes**

380 production modules pass the owned browser architecture check; preserved main UI source has no manual conflict changes.
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
Structured equality audit of all parent ledger JSON entries against the merged ledger
```

**Notes**

All 180 main and 188 local parent entries preserved exactly. M13.1/M13.2 remain Done; M13/M13.3 In progress; M13.4-M13.6 Planned.
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
ps -p 87767 -o pid,ppid,etime,state,time,pcpu,wchan,command; sample 87767 1 -file /tmp/jj-store-integration-setup.sample
```

**Notes**

Read-only cold-hook diagnostic proves active kingfisher/vectorscan rule compilation rather than a deadlock. No installed process or repository modified.
<!-- /run -->
<!-- /evidence-field -->

