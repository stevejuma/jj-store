---
schema: 1
id: E-REMOTE-TEST-PR-001
---

# E-REMOTE-TEST-PR-001

<!-- evidence-field:id markdown -->
## Id

E-REMOTE-TEST-PR-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
- JST-GOV-007
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/remote-test-audit-repairs.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/09-catalog-validation.md
- spec/src/current/progress/M30/index.md
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
jj git fetch --remote origin-https
```

**Notes**

Fetched remote main 5ba2fdad10f0a4b2008aba1398ef27ff12a07863. The prior audit 4c3f25615b4e34c13d28c83d21cd2b6e33224bd1 was not published; no open PR existed for it.
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
jj rebase -s vvorvtzm -o main@origin-https; jj squash --into vvorvtzm --use-destination-message -- <three resolved documentation paths>
```

**Notes**

Three documentation-only conflicts are resolved by retaining the union of M30 evidence and November 1 navigation. Remote E-DEPENDENCY-AUTOMATION-001/002/003 and the complete immutable E-LOCAL-TEST-AUDIT-001 record remain intact. Current main Mira ~0.1.4 and the existing Bits UI compatibility patch are preserved; original Docker evidence used Mira 0.1.2 and is not relabeled as integrated-tree qualification.
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
pnpm view @lapismd/spec-validator version; pnpm exec spec-validator doctor --json; pnpm spec:search JST-ARCH-005
```

**Notes**

Latest and installed Deno-free validator are 0.2.0. Doctor and QMD pass with existing wrapper-alias warnings.
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
pnpm exec prettier --check spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/governance/09-catalog-validation.md .changeset/remote-test-audit-repairs.md
```

**Notes**

Changed authored documents and release-intent note pass. Existing Docker detail is linked through its owning governance topic to retain the 400-line architecture budget without weakening source/toolchain/comparison rules.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:incomplete_attempts yaml -->
## Incomplete attempts

```yaml
- command: jj git fetch --remote origin
  status: interrupted
  note: SSH fetch waited without progress and was stopped; the existing HTTPS
    remote succeeded.
- command: Bounded SSH probe to configured remote Docker host
  status: incomplete
  note: Connection timed out after 12 seconds. No new Docker acceptance is
    claimed. Hosted PR CI will qualify the integrated source and diagnose
    failures.

```
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Publish and verify the exact PR branch head, then complete hosted required and
  selected CI on the integrated remote-main tree.
- Four original individual quarantines retain their owner, disabled coverage and
  re-enable gates; they do not qualify missing acceptance or independent
  release/platform/provider gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Current base 5ba2fdad10f0a4b2008aba1398ef27ff12a07863; Mira ~0.1.4, Design Core ^0.7.0, root Bits UI 2.19.5 plus the existing transitive 2.20.2 patch; pnpm 10.32.1; spec-validator 0.2.0. Original audit tool/source receipts remain immutable.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new API, native identity, storage, WAL or transport change. Preserve remote dependencies and existing patch authority. Add patch release intent for the already-qualified Rust repairs; this does not publish a release.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The user requests synchronization, push, an open PR and investigation/fixing of CI failures, including narrow documented quarantine for confirmed flakes. Rebase preserves current main instead of reverting its dependency/workflow changes. Only documentation navigation, validation-governance placement and release-intent bookkeeping are added in this slice. Hosted integrated-tree qualification remains distinct from the prior passing Docker audit.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push a dedicated branch, open a ready PR, verify GitHub head identity and monitor all selected/required checks and review comments; reproduce failures before classification and repair or individually quarantine with new evidence.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Existing validation toolchain/comparison details move under the linked
  governance chapter to keep the integrated architecture chapter within its
  authored budget. Patch release intent is explicit. Product contracts, required
  CI contexts, resource limits and the four existing individual quarantine
  policies remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Synchronization, an open PR and prior enabled
  Docker passes do not qualify the integrated hosted head or independent
  release/provider/platform gates.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change usxzqxmtvlsvusqkvqrxtxymxpwnnqko
<!-- /evidence-field -->

