---
schema: 1
id: E-REMOTE-LAND-001
---

# E-REMOTE-LAND-001

<!-- evidence-field:id markdown -->
## Id

E-REMOTE-LAND-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-SEC-009
- JST-API-011
- JST-UI-022
- JST-GOV-004
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

landing-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

ytspmmsv
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/M27/history/M27/index.md
- spec/src/current/progress/M27/index.md
- spec/src/history/2026/10/12/E-REMOTE-LAND-001/SUMMARY.nav.md
- spec/src/history/2026/10/12/E-REMOTE-LAND-001/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
jj commit -m "Integrate remote agents with main and repair inherited validation cleanup"
```

**Notes**

Verified integration committed as change zkkykspt, commit 88f97b7f, above rebased mulvtstp and sswpvmvq. E-REMOTE-INTEGRATION-001 records exact passing follow-ups and the completed wider 519-pass/4-fail run separately; no full-suite claim.
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
jj rebase -r ytspmmsv -o zkkykspt
```

**Notes**

Default/main remained clean at pklqtuzx before landing. Advance its empty working change onto the verified stack. Clean afterward; original main avatar/profile/emoji work retained.
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
jj workspace forget remote-agents && trash -v /Users/stevejuma/code/lapismd/jj-store-remote-agents
```

**Notes**

Verify no running process owns the temporary checkout. Forget before moving it to macOS Trash. Only default remains registered and original temporary path is absent. JJ history is retained; main development data and resources remain in place.
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
CARGO_INCREMENTAL=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs PATH="/Users/stevejuma/.local/bin:$PATH" pnpm dev:restart
```

**Notes**

User explicitly requested main restart after landing. Fresh main backend/UI/help build and startup succeed at http://127.0.0.1:7460/. Reuse existing embedded runtime and .local/dev-demo. Search ready; 11353 ms cold / 5847 ms restart. No reset/migration or installed service/runtime changes. /tmp/jj-land-main-restart.log.
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
python3 (compare /v1/repositories inventory before and after the main restart)
```

**Notes**

All four prior repository IDs are retained and ready; four repositories discovered after restart. The initial exploratory /v1/repos URL returned 404 and was corrected to the actual discovery route. Inventories stay under /tmp, outside the checkout.
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
node --input-type=module (cold Playwright load and reload of the live main built App)
```

**Notes**

Local server selected, Add server present, no page errors through cold load and reload. Isolated browser context makes no persisted configuration or user-data edits. /tmp/jj-land-main-browser.log.
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
node --test scripts/published-foundations.test.mjs
```

**Notes**

Registry-owned released foundations and absent active local npm overrides rechecked on main; 1 pass.
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
node scripts/check-progress.mjs
node node_modules/@lapismd/spec-validator/dist/cli.js first
node node_modules/@lapismd/spec-validator/dist/cli.js validate
node node_modules/@lapismd/spec-validator/dist/cli.js build
python3 scripts/spec/book-audit.py
```

**Notes**

Landing record gates pass: 152 tasks, 772 evidence records; no production/protected file change; 7 validators, 3683 chapters and 188 requirements; 2679 HTML files and 101087 local references. Changed Markdown formatting passes. The inherited large-search-index warning remains separate. Restore generator-only formatting of unrelated histories; original committed evidence payloads remain unchanged. No Rust/UI test repeat for this documentation-only execution record under targeted policy.
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

macOS ARM64; Node 26.9.0; Rust/Cargo 1.98.1; JJ 0.45.1; released foundations unchanged. Fresh main development binaries/UI and existing embedded RustFS runtime.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository, configuration, credential, manifest, WAL or native-data migration. Existing .local/dev-demo, installation identity and repository IDs are retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Complete user-authorized local landing, temporary-workspace retirement and main development restart. Documentation-only execution record; no new production change. Update M27 review without promoting deferred gates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify installed remote providers, Linux real storage and desktop delivery separately. Development app is available at http://127.0.0.1:7460/.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-SEC-009/API-011/UI-022 retain the verified integration contracts.
  This slice records JJ and development lifecycle operations, preserves existing
  data and registry dependencies, and changes no production behavior.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "M27 remains In progress: installed provider, Linux real-storage and
  desktop delivery gates remain open. The local landing/restart request is
  complete; earlier wider story results remain recorded accurately."

```
<!-- /evidence-field -->

