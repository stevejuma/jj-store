---
schema: 1
id: E-EXEC-001
---

# E-EXEC-001 details 7

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
node --input-type=module (changed authored path inventory) -> pnpm exec prettier --check
```

**Notes**

All selected authored source/documentation paths pass; generated task-history/navigation formatting is separate.
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
node scripts/desktop.mjs stage --acp-executable /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp
```

**Notes**

Final browser/paired JJ/server/RustFS/ACP resources staged without bundle/publication or installed tool changes.
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
pnpm spec:build
```

**Notes**

Generated the complete canonical specification book; large search-index warning is nonfatal.
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
pnpm spec:links
```

**Notes**

2218 generated HTML files and 82605 local file/anchor references verified.
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

macOS ARM64; Rust 1.98.1; JJ 0.45.1; @agentclientprotocol/codex-acp 1.13.1 from existing npx cache; Cursor 2026.10.01-e373342; Node 24.10.0; pnpm 10.32.1. Existing adapter authentication reused without credential inspection. jj-acp source changes rrxpuvru/0ea15ccb, pnmzyuky/95f16504 and mkmlwqzt/5f7e3fe1; all four source check/test/build/spec lanes passed before consumer acceptance. Source M10 consumer documentation is committed as somnxvvl/f4e97542; it changes no executable bytes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

jj-acp control/v2 and terminal record/v1; strict application execution API/v1 and owner-private JSON journal with independent optional setup_revision and bounded sanitized events/intents; OwnedController projection/v1. No repository manifest, native identity, WAL/receipt or local companion-ref format changes. Tool aliases reversibly encode exact scoped ID tuples (json:) or use bounded sha256:; native projected IDs retain their existing derivation.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Enclosing Jujutsu revision zontovyv. Implements named local setups, explicit Agents/repository/project/task launch, verified isolated workspaces at exact selections, negotiated mode/model/effort, streamed text/tools/plans, exact permissions, queued follow-ups/cancel/end, capability-gated load or labelled saved-context continuation, retained review and safe explicit cleanup. Host owns admission/durable requests and sanitized publication; sibling ACP is consumed as an executable only. Existing dirty source files, drafts, historical selections, credentials and publication authority are preserved. Final publication ranges are journaled before sanitized spool admission and original receipts recover without prompt replay. Process ownership shuts down before storage; scanner preparation cannot delay owner EOF.
<!-- /evidence-field -->

<!-- evidence-field:limits markdown -->
## Limits

The real adapters raised zero permission requests: fixture exact-choice/cancellation/deadline proof is separate from live edit acceptance. Visual baselines remain pending; no signed app/DMG/WKWebView distribution, other platforms/adapters, migration, backup/replication, installed-tool replacement, demo reset, publishing or deployment. Root aggregate pnpm check/test/build/spec:check and unrelated Rust/UI suites were out of scope under the targeted policy. Optional inherited clippy diagnostics remain unresolved and are reported separately; they do not become a passing lane. Early local/native attempts failed on publication coverage/tool alias, native socket length/admission headers and browser locator/context; failures were repaired at their owning boundary and the exact completed gates supersede them without altering their original results.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain the qualified local profile; separate visual/package/platform and live permission qualification from this implemented integration.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Added JST-COLLAB-031/032/033 and owning
  API/service/JJ/collaboration/architecture/UI/CLI/desktop/documentation/governance
  chapters. Manifest CAS, companion-ref authority, pinned reads, native
  identities, credential isolation and unknown-outcome no-replay contracts are
  unchanged. Protected implementation is accompanied by this same-change
  evidence; source fixes remain in jj-acp.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M23 Done for the explicitly bounded macOS LocalTrusted Codex/Cursor
  profile after real ordinary/native edits, durable publication, cold
  browser/restart and managed owner lifetime passed;
  release/visual/live-permission/platform exclusions are explicit. Existing
  milestones are not promoted.

```
<!-- /evidence-field -->

