---
schema: 1
id: E-GOV-002
---

# E-GOV-002

<!-- evidence-field:id markdown -->
## Id

E-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

integrated-workspace-retirement-review
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wllxluns
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Eleven registered jj-store workspaces. After loading the current shared operations, main is clean at006cf352 with integrated parent11b12681. Nine secondary checkouts are empty and conflict-free; agent-capture snapshots an OpenCode API history import draft across10files and81nonempty changes outside main ancestry. The user authorizes integrated-workspace Trash retirement and requests the pending-work list.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ0.45.1; /usr/bin/trash recoverable macOS removal. Existing main Vite/native service and desktop runtime reused without a cleanup-triggered restart.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only workspace registrations and recoverable checkout locations change. No product, schema, object, WAL, receipt, installation, fixture or repository format changes. All integrated implementation parents remain ancestors of main. agent-capture and its pending history/draft remain intact.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
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
jj --no-pager st; jj --no-pager workspace list; python3 - (workspace inventory/status snapshots and before-ledger capture); jj --no-pager --ignore-working-copy diff --summary -r ovlvtqrl; jj version; trash --help
```

**Notes**

Inspected all11registered checkouts and sibling .jj/repo pointers. Every status succeeds without a stale refresh. Main and nine retirement candidates are clean; capture contains8Rust source/test changes and2canonical chapter changes.81nonempty changes outside main comprise80ancestors plus the current draft. Before/after proof files are under ignored .scratch/workspace-review.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 31
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager --ignore-working-copy log with workspaces() and quoted full workspace@ symbols (initial exploratory ancestry queries)
```

**Notes**

One unsupported workspaces() function query and30quoted-full-symbol queries failed to resolve. These are command-syntax mistakes, not product failures. Corrected ancestry checks use pinned commit IDs and supported default@/agent-capture@ workspace syntax.
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
python3 - (pinned commit ancestry audit, main symlink/text-reference scan and exact retirement preflight); lsof -nP -Fpcftn; ps -axo pid=,ppid=,command=
```

**Notes**

For each of9candidates, the implementation parent is in main ancestry and the only unique change is its empty conflict-free working leaf. Fresh statuses confirm no dirty/untracked tracked changes. No process uses a candidate checkout; no main symlink or current source reference depends on it. Main native desktop and development processes belong to main. Trash destination names are collision-free.
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
jj --no-pager workspace forget dev-annotations local-browser local-completion local-management local-settings native-project-tasks projects-hub repo-tabs-edit threads-reactions
```

**Notes**

Removed only the9verified integrated registrations. Default and agent-capture retained; no change abandonment, history rewriting, merge, or pending-work integration.
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
trash -v /Users/stevejuma/code/lapismd/jj-store-dev-annotations /Users/stevejuma/code/lapismd/jj-store-local-browser /Users/stevejuma/code/lapismd/jj-store-local-completion /Users/stevejuma/code/lapismd/jj-store-local-management /Users/stevejuma/code/lapismd/jj-store-local-settings /Users/stevejuma/code/lapismd/jj-store-native-project-tasks /Users/stevejuma/code/lapismd/jj-store-projects-hub /Users/stevejuma/code/lapismd/jj-store-repo-tabs-edit /Users/stevejuma/code/lapismd/jj-store-threads-reactions
```

**Notes**

All9exact directories moved recoverably to matching names under ~/.Trash, including their ignored artifacts. No permanent deletion or Trash emptying.
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
python3 - (post-retirement path/registration/runtime verification); jj --no-pager st; curl --fail --silent --output /dev/null --write-out UI-HTTP-status http://127.0.0.1:7460/; lsof -nP -iTCP:7460 -iTCP:7462 -sTCP:LISTEN
```

**Notes**

Only default and agent-capture remain registered. All9source paths are absent and Trash folders present. runtime.json SHA256 matches preflight; its owned PIDs remain alive. Main UI returnsHTTP200, Vite39014 and backend39013 listen on7460/7462. Main tracked tree is clean before the ledger-only entry.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate; python3 - (before/after evidence comparison)
```

**Notes**

Formatting,125-task progress bookkeeping, spec-first (no protected production change), and5validators/22chapters/152requirements pass. All306previous parsed evidence entries remain identical. Only E-GOV-002 is appended; task statuses unchanged.
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

Workspace review and authorized recoverable retirement only. Pending capture work is retained rather than promoted to integration acceptance. Canonical task/milestone statuses and all previous evidence stay unchanged. Product suites, Rust builds, browser/visual tests, reset/reseed, other repositories and unrelated service operations are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

agent-capture is the sole remaining secondary app workspace: finish and verify explicit OpenCode API history import work, then separately qualify its branch integration. Main development fixture and native desktop installation remain available.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


