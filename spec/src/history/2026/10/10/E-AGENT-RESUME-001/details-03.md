---
schema: 1
id: E-AGENT-RESUME-001
---

# E-AGENT-RESUME-001 details 3

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
Targeted -t "Send prepares an ended" story invocation
```

**Notes**

All six stories skipped because generated IDs are CamelCase; this invocation is not acceptance.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <all 12 AgentExecutionFixture consumer story files>
```

**Notes**

52 pass, including automatic Send; ten existing AgentDiscussion/AgentReplyIndependence failures match E-AGENT-STATE-001 unchanged-parent reproduction. No baseline or assertion was loosened.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Seven unchanged oversized files; same seven reproduced by run(&lt;ignored parent snapshot&gt;). New/changed modules add no over-budget file.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
Read-only parent 5439cc97 snapshot: web-architecture.run(snapshot)
```

**Notes**

Same seven pre-existing oversizes.
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
jj-acp: cargo test -p jj-acp-control --test interactive_wire context_only_prompt_requires_nonempty_prepared_content --locked -- --exact
```

**Notes**

Current source supports context-only input.
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
jj-acp: cargo build -p jj-acp-cli --bin jj-acp --locked; later pnpm build
```

**Notes**

Only explicit development resources refreshed. Full source lanes and the rejected-load regression are E-M12-004.
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
Local no-session parser probe against old and rebuilt helper
```

**Notes**

Old helper rejected a 4116-byte context frame as invalid_request. Current helper returned correlated session_not_open, proving parsing without launching an agent.
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
pnpm dev:backend:build; pnpm dev:backend:restart; pnpm dev:ui
```

**Notes**

Default workspace; literal loopback 7460/7462; existing fixture retained. New helper and resource checksum staged, no installed binary replaced.
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
Cold in-app browser live Send: Can we add an MIT license to the repository.
```

**Notes**

No manual Resume/Continue used. Exact prompt request 2481f6b2b8cd46fcb747e5860b281a55 completed. Published 435/435, one LICENSE (+21 lines) and idle connected session. Four pre-existing keyed rows retained backend DOM IDs during preparation, streaming and publication. Opening-to-configured 854ms; prompt-to-started 1499ms; started-to-record 66.742s. Earlier unresolved prompts were not replayed.
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
cargo fmt --all; targeted Prettier formatting
```

**Notes**

No baseline updates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Four pre-existing Node-test failures, ten existing reply-story failures, seven
  architecture oversizes and five jj-collab Clippy diagnostics remain separately
  owned.
- Independent adapter/platform, real permission/cancellation and visual-baseline
  gates remain open. Root aggregate/release lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; Node 26.9.0; pnpm 10.32.1; Codex ACP 1.13.1; control v3 helper from enclosing jj-acp change mtvyswyw. jj-store enclosing change vlpumryo.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

API/journal/control DTOs unchanged; version-3 context parsing verified; unchanged immutable AgentStore and repository publication authority.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Live session 2aa3e7928c42ac146537751042e9f19b used metadata home 80d8731bbc255ca6373c797e8b9eb4b4 and code repository 4e22c93622772144053914c9c20f9af0 (demo/server-only). Old staged helper hash e04a7f93cba9dbbb4875a0c219c06b688772c756f8cee0b6db729abc795f2b0d; final staged helper 1469f79a75302fa9e06421736ab309e6886c1d2131cf7879c47c95bccffeac9e. Read-only adapter audit confirmed -32603 and no-rollout classification for both load and resume before any acknowledged turn. Raw adapter error text/credentials were not published. Old Resume request 3449f84763c04ac980143ba349fd9780 failed; saved-context request c3e000a28cc44ba2ace4e057435fdca0 opened/configured. The stale helper then stopped prompt 522c5863157842c694f07d536db9882b before start acknowledgement. Initial browser capture showed existing LI message removals and after=0 reload; the repaired cold turn retained all original backend node identities. Screenshot is ignored .local/agent-resume-completed.png. Evidence scaffolding regenerated unrelated navigation; that churn was reverted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain automatic preparation and history-retention regressions; migrate separately recorded legacy reply/Node fixtures and qualify independent M24 gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Update JST-COLLAB-034/035/036 and JST-UI-015/018 preparation/row
  retention behavior. JST-API-001/006, JST-CLI-004 and JST-GOV-002/008 wire,
  custody, authorization and ownership remain unchanged. JST-DOC-001/002/003
  manual ownership, source review and delivery remain unchanged; updated guidance
  and source fingerprints describe the final behavior.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This real Codex run qualifies this default demo
  scenario; it does not close other adapter/platform, permission/cancellation or
  baseline gates.

```
<!-- /evidence-field -->
