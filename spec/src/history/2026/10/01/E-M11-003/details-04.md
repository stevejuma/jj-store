---
schema: 1
id: E-M11-003
---

# E-M11-003 details 4

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
node --test scripts/dev-demo-collaboration.test.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo-state.test.mjs scripts/governance.test.mjs scripts/web-architecture.test.mjs
```

**Notes**

Final shared startup/fixture direct-consumer source lane passes 34 tests.
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
pnpm exec prettier --check AGENTS.md scripts/dev-demo.mjs scripts/real-browser-acceptance.mjs spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/projects.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work-integration.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/store-error.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/work-types.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/repository-sections.ts web/src/lib/ui/store-work catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/StoreWorkFixture.svelte web/tests/real/work-store.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-work.test.ts
```

**Notes**

Exact originally failing source-formatting check passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7478 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm web:test:real
```

**Notes**

Corrected physical cold startup passes: 1242ms first, 1008ms persistent restart, 333746ms full fixture preparation, 172240KiB RSS. WorkStore with a real non-null code operation pin passes; bootstrap and atomic conflict acceptance also pass. Three pre-existing native browser scenarios drifted: hidden JJ Log rail action now uses Commits tab; mutation Description is a Markdown editor textbox; route capture preceded the initial seeded revision-review association. Updated selectors and waited for the associated thread before capturing the complete route, preserving identity/publication/unchanged-route assertions. Runner stopped and removed its owned root.
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
pnpm exec prettier --write web/tests/real/native-repository.spec.ts spec/src/ui.md
```

**Notes**

Updated cold direct-consumer fixtures and owning UI evidence formatted.
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
JJ_STORE_DEV_PORT=7478 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_KEEP=1 pnpm web:test:real
```

**Notes**

Final full fresh fixture and physical cold server/storage restart pass: first 1031ms, restart 666ms, full preparation 342029ms, total RSS 172736KiB. All six real browser scenarios pass in 46.4s: LocalTrusted bootstrap, same-change divergent native identities, external publication/explicit refresh with complete route preservation, atomic conflict resolution, identical preview/apply native mutation and WorkStore project/link/subtask/state/reply/edit/tombstone/unknown receipt recovery/reload. Work test preserves a non-null full native code operation pin; zero axe violations and 420px overflow assertion pass. Strict 30000ms/524288KiB gates unchanged. Exact three formerly failing native scenarios pass within the full run.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-7O3yib pnpm dev:stop
```

**Notes**

Final kept acceptance server/storage stopped, no owned processes remained, and only its disposable root was removed. Checkout hooks removed by fixture teardown; no installations or existing deployments changed.
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
pnpm exec prettier --check AGENTS.md scripts/dev-demo.mjs scripts/real-browser-acceptance.mjs spec/src/{api,cli,compatibility,milestones,projects,spec-governance,ui,verification}.md web/src/lib/controller/{contracts.ts,http-store-data-source.ts,http-work-data-source.ts,native-data-source.ts,production-store-controller.svelte.ts,production-store-lifecycle.svelte.ts,production-store-view.ts,production-work-integration.ts,production-work.svelte.ts,store-error.ts,store-route-codec.ts,store-route-history.svelte.ts,work-types.ts} web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/store-application/{StoreRepositoryApplication.svelte,repository-sections.ts} web/src/lib/ui/store-work catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/StoreWorkFixture.svelte web/tests/real/{native-repository,work-store}.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-work.test.ts
```

**Notes**

All changed source, fixtures, harness and canonical files pass formatting.
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

Final 19 chapters / 115 requirements pass.
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
pnpm spec:first
```

**Notes**

Final 32 protected files map to seven changed canonical chapters.
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

Final 15 package boundaries pass.
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
pnpm check:progress
```

**Notes**

Final 77 tasks and completed M11 scoped evidence pass bookkeeping.
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

<!-- evidence-field:scope markdown -->
## Scope

M11 native local-service profile. Community task presentation and Design Core controls inspected in their owners; existing temporary source-validation hook retained without package/lockfile changes. Fake-store backend gates remain E-M11-001/002; this entry supplies separate real RustFS and browser evidence. No root aggregates, full Rust workspace or unrelated Storybook suites; those lanes are out of scope. Stories remain visual-pending, with no visual baseline approval. Git refs/server-free CLI, assignments, dispatch and origin replication for WorkStore homes remain deferred. Cold resources qualify opening the fully prepared persisted sample; initial search construction is explicitly fixture preparation, with no claim about cold cache-loss rebuild speed.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Before UI implementation the Work unit target could not import its missing module (one failed suite). Initial cold harness attempt rejected before Playwright because restart timing incorrectly included full fixture seeding; timer now stops at server/search readiness and records fixturePreparationMillis separately. Exact original shell invocations for these two pre-handoff failures were not retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M11 focused/native/real browser gates. Visual baselines remain pending; WorkStore origin replication, Git refs, server-free CLI, assignments and dispatch remain deferred.
<!-- /evidence-field -->

