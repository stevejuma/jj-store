---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001 details 5

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
rustc --edition=2024 .local/prime-privacy.rs --extern jj_store_capture=target/debug/deps/libjj_store_capture-f8804c2a9b8c33d9.rlib -L dependency=target/debug/deps -L native=target/debug/build/vectorscan-rs-sys-e6da6281934ad82f/out/lib -L native=target/debug/build/vectorscan-rs-sys-e6da6281934ad82f/out/lib64 -C debuginfo=0 -o .local/prime-privacy
```

**Notes**

Ignored development cache primer linked the exact current production capture/scanner libraries. It calls TranscriptSanitizer::new with distinct private roots and a cache outside the checkout, with no filtered rules or fixture sanitizer. One full current production rules cache prepared; header retains 485 rules and VectorScan 5.4.12 2026-10-07.
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
python3 - <<'PY'
import subprocess,tempfile
from pathlib import Path
root = Path(tempfile.mkdtemp(prefix='jj-store-current-privacy-')).resolve()
Path('.local/public-cache-root').write_text(str(root))
subprocess.run(['.local/prime-privacy',str(root)],check=True)
PY
```

**Notes**

Full current production privacy rule initialization completed once. The public cache is retained for the isolated preview and unchanged CLI integration, outside repository authority and without credentials.
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
python3 - <<'PY'
import os,subprocess
from pathlib import Path
root=Path('.local/public-cache-root').read_text()
environment=dict(os.environ,JJ_STORE_AGENT_PRIVACY_CACHE=str(Path(root)/'rules'))
result=subprocess.run(['cargo','test','-p','jj-store-cli','--test','agent_execution','--locked'],env=environment)
raise SystemExit(result.returncode)
PY
```

**Notes**

All 6 runnable CLI integration cases passed in 224.25 seconds; 2 explicit authenticated installed vendor-agent cases ignored. Parallel assertions and existing readiness deadlines unchanged. Includes real JJ isolated binding reuse, restart without replay, publication, permissions, setup/catalog and fleet lifecycle. Linker compact-unwind warning retained; one debug-strip step reported transient host disk exhaustion but executable linking and all actual integration assertions passed.
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
pnpm exec vite build --config .local/context-probe.vite.ts
```

**Notes**

Built the actual owned LocalComputeFixture(context=true) and production Sent context components in an ignored standalone UI probe; 14.45-second static build. No copied implementation, source edits, aliases or external ownership substitutions.
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
python3 -m http.server 7486 --bind 127.0.0.1 --directory .local/context-probe-dist; cua keyboard: Return on Sent context, Space on Raw context payload, Return to collapse
```

**Notes**

Cold actual browser-native keyboard input expanded the rich captured repository/file/task content, expanded raw payload with exact version 9007199254740993, and collapsed the disclosure. Accessibility state verified after each input. Temporary tab and HTTP server closed. Standalone component semantic acceptance is distinct from the built application cold visit.
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
pnpm exec storybook dev --host 127.0.0.1 --port 7486 --no-open --disable-telemetry --ci
```

**Notes**

Manual preview stayed in dependency optimization and did not serve the selected iframe during host load; terminated only the owned temporary preview. The 24 cold Vitest story cases passed separately, and actual native keyboard verification used the compiled owned component fixture instead.
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
python3 .local/start-compute-preview.py
```

**Notes**

The first fixture launch refused pre-seeded public rule files in an otherwise uninitialized runtime directory. Moved only those public cache files outside that directory; no fixture.json, installation, repository or user state existed. Retry keeps privacy cache outside the checkout and seeds capture cache only after native initialization. No dev:reset used.
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

Final spec-first check: 181 protected files and 10 owning canonical chapters passed.
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
pnpm spec:validate
```

**Notes**

Final specification check: 7 validators, 3165 chapters and 181 requirements passed.
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
pnpm check:progress
```

**Notes**

Revision-pinned checker correctly rejected concurrent evidence/format restoration during an earlier run. Final ledger check is rerun after documentation settles.
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
python3 scripts/local-agent-acp.acceptance.py --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --agent runtime/local-agent/target/debug/jj-store-local-agent --tools runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

2 cold subprocess cases, allow and deny; actual local ACP agent, rebuilt ACP and owned coding helper; fixture LLM distinct from native model.
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
cargo test --manifest-path runtime/compute/Cargo.toml --locked paired_inference_requires_mutual_admission_and_sharing_off_preserves_local_inference
```

**Notes**

Real encrypted two-peer Iroh transport, fixture inference service. Mutual admission, sharing-off connection revocation and continued local inference.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --offline agent_resources
```

**Notes**

Bundled checksum/tamper/missing-helper test passed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Fresh workspace resource staging directory was absent. Created the ignored directory for source checking; no packaged runtime qualification inferred.
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
pnpm catalog:check
```

**Notes**

Fixture API-client import violated catalog ownership. Used owning ComputeValue and AgentSettingsValue types plus released lossless-json.
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
pnpm catalog:check
```

**Notes**

Svelte 0 errors / 0 warnings; 284 catalog modules passed boundary checks.
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
pnpm web:check
```

**Notes**

Final generated-contract aware Svelte check: 0 errors and 0 warnings.
<!-- /run -->
<!-- /evidence-field -->

