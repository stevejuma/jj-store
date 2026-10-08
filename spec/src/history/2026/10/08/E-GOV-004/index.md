---
schema: 1
id: E-GOV-004
---

# E-GOV-004

<!-- evidence-field:id markdown -->
## Id

E-GOV-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-005
- JST-GOV-007
- JST-GOV-009
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Verified integrated-workspace retirement and repeat inventory
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vwrskpzn; clean starting parent nxtqmmuy / 1923eafb
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The user authorizes removal of the four previously audited integrated workspaces and any further verified integrated checkouts. Five registrations remain: main/default and agent-capture, bounded-reads, codex-preview and repository-conversion. All four secondary working copies are empty and conflict-free. Independent intervening work has already integrated and retired local-compute (45497b21) and agent-fleet (1923eafb); both folders are absent before this slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/Bootstrap/index.md
- spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
- spec/src/history/2026/10/08/E-GOV-004/index.md
- spec/src/history/2026/10/08/E-GOV-004/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
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
jj --no-pager st; jj --ignore-working-copy --no-pager workspace list; jj --ignore-working-copy --no-pager log -n 8
cat spec/src/current/governance/{spec-governance,01-specification-governance,02-requirements,17-capture-preview-integration}.md
cat spec/src/current/progress/{index.md,Bootstrap/index.md}
cat spec/src/history/2026/10/03/E-GOV-002/index.md spec/src/history/2026/10/08/E-FLEET-008/index.md
cat package.json scripts/spec/evidence-new.mjs scripts/spec/progress-history.mjs
rg (targeted script, requirement and current retirement-evidence discovery); ls -ld /Users/stevejuma/code/lapismd/jj-store* /Users/stevejuma/.Trash
jj workspace forget --help; /usr/bin/trash --help; jj version
```

**Notes**

Read current governing retirement and evidence requirements. Main is clean at 639b50ff above 1923eafb. Original audit findings were rechecked against current shared history before any removal.
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
pnpm spec:search -- JST-GOV-003
```

**Notes**

Installed command rejects the documented separator with unknown argument: --. No file or product change. The following separator-free retry succeeds.
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
pnpm spec:search JST-GOV-003
rg --files spec/src/history (E-GOV ID inventory)
sed -n 1,230p scripts/spec/evidence-format.mjs; sed -n 1,90p scripts/spec/progress-loader.mjs
```

**Notes**

Search returns canonical governance/02-requirements.md, already read. E-GOV-004 is unused. Read bounded evidence serialization and ledger loading for the required record.
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
python3 .local/workspace-retirement/audit-2026-10-08/audit.py before
Python preflight assertions, prior evidence SHA-256 snapshot and lsof main-listener snapshot
```

**Notes**

The script runs fresh jj st in every candidate, pinned ancestry/empty-change queries, capture bookmark checks, lsof open-file and ps command references, main symlink checks, and a no-follow linked-checkout inventory under ~/code and ~/.codex/worktrees. No candidate is dirty, referenced by a live process or main symlink, and there are no scan errors. Repository, build, dependency, fixture and cache interiors are excluded from source-checkout discovery. Bounded-reads parent 076e0b23, codex-preview parent f554f5b1 and repository-conversion parent c6eeec7a are main ancestors, with zero unique nonempty commits. Capture has the original 81 nonempty commits, consolidated into main ancestor d974b560 per the owning integration chapter. Its 3ad7cf03 history and c4d8d275 deferred OpenCode draft are bookmarked. Every candidate has one empty working leaf. Trash destination names are unused. Proof files remain ignored under .local/workspace-retirement/audit-2026-10-08.
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
jj --no-pager workspace forget agent-capture bounded-reads codex-preview repository-conversion
/usr/bin/trash -v /Users/stevejuma/code/lapismd/jj-store-agent-capture /Users/stevejuma/code/lapismd/jj-store-bounded-reads /Users/stevejuma/code/lapismd/jj-store-codex-preview /Users/stevejuma/code/lapismd/jj-store-repository-conversion
```

**Notes**

All four registrations are forgotten and exact checkout folders move to matching names under ~/.Trash, retaining their ignored artifacts. No history is abandoned and no bookmarks, repositories or installed binaries are removed. Trash is not emptied.
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
python3 .local/workspace-retirement/audit-2026-10-08/audit.py after
Python post-retirement path, Trash, registration, scan and listener assertions
curl --max-time 10 --fail --silent --output /dev/null --write-out HTTP-status http://127.0.0.1:7480/
```

**Notes**

Repeat inventory finds only /Users/stevejuma/code/lapismd/jj-store linked to the shared repository and only default registered. All four source paths are absent and Trash folders present. Capture bookmarks are identical. No further source-checkout candidates are found within the searched roots. Main is clean before the evidence-only edit. Main Vite PID80122 on 7480 and backend PID19903 on 7482 are unchanged; document request returns HTTP200. No runtime is stopped, restarted, reset or reseeded.
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
pnpm evidence:new -- --id E-GOV-004 --task Bootstrap --requirement JST-GOV-003 --requirement JST-GOV-005 --requirement JST-GOV-007 --requirement JST-GOV-009 --requirement JST-GOV-011
apply_patch (Bootstrap maintenance review)
node .local/workspace-retirement/audit-2026-10-08/write-evidence.mjs
```

**Notes**

Create the required bounded Markdown evidence and explicit unchanged milestone review. Local audit script, proof JSON and process snapshots remain ignored.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/workspace-retirement/audit-2026-10-08/write-evidence.mjs
```

**Notes**

First invocation used an incorrect relative import in the ignored helper and failed before writing evidence. Corrected the helper import from four parent segments to three; the successful retry writes the intended record. The intervening progress:history invocation used the scaffold and is rerun after the completed entry.
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
pnpm progress:history
jj --no-pager restore --from 1923eafb -- spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md spec/src/current/progress/M25/history/M25/index.md spec/src/current/progress/M25/index.md spec/src/history/2026/10/08/E-FLEET-006/index.md spec/src/history/2026/10/08/E-FLEET-007/index.md
pnpm exec prettier --write spec/src/current/progress/Bootstrap/index.md spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
pnpm check:progress; pnpm spec:validate; pnpm spec:first # final retry
Python prior evidence SHA-256 and bounded-page assertions
pnpm exec prettier --check spec/src/current/progress/Bootstrap/index.md spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
```

**Notes**

Progress checks pass for 146 tasks and 660 entries. Specification validation passes 7 validators, 3200 chapters and 181 requirements. Formatting and the final spec-first retry pass (no protected files changed). All 2257 pre-existing evidence Markdown files remain byte-identical. Restored generator-only changes in six unrelated M24/M25 and historical files to the pinned clean starting parent. The retirement record stays within authored page limits. Completed retirement and these passing document checks support the unchanged-contract confirmation.
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
pnpm spec:first
```

**Notes**

Initial spec-first invocation correctly rejects the still-pending evidence result as incomplete. The actual retirement, preservation, formatting, progress and specification checks have now passed; the completed record and exact spec-first retry pass with no protected files changed.
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

macOS ARM64; Jujutsu 0.45.1; system /usr/bin/trash; existing repository Node/pnpm document toolchain.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only Jujutsu workspace registrations and recoverable checkout locations change. Source, schemas, native identities, object/WAL/receipt/manifest formats and runtime fixtures remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Authorized recoverable retirement of the four integrated secondary checkouts, followed by a second registration and source-checkout audit. Product Rust/UI/browser/visual suites, root aggregate checks, builds, installation, publication, fixture resets and unrelated repositories are out of scope under JST-GOV-007.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue development from main/default; recover retained folders from Trash if their local artifacts are needed. Deferred capture work remains under its existing bookmark and milestone gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-GOV-003/005/011 require fresh retirement evidence and a verified
  Jujutsu commit; JST-GOV-009 retains existing main-owned runtime and fixture.
  Removal follows existing ancestry, clean-status and process-ownership rules.
  Capture uses the explicitly documented single-change integration and retains
  its original-history/deferred-work bookmarks. No protected product behavior
  changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Bootstrap remains Done with its original gate evidence. All product
  milestones, qualification gaps, historical entries and selected completion
  gates remain unchanged. This is source-workspace maintenance only.

```
<!-- /evidence-field -->

