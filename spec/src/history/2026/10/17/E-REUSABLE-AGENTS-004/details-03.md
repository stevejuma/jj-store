---
schema: 1
id: E-REUSABLE-AGENTS-004
---

# E-REUSABLE-AGENTS-004 details 3

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

An initial combined architecture pattern selected the older M32 chapter. Replaced it with a dedicated conversation-to-M33 chapter mapping; original store mapping remains strict and unchanged.
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
node --test scripts/progress-mapping.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

15owning evidence/navigation/task mapping cases pass.
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
cargo package -p jj-store-conversations --list --allow-dirty --offline
```

**Notes**

Read-only package contents list includes contained source/tests, manifest, license, notice and short README. This is not registry publication or independent archive acceptance.
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
pnpm spec:first
```

**Notes**

25protectedfiles/7canonicalchapters pass with the dedicated conversation mapping.
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
pnpm spec:validate
```

**Notes**

7validators/4170chapters/207requirements pass with complete evidence navigation.
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

Rust1.99.0; pnpm10.32.1; new jj-store-conversations0.1.0; original source parent8a487398; native privacy policy/scanner pins unchanged. Source provenance: runtime_privacy.rs SHA-256 6cc339cb1414116e85463d7a1cc5d1ef13b32cf1e90da42377fc779c81d7039e; conversation_stream.rs SHA-256 0818c712dd618c5b94c5418601899f6a6c1b0f54cd7bffae90176b1c8f121c69.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Original Control V3 frame keys/order, prefix flush before terminal,512KiB assistant/256tool/256participation bounds, private execution journal fields, capture payloads and receipt identities unchanged. New library error mapping retains existing service variants.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

M33.3 initial shared stream slice only. Pure library injects host privacy and projects safe assistant/tool frames. The existing server single-runtime and fleet consumers delegate through admitted native sanitizer adapters. Full lifecycle/store/runtime/context/journal ports, typed client, UI and Notes remain pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Extract host-injected conversation lifecycle and durable publication coordination; preserve distinct publication, delivery and capture outcomes.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007 initial reusable stream implementation preserves
  JST-COLLAB-031 native privacy and durable capture-before-display. Original
  scanner tests remain in the consumer; independent policy fixtures establish
  injection and stream mechanics only. JST-ARCH-005 adds a coordinated library
  and ordinary CI cohort; release/provider/desktop authority unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M33 remains In progress; M33.3 starts In progress with shared stream
  primitives consumed by the server. Lifecycle/store/runtime ports and
  downstream cold acceptance remain open. M33.1/.2 retain their source
  portability qualification status; later stages remain Planned.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kwwpzuum
<!-- /evidence-field -->

