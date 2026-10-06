---
schema: 1
id: E-M17-072
---

# E-M17-072 details 3

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
pnpm check:boundaries
```

**Notes**

17 package boundaries; no new dependency edge.
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
pnpm check:architecture
```

**Notes**

732 production web modules; new Rust production modules remain below 300 lines.
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
pnpm check:progress
```

**Notes**

129 tasks; evidence bookkeeping only; milestone remains In progress.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final changed-document formatting passes.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; pinned OpenCode API 1.18.30. Pinned Kingfisher cache reused after initial cold rule compilation.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing private source catalog, sanitized pending spool, snapshot cursor, native AgentStore and receipt formats unchanged. Endpoint/session/directory/version keys are private digests. Per-record opencode-snapshot/v1 ingress retains API versus export trust. Snapshot source progress is not the JSONL byte publication-management inventory.
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

Explicit source driver and direct Rust consumers only. Final selected set: 30 top-level tests plus the sanitizer child, native MemoryStore publication and recorded reads, private real filesystem state and controlled loopback API. Exclusive source identity, per-record current native repository/socket/catalog/Write admission, sanitization before durability, exact acknowledgement, bounded raw prefix reconstruction, restart/rewrite/offline tail and exact native publication. API binding and source/spool ownership are checked on native prepare/publish. Code heads do not advance. Unbound omissions publish source gaps without fictional sessions. Current AgentStore only. OpenCode startup stays withheld: foreground supervision/control/import management, lifecycle projection, directory-aware discovery, hook/API aliases, streaming export, accepted publication management, full cold startup and real harness/RustFS acceptance remain separate gates. Earlier superseded-witness recovery and independent partition scheduling remain integration concerns. Cursor and M17.3-M17.7 remain pending. All 326 historical evidence objects/raw prefix preserved; M17/M17.2 remain In progress. Root aggregates, UI lanes, public wire regeneration and unrelated suites are out of scope.
<!-- /evidence-field -->

