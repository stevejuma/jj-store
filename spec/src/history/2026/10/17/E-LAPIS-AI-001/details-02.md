---
schema: 1
id: E-LAPIS-AI-001
---

# E-LAPIS-AI-001 details 2

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
pnpm dev:cargo -- check -p jj-store-server
```

**Notes**

Shared local client/config integration checks. Cargo.lock updated; storage/model/WAL retain no inference dependencies.
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
pnpm dev:cargo -- test -p jj-store --test agent_execution --locked
```

**Notes**

Before: 20 passed, 2 failed, 2 installed-provider ignores. Old helper lookups refused runtime readiness; retained metadata publication incorrectly required active-snapshot membership.
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
pnpm dev:cargo -- test -p jj-store --test agent_execution metadata_tests::recovery::metadata_stale_generation_and_unknown_publication_recover_without_new_inference --locked --offline -- --exact
```

**Notes**

After: original unknown-publication/stale-title regression passes without new inference. Subsequent suite also proves public active-snapshot reads still refuse the retained source; foreign session IDs remain rejected.
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
pnpm dev:cargo -- test -p jj-store --test agent_execution tooling::runtime_receipts_recover_without_execution_or_automatic_install_replay --locked --offline -- --exact
```

**Notes**

After: exact helper-selection regression passes, preserving receipts and no install/execution replay.
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
pnpm dev:cargo -- test -p jj-store --test agent_execution --locked --offline
```

**Notes**

Final suite: 22 passed, 0 failed, 2 explicitly gated installed-provider tests ignored. Fixtures strictly assert V3; warm/open/session IDs and adapter IDs are distinct.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-server --lib agent_execution --locked
```

**Notes**

The 57 unit tests passed with one inherited ignore, but managed postflight correctly refused the 30 GiB reserve. This command is not claimed as an overall pass.
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
pnpm dev:stop
pnpm dev:clean -- --target target --dry-run
pnpm dev:clean -- --target target --apply
```

**Notes**

User explicitly approved stop, clean, restart. All Cargo jobs were finished before cleanup. Only idle root outputs cleaned; fixture, RustFS resource and retained public-release target preserved. Free space rose from 30.0 to 64.5 GiB.
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
pnpm dev:cargo -- test --manifest-path desktop/Cargo.toml --no-default-features --locked
```

**Notes**

20 passed, one managed-runtime gate ignored. The host uses the shared ResourceBundle verifier. Signed desktop/WKWebView qualification is separate.
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
python3 scripts/local-agent-acp.acceptance.py --acp .local/lapis-ai/lapis-acp --agent .local/lapis-ai/lapis-agent --tools .local/lapis-ai/lapis-agent-tools
```

**Notes**

Real shared ACP/agent/tools with controlled provider: allow and deny both passed. Native compute acceptance separately passed configure, prepare, download, start, inference and approved coding edits. Logs: .local/lapis-ai-migration/real-helper.log and native-compute.log.
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
python3 scripts/lapis-ai-acceptance.py --server target/debug/jj-store --jj target/debug/jj-store-jj --rustfs desktop/resources/jj-store-rustfs --bundle .local/lapis-ai --model .local/lapis-ai-native/managed-models/782855a0d5aef42bea734bc6a8d3e9c8b97c2d0b89ef1d77c337bc2c3b813b63.gguf --web web/dist --browser
```

**Notes**

Cold isolated real-RustFS app and native compute passed prepare/import/configure/start, exact duplicate Start receipt, built-browser Compute settings, Stop, owner-EOF shutdown, restart, retained model index and compute identity. Harness corrections used a short macOS socket path, desktop supervision and actual Settings navigation; initial attempts failed before these corrections.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/ci*.test.mjs scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs scripts/dev-demo*.test.mjs scripts/build-storage.test.mjs scripts/managed-cargo.test.mjs
```

**Notes**

Before complete infrastructure selection: 98 passed, 3 failed because HMR fixtures supplied old standalone jj-acp resources. Complete synthetic Lapis bundles replace those fixtures; no assertion was weakened.
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
node --test scripts/ci*.test.mjs scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs scripts/dev-demo*.test.mjs scripts/build-storage.test.mjs scripts/managed-cargo.test.mjs
```

**Notes**

After: 101 tests passed. Shared CI/package selection, source-pin guards, all launcher consumers, storage leases and bundle-tampering preflight are covered. The stale standard cohort assertion was corrected to its already-existing 20-package manifest.
<!-- /run -->

<!-- run -->
### Run 13

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

Production JS/CSS built; Help stage rejected the stale generated CLI reference. Run docs:generate, review and docs:check corrected the reference before final staging. Existing bundle-size/dynamic-import warnings retained.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:generate
pnpm docs:review injectable-stores "Reviewed retained-source metadata validation: selected inventory preserves historical source snapshots while public active-closure reads remain pinned; published store interfaces and SDK usage are unchanged."
pnpm docs:check
```

**Notes**

Updated helper ownership, local path consumption and V3-only guides, references, README orientation and explicit review fingerprints. Nine Mermaid diagrams render; all 143 documentation assets (26,627,245 bytes), links and anchors validate.
<!-- /run -->

<!-- run -->
### Run 15

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

23-package workspace passes. Only server composition may depend on lapis-ai-runtime and lapis-ai-config; desktop is separately locked. CI allows exactly the two sibling paths, rejecting unexpected external edges.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Three pre-existing presentation size violations: StackWorkspaceApplication story 401/400; StoreApplicationMain 313/300; StoreStackWorkspace 310/300. Audit implementation, baseline and all three files are byte-identical to parent wmzlvsps (verified with jj file show and countedLines). No UI source changed; this inherited audit is outside the migration acceptance target.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:disk
```

**Notes**

After cleanup/rebuild: 23.6 GiB managed, 55.5 GiB available. Obsolete runtime source roots are retired; their ignored cache markers contain no substantial artifacts. No unrelated repository or retained public-release output was cleaned.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-git --locked --offline
```

**Notes**

12 tests passed (2 unit, 10 Git factory integration); doc tests contain no cases. Covers selected vs staged read capability, source admission, all five domains and original receipt replay.
<!-- /run -->
<!-- /evidence-field -->

