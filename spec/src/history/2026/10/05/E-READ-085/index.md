---
schema: 1
id: E-READ-085
---

# E-READ-085

<!-- evidence-field:id markdown -->
## Id

E-READ-085
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M4.6
- M4.9
- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-004
- JST-GOV-005
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main workspace landing of completed graph, feedback and text-diff slices
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pkkupzrv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main was clean at empty pkkupzrv/19cefaa1 above xzwnoxru/4fcda245. Completed E-UI-101, E-UI-102 and E-READ-084 were a linear descendant chain ending at mrqotlum/85aababb in bounded-reads and codex-preview. No intervening main work required a merge.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Advance the main empty change onto the three completed slices without rewriting their commits; preserve all existing ledger entries and byte-identical source evidence. Both Design Core fixes wsqpzxvk/b677c9fb and lqkxrkvp/a47490e6 are already ancestors of its main workspace. Retain temporary and preview workspaces and their running fixture. Other registered workspaces are outside this landing.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, generated contract, native identity, index, companion, WAL, manifest, receipt or installed format change. Only this integration evidence is new. Product-tree identity reuses the source regression and cold browser evidence recorded in E-UI-101, E-UI-102 and E-READ-084; no milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Same source and pinned dependency versions as E-UI-101, E-UI-102 and E-READ-084. No package publication, dependency update, executable replacement or runtime restart.
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
jj --no-pager st; jj --no-pager log -r '@ | @- | (@-::mrqotlum)' --summary; jj --no-pager diff --from @- --to mrqotlum --summary; jj workspace list
```

**Notes**

Read-only preflight confirmed clean main, completed three-slice chain and clean bounded-reads/preview. Unrelated workspace heads preserved.
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
jj --no-pager log -r '@ | @- | ((wsqpzxvk | lqkxrkvp) & ::@)' (Design Core main)
```

**Notes**

Both foundational fixes already landed in Design Core main; no duplicate integration or publication.
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
jj rebase -r pkkupzrv -o mrqotlum; jj --no-pager st; jj --no-pager diff --from mrqotlum --to @ --summary
```

**Notes**

Moved only main empty change; clean, no conflicts and empty source-tree diff. Original completed commits retained.
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
python3 - <<'PY' (tracked-file comparison and SHA-256 evidence copy)
```

**Notes**

All 2632 tracked files matched bounded-reads. Copied and verified 125 source evidence files under .local/main-ui-diff-landing/source-evidence; manifests tracked-source-identity.json and copied-evidence-sha256.json retain hashes. Temporary originals retained.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
rg -n (inspection with scripts/check-spec-first*); rg -n (inspection with spec.config.*)
```

**Notes**

Two zsh inspection globs had no matches. Corrected by reading scripts/check-progress.mjs and the actual spec-validator.config.mjs. No product or verification failure.
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
pnpm exec spec-validator validate --only governance,summary,verification,book
```

**Notes**

Main canonical.log: four validators, 23 chapters, 172 requirements.
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
pnpm check:ui-ownership
```

**Notes**

Main ownership.log: repository-owned UI boundary passed. Source production files are byte-identical to the already-qualified slice.
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
pnpm exec prettier spec/src/milestones.md; python3 - <<'PY' (new-entry formatting and historical-prefix preservation)
```

**Notes**

Formatted only the added evidence from formatter output. All 527 prior evidence entries and task rows remain byte-identical apart from the required array separator.
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
pnpm check:progress
```

**Notes**

Main progress.log: 139 tasks; ledger bookkeeping passed without changing milestone status.
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
pnpm exec spec-validator first
```

**Notes**

Main spec-first.log: no protected files changed in the new integration evidence change. Original protected implementation commits retain their owning canonical updates.
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
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Main format.log: canonical ledger formatting passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- E-READ-084 retains 17 reproduced pre-existing Clippy diagnostics in untouched
  modules; this landing neither fixes nor suppresses them.
- Design Core publication and registry-resolved consumer acceptance, visual
  baselines, advanced-query optimization and full scale/RSS/RustFS/release
  qualification remain deferred. Root aggregate, Rust/UI product suites and
  browser reruns are out of scope for this evidence-only advance; existing
  product and cold browser evidence is retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit main integration evidence with Jujutsu and advance codex/bounded-reads-landed. Keep bounded-reads and codex-preview open for follow-up work; no runtime churn or unrelated-workspace landing.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


