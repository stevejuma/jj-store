---
schema: 1
id: E-CI-QUALIFICATION-READINESS-001
---

# E-CI-QUALIFICATION-READINESS-001

<!-- evidence-field:id markdown -->
## Id

E-CI-QUALIFICATION-READINESS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-API-001
- JST-CLI-001
- JST-GOV-003
- JST-GOV-007
- JST-JJ-005
- JST-LOCAL-004
- JST-UI-001
- JST-UI-005
- JST-UI-008
- JST-UI-020
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte
- catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-server/tests/configuration.rs
- web/tests/real/event-freshness.spec.ts
- web/tests/real/native-repository.spec.ts
- spec/src/history/2026/11/03/E-CI-QUALIFICATION-READINESS-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-server/tests/configuration.rs#operator_updates_serialize_and_state_locks_release_with_the_owner
- web/tests/real/event-freshness.spec.ts#cold native and filesystem views stay idle through real five-minute renewal
- web/tests/real/native-repository.spec.ts#browser mutation previews and publishes one native request
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
gh workflow run ci-heavy.yml --ref main -f ref=f580e7c6970a799619e47c890576ea289f2754eb
gh run view 41651194784 --json jobs,conclusion,url
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125969650046/logs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125969918436/logs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125975732995/logs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125986910254/logs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125986910477/logs
```

**Notes**

Complete qualification run 41651194784 finished 52 jobs. All standard Rust
shards, both application-browser shards, Storybook shards 2 and 3, and the
remaining storage, native, backend and cold lanes passed. Five root jobs exposed
bounded readiness assumptions: a terminal index `Busy`, a Storybook group
lookup timeout, a 200 ms operator-lock retry budget, a cold freshness sample
before lazy reads drained, and a native mutation submitted while initial reads
still held admission. The required and complete-gate failures were aggregates.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-store-server --test configuration --locked -- operator_updates_serialize_and_state_locks_release_with_the_owner --exact
```

**Notes**

The exact operator-lock case passed 1/1 after replacing the scheduler-sensitive
200 one-millisecond attempts with the same bounded five-second deadline used by
the acceptance contract.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

The first command selected the wrong package name and failed before compiling
or running the target. The `jj-store-cli` sources are packaged as `jj-store`;
this was a local harness error, not an acceptance failure.
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
RUNNER_TEMP="$install_dir" GITHUB_PATH="$install_dir/github-path" node scripts/ci-setup-jj.mjs
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_TEST_STOCK_JJ="$install_dir/jj-store-ci-tools/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-store --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

The exact filesystem-runtime case passed 1/1 in 5.69 seconds with
checksum-verified JJ 0.45.1. Index preparation retains one 15-second overall
bound and retries a fresh task only for an explicitly terminal `Busy` outcome.
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
npx --yes pnpm@10.32.1 test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte --reporter=default
```

**Notes**

All five focused spacer stories passed. The interaction now gives the named
group the established ten-second rendered-story budget. Validation report:
`.local/validation/runs/jj-store-validation-4f3f4e13-441b-46b0-a438-53571d501a30`.
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
npx --yes pnpm@10.32.1 web:check
npx --yes pnpm@10.32.1 catalog:check
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- check -p jj-store -p jj-store-server --tests --locked
```

**Notes**

Web and catalog type checks passed with zero diagnostics; catalog ownership and
story parity also passed. The two affected Rust packages and test targets passed
`cargo check`. UI reports:
`.local/validation/runs/jj-store-validation-5da0f32e-d970-4cf2-be15-a86ac00adf63`
and
`.local/validation/runs/jj-store-validation-0b5f5e08-6d71-470d-9f30-2dc31f660f36`.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_REAL_TEST=native-repository.spec.ts npx --yes pnpm@10.32.1 web:test:real
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=native-repository.spec.ts npx --yes pnpm@10.32.1 web:test:real
node scripts/validation.mjs run web:test:real
```

**Notes**

No Playwright case ran. The first attempt found the unconfigured mise Cargo
shim. The pinned retry reached `dev:reset` but an existing built-bundle
lightningcss `:global` minification failure stopped setup. The validation runner
then reported that `web:test:real` is not a registered local lane. Remote PR
cold shards remain the acceptance proof for both cold files.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

The four selected document validators passed. Spec-first then rejected the
Storybook source as a test reference and the two abbreviated browser titles as
missing named tests; progress reported the same evidence-shape findings. The
test list now omits the catalog source, as prior Storybook evidence does, and
uses the two exact Playwright titles.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

The selected document validators and progress check passed. Spec-first accepted
all five protected paths but found that the unchanged-contract confirmation did
not name the CLI family mapped from both Rust test paths. The record now names
JST-CLI-001 for LocalTrusted composition and JST-API-001 for the two browser
test paths, in addition to the already recorded owning requirements.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- fmt --check
npx --yes pnpm@10.32.1 exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte web/tests/real/event-freshness.spec.ts web/tests/real/native-repository.spec.ts spec/src/history/2026/11/03/E-CI-QUALIFICATION-READINESS-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Rust, Svelte, TypeScript and Markdown formatting passed. Four selected document
validators checked 4,603 chapters and 211 requirements, spec-first accepted all
five protected paths through the qualified unchanged-contract confirmation, and
progress accepted 186 tasks and 1,025 evidence records.
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

Node.js 26.8.2; pnpm 10.32.1; JJ 0.45.1; Rust toolchain per `rust-toolchain.toml`;
Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, wire, generated asset, cache or persisted format
changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused CI acceptance readiness across one catalog interaction, two Rust
integration cases and two cold browser cases. The tests observe actual bounded
readiness or retry the explicit recoverable `Busy` result instead of assuming
that elapsed wall-clock time implies readiness. Production behavior is
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the focused acceptance repairs, require the PR's Rust, UI and cold lanes,
then rerun complete integrated qualification on merged `main`.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The existing bounded index, serialized operator, event-driven freshness,
  exact mutation and owned catalog contracts remain unchanged. The repaired
  tests wait for observable readiness within explicit bounds and retain their
  protected assertions.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Focused Rust, catalog and type-check evidence
  passes, while the PR cold gates and complete merged qualification remain the
  next release evidence.
```
<!-- /evidence-field -->
