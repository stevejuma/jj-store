---
schema: 1
id: E-M18-003
---

# E-M18-003 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (first, then first extraction)
```

**Notes**

Response validator exceeded300 counted lines at319, then301; extracted root paths and coverage field list to focused helper.
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
pnpm check:architecture (after extraction); pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

869 modules; ownership validator;17 package boundaries.
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
cargo fmt --all; cargo fmt --all -- --check; pnpm exec prettier --write changed UI/tests/specs; pnpm exec prettier --check changed non-Rust files excluding historical milestones ledger
```

**Notes**

Formatting passed without rewriting archived ledger entries.
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
pnpm dev:backend:build
```

**Notes**

Preview CLI/server binaries rebuilt; existing compact-unwind linker warning retained.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:capture:restart
```

**Notes**

Only isolated preview backend restarted; Vite HMR and complete fixture/history preserved; ports7470/7472.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
Python urllib GET http://127.0.0.1:7470/v1/local/capture with X-Jj-Store-Protocol:1; bounded three reads
```

**Notes**

All three proper reads returned existing409 capture_control_busy. Initial exploratory requests missing protocol or complete Fetch Metadata correctly returned403; admission remained intact.
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
CUA tab2: profile > Settings > Agents; inspect AX; save .local/integration/agents-settings-tabs-live.png
```

**Notes**

Live Capture shows /Users/stevejuma/.codex/sessions, four shared tabs and dedicated Agents sidebar. Intermittent busy source read retains available root snapshot and disables commands. User historical repository route retained. No native-app retry or capture mutation.
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
cargo test -p jj-store-server --test capture_management --locked imports::inspection::missing_reader_link_and_unqualified_publication_withhold_completion_without_repair -- --exact (corrected)
```

**Notes**

1 cold test passed; exact original completed receipt required before corrupting reader/publication evidence. Removed now-unused short-settlement helper; final binary verifies no new dead-code warning.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/architecture.md spec/src/spec-governance.md
```

**Notes**

Spec-first30 protected paths/8 chapters before final added test files;5 validators/22 chapters/166 requirements;135 tasks; changed documents/Rust format passed.
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
cargo test -p jj-store-server --test capture_management --locked (final complete binary)
```

**Notes**

14 passed;0 failed. Corrected cold inspection requires original completed receipt; unused setup helper removed; no new dead-code warning.
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
pnpm spec:first (final); evidence-preservation/ledger-structure scripts
```

**Notes**

30 protected files/9 canonical chapters;395 entries including this slice; all394 historical evidence objects retained byte-for-byte;39 unique files.
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
pnpm check:progress (final pass ledger); pnpm exec prettier --check .local/integration/e-m18-003.json
```

**Notes**

135 tasks; new evidence entry formatting passed; historical ledger formatting and objects preserved.
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

JJ0.45.1;Codex CLI0.156.1;pnpm10.32.1; inherited Design Core temporary local validation unchanged
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Current AgentStore only; native/capture persistence formats unchanged. Additive optional bounded configured_roots status metadata;221 actual HTTP/SSE response contracts. All394 earlier evidence objects preserved byte-for-byte.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Four Agents settings tabs with shared rows/action columns, one observation owner, retained drafts and exact pending actions. Actual startup-resolved directory roots under unchanged LocalTrusted Admin admission, including HOME/CODEX_HOME and explicit selections. Complete contract lane required; root aggregates, unrelated UI/Rust suites out of scope. Stories remain visual-pending; no baseline approval. Existing cold sanitizer performance and intermittent capture busy remain deferred; broader M18 harness/release scope unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Try Settings &gt; Agents in the running preview; gather tab/roots feedback before reopening deferred capture performance or harness work.
<!-- /evidence-field -->

