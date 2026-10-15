---
schema: 1
id: E-FORGE-STACKS-001
---

# E-FORGE-STACKS-001 details 3

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
pnpm check:boundaries
```

**Notes**

20 package boundaries pass.
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
pnpm check:ui-ownership
```

**Notes**

Owned production source only; no checkout aliases or external source imports.
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
node scripts/web-architecture.mjs
```

**Notes**

All production modules remain within the architecture size boundary.
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
pnpm dev:cargo -- test -p jj-forge --test providers --locked
```

**Notes**

Four provider fixtures pass, including expected-head/protection fields and pending/enqueued async merge receipts remaining distinct from merged.
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
pnpm dev:cargo -- test -p jj-store-jj --lib local_stacks::forge::tests::frozen_local_chain_pushes_only_members_and_restacks_after_squash --locked -- --exact
```

**Notes**

The final real-Git scenario additionally verifies automatic landed-link maintenance leaves the JJ operation and PR base unchanged until explicit Restack.
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

Fresh evidence covers every protected changed file, with explicit M31/M21 milestone review.
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
rustfmt --check --edition 2024 --config skip_children=true crates/jj-forge/src/client.rs crates/jj-forge/src/engine.rs crates/jj-forge/src/journal.rs crates/jj-forge/src/lib.rs crates/jj-forge/src/planner.rs crates/jj-forge/src/providers/checks.rs crates/jj-forge/src/providers/merge.rs crates/jj-forge/src/providers/mod.rs crates/jj-forge/src/providers/pull_requests.rs crates/jj-forge/src/types.rs crates/jj-forge/tests/planner.rs crates/jj-forge/tests/providers.rs crates/jj-forge/tests/submission.rs crates/jj-store-cli/src/stack_commands/args.rs crates/jj-store-cli/src/stack_commands/forge.rs crates/jj-store-cli/src/stack_commands.rs crates/jj-store-cli/tests/forge_cli.rs crates/jj-store-http/src/local_browser/forge.rs crates/jj-store-http/src/local_browser.rs crates/jj-store-http/tests/forge_admission.rs crates/jj-store-jj/src/local_stacks/forge/restack.rs crates/jj-store-jj/src/local_stacks/forge/templates.rs crates/jj-store-jj/src/local_stacks/forge/tests.rs crates/jj-store-jj/src/local_stacks/forge.rs crates/jj-store-jj/src/local_stacks/graph.rs crates/jj-store-jj/src/local_stacks/mod.rs crates/jj-store-jj/src/local_stacks/operation_description.rs crates/jj-store-jj/src/local_stacks/preview.rs crates/jj-store-service/src/lib.rs
```

**Notes**

Formatting checked only changed Rust files. Prettier checked/formatted changed supported source/docs; generated evidence remains in its canonical writer format.
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
pnpm spec:first
```

**Notes**

Owning chapters and change-map coverage pass after evidence refresh.
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
pnpm exec spec-validator validate --only fileLimits,verification,governance,summary
```

**Notes**

Requirement definitions, verification rows, references, navigation and physical page limits pass. The initial chapter needed its Requirements heading and the governance page was shortened to fit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Credentialed disposable GitHub/GitLab/Forgejo acceptance, including real
  forks, CI control permissions, draft conversion, protection rules, unknown
  outcomes and rate limits.
- Live GitHub native stack reshape, async prefix/queue completion and expired
  async-result observation.
- Cold packaged application connection-to-Restack flow, supported-platform
  qualification and native-storage submission.
- Shared-root/cross-combination Restack, journal archival/migration, additional
  forge adapters and always-on maintenance are future scope.
- Visual review remains separate; wider suites and test fixes are out of scope
  per the user.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ/jj-lib 0.45.1; Git 2.55.0; rustc 1.99.0; Node v26.9.0; pnpm 10.32.1; released Design Core 0.3.2. Reference source revisions are in comparison-and-rollout.md.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local forge request/response version 1 and private device journal version 1, outside checkouts. Atomic temporary-file/fsync persistence plus exclusive locking; 8 MiB/128 submission/128 Restack/1,024 action bounds fail explicitly. No token storage. Native object, manifest, WAL, companion and standard JJ formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Wider root tests, builds, Storybook, Rust workspace and contracts:check fixtures were not run under the user scope. Build/check wrappers compiled only required dependencies. Managed disk preflight cleaned eligible idle build artifacts; original source and services were untouched. Initial validator invocation used file-limits instead of fileLimits, then was corrected. The final ownership gate also required the owning jj-integration chapter for the adapter manifest; its ordinary-local chapter now records the new dependency boundary. One spec-first attempt detected concurrent formatting and correctly refused a moving revision; the final check runs after all edits stop. Spec-first initially requested fresh evidence for the newly added docs, then was refreshed. Ordinary Git-backed JJ: shared CLI/app submission, ancestor bookmark segments and exact pushes; authored editing and managed navigation across landing; fork cumulative mode; CI status/jobs/logs and supported controls; guarded portable and optional native GitHub merging; explicit native Restack. See comparison-and-rollout.md. Status maintenance runs in active controllers/explicit CLI status, not a background daemon.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run the credentialed disposable-forge matrix, then cold packaged application flow. Do not infer external provider or native-storage qualification from fixtures.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-FORGE-001 through 007 introduce owned external-effect contracts.
  Updated owning architecture, API, local-JJ, CLI, mutation, UI and
  compatibility chapters in the same change.
  JST-LOCAL-005/JST-CLI-001/JST-UI-020 are extended only at this explicit forge
  boundary; storage and publication authority are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: "M31 remains In progress: implementation and focused local acceptance
  are recorded, while live provider, platform/native and packaged acceptance
  remain open. M21 remains In progress with no gate promoted. The user
  explicitly defers wider test-suite changes/fixes."

```
<!-- /evidence-field -->

