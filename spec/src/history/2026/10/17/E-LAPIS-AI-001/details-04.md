---
schema: 1
id: E-LAPIS-AI-001
---

# E-LAPIS-AI-001 details 4

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
pnpm docs:review configuration "Reviewed local Lapis AI executable and verified bundle selection; removing an identity PathBuf conversion preserves configuration, admission and lifecycle behavior."
JJ_STORE_DOC_REFERENCE=/Users/stevejuma/code/lapismd/jj-store/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Final configuration source fingerprint and complete manual build/asset/link closure pass after the compiler-only cleanup. Repo guide now documents actual Lapis bundle preparation and V3-only behavior. Retired source ignore patterns removed.
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
pnpm spec:first
pnpm spec:validate
pnpm check:boundaries
```

**Notes**

Final stable revision checks pass: 162 tasks/857 evidence records; mapped spec-first141 protected paths/14 canonical chapters; 7 validators/4,131 chapters/206 requirements; package boundaries23 packages. Prior revision-pinned attempt correctly refused a concurrent evidence edit; reran with stable source, without weakening the gate.
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

macOS ARM64; JJ 0.45.1 native identity profile; Rust 1.99.0; Node v24.10.0; jj-store pnpm 10.32.1. Local Lapis AI 0.1.1 at source change zpmzxpuz, signed SHA a93e795f551df4e81c6a87a96e0ae68ec49365b4. Lapis source clients and complete upstream checks/tests/build/spec passed independently (977 Rust tests, 44 governance, 27 renderer). No publication performed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Live Control V3 only; Lapis runtime-manifest.toml version 1, native ABI 0.1.44 / Mesh 0.76.0-rc9. Existing AgentStore/ThreadStore/Git companion/WAL/manifest identities and receipts unchanged. Historical acp_controller_v2() and jj-acp-control-2/v1 remain canonical normalization names, not live IPC compatibility. Existing compute settings, identity, model index, peers and cache paths retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace duplicated native transports and runtime roots with locally linked Lapis AI clients/config and the four-helper bundle. Server keeps authorization, prepared launches, settings, secrets and durable publication. Desktop delegates bundle verification. Development stages local sources; CI materializes the recorded immutable source revision. Later crates.io consumption removes path selectors. Full direct infrastructure selection is justified because shared build/resource/launcher code affects independent consumers. Root aggregate pnpm check/test/build/spec:check, unrelated UI/Storybook suites, full Rust workspace tests, release/platform qualification and visual baseline updates are out of scope and were not run in jj-store; upstream source validation is recorded separately. Runtime Rust validation remains scoped to affected packages and direct agent/remote/workflow consumers. Storage-adjacent recovery validates exact retained source membership through selected inventory and existing closure/evidence checks, without widening public read capabilities.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified local integration with Jujutsu. Installed-provider, signed macOS/WKWebView/Linux CPU and release gates remain separate milestone work; crates.io consumption follows publication.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Control V3 is now mandatory for every launch and discovery path, and
  generic native clients/helpers move to Lapis AI. Owning
  architecture/collaboration/compatibility/desktop/CLI/documentation/governance
  topics updated in the same change. JST-LOCAL-001/002 committed selection,
  native identity, authorization and no-prompt-replay contracts are preserved.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M13 remains In progress; M23 retains historical completed qualification;
  M24/M25 remain In progress with installed-adapter/platform gates open; M30
  remains In progress with full release qualification required; M32 retains
  completed embedding qualification and unchanged public store contracts. This
  local integration changes no publication, destructive GC, automatic promotion
  or distribution gate.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

kkoklxpw
<!-- /evidence-field -->

