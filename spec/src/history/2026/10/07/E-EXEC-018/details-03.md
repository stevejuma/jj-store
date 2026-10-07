---
schema: 1
id: E-EXEC-018
---

# E-EXEC-018 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:first
node --input-type=module <<'FORMAT'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'})
  .trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
FORMAT
```

**Notes**

Historical evidence named the original timestamp test. Kept its stable name while extending its assertions. The navigation generator also rewrote unrelated old ledgers/evidence; restored exactly those generated changes from @- and retained only new evidence/date navigation plus M23 review. Source/spec validation had passed. No historical evidence or unrelated milestone was rewritten. Three failed gate invocations: progress, spec-first and scoped formatting. Final review found the new M23 history rows separated from their table; restored contiguous Markdown rows before final document validation.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
node --input-type=module <<'FORMAT'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'})
  .trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
FORMAT
```

**Notes**

Final corrected documents pass: 144 tasks, 642 evidence records; seven validators, 3114 chapters, 178 requirements; 26 protected files and one canonical chapter; all scoped files formatted. M23 history contains contiguous E-EXEC-018/E-EXEC-019 table rows. Historical timestamp test name retained; unrelated evidence/navigation restored. Product checks were already complete and no production edits followed them.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-002
- JST-UI-006
- JST-UI-015
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Agent inbox refinement. Before: status text tags, fixed chat column, plain turn notices and repeated unknown clocks. After: independent repository icon links, Community presence, persistent bounded pointer/keyboard resizing, thread day/turn dividers, top padding and icon-only tooltipped gallery actions. Current saved chat has 275 events, only four with observation time; 271 predate timestamp capture and all three turn records lack another timestamp. No last-activity, file metadata or neighboring-message time is substituted. Later known chunk times remain eligible.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated owning JST-UI-015/JST-UI-002/JST-UI-006 chapter for session row
  links/presence, versioned resizing, first-available clocks and honest legacy
  groups; component presentation uses existing Community and Design Core
  primitives.

```
<!-- /evidence-field -->

