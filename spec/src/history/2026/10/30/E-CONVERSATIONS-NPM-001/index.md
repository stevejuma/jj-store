---
schema: 1
id: E-CONVERSATIONS-NPM-001
---

# E-CONVERSATIONS-NPM-001

<!-- evidence-field:id markdown -->
## Id

E-CONVERSATIONS-NPM-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-ARCH-007
- JST-GOV-011
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/npm-publish-jj-conversations.yml
- package.json
- pnpm-lock.yaml
- packages/jj-conversations/package.json
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- docs/site/technical/conversations.md
- spec/src/current/contracts/documentation/02-requirements.md
- docs/maintenance/coverage.json
- scripts/conversations-npm-artifact.mjs
- scripts/conversations-npm-plan.mjs
- scripts/conversations-npm.mjs
- scripts/conversations-provenance.mjs
- scripts/conversations-npm.test.mjs
- scripts/validation/lanes.json
- scripts/ci.test.mjs
- scripts/fixtures/design-core-0.7.1-provenance.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/conversation-client.test.mjs#conversation package exports only its
  built independent client boundary
- scripts/conversations-npm.test.mjs#real Sigstore verification rejects a valid
  foreign workflow and payload tampering
- scripts/ci.test.mjs#parallel cold shards use disjoint result directories and CI stops on failure
- scripts/ci.test.mjs#browser orchestration selects one suite per lane and forwards real shard arguments
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
pnpm --filter @lapismd/jj-conversations test
```

**Notes**

Eight pure client tests pass after a clean frozen registry install; TypeScript build and test types also pass.
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
pnpm exec spec-validator doctor
```

**Notes**

Ten checks pass, one expected wrapper-alias warning. The workspace QMD SQLite native dependency was rebuilt normally for Node 24 after the initial native-module failure.
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
actionlint .github/workflows/npm-publish-jj-conversations.yml
```

**Notes**

The exact newly added workflow passes lint. Full reused qualification remains mandatory.
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
node --test scripts/conversation-client.test.mjs scripts/spec/*.test.mjs
```

**Notes**

All 32 source/governance cases pass, including the independent packed consumer; archive SHA256 efb243f47210df39b0c29d5a390153438faedf16c0435d3215e11be0aa2a4488. No Rust test or product UI lane is substituted for complete publication qualification.
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
node --test scripts/conversations-npm.test.mjs
```

**Notes**

Seven release guards pass: immutable source/run/attempt and bytes, complete heavy/package-gate admission, exact scope, registry provenance and real public-library signer verification. The public Design Core 0.7.1 bundle is 12,005 bytes, SHA256 e68da538522ac91ccf86a1ca0472c71f3895e79a1adf997e27155911c07d7c60. It contains public provenance and no signing/npm credential. Sigstore 4.1.1 uses its two-argument verification overload plus exact returned issuer/SAN assertions.
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
node --test scripts/conversation-client.test.mjs scripts/spec/*.test.mjs scripts/conversations-npm.test.mjs
```

**Notes**

All 39 source, specification and release cases pass, including the real packed consumer and foreign-signer/tamper regression. The client package also reruns all eight tests and TypeScript checks successfully.
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
pnpm docs:review conversation-client "Reviewed public client publication metadata and the scoped main/tag workflow, complete heavy qualification, immutable artifact/run reuse, exact Sigstore issuer and workflow identity, owner bootstrap, and actual registry/consumer verification; private application and Rust publication stay separate."
```

**Notes**

The technical manual was read and updated before review. Coverage now fingerprints the client manifest, release workflow and all four owning release modules. A subsequent unmodified review validation passes; no source fingerprint was blindly refreshed.
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
node --test --test-name-pattern 'npm provenance' scripts/conversations-npm.test.mjs
```

**Notes**

The exact npm-schema regression changed from one failure to zero. The predicate
now includes `internalParameters.github` with the admitted workflow-dispatch
event and exact numeric repository/owner ID strings. Foreign IDs are rejected.
The first wider source/spec run was invalidated by subsequent source edits;
its revision-pinned failure is retained. The stable-source rerun passes all 39
cases with no failures. The isolated tarball rehearsal contains 12 files and
8,993 bytes; preparation/loading verifies its immutable manifest. Its synthetic
run identity supplies no heavy qualification or publication authority.
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

The current-source Docker run passes eight validators, 4,540 chapters and 211
requirements. Both governing 400-line pages retain their limit. The earlier
obsolete aggregate snapshot was collected then explicitly interrupted; it
does not establish any full-suite qualification.
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
JJ_STORE_CI_REPORTS=1 node --test --test-name-pattern 'parallel cold shards|browser orchestration selects' scripts/ci.test.mjs
```

**Notes**

The full Docker release gate first reports 65 passes and two failures. Its
reporting environment leaked into ordinary-mode CI fixtures. This exact target
reproduces both failures before edits and passes both afterward. Fixtures now
choose their own reporting profile; all existing assertions remain and exact
heavy Storybook/catalog reporter arguments are additionally covered. Production
reporting and test coverage are unchanged.
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
pnpm release:check
pnpm docs:check
pnpm spec:first --base 8e418ec9221c5b0a7871f86e2621a614272abdae
pnpm check:progress
```

**Notes**

All 67 release/CI cases pass in Docker, including the seven scoped npm guards
and both repaired reporting fixtures. All 28 Cargo package metadata checks pass;
coordinated Cargo versions remain 0.1.0 with no Rust publication.
The strict documentation build passes: nine diagrams, 144 closure files and
all links/anchors valid. Changed source/manual fingerprints and formatting pass.
Spec-first passes for 14 protected files and four canonical chapters. Its first
invocation omitted a base while the worker selected an exact head; rerunning
against the actual immutable parent supplies the required explicit pair.
The Docker progress ledger passes 186 tasks and 995 evidence records; M30's
status and its complete hosted/publication gates remain unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Finish source workflow/governance checks and protected PR checks.
- Complete the unchanged full manual qualification on the immutable source.
- Publish the exact approved artifact and verify registry integrity, provenance
  and clean consumer.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

@lapismd/jj-conversations 0.1.0; Spec Validator 0.2.0; Sigstore 4.1.1; Node 24; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing conversation requests and wire/native storage formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add the missing public client npm workflow with canonical main/tag admission, unchanged complete heavy qualification, exact tested tarball, protected OIDC publication and detached signed owner bootstrap. Reuse requires a coherent successful canonical run/attempt with both complete qualification and package gates. Actual registry bytes, signatures, provenance, clean consumer and GitHub release remain required. The current source change is kuounqmz. The private application and all Rust Git pins remain unchanged. Validator discovery removes its old Deno dependency. The evidence helper refreshed unrelated navigation; those unrelated generated edits were restored and only the new evidence navigation is included.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish focused source gates, merge the reviewed PR after required checks, qualify the immutable main source and publish only the npm client.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now records independent npm publication, exact signed
  bootstrap and admitted artifact reuse while preserving every complete manual
  qualification gate. JST-DOC-002 source fingerprints retain reviewed manuals.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress until full qualification and public artifact
  receipts are complete; this source preparation does not mark it Done.

```
<!-- /evidence-field -->

