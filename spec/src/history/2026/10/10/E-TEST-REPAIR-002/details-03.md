---
schema: 1
id: E-TEST-REPAIR-002
---

# E-TEST-REPAIR-002 details 3

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
pnpm spec:validate
```

**Notes**

Seven validators, 3503 chapters and 181 requirements pass.
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
pnpm check:progress
```

**Notes**

Literal Story names are also unsupported by the named-test definition checker, which recognizes only test(), fn and def declarations. Retain 20 actual controller test references and record all ten named Story cases separately in story_cases. No checker change or test wrapper added.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base 9907dd70 --head mxqyuuzn
```

**Notes**

The same evidence named-story references caused SPEC-FIRST-VCS to refuse the change set (ten missing definitions). Correct evidence representation; no product failure.
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
pnpm check:progress
```

**Notes**

Final current named controller tests and story_cases representation passes; 146 tasks and 737 evidence entries. Removed tests are recorded without altering committed evidence.
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
pnpm spec:first --base 9907dd70 --head mxqyuuzn
```

**Notes**

Five protected catalog files are covered by a justified unchanged-contract record; zero canonical chapter changes required for this test-only migration.
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
pnpm spec:build
```

**Notes**

Book generated. Existing unrelated warnings retained: unclosed execution-id tag in unchanged API chapter 41; large search index about 50 MB.
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
pnpm spec:links
```

**Notes**

Book links/anchors valid: 2546 HTML files, 95984 local references in the initial updated book. Rechecked after final evidence content.
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
pnpm exec prettier --check catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/fixtures/AgentInlineCommentsFixture.svelte catalog/fixtures/agent-inline-comment-play.ts catalog/fixtures/agent-inline-comment-transport.ts spec/src/current/progress/M24/index.md spec/src/current/progress/M24/SUMMARY.index-index/002/index.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/test-repairs-review.md
```

**Notes**

Changed authored catalog/progress files use Prettier style. Preserve all prior history row context and append the new table row contiguously; evidence pages use their structural generator.
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

macOS ARM64; Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Existing local packed Design Core validation runtime; no dependency or runner changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged version-1 comment batches and receipts, immutable agent/tool content references, exact UTF-8 source offsets and quote bytes, expected human-message revisions. Drafts remain session-only. No production, API/schema, native, storage or ownership changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Test-only migration of two reply story families and a dedicated fixture/helper consumed only by them, in test-repairs change mxqyuuzn based on 9907dd70. Tests exercise production StoreAgentConversation, AgentExecution, ConversationSession, shared comment editors, reaction controls, read sidebar and ThreadStore transport. Fresh Vitest browser processes provide cold fixture lifecycle, interaction and Axe checks. Sequential bounded batches avoid shared resource pressure. Main workspace and live app remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M24 In progress. Broader release interaction acceptance, native/provider/desktop/platform qualification, architecture budgets, registry-only dependencies and recorded ResizeObserver diagnostics remain independent.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-UI-001/015/018 and JST-COLLAB-034/035/036 remain unchanged: current
  inline/queue drafts, exact-source comments, explicit comments-only Send,
  revision edits, offline dispatch gating, immutable reactions and receipt
  recovery. Supersede old Reply panels, Save reply, independent offline saves,
  nested-message editing, replay recovery and grouped-tool presentation
  assertions at the user request. New tests assert the existing live behavior
  instead of restoring removed controls. Fixture publication proves UI
  projection, not native durability or adapter compliance."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress and visual baselines remain unapproved.
  E-TEST-REPAIR-002 supersedes the ten obsolete reply cases from
  E-TEST-REPAIR-001; no real adapter, cold native restart, live service or
  release gate is closed. See
  spec/src/current/progress/M24/test-repairs-review.md; enclosing revision
  mxqyuuzn.

```
<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->
## Excluded scope

Production web:check/web:test/build and native/Rust/storage/provider/desktop/live-demo checks are out of scope for this catalog-only slice. Root aggregate and full interaction/release lanes are not run: the new fixture/helper has exactly these two direct-consumer families and does not modify shared runner infrastructure. No server restart/reset, dependency install, publish, landing or workspace retirement.
<!-- /evidence-field -->

<!-- evidence-field:diagnostics markdown -->
## Diagnostics

Fresh bounded runs recheck ResizeObserver and runtime diagnostics. No warning observed so far; the prior recorded warning remains unresolved. No diagnostic guard, accessibility gate or visual baseline is relaxed.
<!-- /evidence-field -->

