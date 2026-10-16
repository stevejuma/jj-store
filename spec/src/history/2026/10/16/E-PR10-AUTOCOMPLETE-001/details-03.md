---
schema: 1
id: E-PR10-AUTOCOMPLETE-001
---

# E-PR10-AUTOCOMPLETE-001 details 3

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
gh run view 38930175256 --repo lapismd/design-core --json status,conclusion,jobs
```

**Notes**

design-core-version-ci-100: exit 0. All 11 protected version PR checks passed. PR #30 squash merged as 9560a62dc77ef1a5fff697213fd1e228235fde5b without bypass; only expected 0.5.1 version/changelog/Changeset edits.
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
npm view @lapismd/design-core@0.5.1 version dist --json
```

**Notes**

registry-artifact-early-100: exit 1. Initial registry lookup immediately after successful publication returned E404. No lockfile or dependency change was made. Waiting for the existing clean-install/provenance gate and registry visibility.
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
npm view @lapismd/design-core@0.5.1 version dist.integrity --prefer-online --json (16:32 UTC)
```

**Notes**

registry-artifact-recheck-100: exit 1. E404 while registry metadata remained cached at latest 0.5.0. No dependency/lockfile change; publication log confirms trusted publish and Sigstore entry. Existing provenance gate remains pending.
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
npm view @lapismd/design-core@0.5.1 version dist.integrity --prefer-online --json (16:33 UTC)
```

**Notes**

registry-artifact-recheck-2-100: exit 1. E404 while registry metadata remained cached at latest 0.5.0. No dependency/lockfile change; publication log confirms trusted publish and Sigstore entry. Existing provenance gate remains pending.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 38931424459 --repo lapismd/design-core --json status,conclusion,jobs
```

**Notes**

design-core-publication-first-100: exit 1. Exact artifact build and trusted publication passed, but clean-install/provenance verification failed its existing 30 visibility checks. Release notes skipped. No republish, version change, lockfile edit or gate weakening. Full failed log retained.
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
npm view @lapismd/design-core@0.5.1 version dist --prefer-online --json; npm pack @lapismd/design-core@0.5.1 --ignore-scripts --pack-destination .local/main-qualification-2026-10-16/registry-pack-100 --json
```

**Notes**

registry-artifact-visible-100: exit 1. Version metadata became visible with SHA-512 matching the approved artifact, but its official tarball URL returned E404. Original command not modified. Holding consumer dependency changes until archive and provenance checks pass.
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
Download official metadata dist.tarball and verify registry SHA-512, approved release SHA-256 and qualified autocomplete source SHA-256
```

**Notes**

registry-artifact-100: exit 0. Exact registry bytes match approved release artifact and completely qualified source. Earlier metadata and tarball E404 propagation attempts remain recorded.
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
gh run rerun 38931424459 --repo lapismd/design-core --failed; observe verification attempt 2
```

**Notes**

design-core-publication-second-100: exit 1. Metadata visibility passed, then official tarball install returned E404. Exact failed log retained. The tarball subsequently returned 200 locally with the approved artifact SHA-256 and registry SHA-512, establishing a changed external state before retrying only failed verification.
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
pnpm release:verify /Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/design-core-release-100/release-manifest.json /Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/npm-signatures-local-100.json
```

**Notes**

design-core-local-verify-100: exit 0, 23.5 seconds. Original released-package clean install and npm signature/provenance verifier, unchanged source and policy.
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
pnpm add --save-dev @lapismd/design-core@^0.5.1
```

**Notes**

registry-install-100: exit 0, 3.89 seconds. Normal released registry dependency; original upstream and local signature/provenance verification passed before installation.
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
gh run view 38931424459 --repo lapismd/design-core --json status,conclusion,jobs
```

**Notes**

design-core-publication-100: exit 0. Final attempt 3 passes every job, including clean install, integrity/signature/provenance and release notes. Attempts 1/2 propagation failures remain recorded; no republish or changed policy.
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
Verify installed @lapismd/design-core version and autocomplete source SHA-256 against the official registry archive and approved artifact
```

**Notes**

registry-integrity-100: exit 0. Installed public registry 0.5.1; autocomplete bytes exactly match the fully qualified packed source. Minimum ^0.5.1; no local source or tarball manifest references.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/released-workflows-100.xml
```

**Notes**

released-workflows-100: exit 0, 34.51 seconds. 
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

released-web-check-100: exit 0, 12.84 seconds. 
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
pnpm catalog:check
```

**Notes**

released-catalog-check-100: exit 0, 17.69 seconds. 
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
pnpm web:test
```

**Notes**

released-web-units-100: exit 0, 2.09 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm catalog:test
```

**Notes**

released-catalog-units-100: exit 0, 16.45 seconds. 
<!-- /run -->

<!-- run -->
### Run 18

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

released-ownership-100: exit 0, 2.7 seconds. 
<!-- /run -->
<!-- /evidence-field -->

