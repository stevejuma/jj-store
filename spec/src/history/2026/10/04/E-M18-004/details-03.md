---
schema: 1
id: E-M18-004
---

# E-M18-004 details 3

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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check changed files excluding historical ledger
```

**Notes**

Spec-first 10 protected paths/5 canonical chapters; 5 validators/22 chapters/167 requirements; 136 tasks; final changed-file formatting passes.
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
pnpm spec:first --base eb25eacb78b1047e6fdb23e785b3796b593abeed --head 65e55df3da8245e00fc3b434936231d8d1393d6a (preview workspace)
```

**Notes**

Explicit range uses Git discovery, unavailable in secondary JJ workspace. Ordinary JJ-aware spec-first passes; reran the complete range from colocated main below.
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
jj --no-pager st; jj rebase -r @ -o krnstxms; jj --no-pager st (main workspace)
```

**Notes**

Main still clean on eb25eacb before integration. Advanced its empty tkwpktrq change onto verified feature tip; 352 added/119 modified/1 removed files, no conflicts; main clean afterwards. All existing main history retained.
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
pnpm spec:first --base eb25eacb78b1047e6fdb23e785b3796b593abeed --head 65e55df3da8245e00fc3b434936231d8d1393d6a (colocated main workspace)
```

**Notes**

Complete rebased integration gate passed: 457 protected files/14 canonical chapters. Same combined tree, no substitute tests.
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

JJ 0.45.1; pnpm 10.32.1; Svelte 5; Vitest 4.1.11; Playwright 1.63.0; existing temporary Design Core source validation refreshed, registry release qualification deferred under M4.31.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Current AgentStore only. Rust code, public schemas/generated contracts, capture persistence and exact pending request identities unchanged. Main and capture ledger histories preserved verbatim.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI/controller notification follow-up to already qualified capture stack, rebased on main eb25eacb. Installation-scoped condition notices for operational failures and timed information outcomes; validation/unavailable-content/unknown receipt recovery contextual. Direct UI/controller and cold browser lanes only; Rust/full contract/root aggregate lanes out of scope because their boundary trees are identical. Broader M18 harness/release scope unchanged; existing intermittent capture busy/performance deferred. Stories visual-pending; no baseline approval, push, release or installed service changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Main workspace now contains the rebased capture preview and notification integration. Use Settings &gt; Agents in the preserved isolated preview; defer additional harness, capture performance and registry-release qualification as already governed.
<!-- /evidence-field -->

