---
schema: 1
id: E-CHAT-MAIN-001
---

# E-CHAT-MAIN-001 details 2

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 and M13.3 remain In progress; M19 remains Partial. Existing ResizeObserver
  diagnostics reproduce in E-UI-MENU-002 responsive stories. Visual baselines
  remain unapproved.
- This is conflict-free source integration and a cold main development-server
  handoff, not full native/local publication, restart-recovery, multi-agent,
  retained-source-after-GC or packaged platform qualification. Earlier slice
  evidence and outstanding gaps remain authoritative.
- No full Rust test suite, root aggregate, full catalog, installed harness turn
  or production built-bundle test was rerun. Main backend was rebuilt for the
  requested server switch; CSS verification is recorded in E-UI-MENU-002.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Main source tip before this evidence is 7ea50d09; 42 changes since e26a177a. JJ 0.45.1, pnpm 10.32.1, rustc 1.98.1, Node 24.10.0, macOS ARM64. Revision is the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No additional storage, API, request or preference format change in this integration slice. Generated documentation reflects the already landed versioned contracts; runtime marker remains ignored format 2.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Land the existing chat-body stack, refresh stale generated manual/review coverage, and move only the owned development server to main using the same LocalTrusted fixture and saved configuration.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue separately governed M24/M13.3 and platform qualification from main; investigate existing resize diagnostics without claiming this handoff resolves them.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-DOC-001/002/003 manual coverage now explicitly includes workspace
  inspection, qualified retained file comments and generated reply-reference
  contracts. The owning execution-manual chapter records this expanded
  documentation scope. Product contracts JST-UI-001/018, JST-API-001 and
  JST-CLI-004 remain unchanged by integration; existing qualification limits
  stay open.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress and M19 Partial; integration and cold read-only UI
  verification do not close independent M13.3, native, multi-agent or release
  gates.

```
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
pnpm check:progress
```

**Notes**

146 tasks; 712 evidence records.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

4 document validators pass: 3395 chapters, 181 requirements.
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
pnpm exec prettier --check docs/maintenance/coverage.json docs/site/guides/agent-execution.md docs/site/guides/reviews.md docs/site/technical/publication.md docs/site/technical/local.md docs/site/technical/architecture.md docs/site/technical/lifecycle.md docs/site/reference/transport.md docs/site/guides/browsing.md docs/site/reference/capabilities.md spec/src/current/contracts/documentation/08-interactive-agent-execution.md spec/src/current/progress/M24/footer-review.md
```

**Notes**

Authored documentation and coverage formatting pass; generated API text stays generator-owned.
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
pnpm spec:first
```

**Notes**

The unchanged-contract classification requires completed passing evidence; this record remains pending for separate qualification gates. Corrected classification to changed documentation coverage under JST-DOC-001/002/003, as recorded in the updated owning chapter. Integration still changes no product authority contract.
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
pnpm spec:first
```

**Notes**

Final gate passes: 13 protected documentation files, 1 owning chapter.
<!-- /run -->
<!-- /evidence-field -->
