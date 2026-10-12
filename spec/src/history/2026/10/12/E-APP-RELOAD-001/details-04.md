---
schema: 1
id: E-APP-RELOAD-001
---

# Runtime and browser qualification

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
JJ_STORE_AGENT_PRIVACY_CACHE="$(cat .local/app-reload-test-cache-path)" JJ_STORE_TRANSCRIPT_CACHE="$(cat .local/app-reload-test-cache-path)" python3 scripts/runtime-reload-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist
```

**Notes**

Cold built App on two disposable macOS real-RustFS instances: saved service activation with unchanged PID/listeners/catalog, retained unsaved draft, deep-link reload, keyboard, light/dark/mobile, Axe/CSP; Capture first enable/drain/disable/re-enable preserves generation/pause. Sync replacement preserves same-peer checkpoint observations and pause. Initial final and unit-aware runs pass; the normalized-label bundle also passes with 0 MiB / 64 MiB, unchanged canonical slider values/bounds and Save/Cancel disabled. Temporary instances/roots are cleaned up. .local/app-reload-complete-cold.log and normalized-cold.log.
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
python3 scripts/remote-agent-administration-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist --acp desktop/resources/jj-acp --privacy-cache "$(cat .local/app-reload-test-cache-path)" --tooling-bin .local/dev-demo/local/agent-execution/tooling/ffdb4d8db47046d0a6e3a350490fb435/bin
```

**Notes**

Installed Codex through the authenticated gateway on two disposable real-storage servers: sharing activates without replacing the server process, one isolated file edit/publication completes, normal restart retains the original prompt receipt without replay, reload revocation fences retained session/tool/publication access. Cold App includes independent role controls, identity/Write onboarding, sharing drafts, reload/history, mobile/Axe and CSP. .local/app-reload-provider-final.log passes before the per-object index correction. Final installed-provider acceptance is rerun against the corrected executables and normalized-label bundle, and passes again in .local/app-reload-provider-complete.log. This closes that artifact gap; independent platform/provider gates remain unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreNumericSetting.stories.svelte
```

**Notes**

Unit-label before state: 10 existing stories passed and 2 new regressions failed because endpoints remained base units. After selected-unit formatting, a 41-story run had 2 harness pointer-transition failures; awaiting interactive dropdown state fixes them. A case-sensitive test-name attempt skipped all 12 tests and is not acceptance. Normalized-label before state again fails the 2 endpoint regressions; final 55-story slice passes. .local/app-reload-slider-unit-labels-before-2.log, slider-label-stories-final.log, slider-normalization-before.log and normalized-stories.log.
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
JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/.local/app-reload-runtime/jj-store pnpm dev:backend:restart
```

**Notes**

Development backend refreshed from explicitly copied development binaries to install Rust implementation fixes. Vite PID 26391 and .local/dev-demo retained. Four repository IDs, agent IDs and original session IDs compare equal across saved before/after observations. Main configuration is never saved or reloaded as an acceptance mutation target. This development binary upgrade is distinct from configuration reload, whose acceptance keeps process/listener identities. Final index-corrected binary refresh also retains those exact identities and shows no pending reload. .local/app-reload-main-index-upgrade.log and main-index-before/after.json.
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
CUA: inspect Agents / Configuration; change only display units; inspect sliders and Save/Cancel; save screenshot
```

**Notes**

The running main App shows 0 MiB / 64 MiB after user-requested normalization; exact-value helper remains 67,108,864 bytes, slider canonical value remains unchanged and Save/Cancel stay disabled. Unit switching also changes time endpoints. Top-header menu is vertical, version follows Help. Screenshot .local/runtime-reload-review/capture-slider-unit-labels.png is review proof only; no visual baseline approval. The huge first slider exhausted two browser tabs before the bounded step correction; a fresh working tab confirms the repaired UI.
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
python3 - <<'CLEANUP'
from pathlib import Path
import shutil
p=Path(Path('.local/app-reload-test-cache-path').read_text().strip())
assert p.parent==Path('/tmp') and p.name.startswith('jj-store-reload-rules-')
assert p.is_dir() and not p.is_symlink()
files=[f for f in p.rglob('*') if f.is_file()]
assert files and all(f.suffix=='.vscdb' for f in files)
shutil.rmtree(p)
CLEANUP
ps -ax -o pid=,command= | rg '/tmp/(jreload-|jraa-)'
```

**Notes**

The disposable compiled-rules cache is removed after all qualification finishes.
No isolated acceptance processes remain; both harnesses report root/process
cleanup. Ignored final development binaries remain because the main backend
uses them. Vite and .local/dev-demo are preserved; no reset, migration, package
publication, installed-service change or visual-baseline update occurs.
<!-- /run -->

<!-- /evidence-field -->
