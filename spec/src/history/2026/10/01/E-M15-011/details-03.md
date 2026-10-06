---
schema: 1
id: E-M15-011
---

# E-M15-011 details 3

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
pnpm exec prettier --check crates/jj-store-http/src/contract.rs scripts/contracts.mjs scripts/contracts.test.mjs scripts/contracts.types.ts spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/spec-governance.md spec/src/milestones.md schemas/openapi.json web/src/lib/api/generated.ts
```

**Notes**

Rust file mistakenly included in the document formatter, which has no Rust parser. Rust formatting had separately passed. Corrected the formatting scope.
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
cargo fmt -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting passes.
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

15 protected files; six canonical chapters.
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

Four validators; 21 chapters; 140 requirements.
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
pnpm check:progress (fixed)
```

**Notes**

111 tasks; evidence bookkeeping only.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries.
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
pnpm exec prettier --check scripts/contracts.mjs scripts/contracts.test.mjs scripts/contracts.types.ts spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/spec-governance.md spec/src/milestones.md schemas/openapi.json web/src/lib/api/generated.ts
```

**Notes**

Focused document/tool/generated formatting passes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check (initial)
```

**Notes**

All 102 actual responses conform and seven schema tests pass; TypeScript negative-fixture marker was one property too high (unused marker plus expected missing-version diagnostic). Moved the marker to value; /tmp/jj-collab-http-contracts-check.log.
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
pnpm test:contracts (fixed)
```

**Notes**

Seven schema regressions plus compile-time TypeScript consumer pass; /tmp/jj-collab-http-contracts-types-fixed.log.
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
pnpm contracts:check (final)
```

**Notes**

102 actual HTTP/SSE responses conform, generated artifacts match, seven schema tests and compile-time consumer pass; /tmp/jj-collab-http-contracts-check-final.log.
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
pnpm exec prettier --write scripts/contracts.types.ts
```

**Notes**

Negative fixture marker formatting after repair.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Evidence updated and formatted after every gate outcome.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Shared HTTP admission/media checks, registered route inventory and OpenAPI generation affect all HTTP consumers. Before widening, select the complete HTTP package suite, every native_lifecycle http_ consumer, the full generated contract capture/validation and compile-time lane, strict HTTP/CLI lint and generated web type check. Unrelated UI interactions, full Rust workspace and root aggregate lanes remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Optional atomic capture binding, reducer 3, canonical cross-source
  timeline/search, released Community channel UI template, real-RustFS/cold
  full-flow and jj-acp consumer qualification remain pending.
- Native code writes remain reducer-3 gated; WorkStore homes retain their frozen
  origin ledger. Handler/router reconstruction over MemoryStore is transport
  evidence, not persistent server restart or real RustFS/browser/runner
  qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify explicit optional capture binding with atomic Agent/Thread publication and durable retry identities.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs#http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs#http_collab_refuses_injection_permissions_media_ranges_limits_and_preserves_unknown_identity
<!-- /evidence-field -->

<!-- evidence-field:channel_template markdown -->
## Channel template

User steering on 2026-10-01: a Thread is presented as a Community channel. Inspected source at Community parent 5041b4c9 with a clean working copy: public CommunityConversation channel timeline/history/composer/actions/replies/reactions plus shared catalog contracts. Canonical chapters and M15.5 scope now require this template; channel adapter/released package/cold UI qualification remain pending.
<!-- /evidence-field -->

