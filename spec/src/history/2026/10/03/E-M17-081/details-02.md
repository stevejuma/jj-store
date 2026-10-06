---
schema: 1
id: E-M17-081
---

# E-M17-081 details 2

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
jj --no-pager st; jj workspace list
```

**Notes**

Reviewed both workspaces. Main active desktop/glass changes are unrelated and preserved; capture OpenCode draft is incomplete.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager log -r 'heads(common_ancestors(default@, @))' --no-graph
```

**Notes**

Unsupported revset function; corrected to heads(ancestors(default@-) &amp; ancestors(agent-capture@-)). Read-only query; no state changed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --from 'heads(common_ancestors(default@, @))' --to default@- --name-only
```

**Notes**

Same unsupported function in the independent diff query, corrected before recording ancestry.
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
jj --no-pager log -r 'heads(ancestors(default@-) & ancestors(agent-capture@-))' --no-graph
```

**Notes**

Common ancestor lwqssrow / 2852a3e3. Pinned main parent wllxluns / 0400ae22; capture ypoxtqtz / d2931163.
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
Python subprocess inspection of jj log ancestors differences and jj diff --name-only for both branches
```

**Notes**

Main: 51 committed changes/599 changed paths. Capture: 80 changes/446 paths. 40 overlapping paths. Main M17 desktop conflicts with capture M17 identifiers; mapping is an explicit rebase deliverable.
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
codex --version
```

**Notes**

Installed /opt/homebrew/bin/codex reports codex-cli 0.156.1, matching the implemented CLI profile. No model/session launched; Desktop not qualified by this query.
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
jj bookmark create codex/deferred-opencode-history -r ovlvtqrl
```

**Notes**

Preserved the unfinished 10-file OpenCode import draft, including two previously failing selector regressions. No draft implementation is claimed accepted.
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
jj new ypoxtqtz -m "Prioritize a usable local capture preview and defer extended roadmap"
```

**Notes**

Planning change based on last verified capture tip; parked draft remains separately reachable and main workspace is untouched.
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
pnpm exec prettier --write docs/agent-capture-preview-plan.md docs/implementation-plan.md spec/src/collaboration.md spec/src/ui.md spec/src/cli.md spec/src/architecture.md spec/src/spec-governance.md spec/src/verification.md
```

**Notes**

Formatted changed plan and canonical chapters.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node /tmp/jj-store-capture-ledger.mjs /tmp/jj-store-preview-plan-entry.json
```

**Notes**

Temporary helper from the previous slice had been removed; recreated locally before appending any evidence.
<!-- /run -->

<!-- run -->
### Run 11

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

New document name lacked an existing protected-path mapping. Renamed to docs/implementation-plan-capture-preview.md under the existing implementation-plan mapping; no checker weakened.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Missing E-M17-081 because the append helper was absent. Eight task-reference failures plus the missing new evidence check; corrected by appending the entry.
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
pnpm exec prettier --check docs/agent-capture-preview-plan.md docs/implementation-plan.md spec/src/collaboration.md spec/src/ui.md spec/src/cli.md spec/src/architecture.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Ledger table needed formatting before evidence append.
<!-- /run -->

<!-- run -->
### Run 14

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

Second attempt still unmapped because existing rule matches exact implementation-plan.md. Moved the new plan to the top of that governed document; no executable/configuration changes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Concurrent pnpm temporary-file removal raced a JJ snapshot. Re-run sequentially after formatting; no product test or ledger corruption.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/collaboration.md
```

**Notes**

Final plan consolidated into the existing governed implementation plan.
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
pnpm spec:first
```

**Notes**

One protected plan and six canonical chapters pass.
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
pnpm check:progress
```

**Notes**

130 tasks pass; scope and evidence bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

