---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001 details 4

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
pnpm exec prettier --write scripts/agent-runtime-resources.mjs scripts/desktop-resources.mjs && node --test scripts/docs.test.mjs
```

**Notes**

8 documentation/staging closure tests passed after optional copy-on-write executable/native-resource staging. Unsupported filesystems retain the standard copy fallback. Content checksums and independent regular resource files retain the same contract. Under disk pressure, removed obsolete archives/executables only from this workspace’s ignored targets; active incremental caches, model/fixture data and unrelated services were preserved.
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
cargo clippy --manifest-path runtime/compute/Cargo.toml --locked --all-targets -- -D warnings && cargo fmt --manifest-path runtime/compute/Cargo.toml --check && cargo test --manifest-path runtime/compute/Cargo.toml --locked paired_inference_requires_mutual_admission_and_sharing_off_preserves_local_inference
```

**Notes**

Final all-target compute clippy and formatting passed. Real encrypted QUIC mutual admission/sharing-off regression passed in 0.70 seconds; fixture inference source, separate from real native model evidence. Linker warning retained.
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
cargo fmt --all --check && cargo fmt --manifest-path runtime/local-agent/Cargo.toml --all --check && cargo fmt --manifest-path desktop/Cargo.toml --all --check
```

**Notes**

Final source Rust formatting passed across the root, owned local-agent and desktop workspaces.
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
python3 scripts/local-agent-tools.acceptance.py runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

5 cold helper cases from the preceding implementation continuation: edit/search, traversal/symlink/metadata limits, shell deadline/output, EOF descendants and frame bounds.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --test permission_boundary --locked
```

**Notes**

9 vendored-agent permission tests in preceding continuation; fixture LLM/MCP, distinct from native inference.
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
python3 scripts/local-agent-acp.acceptance.py --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --agent runtime/local-agent/target/debug/jj-store-local-agent --tools runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

Cold integration exposed missing ACP mode config options in the maintained local-agent fork. Fixed owning advertisement and selection method; exact rerun pending.
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
cargo check -p jj-store-server --locked
```

**Notes**

Build started before saved-setup catalog trait was added, so HTTP compiled against an older dependency snapshot. Rerun after lock resolution; no contract workaround.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 58
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Generated contracts had not yet caught up with compute models, Keychain capability and live inference host. Regeneration and exact rerun required.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Exact Compute heading regression repaired; both cold stories passed after the branch fix. Rich captured context and lifecycle assertions retained; no assertion weakening.
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
pnpm web:check && pnpm catalog:check
```

**Notes**

Final metadata-fix closure: web and catalog Svelte 0 errors / 0 warnings; 286 catalog modules passed ownership boundaries.
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
pnpm web:build
```

**Notes**

Final bundle after the Compute heading/icon correction passed, including strict docs/help closure: 141 files, 17,711,744 bytes. Existing bundler/dependency/linker warnings retained. Built application pins assets at backend startup; a browser reload alone still showed the prior heading until the planned own-backend restart.
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
node --input-type=module - <<'JS'
import {cp,rename,rm} from 'node:fs/promises';
import {constants} from 'node:fs';
await cp('web/dist','desktop/resources/web-next',{recursive:true,mode:constants.COPYFILE_FICLONE});
await rename('desktop/resources/web','desktop/resources/web-previous');
await rename('desktop/resources/web-next','desktop/resources/web');
await rm('desktop/resources/web-previous',{recursive:true});
console.log('Final browser assets staged; verified helper/native resources retained');
JS
```

**Notes**

Final UI/help resources staged with a directory swap after bundle closure. Previously checksum-verified and native-tested helper/native files were retained without replacing the active storage helper.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cua cold preview: Agents → Settings → Agents (Runtimes/Advanced/Configuration) → Compute
```

**Notes**

Actual built application against the fresh 7480 backend showed Conversations, Preferences, agent defaults and Advanced environment controls, initially collapsed Codex/Cursor/OpenCode Harnesses, and real 16-GiB/10-thread hardware, GGUF controls, peer pairing and sharing-off state. A temporary 1351x988 desktop viewport was needed to reach the desktop navigation; reset after verification. The old Search heading found here is separately fixed and rechecked after final backend startup.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte -t 'Explicit preparation download start and stop'
```

**Notes**

Generated Storybook test names did not match the human-label filter, so both cases were skipped. This is not regression evidence; immediately ran the selected file without a name filter.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Exact newly added heading assertion reproduced the cold application defect: 1 failed / 1 passed. Compute content inherited the Search title/icon fallback from shared settings page metadata. Added only the Compute title/icon branch, matching navigation; other section metadata remains unchanged.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/start-compute-preview.py
```

**Notes**

After cache provisioning was corrected, the normal development launcher reached initialization but timed out waiting for its temporary backend under host load. Preserved initialized installation/storage state. The storage child reported a broken stdout pipe after parent shutdown; retained in the private preview log. No data/receipt reset or relaxed production deadlines.
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
python3 .local/recover-compute-preview.py
```

**Notes**

Started the same own backend/configuration directly with an independently bounded manual readiness wait; literal loopback 7480 served the final built app. Existing storage and installation state retained. This is manual preview startup acceptance, not a claimed dev:restart pass or signed/WKWebView distribution qualification.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/complete-compute-fixture.mjs
```

**Notes**

Native repository fixture was created through JJ, but offline repository registration correctly refused the live server state/policy lock. Stopped only the owned backend, registered the already-created stable repository ID through the normal CLI, then restarted it. No duplicate repository creation or lock bypass.
<!-- /run -->
<!-- /evidence-field -->

