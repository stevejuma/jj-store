---
schema: 1
id: E-UI-101
---

# E-UI-101 details 3

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
pnpm check:architecture
```

**Notes**

architecture.log:993 production modules.
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
jj rebase -r uqwlnutu -d zpvqmolk; jj workspace update-stale (preview)
```

**Notes**

Original clean preview reused. Initial parent snapshot was stale; refreshed source workspace then updated preview before final live evidence.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store pnpm dev:ui
```

**Notes**

Two explicit preview restarts reuse existing binaries/fixture. Initial live page retained old compiled Body orientation; final restart follows forced Vite dependency regeneration. No build/reset or unrelated main 7460 restart.
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
pnpm exec vite optimize --config web/vite.config.ts --force (preview)
```

**Notes**

Public Vite cache regeneration; deprecated-command notice recorded. Clears stale compiled dependency, not a source patch or package install.
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
CUA actual Linux split view at 1005x 988: horizontal/vertical wheel input and screenshot
```

**Notes**

linux-wheel-proof.json and linux-after.png:371px viewport/640px content;horizontal 0→149px with vertical 0;vertical→170.5px with horizontal 149 retained;restored 0/0. First row padding 44/56/68/80px; message widths 418/406/394/382px.23 rendered rows retain virtualization. Original legacy-tab focus and transient loading lookup failures recovered through fresh validation tab.
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
pnpm spec:validate (initial)
```

**Notes**

spec.log: 5 validators, 23 chapters, 172 requirements.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

spec-first.log: mapped web changes require api.md as well as ui.md. Added browser scroll/paging boundary documentation to the owning API chapter.
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
pnpm check:progress (initial)
```

**Notes**

progress.log: 139 tasks; evidence bookkeeping only.
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
pnpm spec:validate (final)
```

**Notes**

spec-final.log: 5 validators, 23 chapters, 172 requirements.
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
pnpm spec:first (final)
```

**Notes**

spec-first-final.log: 5 protected files, 3 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (bookkeeping attempt)
```

**Notes**

New-entry prose spacing accidentally changed its two task identifiers; restored exact M4.9 and M4.10 references before final validation.
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
pnpm exec prettier --write spec/src/api.md spec/src/milestones.md; pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

ledger-final-format.log and ledger-final-format2.log: canonical/evidence formatting.
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
pnpm check:progress (final)
```

**Notes**

progress-final2.log: 139 tasks; evidence bookkeeping only.
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
pnpm exec prettier --check catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/store-application/StoreEvolutionColumn.svelte spec/src/ui.md spec/src/api.md spec/src/milestones.md
```

**Notes**

final-format-check.log: all eight changed files use Prettier style.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Design Core package publication and registry-resolved consumer acceptance
  remain separate; current dev uses the existing local source validation setup.
- Visual review/baselines remain pending. Rust/backend, root aggregate, full
  repository/release/fault suites are out of scope for this UI slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the verified UI slice in bounded-reads and the refreshed 7470 preview; stop after committing. Keep advanced-query work deferred.
<!-- /evidence-field -->

