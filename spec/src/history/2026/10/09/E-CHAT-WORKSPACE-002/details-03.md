---
schema: 1
id: E-CHAT-WORKSPACE-002
---

# E-CHAT-WORKSPACE-002 details 3

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
cargo test --locked -p jj-store-service --test local_review --test local_collab --test conversion --test conversion_agent_discussion
```

**Notes**

29 tests pass across direct retained-evidence consumers. Final local review run includes repository binding checks before read, receipt and mutation. Conversion behavior remains refused for unsupported code carriers.
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
pnpm contracts:generate
```

**Notes**

Generated additive local command/portable anchor schemas.
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
pnpm evidence:new -- --id E-CHAT-WORKSPACE-002 --task M24 --task M13.3 --requirement JST-LOCAL-002 --requirement JST-LOCAL-003 --requirement JST-COLLAB-001
```

**Notes**

Created fresh evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Session Send source validation, repository-qualified submissions/context/reply
  links, inspector UI and isolated cold browser acceptance remain subsequent
  slices.
- Turn boundary evidence retention beyond current operation lifetime remains to
  be connected; this provider slice alone does not close Last turn cleanup
  acceptance.
- Portable agent review publication and native portable-carrier/conversion
  qualification remain out of scope.
- ResizeObserver warnings are not yet rechecked; visual baselines remain
  unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; cargo 1.98.1; JJ 0.45.1. Enclosing Jujutsu change is the revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive portable_code_text/portable_code_artifact review targets reuse portable V2 selectors. Existing target bytes and hashes unchanged. Local descriptor optional evidence inventory omitted when empty; native manifest and canonical stores unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Retained local file-review provider and exact source reads. Complete Git capsule closure is selected with reviews through the existing companion CAS and receipt. Native code anchors remain supported through their existing native provider; native portable carriers remain refused. No changes to main workspace, live fixture or services.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish consumer checks, then wire frozen file submissions and repository-qualified answer links before building the shared workspace footer and inspector.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning collaboration/local publication contracts now define portable
  file evidence, exact bounded reads and inventory audits. Native carrier
  refusals, conversion boundaries and original encodings are preserved.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M13.3 and M24 remain In progress. This qualifies the provider subset
  only; full mixed file/comment send, live browser restart, source/reply
  navigation and visual approval remain incomplete.

```
<!-- /evidence-field -->

