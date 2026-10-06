---
schema: 1
id: E-GOV-003
---

# E-GOV-003 details 5

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

Baseline: 173 requirements; 615 evidence records and 142 task rows.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
cwd: /Users/stevejuma/code/lapismd/spec-validator
hasNote: true
noteEncoding: markdown

```

```shell
node --import tsx --import ./src/test-setup.ts --test src/validators/change-confirmations.spec.ts
```

**Notes**

Owning topic alternatives, fresh confirmations and explicit JJ ranges failed before implementation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
cwd: /Users/stevejuma/code/lapismd/spec-validator
hasNote: true
noteEncoding: markdown

```

```shell
npm exec --yes --package=deno@2.9.5 -- deno task check
```

**Notes**

Complete required package checks, 27 Deno tests, 45 Node tests, mdBook, runtime/architecture, spec-first and publint pass locally.
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
GitHub trusted publication v0.1.1
```

**Notes**

The runner lacked mdBook. A separate v0.1.2 release slice installs the required pinned tool without replacing the failed tag.
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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs scripts/spec/evidence-format.test.mjs
```

**Notes**

35 tests pass, retaining original completion, real-storage, named-test and merge-evidence checks.
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
node scripts/spec/migrate.mjs
```

**Notes**

615 records round-trip without data loss. 142 task rows and 20 verification families migrated; all authored pages stay below 400 lines/32768 bytes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
url: https://github.com/lapismd/spec-validator/actions/runs/37462389817
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions npm-publish.yml @ v0.1.2
```

**Notes**

All canonical tests passed; upload-artifact excluded the hidden .release directory. v0.1.3 includes the exact tarball without retagging v0.1.2.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
cwd: /Users/stevejuma/code/lapismd/spec-validator
hasNote: true
noteEncoding: markdown

```

```shell
node --import tsx --import ./src/test-setup.ts --test src/book-staging.spec.ts
```

**Notes**

Actual mdBook served stale content after a source edit before the watcher implementation.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
cwd: /Users/stevejuma/code/lapismd/spec-validator
hasNote: true
noteEncoding: markdown

```

```shell
node --import tsx --import ./src/test-setup.ts --test src/commands/search-results.spec.ts
```

**Notes**

Named-scope result paths were relative before the source-path resolver.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
cwd: /Users/stevejuma/code/lapismd/spec-validator
hasNote: true
noteEncoding: markdown

```

```shell
npm exec --yes --package=deno@2.9.5 -- deno task check
```

**Notes**

Final v0.1.4 source gate: 27 Deno tests, 47 Node tests including actual live mdBook edits/navigation/deletion, types, runtime/architecture, canonical spec-first, build and publint.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
url: https://github.com/lapismd/spec-validator/actions/runs/37463561721
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions npm-publish.yml @ v0.1.3
```

**Notes**

Publication and package gate succeeded; verification did not observe the registry version inside its 120-second window. Failed verification was rerun and passed.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
url: https://github.com/lapismd/spec-validator/actions/runs/37465194473
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions npm-publish.yml @ v0.1.4
```

**Notes**

Publication and all package tests succeeded; initial registry-version visibility check timed out. The same immutable tag qualified on rerun.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
cwd: /Users/stevejuma/code/lapismd/spec-validator
url: https://github.com/lapismd/spec-validator/releases/tag/v0.1.4
hasNote: true
noteEncoding: markdown

```

```shell
gh run rerun 37465194473 --failed
```

**Notes**

Final trusted release verification passed registry installation, signatures, Sigstore provenance and GitHub Release creation for v0.1.4.
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
node --test --test-name-pattern literal_evidence_markers scripts/spec/evidence-format.test.mjs
```

**Notes**

The new parser regression exposed literal record delimiters inside shell fences. Fence-aware parsing retains these commands exactly.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:spec
```

**Notes**

Initial native-JJ fixtures used the newer default colocated init. Explicit --no-colocate repaired the fixture; both actual native range/merge tests pass. Empty failed fixtures were cleaned up.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:build
```

**Notes**

The first repository-reference preprocessor assumed the older sections key. It now accepts actual mdBook 0.5 items and the legacy schema.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:build
```

**Notes**

A repository-directory link required a generated tracked-file index. The same real book build passes with closed transitive file and directory references.
<!-- /run -->
<!-- /evidence-field -->

