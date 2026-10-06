---
schema: 1
id: E-M17-056
---

# E-M17-056

<!-- evidence-field:id markdown -->
## Id

E-M17-056
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

durable-inactive-history-import-specification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/collaboration.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/mutations.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
jj --no-pager st
```

**Notes**

After verified E-M17-055 commit a61d924b, fresh uoqmplzv is empty; no overlapping work.
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
jj describe -m "Specify durable inactive-history import and restart semantics"
```

**Notes**

Narrow next change for the active full roadmap, current AgentStore only.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/api.md spec/src/cli.md spec/src/mutations.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-plan-format.log 2>&1
```

**Notes**

Documentation-only owning check passes; final sealing repeats these exact commands.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-plan-validate.log 2>&1
```

**Notes**

Documentation-only owning check passes; final sealing repeats these exact commands.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-plan-first.log 2>&1
```

**Notes**

Documentation-only owning check passes; final sealing repeats these exact commands.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-plan-progress.log 2>&1
```

**Notes**

Documentation-only owning check passes; final sealing repeats these exact commands.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/api.md spec/src/cli.md spec/src/mutations.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-plan-docs.log 2>&1
```

**Notes**

Documentation-only owning check passes; final sealing repeats these exact commands.
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

<!-- evidence-field:versions markdown -->
## Versions

Documentation-only; current Rust 1.98.1/JJ 0.45.1/macOS ARM64 product qualification remains E-M17-055.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Executable 89-path HTTP inventory, CLI commands, source/spool/control/native formats and current AgentStore V1 formats unchanged. Planned separate versioned private import intent: at most 4096 retained requests within 2 MiB; not implemented.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uoqmplzv
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Documentation-only continuation defining JST-COLLAB-018/JST-COLLAB-030 explicit inactive-history acceptance, current native directory/Write admission, private durable intent and exact retry after ephemeral selector invalidation, retained owner/storage/frame bounds, capacity and acceptance-versus-publication progress. Adds the planned strict actor-free import command/API shape and necessary regression families without exposing an executable route, command or permissive acquisition. Orders journal/replay, owned source scheduling, API/CLI then browser/cold work. Before state: source pages are verified but import is only broadly required; exact durable selection/replay and progress semantics were unspecified. One initial multi-file patch missed formatter-wrapped CLI text and applied no changes; current text was read and the exact patch repaired. Required verification is changed-document formatting plus spec validation, spec-first and progress checkers that consume these requirements. Rust, contract generation, web/UI, storage, root aggregates and functional tests are out of scope because no production/public type/runner changed. Preserve all 310 prior parsed entries; M17.2 remains In progress and M17.3-M17.7 Planned. Full goal active; no installation, migration, restart, publication or subagents. The accepted backfill pins a complete-record boundary/prefix witness, retains partial EOF without acknowledgement and reports subsequent tail separately; prefix rewrite cannot retarget intent. New headings distinguish planned import from implemented runtime/storage ownership. All five doc-consuming lanes pass; final sealing repeats them after evidence. Prior 310 entries remain unchanged, total 311.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


