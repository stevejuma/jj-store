---
schema: 1
id: E-M17-074
---

# E-M17-074 details 3

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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Changed-document formatting passes; parent historical raw prefix preserved.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; pinned OpenCode 1.18.30; reused pinned sanitizer cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No durable repository, spool, source catalog or public wire format change. Internal status adds a third known optional harness to existing dynamic serialization. Body-free exact entry digests and Arc-retained existing leases remain device memory only. Current AgentStore only.
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

Retained library supervision only. Final selected set: 173 top-level server tests (105 owning library, 59 direct existing consumers, nine API source/supervisor), with existing cold/helper child processes counted separately. Controlled loopback API, private real files and native MemoryStore are distinct from a real OpenCode/RustFS gate. Same reader/source/raw-frame budgets across Codex/API; alternating harness priority and monotonic selector catalog; independent discovery/reading/publication; exact body-free proof digests retain original leases; idle API scanner/cursor/sanitizer release; admitted ended-session restart, pause/resume/rescan/drain/storage ownership. API delivery and Codex delivery use separate proof paths; earliest-per-partition fairness keeps unavailable intent without starving other repositories. Escalation: two independent harness consumers share scheduler, public Rust status array and control/lifetime observations, so the complete owning server library plus capture supervisor/runtime/management/source/read-API test binaries are required. No root aggregate, unrelated package, UI or public contract generation is required. Configured process startup still withholds OpenCode pending lifecycle, directory-aware discovery, import/export/publication management, aliases and real harness/RustFS qualification. Earlier superseded-witness checkpoints, complete cold OpenCode startup and Cursor remain pending. M17/M17.2 In progress; M17.3-M17.7 Planned. All 328 historical evidence objects/raw prefix preserved.
<!-- /evidence-field -->

