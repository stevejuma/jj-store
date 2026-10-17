---
schema: 1
id: E-INJECTABLE-STORES-002
---

# E-INJECTABLE-STORES-002 details 5

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
python3 <assert empty jj diff --name-only -- schemas/openapi.json web/src/lib/api/generated.ts>
```

**Notes**

Generated contracts exactly match the original tracked files; no public wire/schema drift. git-contract-equality-2.log.
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

162 tasks;851evidence records after retaining the pending provider-composition gates.
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
pnpm spec:first
```

**Notes**

Pending full provider qualification cannot use an unchanged-contract shortcut. Corrected contract_review to changed for the new additive factory/validator/receipt interfaces, governed by updated JST-ARCH-006 chapters; old authoritative formats remain covered by focused passing regressions.
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
pnpm spec:validate
```

**Notes**

7validators,4104chapters,206requirements before final evidence pagination. No product gate inferred.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The protected-change map required current-slice public-distribution, CLI and governance updates for the changeset, direct consumer fixes and boundary allowlist. Added exact ownership/unchanged-command and focused-verification descriptions in those owning chapters.
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
pnpm check:progress
```

**Notes**

Final ledger validates162tasks/851records. M32/M32.1/M32.2 remain In progress; E002 remains pending for explicitly retained composition gates, despite passing extraction regressions.
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
pnpm spec:first
```

**Notes**

105protected files/8canonical chapters passed with the additive library contract change and exact owning-topic updates.
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
pnpm spec:validate
```

**Notes**

7validators/4104chapters/206requirements passed. Historical evidence remains immutable.
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
node scripts/package-check.mjs
```

**Notes**

Final23coordinatedpackages have contained source and portable registry dependencies.
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
pnpm check:boundaries
```

**Notes**

Final22workspacepackage boundary audit passed.
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
rustfmt --edition2024 --config skip_children=true --check <changed existing Rust files>; pnpm exec prettier --write <changed Markdown/JavaScript files>
```

**Notes**

Changed Rust sources and owning documents formatted. Unrelated UI, root aggregate build/test/check/spec:check, full workspace and unrequested release lanes are out of scope for this Rust extraction slice.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete selected source-participant admission for cross-domain substitution;
  do not fall back to an unselected built-in source. M32.1 and M32.2 remain In
  progress until this composition gate is complete.
- Native factories, application factory routing, the independent external
  consumer and cold native real-RustFS qualification remain later M32 work. No
  full-plan completion is claimed.
- Restore the authorized development stack in its original Vite mode after
  remaining validation, without resetting its retained fixture.
- The user additionally requested concise role/usage READMEs at every crate
  root, with detailed documentation retained under docs. That follow-up has been
  inventoried and remains a subsequent slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; existing pinned JJ0.45.1, Git2.55.0 and Rust1.99 toolchain; workspace libraries0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing refs/lapis/state/v1, companion v1/v2 metadata, current Agent descriptor v2, immutable record envelope v1 and native manifest/WAL/receipt formats remain selected. The new Rust interfaces add no repository marker.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Verified extraction of generic Git metadata and five local collaboration stores with direct Git canonical records, independently named factories, injected host actor/clock, selected-history admission and qualified JJ evidence validation. Shared full-reference decoding preserves source consumers and rejection checks. Service retains presentation, admission, cache and conversion policy. Native migration and complete cross-domain substitution remain later gates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified Git extraction with Jujutsu; complete private participant admission, native adapters, application consumers, external embedding qualification and the requested crate READMEs. Restore the dev stack in original Vite mode after validation.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-006 introduces new Git factory, provider-scope,
  evidence-validator and receipt adaptation contracts, documented in the owning
  architecture chapter. This is an additive library contract change. Owning
  chapters retain JST-LOCAL-001/002/003 immutable ref authority, selected
  closure and original receipts; JST-JJ-001/002/003/004/005 pinned integration;
  JST-PUB-001/004/005/007/009 native authority. JST-ARCH-006 already owns the
  new library interfaces and publication participant boundary. Focused exact and
  affected-family tests, qualified companion/default-JJ codec and retention
  checks, dependency closure and strict direct-consumer compilation preserve
  this extraction contract. Remaining provider-composition and native/external
  gates are explicit.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M32.2 starts In progress. M32 and M32.1 remain In progress, with M32.3
  and M32.4 still Planned. The authorized stop and eligible cleanup enabled
  verification; remaining source substitution and later migration prevent
  milestone completion. All failed attempts and final focused results remain
  recorded.

```
<!-- /evidence-field -->

