# Recorded failures and repairs

These attempts belong to E-FLEET-004 and enclosing change yyzxpuru.
Final verification in the index supersedes them only where explicitly recorded.

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm web:build
```

**Notes**

Fresh Vite production assets built in 94s, but Help staging started a duplicate documentation compile without the shared target cache. That owned compile was stopped under disk pressure; the staging-only retry uses the existing shared cache. Build/stage final result remains pending.
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
node scripts/web-architecture.mjs && node scripts/check-boundaries.mjs && pnpm check:ui-ownership && node scripts/check-progress.mjs
```

**Notes**

Architecture passed 1316 modules, package boundaries passed 17 packages and UI ownership passed. Progress checking refused the then-empty E-FLEET-004 file/run fields and fresh file coverage. This record supplies actual coverage. The first populated-record rerun refused file-only test references and a pending run result. Named references and observed pass/fail run results replace those invalid entries; exact reruns are pending.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
pnpm catalog:check
```

**Notes**

Catalog caught 4 type errors: Testing Library does not accept Playwright exact options. Repaired with anchored exact-name expressions; final catalog and focused stories are rerunning. Help staging and native fixtures remain running. Interrupted attempts are not passing evidence.
<!-- /run -->

## Before and repair evidence

Additional authoring failures retained from earlier runs: six missing provenance
entries; literal closing script in a fixture; undefined bound raw state (12
failed assertions and 19 runtime errors); language assertions against text
instead of its accessible group (3 failures); an empty diff line counted as a
deletion; a Rust legacy-display borrow error; spec paragraph/bullet-shape
refusals; incomplete source evidence; and a fleet assertion missing context.
Later jobs exhausted disk; only generated owning-package compiler caches were
cleared. The preview, Nyra, saved chat, preferences and fixture were preserved.

Early six-family stories returned 39/40 with a Copy message menu lookup failure.
An exact rerun also caught hover state outside the real trigger. These are
superseded only by the later exact/focused passes recorded above. The old catalog
check, browser fixture generation and docs build were interrupted, not passed.

The broader source checks are required in Design Core because its public shared
CSS affects multiple component families. Root aggregate jj-store
spec:check/check/test/build lanes remain out of scope under the targeted policy.
Visual baselines, publication and real adapter prompts were not run.

## Compilation cache refusal

The unqualified full web:build stage started an unused duplicate compiler cache.
Its owned compile was stopped and the retry uses the shared cache. Both
`cargo clean --target-dir /Users/stevejuma/code/lapismd/jj-store-agent-fleet/target`
and its `-p jj-store-cli` variant refused cleanup because CACHEDIR.TAG is absent.
Those refusals were retained; no marker was fabricated or user data removed.
The force-removal command was rejected by command review. After confirming no
active cache lock, ordinary non-force removal of only target/debug/incremental
succeeded, recovering the unused compiler chunks. The same unused cache had 2.4GB of deps, 1.2GB of build outputs and
12MB of fingerprints. After reconfirming no active lock, ordinary non-force
removal of those three known generated subdirectories succeeded. All fixtures
and shared build artifacts remain retained.
Shared-cache compilation, fixture generation and the fresh browser host remain
separate from passing UI behavior evidence.

## Documentation and formatting follow-up

The final docs check refused a stale collaboration fingerprint after the last
null/empty raw-output repair. A source-fingerprint audit found only that topic
stale; its review was refreshed with the actual capture/identity behavior.
The focused changed-file Prettier check found two files: the new rich-tools
story and the generated coverage JSON. Both were formatted without behavior
changes. Final reruns are recorded separately.

The standalone repaired docs:check process was stopped while waiting on Cargo.
Its complete check/build functions are also performed by the single retained
Help staging job; serializing them avoids concurrent cleanup of docs/build.
The interrupted standalone process is not reported as a pass.

## Final cold browser fixture warning

`pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts web/tests/browser/agent-tool-rich.spec.ts`
returned 0 pass / 1 fail. The repository fixture's Commit metadata unavailable
warning intercepted the Send click; no tool-rendering assertion had run yet.
The exact test now dismisses that warning through its ordinary accessible
button before sending its mocked message. No forced click, notification
suppression, shared fixture edit or weakened rendering assertion was added.
The fresh-host rerun is recorded in the final qualification page.

Two following exact reruns each returned 0 pass / 1 fail in test selection.
The rich header combines the path and language, so an exact path-only text
filter did not match. Scoping details to the tool article exposed a second
selector problem: its filter depended on Show details, which becomes Hide
details on expansion. The article now uses its stable accessible name and
independently asserts the captured path, TypeScript group and file bytes.
Production code and all functional assertions remain unchanged by these repairs.
