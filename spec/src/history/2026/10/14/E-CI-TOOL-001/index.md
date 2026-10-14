---
schema: 1
id: E-CI-TOOL-001
---

# E-CI-TOOL-001

<!-- evidence-field:id markdown -->
## Id

E-CI-TOOL-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
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

twuoppqm
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-setup-jj.mjs
- scripts/ci.test.mjs
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci.test.mjs#JJ extraction selects the exact safe archive member
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions PR CI 38627123282 and Pages 38627122962: node scripts/ci-setup-jj.mjs
```

**Notes**

GNU tar rejected the requested jj member because the verified upstream archives contain ./jj. Linux setup failed in five CI jobs and the Pages job; macOS BSD tar had accepted the shorter member.
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
pnpm release:check
```

**Notes**

Twelve release/CI regressions passed. Exact member selection accepts jj or ./jj, rejects duplicates/unsafe/missing roots, and retains checksum/version verification; all 20 static package checks passed.
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
docker run --rm (isolated fixture) node:24.13-bookworm-slim node ci-setup-jj.mjs
```

**Notes**

Actual Linux ARM64 GNU tar extraction, pinned archive checksum and official executable/version succeeded. The fixture was removed; installed JJ was untouched.
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
docker run --rm --platform linux/amd64 (isolated fixture) node:24.13-bookworm-slim node ci-setup-jj.mjs
```

**Notes**

Actual Linux x86-64 archive/executable passed the same verification under emulation. Node image digest a81a03dd965b4052269a57fac857004022b522a4bf06e7a739e25e18bce45af2 recorded; fixture removed.
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
node scripts/ci-setup-jj.mjs (isolated RUNNER_TEMP and GITHUB_PATH)
```

**Notes**

Actual macOS ARM64 BSD tar extraction and pinned executable/version passed again; isolated tool paths were removed.
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

Upstream JJ 0.45.1 source 7c41cdeb16b6b321c64e789a966b6adf723816a5; existing four SHA-256 pins unchanged; Node 24.10 macOS and 24.13 Linux container verification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, native object, API, package or persistence changes. Archive member names are selected exactly before extraction; checksum verification still precedes inspection/execution.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair GNU/BSD tar member selection in the private CI tool installer. Real verified platform archives qualify installation; overall hosted PR/full release gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push follow-up to PR 1 and require fresh hosted CI before merging; no direct main push.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 pinned, verified CI-tool admission remains unchanged.
  Installer now asks each tar implementation for the exact safe root member.
  Unsafe/duplicate shapes fail closed, existing digest/version checks remain
  required, and installed/deployed binaries are not replaced.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress; the previous hosted failure is retained and this
  narrow verified tool repair does not promote full CI or release gates.

```
<!-- /evidence-field -->

