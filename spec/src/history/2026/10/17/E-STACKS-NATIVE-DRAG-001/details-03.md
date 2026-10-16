---
schema: 1
id: E-STACKS-NATIVE-DRAG-001
---

# E-STACKS-NATIVE-DRAG-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

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

Initial spec-first could not trust the new draft evidence with omitted run failure counts. Fixed evidence metadata, preserving all actual test/audit failures.
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
```

**Notes**

157 tasks / 849 evidence records validated. M21 and its UI/backend task statuses remain unchanged; focused evidence is added to each selected ledger/history.
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

Passed for 16 protected files and 5 canonical chapters.
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
pnpm spec:links
```

**Notes**

Book/link audit passed for current canonical pages and new evidence/date navigation.
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
pnpm exec prettier --check <all paths from jj --no-pager diff --name-only ending .md, .ts or .svelte>
```

**Notes**

Exact changed-file selection was passed as an argument array; all matched files use Prettier style.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

JJ 0.45.1 does not provide diff --check. No files changed; exact-file Prettier and the successful specification/type checks supply the applicable validation.
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
pnpm spec:validate; pnpm check:progress; pnpm spec:first
```

**Notes**

Final document checks passed: 7 validators, 4078 chapters, 205 requirements; 157 tasks and 849 evidence records; 16 protected files and 5 canonical chapters.
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

<!-- evidence-field:next markdown -->
## Next

Review the current UI at http://127.0.0.1:7460/. Preserve exact-source guards. Broader provider/platform/release and visual approval remain separate milestone gates; independently reduce the three pre-existing architecture size violations.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and the owning frontend boundary now qualify bounded
  concrete ownership resolution for native branch drags, replacing the blanket
  production aggregate prohibition. JST-API-001/002 transport schemas and
  authorization/execution guards are unchanged: only existing pinned read and
  native hunk endpoints are used; client net patches never execute. JST-GOV-008
  source ownership remains unchanged; all new code is application-authored under
  owned production paths and isolated catalog fixtures."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M21 stays In progress; M21.1 stays Partial; M21.2 stays In progress.
  This closes the focused production branch drag integration gap, with explicit
  bounded/ambiguous exclusions. It does not claim new provider/backend
  qualification, release, platform or visual approval. Existing open gates and
  next actions remain.

```
<!-- /evidence-field -->

