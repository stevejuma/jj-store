---
schema: 1
id: E-SCOPE-001
---

# E-SCOPE-001

<!-- evidence-field:id markdown -->
## Id

E-SCOPE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.2
- M4.3
- M5
- M5.1
- M6.2
- M8a
- M8a.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

user-requested-scope-deferral
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/api.md
- spec/src/mutations.md
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

Clean starting state after verified M5 preparation commit 464f05b5; no UI implementation changed.
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
pnpm spec:validate
```

**Notes**

58 requirements and one verification row per requirement pass.
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
pnpm spec:first
```

**Notes**

Scope update maps to architecture and governance chapters.
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
pnpm check:progress
```

**Notes**

32 tasks with deferred UI separated from backend dependencies pass.
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
pnpm spec:build
```

**Notes**

Canonical mdBook builds.
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
pnpm format:check
```

**Notes**

Pinned rustfmt and Prettier checks pass; no UI sources edited.
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

Rust 1.97.1; jj-cli/jj-lib 0.45.1; released spec-validator ^0.1.0 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No implementation, lockfile or storage/API format changes. User requested deferring further UI changes while continuing all other tasks. M4.3 preserves browser mutation/conflict/search and deployment acceptance; M4.2 retains SSE/API conformance; M5 depends on completed M4.1. Full UI release requirements remain uncompleted. Existing browser sources/tests are unchanged. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


