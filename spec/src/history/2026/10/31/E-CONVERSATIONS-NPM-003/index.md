---
schema: 1
id: E-CONVERSATIONS-NPM-003
---

# E-CONVERSATIONS-NPM-003

<!-- evidence-field:id markdown -->
## Id

E-CONVERSATIONS-NPM-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-CLI-002
- JST-CLI-004
- JST-API-006
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-31
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/server_desktop.rs
- schemas/openapi.json
- web/src/lib/api/generated.ts
- docs/maintenance/coverage.json
- spec/src/current/contracts/cli/16-supervised-desktop-launch.md
- spec/src/current/contracts/api/14-generated-application-contracts.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/progress/M30/index.md
- docs/site/reference/api/code.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/api/work.md
- docs/site/reference/api/workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/src/server_desktop.rs#owner_bytes_keep_the_watchdog_open_until_eof
- crates/jj-store-cli/tests/desktop_supervision.rs#desktop_flags_require_supervision_and_a_pipe_before_startup
- scripts/contracts.test.mjs#openapi_document_validates_without_external_references
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
gh workflow run npm-publish-jj-conversations.yml --repo stevejuma/jj-store --ref main -f mode=attest
```

**Notes**

Run 41158072264 at main 95f54da1e151858c3e0305bad8a87ed8c3b590fe finishes with 28 successful jobs. Complete standard required passes, including 1048 Storybook cases, 114 application browser cases, 20 Help cases, one cold static catalog and all four ordinary Rust shards. All three native compatibility platforms, real RustFS and embedded storage lifecycle pass. Dedicated backend Clippy rejects the deprecated AsyncFd constructor; generated contracts reject byte drift. Their complete qualification gate fails, dependent backend/cold application suites skip, and all package/attestation/publication stages correctly skip. No publishable artifact or bundle exists.
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
pnpm validate:docker validation:rust clippy -p jj-store --all-targets --locked -- -D warnings
```

**Notes**

Exact local strict CLI baseline reproduces the deprecated AsyncFd::new diagnostic. Retained run jj-store-validation-00166e1b-41a7-4056-870b-8c6610612432. Original source is the empty 454eb467 change over main 95f54da1. An initial documented -- form was rejected before checks, then corrected; a concurrent docs request was rejected by the checkout lease and is rerun sequentially. Neither preflight invocation ran tests or provides acceptance.
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
pnpm validate:docker validation:rust clippy -p jj-store --all-targets --locked -- -D warnings
```

**Notes**

The safe owned Unix pipe Receiver adapter passes the identical strict command, run jj-store-validation-d490888f-3f2f-451f-9d28-a618ab848d19. No warning allowance, unsafe block or change to the crate unsafe-code ban.
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
pnpm validate:docker validation:rust test -p jj-store --test desktop_supervision --locked
```

**Notes**

Existing process admission test passes (one case). The separate real paired-RustFS EOF case remains explicitly ignored and does not count as acceptance. Retained run jj-store-validation-fd9a42f0-871d-4766-ae0b-c4fd13b805dc.
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
pnpm contracts:generate
```

**Notes**

Normal repository authoring command regenerates OpenAPI and TypeScript. Parsed old/new JSON documents are structurally identical across all 204 paths; only object property ordering changes. OpenAPI SHA256 changes from 56f14ff81dac3356e44a70fa8d249b1f159be17804ae9da560558aa35fb8f3b3 to 1bbb1c91971a47d781da983d9f17ea054186222e103c889cae3ff00990360612. Generated files are not edited by hand.
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
pnpm contracts:check
```

**Notes**

Strict byte drift checking passes on both normally generated files; 300 actual HTTP/SSE responses conform, all eight contract tooling cases pass and TypeScript contracts compile. Run jj-store-validation-383129f1-3280-4242-a32d-28c3b54e4ba4 records status passed and exitCode 0. Report collection prints Premature close, but the retained result/log prove completion; this warning does not substitute for acceptance.
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
pnpm validate:docker validation:rust test -p jj-store --lib server_desktop::tests::owner_bytes_keep_the_watchdog_open_until_eof --locked -- --exact
pnpm validate:docker validation:rust test -p jj-store --lib --locked
pnpm validate:docker validation:rust clippy -p jj-store -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

The exact real Unix-pipe watchdog case passes (one selected case, 22 intentionally filtered); it drains 128 bytes without finishing and completes on writer EOF. The full CLI library then passes all 23 cases with none ignored. All three original heavy backend packages pass strict all-target Clippy with the new test included. Reports: jj-store-validation-4a9722a7-efdf-4116-a3ea-e7ed1168cb0c, jj-store-validation-e25f8d0b-5e06-4571-a45d-18eed44e3c13, jj-store-validation-a6207d90-62b1-44aa-949b-692246bc0130.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

The first consuming documentation check rejects stale generated reference docs/site/reference/api/installation.md after normal OpenAPI property reordering. Run jj-store-validation-c3a73875-d1d7-4a88-a434-78e5fd325c01. This is an additional required projection refresh, not a public contract change. Normal pnpm docs:generate refreshes all references; source review fingerprints cover unchanged CLI, application and transport documentation.
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
pnpm docs:generate
```

**Notes**

Normal generator refreshes six HTTP reference projections to match the unchanged parsed OpenAPI contract. Its native command-reference compilation also succeeds on macOS ARM; this is source compilation, not packaged application qualification. Only this workspace idle native target cache is cleaned through dev:clean --target target after an explicit dry run; source, reports, other workspaces and application data remain retained.
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
pnpm docs:check
```

**Notes**

After normal reference regeneration, strict full documentation check/build passes: nine Mermaid diagrams rendered in both themes, 144-file/26735327-byte closure, valid links and anchors. Run jj-store-validation-019d05e4-8a04-48b6-b8ca-eab3147a1be7 records status passed and exitCode 0. The earlier single stale-reference failure is resolved; delayed report transport is distinct from completed product validation.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

PR #27 run 41163062578 passes formatting, tooling and static specification validation, but progress reports seven metadata diagnostics: three nonexistent test paths, three malformed test references, and a pass result with retained gaps. Correct the record to actual path#name references and pending status, preserving every unfinished release gap and the unmodified checker. Pages run 41163062645 also passes.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Draft head 50263a42 in PR run 41163752096 rejects modification of E003 across the earlier unmerged source commits. Preserve that full draft history under recovery/conversations-heavy-release-20261031 and recreate the final reviewed tree as one coherent change on unchanged main 95f54da1, so E003 is fresh once for the source slice. All old commits/payloads remain recoverable, canonical main history is untouched, and no checker or completion gate is changed. The same attempt also reports missing fresh evidence coverage for the newly added owning documentation chapter; the coherent change includes that path in its fresh E003 record.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

At d5688317, PR run 41164874752 passes progress (186 tasks/997 records), formatting, tooling and Pages, but rejects an unchanged-contract confirmation with overall pending status. The record had mixed completed source acceptance and uncompleted publication work. Bound this passing record to the completed source repair and keep all remaining release gates explicitly in M30; none is waived. Retain d568 under recovery/conversations-heavy-release-d568-20261031 and create this final source/evidence slice coherently over unchanged main.
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

Locked Tokio 1.54.1; jj-store private application/native versions and jj-conversations 0.1.0 unchanged. Root pnpm 10.32.1 and Node-only Spec Validator 0.2.0 remain in use.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/RPC DTO, wire, native storage, authorization, CLI flag or package version change. Strict generated-byte comparison remains enabled.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed source acceptance for the safe desktop owner-pipe adapter and the normally regenerated OpenAPI/TypeScript/reference projections only. Exact real-pipe and full CLI unit tests, original strict heavy Clippy, actual HTTP/SSE captures and the complete documentation build pass with zero remaining failures in this source scope. Protected PR merge, full main release qualification, immutable artifact/provenance, actual npm publication, registry verification and GitHub release remain separate blocking M30 work; this record makes no package-availability or whole-release qualification claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

M30 retains the pending protected PR #27 merge, fresh complete heavy/client qualification, exact artifact and actual npm/registry/GitHub release gates. Append separate receipts when those operations complete.
<!-- /evidence-field -->

