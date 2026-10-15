---
schema: 1
id: E-CI-ACTIONLINT-001
---

# E-CI-ACTIONLINT-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
taiki-e/install-action@1168f6810a3cb30a3514bf5756a3b7f0422c6df8 with tool: actionlint@1.7.12
```

**Notes**

Hosted governance job 116304462766 in run 38699434866: pinned action did not support actionlint, fell back to cargo-binstall, and failed with actionlint is not found (exit 76). All preceding governance gates passed. Prior local actionlint invocation used an installed binary and did not qualify this installer.
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
node --test scripts/ci.test.mjs
```

**Notes**

All 13 CI workflow/fixture regressions passed on Node 22.23.3.
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
pnpm exec prettier --check .github/workflows/ci.yml
```

**Notes**

Updated workflow formatting passed.
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
docker run --rm --platform linux/amd64 --mount type=bind,source=/tmp/jj-store-install-actionlint.sh,target=/test/install.sh,readonly --mount type=bind,source=/Users/stevejuma/code/lapismd/jj-store/.github,target=/repo/.github,readonly -w /repo ubuntu:22.04 bash -euc 'apt-get update -qq && apt-get install --no-install-recommends -y -qq curl ca-certificates shellcheck git > /tmp/packages.log; export RUNNER_TEMP=/tmp GITHUB_PATH=/tmp/tool-path; bash -e /test/install.sh; export PATH="$(cat "$GITHUB_PATH"):$PATH"; actionlint'
```

**Notes**

Exact extracted installer downloaded the official archive, verified its pinned SHA-256, extracted the root executable and printed actionlint 1.7.12 linux/amd64. Lint project discovery then rejected the deliberately minimal mounted fixture because it lacked a .git marker. This was a local fixture error; corrected the marker for the complete run.
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
docker run --rm --platform linux/amd64 --mount type=bind,source=/tmp/jj-store-install-actionlint.sh,target=/test/install.sh,readonly --mount type=bind,source=/Users/stevejuma/code/lapismd/jj-store/.github,target=/repo/.github,readonly -w /repo ubuntu:22.04 bash -euc 'apt-get update -qq && apt-get install --no-install-recommends -y -qq curl ca-certificates shellcheck git > /tmp/packages.log; mkdir .git; export RUNNER_TEMP=/tmp GITHUB_PATH=/tmp/tool-path; bash -e /test/install.sh; export PATH="$(cat "$GITHUB_PATH"):$PATH"; actionlint'
```

**Notes**

Executed the exact run block selected by jobs.governance.steps[name=Install checksum-pinned actionlint] from the changed workflow, with .github mounted read-only and an empty Git project marker. Official 2.3 MB archive checksum passed; actionlint 1.7.12 linux/amd64 installed in disposable RUNNER_TEMP, appended GITHUB_PATH, and linted every workflow with ShellCheck present. Container removed on exit.
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
pnpm check:architecture
```

**Notes**

1903 modules passed.
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
pnpm check:ui-ownership
```

**Notes**

Ownership validator passed.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

All four validators passed; 198 requirements.
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
pnpm check:progress
```

**Notes**

155 tasks and 809 records passed.
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
pnpm spec:first --base a7458b22d0c9e4913a46132a7d4fc83684b44511 --head fa25559f6e6ed7e8b5ff0728cd7db952c5bab310
```

**Notes**

Full PR gate passed: 269 protected files and 31 canonical chapters.
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
CI_EVENT=pull_request CI_PR_AUTHOR=stevejuma GITHUB_HEAD_REF=codex/ci-portability-and-bootstrap JJ_STORE_CHECK_BASE=a7458b22d0c9e4913a46132a7d4fc83684b44511 JJ_STORE_CHECK_REVISION=fa25559f6e6ed7e8b5ff0728cd7db952c5bab310 node scripts/changeset-check.mjs
```

**Notes**

Initial-publication release-intent gate passed; v0.1.0 tag remains absent.
<!-- /run -->

<!-- run -->
### Run 12

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

Installed host actionlint 1.7.12 also passed all workflows; real Linux bootstrap result is recorded separately.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CI_STAGE=early CI_PLAN='{}' CI_JOBS='{"format":{"result":"success"},"tooling":{"result":"success"},"governance":{"result":"success"}}' node scripts/ci-required.mjs
```

**Notes**

Fast aggregator accepted passing prerequisites after local governance completed.
<!-- /run -->

<!-- run -->
### Run 14

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

Additional isolated-change check required the workflow's exact mapped canonical distribution chapter, which the full PR already changed. Added the narrow installer description to architecture/23-public-distribution.md in this change; no gate was waived.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Final installer evidence and owning chapter: four validators, 3859 chapters and 198 requirements passed.
<!-- /run -->

<!-- run -->
### Run 16

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

Final selected-change evidence passed for 155 tasks and 809 records.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final isolated change passed with one protected workflow and its one mapped canonical chapter.
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

Node 22.23.3; actionlint 1.7.12 built with Go 1.26.1 for linux/amd64; Ubuntu 22.04 image digest sha256:5ec03bb3441e8b0bf3b4f9cd4629a1ae763010dc3035bb8da3ae6cf026486401 under local Docker amd64 emulation.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Workflow YAML only. Official archive actionlint_1.7.12_linux_amd64.tar.gz SHA-256 8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8 checked against GitHub release v1.7.12 metadata. No repository object or public API format changes.
<!-- /evidence-field -->
