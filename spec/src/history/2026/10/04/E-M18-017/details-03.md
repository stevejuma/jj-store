---
schema: 1
id: E-M18-017
---

# E-M18-017 details 3

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
Cold disposable jj-store restart with OpenCode API offline, then API restart; python3 .local/multi-harness/live-proof.py before/after/outage
```

**Notes**

Real embedded RustFS, not fake store. Both real sessions retain identical IDs, two-message counts, complete activity and native timing through API outage, cold server restart and repeated scans. /tmp/jj-live-capture-before-restart.json, /tmp/jj-live-capture-api-outage.json, /tmp/jj-live-capture-after-restart.json. No duplicate unchanged messages.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Cursor Desktop CUA disposable workspace selection and read-only verification
```

**Notes**

Desktop prompt withheld: automation could not verify exact disposable workspace and later returned noWindowsAvailable. Fixture tests separately exercise both Desktop and CLI; Desktop installed-product cold qualification remains pending. No existing user project prompt was sent.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules cargo test -p jj-store-wal --test agent_catalog --locked opencode_aliases:: -- --test-threads=1
```

**Notes**

4 exact OpenCode alias tests pass,22 filtered. /tmp/jj-opencode-wal-alias-final.log. Original normalizer/adoption/publication semantics retained behind the new pure facade.
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
Production jj agent remove cursor; restore disposable original hook state; stop only owned jj-store/OpenCode/Cursor worker processes
```

**Notes**

First cleanup detected owned-configuration drift. Restored verified trace-only install backup, then production removal passed and original absent hooks restored. No trace handlers or owned Cursor install record remain. Final live snapshot matches all3 prior restart/outage snapshots. Owned server shutdown reported capture still draining while retaining old diagnostic pending intents; managed RustFS exited. User services/previews untouched.
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
Focused formatter (before/after)
```

**Notes**

Existing ledger table formatting and E-M18-016 indentation produced baseline formatter warnings. Corrected E-M18-016 indentation without changing values/statuses; kept unrelated historical table bytes. New evidence formatted separately. All414 historical evidence objects remain identical.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; cargo fmt --all -- --check; changed-document/UI Prettier check
```

**Notes**

Spec-first64 protected files/10 canonical chapters; spec validation5 validators/22 chapters/167 requirements; progress136 tasks. Rust and changed document/UI formatting pass. Initial progress check failed because a table-byte preservation script duplicated historical mapping keys; restored the original pre-evidence prefix except2 authorized task rows. Final progress passes;414 historical evidence objects unchanged. Full-ledger Prettier baseline warnings remain separate from new evidence formatting. Root aggregate lanes not run under targeted policy.
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
jj rebase -s kzkswsvy -o 'default@-'; jj new wwutzuqw -m 'Continue after Cursor and OpenCode capture integration' (default workspace)
```

**Notes**

Main remained clean at committed b11c0486. Explicit rebase skipped3 already-in-place commits; code/tree unchanged and no conflicts. Default workspace advanced cleanly to the verified settings/runtime stack. Parked codex/deferred-opencode-history remained c4d8d275. Only local checkout integration; no publication, installed CLI replacement or user service/preview restart.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live Cursor Desktop3.22.7 disposable capture and cold restart gate; UI
  automation must verify the exact workspace before sending any prompt.
- Codex full-suite surveys contain load-sensitive timeouts; exact reruns pass
  but a complete survey pass is not claimed.
- Visual baseline approval and broad multi-harness release qualification remain
  separate.
- Disposable shutdown retains capture-draining diagnostic from pre-fix pending
  intents; recovery preserves those records, and clean full release shutdown is
  not claimed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Approved opt-in LocalTrusted active capture in exactly linked native repositories, current AgentStore only. Independent fixtures and real CLI/API/embedded-RustFS evidence. M18 In progress and M18.2 Partial retained. Parked OpenCode history bookmark unchanged. Root aggregate lanes, unrelated preview/service restarts, publication, inactive-history import, export/database readers, cloud Cursor, advanced subagents, setup wizard and extra platforms/versions out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Enable the selected harness in Settings &gt; Agents &gt; Configuration, restart the local server, and follow docs/capture-preview.md for pinned Cursor transcript hooks or a running OpenCode API. Complete the remaining Desktop live gate in a verified disposable workspace. Changes are locally integrated; full multi-harness release qualification remains pending.
<!-- /evidence-field -->

