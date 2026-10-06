---
schema: 1
id: E-M15-022
---

# E-M15-022 details 3

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
pnpm spec:validate
```

**Notes**

5 validators, 21 chapters, 145 requirements pass; /tmp/jj-rebase-spec.log.
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
pnpm spec:first
```

**Notes**

209 protected files and 10 chapters pass; /tmp/jj-rebase-spec-first.log.
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
pnpm check:boundaries
```

**Notes**

16 packages pass; /tmp/jj-rebase-boundaries-final.log.
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
pnpm check:ui-ownership
```

**Notes**

Focused ownership validator passes; /tmp/jj-rebase-ownership-final.log.
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
cargo fmt -p jj-store-service -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

Changed Rust packages pass formatting; /tmp/jj-rebase-rust-format-final.log.
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
pnpm exec prettier --check (207 changed supported files from jj diff --name-only)
```

**Notes**

All changed supported files pass; /tmp/jj-rebase-format-final.log.
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
python3 ledger union comparison using jj file show -r c347df2d and -r ed22112b
```

**Notes**

All 243 original evidence objects from both parent trees remain unchanged; 244 entries including this integration evidence.
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
CARGO_INCREMENTAL=0 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/threads-alignment.spec.ts
```

**Notes**

Fresh production preview: 1 cold LocalTrusted Threads settings/reference/scoped-search/reply-focus test passes; /tmp/jj-rebase-thread-browser.log. MemoryStore preview evidence is distinct from real RustFS qualification.
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
JJ_STORE_FILESYSTEM_PORT=17478 node scripts/local-browser-acceptance.mjs
```

**Notes**

Fresh isolated filesystem-only service on port 17478 and built assets: all 3 real local browser tests pass, including recorded Agents, WorkStore unknown-outcome recovery and folder reconnect/removal; /tmp/jj-rebase-local-browser.log. Fixture and isolated service cleaned up; main PID 23243 untouched.
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
pnpm check:architecture; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Final 619-module architecture, 119-task progress, 5-validator/21-chapter/145-requirement specification and 209-file/10-chapter same-change gate all pass; /tmp/jj-rebase-architecture-final.log, /tmp/jj-rebase-progress-final.log, /tmp/jj-rebase-validate-final.log and /tmp/jj-rebase-first-final.log.
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
pnpm check:progress (final qualified ledger, initial)
```

**Notes**

Passing integration evidence listed broader out-of-scope milestone gates as its remaining gaps. Moved those unchanged limitations to scope/next; no product gate waived.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj log --no-graph -r 'conflicts(msszrzpn)' -T commit_id
```

**Notes**

Read-only conflict audit used unsupported revset signature; correct expression is msszrzpn &amp; conflicts(). No history mutation.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first (qualified ledger rerun)
```

**Notes**

119 tasks, 5 validators, 21 chapters, 145 requirements and 209 protected files across 10 chapters pass after keeping independent milestone limitations explicit in scope/next; /tmp/jj-rebase-progress-qualified.log, /tmp/jj-rebase-validate-qualified.log and /tmp/jj-rebase-first-qualified.log.
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
jj --no-pager log -r 'msszrzpn & conflicts()' --no-graph -T commit_id
```

**Notes**

Corrected read-only revset returns no conflicts.
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

<!-- evidence-field:escalation markdown -->
## Escalation

Rebase combines shared profile-aware connection/route/surface contracts with Thread/global search and changed HTTP schema consumers. Full relevant catalog/controller units, direct consumer Storybook, contract captures, production build and isolated native/filesystem cold browser checks are required. Root aggregates, full Rust workspace, publication and installed-service changes remain outside this integration scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this verified descendant of main c347df2d into the clean default workspace, recording separate landing evidence. Keep M15/M15.5 In progress and preserve the running main dev server.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

 Rebase qualification is complete. Visual baseline approval remains separate; M15.7 real RustFS full-flow and JJ ACP runner gates and registry-only Design Core foundation qualification remain outside this integration and unchanged.
<!-- /evidence-field -->

