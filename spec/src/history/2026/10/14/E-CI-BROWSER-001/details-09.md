---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 9

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
pnpm exec prettier --check .; pnpm check:architecture; pnpm check:ui-ownership (/tmp/jj-store-ci-format-final21.log, -architecture-final21.log, -ownership-final21.log)
```

**Notes**

Complete repository formatting passes; 1903 owned architecture modules pass; UI source ownership passes. Generated history uses its canonical bounded writer and the existing formatter exclusion.
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
pnpm spec:validate --only governance,summary,fileLimits,verification; pnpm check:progress; pnpm spec:first --base a7458b22d0c9e4913a46132a7d4fc83684b44511 --head @; CI_EVENT=pull_request GITHUB_HEAD_REF=codex/ci-portability-and-bootstrap CI_PR_AUTHOR=stevejuma JJ_STORE_CHECK_BASE=a7458b22d0c9e4913a46132a7d4fc83684b44511 JJ_STORE_CHECK_REVISION=@ node scripts/changeset-check.mjs (/tmp/jj-store-ci-*-final23.log)
```

**Notes**

Final corrected evidence: 3850 chapters, 198 requirements; 155 tasks and 807 records. Full PR-base spec-first comparison passes for 267 protected files and 31 canonical chapters. Initial coordinated publication uses its existing metadata/evidence gate. The original author Changesets patch note remains included. This static gate is repeated after recording results; hosted CI remains pending.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0, pnpm 10.32.1, JJ 0.45.1, Rust 1.99.0, published jj-acp 0.1.0;
macOS ARM64. Hosted Node 22.23.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ, manifests, WAL, receipts, APIs, WorkStore and companion formats
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair hosted architecture failure and current cold-browser regressions;
partition browser acceptance into four independent cold shards, an isolated
profile restart lane and the original real-RustFS Git parent. Complete cold and
served-browser suites escalate because shared fixture selection and production
freshness span independent consumers. Before: hosted governance failed at 307
counted controller lines; prior cold suite had 20 failed, 9 passed and 15
fixture skips. The staged workflow demonstrably skipped all expensive lanes
after that early failure. Shared native query-cache admission also requires the
complete affected native library suite and the original resource-limit HTTP
regression. The public limits and active-lock contracts are retained. Storybook
has three disjoint shards and served Playwright has two, with independent static
catalog and dual-engine Help jobs. Freshness, task-scrolling and prompt-context
changes require all directly affected UI consumers; root aggregate and unrelated
Rust suites remain out of scope under the repository test policy.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-005/015 retains idle reload and task reaction freshness. Native
  version-1 threads witnesses already include reaction roots; task views now
  consume that domain and defer hidden invalidation until restoration.
  JST-ARCH-005 adds isolated complete cold shards and the original real-RustFS
  Git parent. Working-copy extraction retains capability and session-switch
  guards. Tests use current menus, joined numerical controls, final CLI
  operations and explicit parents. Native publication, admission, schemas and
  persistence are unchanged. JST-UI-005 retains captured selected-project
  repository views while secondary summaries refresh; no submitted link identity
  is lost by an initial skeleton remount. The original minimal profile parent
  remains separate from seeded demo and original Git parent. Hidden-event checks
  begin after visible replay reads drain; data and draft assertions remain.
  JST-API-001/004 shared cache eviction now admits ordinary graph work by
  reclaiming only idle managed code namespaces; repository byte pressure is
  scoped to that repository and active locks are preserved. Quotas, identity,
  permission, cursor and publication contracts are unchanged. JST-UI-001/015
  context projection retains complete immutable Markdown and whole-section
  anchors in both layouts, including legacy chunks and original copy menus.
  JST-UI-012/015 retains saved task scroll before viewport teardown. Wire
  schemas remain unchanged under JST-API-006. Catalog committed-mutation
  adoption mirrors production internal replacement; ordinary navigation still
  honors draft guards. Historical refresh observes current-head witnesses while
  reusing pinned immutable reads. JST-UI-016 newly visible Capture recaptures
  current status/source interests, including notices admitted while closed. The
  browser fixture retains an open cancellable SSE body rather than immediately
  entering offline mode. JST-UI-005 loading labels name Threads or Sessions.
  JST-ARCH-005 served shards exclude duplicate Help files and isolate all output
  folders. JST-UI-001/010/015 retain native reply focus on the shared navigable
  item, current inbox/outer header geometry, captured standalone grammars and
  readable application-owned syntax palettes. Exact captured bytes, copy values,
  UTF-16 anchors, authorization, schemas, manifest publication and request
  identities are unchanged. Published Design Core 0.3.2 is consumed through the
  registry and its Svelte exports are explicitly compiled in cold catalog units.
  JST-UI-005/015 conversion observers do not track their own busy state during
  subscription construction; restored unknown receipts read once and remain idle
  until a matching event or explicit action, retaining exact retry intent.
  JST-ARCH-005 installs only the browser engines and archive tools used by each
  isolated CI lane. The thread request-count test uses a healthy open
  cancellable feed; its one-target and no-unrelated-read contract is unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; full hosted required checks, optimized
  native/container/registry publication and other milestone qualifications are
  not promoted.

```
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Fresh hosted required CI, native platform and full release qualification
  remain mandatory before merge/publication. Visual approval remains separate;
  no baselines were changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push this locally verified slice to PR #1, inspect the new hosted check graph
and retain M30 In progress until all release gates qualify.
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CI-BROWSER-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nzxrxuqo
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-004
- JST-GOV-007
- JST-API-001
- JST-API-004
- JST-UI-005
- JST-UI-010
- JST-UI-015
- JST-UI-016
- JST-UI-019
- JST-CLI-002
- JST-CLI-004
- JST-DOC-001
- JST-DOC-002
- JST-GIT-002
- JST-GIT-004
- JST-JJ-005
- JST-API-006
- JST-UI-001
- JST-UI-012
<!-- /evidence-field -->
