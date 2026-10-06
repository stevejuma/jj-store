---
schema: 1
id: E-M21-042
---

# E-M21-042 details 3

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
pnpm dev:backend:build; JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj-store JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:backend:restart
```

**Notes**

backend-build.log/backend-restart.log: only owned backend replaced; fixture retained and Vite 7476 remains running. No installed JJ or unrelated services changed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build (first two attempts)
```

**Notes**

web-build.log: stale generated API reference; web-build-final.log: stale documentation source reviews. Vite builds passed but full build was not accepted. Generated refs, owning user docs and three topic reviews updated.
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
pnpm docs:generate; pnpm docs:review local (specific ordinary Settings semantics); pnpm docs:review application (specific UI review); pnpm docs:review contracts (specific Update/Delete review)
```

**Notes**

docs-generate.log/docs-review.log: generated API reference, settings user flow/local model/transport pages and coverage fingerprints updated. Application review refreshed after final identity change in docs-review-final.log.
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
pnpm web:build
```

**Notes**

web-build-verified.log and web-build-final-identity.log: Vite and complete docs succeed; nine Mermaid diagrams, 139 files/13,952,340 bytes, valid links/anchors. Existing foundational CSS and large-chunk warnings remain non-failing.
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
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-browser.log and cold-browser-final.log: two fresh disposable real-JJ folders/server processes, one browser test each. Setup/apply/unapply, independent WIP, name and target edit, reload, Delete cancel/confirm, unchanged physical WC/files, TOML removal, companion immutability and axe pass. Temporary servers/folders cleaned up.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check (changed supported sources/spec/docs, excluding historical ledger)
```

**Notes**

format-check.log: five unformatted current files. No historical ledger reformat.
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
pnpm exec prettier --write (five indicated files, then identity sources and coverage); pnpm exec prettier --check (changed supported sources/spec/docs); rustfmt --edition 2024 --check crates/jj-store-jj/src/local_stacks/mod.rs crates/jj-store-jj/src/local_stacks/config.rs crates/jj-store-jj/tests/local_stacks.rs crates/jj-store-cli/tests/local_browser/ordinary_stacks.rs
```

**Notes**

format-write.log, identity-format.log, coverage-format.log, format-verified.log and rustfmt-check.log pass.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

spec-first.log: protected file/chapter mapping passes. spec-validate.log: five validators, 23 chapters, 173 requirements.
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
CUA live isolated demo tab inspection at http://127.0.0.1:7476/
```

**Notes**

12px gutter, zero canvas leading padding, Settings layout, Command View control and Delete action verified. Warm identity observation was stale; explicit reload verifies injected workspace icon. Only Settings opened/closed; live workspace data and user tab untouched.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

Unsupported option in installed JJ; no whitespace acceptance claimed. Changed supported source formatting is verified by Prettier and focused rustfmt.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

progress-final.log: 142 tasks; spec-first-final.log: 35 protected files/eight canonical chapters; spec-validate-final.log: five validators/23 chapters/173 requirements. All 523 historical evidence objects and their raw JSON prefix preserved.
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

<!-- evidence-field:next markdown -->
## Next

Commit this verified local Settings slice; review remaining graph and whole-file/hunk capabilities separately. Preserve the live Vite 7476/backend 7478 fixture.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Nine native engine tests, one real local HTTP test with 48 captured responses, seven contract-tool tests, 71 direct controller units, 19 affected stories and final 15 directly changed stories pass. Two fresh real-folder browser runs verify Settings, reload, confirmed deletion and retained independent unfinished work; checks and built documentation closure pass. M21 remains In progress.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#workspace_settings_and_removal_preserve_dirty_jj_work_and_reload
- crates/jj-store-jj/tests/local_stacks.rs#workspace_settings_reject_invalid_and_stale_intent_without_config_changes
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/production-stack-adapter.test.ts#local state enables only core
  commands and projects named WIP into Unstaged
- web/tests/real/stacks-browser.spec.ts#cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Root aggregate check/test/build/spec:check, full Rust workspace and unrelated UI suites, full registered capture inventory, RustFS qualification, registry publication, visual comparisons/baselines and unqualified graph/file/hunk commands. No milestone promotion.
<!-- /evidence-field -->

