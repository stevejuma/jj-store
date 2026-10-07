---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001 details 6

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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Final closure initially used pass while retaining independent platform gaps. The evidence schema correctly requires pending until every stated gap is closed. Restored pending; all qualified source/helper/browser runs remain recorded as passes and M25 stays In progress.
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
pnpm exec prettier --check web/src/lib/ui/store-settings/StoreSettingsPage.svelte catalog/components/store-application/LocalCompute.stories.svelte spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/progress/M25/index.md docs/maintenance/coverage.json
```

**Notes**

Progress generation rewrote milestone table spacing; JSON review writer emitted expanded source arrays. Format the two generated/reviewed documents after generation. No semantic source or fingerprint values change.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

New Runtimes default and initially collapsed Harnesses made legacy fixture navigation stale. Updated explicit panel selection and disclosure interactions.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

One fixture helper still failed to select Capture before asserting rows; fixed the helper.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

22 cold interaction/accessibility cases passed. Initial Codex/Cursor/OpenCode collapse verified with draft/recovery assertions retained.
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
pnpm test:contracts
```

**Notes**

7 contract Node/type cases passed.
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
pnpm spec:first
```

**Notes**

171 protected files / 10 canonical chapters passed spec-first coverage.
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

7 validators, 3163 chapters and 181 requirements passed.
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
pnpm docs:check
```

**Notes**

Final strict documentation build/closure passed: 9 offline Mermaid diagrams, 141 files / 17,711,744 bytes, links and anchors valid. Generated references and source fingerprints current.
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
pnpm web:build
```

**Notes**

Final production bundle after the peer-controls and Capture-play extraction passed: 5,256 modules, browser manifest and strict help staging. Retained existing Design Core CSS global-selector, large-chunk and Tauri static/dynamic import warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

Final cold selected rerun passed all 24 interaction/accessibility stories after extraction. Earlier watcher result was not recoverable after continuation, so this explicit rerun supplies the final result.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_AGENT_PRIVACY_CACHE=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/test-privacy-cache cargo test -p jj-store-cli --test agent_execution --locked
```

**Notes**

1 passed, 5 readiness deadline failures, 2 intentionally ignored installed authenticated adapter cases. The manually supplied cache path was inside the checkout without the development-state override, and the unchanged path policy refused it. Correct runner provisioning only; no production policy or deadline changes.
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
python3 - <<'PY'
import os, shutil, subprocess, tempfile
from pathlib import Path
with tempfile.TemporaryDirectory(prefix='jj-store-compute-privacy-') as temporary:
    cache = Path(temporary) / 'rules'
    shutil.copytree('.local/test-privacy-cache', cache)
    environment = dict(os.environ, JJ_STORE_AGENT_PRIVACY_CACHE=str(cache))
    result = subprocess.run(['cargo','test','-p','jj-store-cli','--test','agent_execution','--locked'],env=environment)
    raise SystemExit(result.returncode)
PY
```

**Notes**

Cancelled the owned test process after sampling proved five simultaneous full rule compilations. The copied public cache header belonged to VectorScan 5.4.12 2026-09-28; this build is 5.4.12 2026-10-07 and correctly refuses that cache. Prepare a matching cache once outside the checkout, then rerun unchanged assertions/deadlines.
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
node scripts/desktop.mjs stage --acp-executable /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --native-runtime-dir .local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal
```

**Notes**

Actual paired resources staged with own server/JJ/UI and pinned ACP/storage/native resources. Four helpers and 27 regular native files independently SHA-256 verified against agent-resources.json. 212 browser assets and strict help closure included; weights remain separate.
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
python3 scripts/local-compute.acceptance.py --compute desktop/resources/jj-store-compute --root .local/compute-acceptance --native-runtime desktop/resources/native-runtime --acp desktop/resources/jj-acp --agent desktop/resources/jj-store-local-agent --tools desktop/resources/jj-store-agent-tools
```

**Notes**

Cold staged-helper acceptance passed explicit configure/prepare/download/start, bundled native runtime with empty runtime cache, real Qwen3 inference, permission-before-edit ACP coding with exact content and graceful shutdown. Separate from source-helper and fixture-provider results.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture && pnpm check:ui-ownership && pnpm web:check
```

**Notes**

Final peer-control extraction closure: 1,323 modules, UI ownership passed; web Svelte 0 errors / 0 warnings.
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
pnpm catalog:check
```

**Notes**

Final Capture-play extraction closure: Svelte 0 errors / 0 warnings; 286 modules passed catalog boundaries.
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
cargo check -p jj-store-server --locked
```

**Notes**

Owning service package check passed after all source changes.
<!-- /run -->
<!-- /evidence-field -->

