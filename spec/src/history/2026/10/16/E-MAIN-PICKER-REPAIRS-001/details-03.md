---
schema: 1
id: E-MAIN-PICKER-REPAIRS-001
---

# E-MAIN-PICKER-REPAIRS-001 details 3

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
pnpm format:check
```

**Notes**

format-83: exit 0, 59.05 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 JJ_STORE_REAL_TEST=native-repository.spec.ts pnpm test:local-release
```

**Notes**

picker-cold-82: exit 0, 400.28 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

picker-consumers-81: exit 0, 4.79 seconds. 
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

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Rust 1.99.0; JJ 0.45.1; Design Core 0.5.0; Mira 0.0.3; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native identity, snapshot, receipt, wire, preference or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed follow-up repair acceptance only. Full cold67 previously finished 25 passing, two failing and 16 declared fixture-specific skips; profile acceptance did not start after that failure. This record qualifies the exact reference-cancellation and response-readiness repairs, their direct consumers, fresh native browser regression and full formatting. Comprehensive local qualification and hosted PR #10 remain separately pending; no overall pass is claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue complete local checks; append fresh qualification evidence and update draft PR https://github.com/stevejuma/jj-store/pull/10. No merge or release is authorized.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-005/014/016 retain operation-pinned loading and exact selection
  ownership. Accepted bookmark/working-copy selections now restart interrupted
  reference picker reads immediately under the new read generation. Complete
  current windows stay cached; old responses remain rejected. Two new regression
  cases failed before the fix and pass afterward; an additional real-reader case
  proves no redundant references requests. The original private workspace
  fixture supplies the owned picker reader method without relaxing assertions.
  The mutation preview test awaits its exact original POST response, asserts
  HTTP success, prepared status and the same request ID, then retains the
  original five-second prepared-UI assertion. Existing Playwright response and
  overall test limits remain; no timeout literal or request replay was added.
  API/receipt semantics under JST-API-006 are unchanged. Documentation review
  fingerprints and two milestone Markdown formatting corrections change no
  contract or gate. The canonical startup chapter clarifies the existing
  recovery requirement.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "Explicit M4.4, M16, M17.2, M28 and M30 review: each retains its
  existing status. This completed repair slice does not complete comprehensive
  local qualification, hosted checks, release, other platforms, signed desktop,
  remote Forgejo protocol, live providers or visual baselines."

```
<!-- /evidence-field -->

