---
schema: 1
id: E-CI-PIPELINE-001
---

# E-CI-PIPELINE-001

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

snxrpykx
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99.0; JJ 0.45.1; jj-acp 0.1.0; pnpm 10.32.1; Node 24.10.0 macOS ARM64 and 24.13 Linux container; hosted Node 22.23.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, manifest/WAL/receipts, API and companion formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local targeted evidence and CI repairs do not
  promote hosted, optimized archive, container, registry, provider or full
  release qualification.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CI-PIPELINE-001
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-009
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .config/nextest.toml
- .github/actions/setup/action.yml
- .github/workflows/ci-heavy.yml
- .github/workflows/ci.yml
- README.md
- scripts/automation-governance.test.mjs
- scripts/ci-browser.mjs
- scripts/ci-build.mjs
- scripts/ci-required.mjs
- scripts/ci-setup-acp.mjs
- scripts/ci-storage.mjs
- scripts/ci.test.mjs
- scripts/dev-agent-probe.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-demo-repository.test.mjs
- scripts/dev-demo-server.mjs
- scripts/dev-demo.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci.test.mjs#failed early or ordinary lanes cannot start expensive
  qualification
- scripts/automation-governance.test.mjs#bot PRs skip only change-specific
  governance
- scripts/dev-demo-repository.test.mjs#disposable server cleanup preserves an
  already-exited startup failure
- scripts/dev-agent-runtime.test.mjs#version 3 alone does not qualify a helper
  that rejects prepared context
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
pnpm test:release
```

**Notes**

17 release/CI regressions passed, including failure/cancellation/unselected scheduling, runtime admission, same-run artifact wiring and retained bot governance. Shared workflow/tooling changes justify the complete release/governance/spec tooling suites.
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
pnpm test:governance
```

**Notes**

71 passed, including live-process cleanup and manifest generation bounds.
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
node --test scripts/dev-demo-repository.test.mjs scripts/dev-agent-runtime.test.mjs
```

**Notes**

Nine direct startup/seed tests passed; cleanup no longer masks an already-exited service failure.
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
node scripts/ci-setup-acp.mjs (isolated RUNNER_TEMP/GITHUB_ENV)
```

**Notes**

Real macOS pinned archive digest, extraction, version and prepared-context checks passed without replacing installed tools.
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
docker run --rm --platform linux/amd64 -e CI=true -e RUNNER_TEMP=/tmp/acp -e GITHUB_ENV=/tmp/ci-env node:24.13-bookworm-slim node /scripts/ci-setup-acp.mjs
```

**Notes**

Actual checksum-pinned Linux x86-64 ACP admitted without installed Codex or opening an agent session.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CI=true real-browser ACP prepared-context preflight (before stream repair)
```

**Notes**

Immediate EOF canceled the second queued request, producing a record instead of session_not_open and a false unsupported-capability error. Keep input open until both terminal replies; retain five-second and 64-KiB bounds.
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
CI=true node <probePreparedContext against actual jj-acp 0.1.0, ten iterations>
```

**Notes**

Ten real published ACP probes passed after the stream lifetime repair.
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
actionlint
```

**Notes**

Both root and reusable workflows and action inputs passed.
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
CI=true node --input-type=module (ten sequential probePreparedContext calls against /tmp/jj-acp-0.1.0-aarch64-apple-darwin/jj-acp)
```

**Notes**

Actual published helper passed all ten session-not-open probes; no session or adapter opened.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/real-browser-acceptance.mjs (disposable ci-cold workspace, explicit binary/runtime/ACP paths and port 7530)
```

**Notes**

Complete fresh cold run: 9 passed, 15 skipped, 20 failed. Setup used real RustFS and current compact UI; owned service was stopped and fixture removed. Runtime preflight/startup succeeded. Broader browser fixture/contract failures remain explicit and block merge; no assertion or release gate waived.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Fresh hosted staged workflow qualification and remaining cold/served-browser
  acceptance repairs.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Early formatting/tooling/governance gates, ordinary checks before a reusable heavy pipeline, fail-fast matrices/shards, artifact reuse, pinned ACP and bounded fixture preflight/cleanup. Product API/native formats and interactive state remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete fresh browser/resource evidence, commit this slice and push only to existing PR 1; keep required gate and M30 incomplete until hosted success.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and JST-GOV-007 CI control/qualification ordering are
  tightened by mapped architecture/governance chapters. JST-GOV-009 private
  preflight now waits for both bounded terminal responses before EOF; disposable
  cleanup preserves original startup errors. Normal development builds,
  authority, deployed services and data remain unchanged.

```
<!-- /evidence-field -->

